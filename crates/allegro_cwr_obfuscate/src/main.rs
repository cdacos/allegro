use std::fs;
use std::io::{self, Read};
use std::process;
use std::time::Instant;

use log::{error, info};

#[derive(Default)]
struct Config {
    input_files: Vec<String>,
    output_filename: Option<String>,
    cwr_version: Option<f32>,
    read_stdin: bool,
}

fn parse_args() -> Result<Config, String> {
    let mut config = Config::default();
    let mut parser = lexopt::Parser::from_env();

    while let Ok(Some(arg)) = parser.next() {
        match arg {
            lexopt::Arg::Long("cwr") => {
                let version_str = get_value(&mut parser, "cwr")?;
                let version: f32 = version_str
                    .parse()
                    .map_err(|_| format!("Invalid CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version_str))?;

                if ![2.0, 2.1, 2.2].contains(&version) {
                    return Err(format!("Unsupported CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version));
                }

                config.cwr_version = Some(version);
            }
            lexopt::Arg::Short('o') | lexopt::Arg::Long("output") => {
                let output_filename = get_value(&mut parser, "output")?;
                config.output_filename = Some(output_filename);
            }
            lexopt::Arg::Value(val) => {
                config.input_files.push(val.to_string_lossy().to_string());
            }
            lexopt::Arg::Short('h') | lexopt::Arg::Long("help") => {
                print_help();
                process::exit(0);
            }
            _ => {
                return Err("Unknown argument".to_string());
            }
        }
    }

    if config.input_files.is_empty() {
        config.read_stdin = true;
    }

    Ok(config)
}

fn get_value(parser: &mut lexopt::Parser, arg_name: &str) -> Result<String, String> {
    parser
        .value()
        .map(|val| val.to_string_lossy().to_string())
        .map_err(|e| format!("Missing value for --{}: {}", arg_name, e))
}

fn main() {
    env_logger::init();

    let config = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            error!("Configuration error: {}", e);
            print_help();
            process::exit(1);
        }
    };

    let start_time = Instant::now();

    if config.read_stdin {
        process_stdin(&config, start_time);
    } else {
        process_files(&config, start_time);
    }
}

fn process_stdin(config: &Config, start_time: Instant) {
    info!("Reading CWR data from stdin");

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        eprintln!("Error reading from stdin: {}", e);
        process::exit(1);
    }

    // Write stdin content to a temporary file for processing
    let temp_file = std::env::temp_dir().join("cwr_obfuscate_stdin.tmp");
    if let Err(e) = fs::write(&temp_file, &buffer) {
        eprintln!("Error writing temporary file: {}", e);
        process::exit(1);
    }

    let temp_path = temp_file.to_string_lossy();
    
    let result = if let Some(output_file) = &config.output_filename {
        // Write to specified output file
        allegro_cwr_obfuscate::process_cwr_obfuscation(&temp_path, Some(output_file), config.cwr_version)
    } else {
        // Write to stdout
        use std::io;
        allegro_cwr_obfuscate::process_cwr_obfuscation_to_writer(&temp_path, io::stdout(), config.cwr_version)
    };
    let elapsed_time = start_time.elapsed();

    // Clean up temporary file
    let _ = fs::remove_file(&temp_file);

    let count = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error processing stdin after {:.2?}: {}", elapsed_time, e);
            process::exit(1);
        }
    };

    println!(
        "Successfully obfuscated {} CWR records from stdin in {:.2?}",
        allegro_cwr::format_int_with_commas(count as i64),
        elapsed_time
    );
}

fn process_files(config: &Config, start_time: Instant) {
    let mut total_records = 0;
    let mut files_processed = 0;

    for input_filename in &config.input_files {
        info!("Obfuscating CWR file: {}", input_filename);

        let output_filename = if config.input_files.len() > 1 && config.output_filename.is_some() {
            // For multiple files with explicit output, append file index
            let base_name = config.output_filename.as_ref().unwrap();
            Some(format!("{}.{}", base_name, files_processed + 1))
        } else {
            config.output_filename.clone()
        };

        let result = allegro_cwr_obfuscate::process_cwr_obfuscation(
            input_filename,
            output_filename.as_deref(),
            config.cwr_version,
        );

        match result {
            Ok(count) => {
                total_records += count;
                files_processed += 1;
                info!("Processed {} records from '{}'", count, input_filename);
            }
            Err(e) => {
                eprintln!("Error processing file '{}': {}", input_filename, e);
                process::exit(1);
            }
        }
    }

    let elapsed_time = start_time.elapsed();
    info!("Processing completed");

    if files_processed == 1 {
        println!(
            "Successfully obfuscated {} CWR records from '{}' in {:.2?}",
            allegro_cwr::format_int_with_commas(total_records as i64),
            &config.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Successfully obfuscated {} CWR records from {} files in {:.2?}",
            allegro_cwr::format_int_with_commas(total_records as i64),
            files_processed,
            elapsed_time
        );
    }
}

fn print_help() {
    eprintln!("Usage: cwr-obfuscate [OPTIONS] [FILES...]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  [FILES...]          CWR files to obfuscate. If no files specified, reads from stdin");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output file path (defaults to <input>.obfuscated or stdout for stdin)");
    eprintln!("      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Obfuscates sensitive information in CWR files while maintaining referential integrity.");
    eprintln!("Names, titles, IPIs, and work numbers are consistently mapped throughout the file.");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cwr-obfuscate input.cwr                       # Obfuscate single CWR file");
    eprintln!("  cwr-obfuscate *.cwr                           # Obfuscate multiple CWR files");
    eprintln!("  cwr-obfuscate -o obfuscated.cwr input.cwr     # Specify output file");
    eprintln!("  cat input.cwr | cwr-obfuscate                 # Process CWR data from stdin");
    eprintln!("  find . -name '*.cwr' | xargs cwr-obfuscate    # Process all CWR files recursively");
}
