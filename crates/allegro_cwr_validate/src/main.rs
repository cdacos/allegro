use std::fs;
use std::io::{self, Read};
use std::process;
use std::time::Instant;

use log::{error, info};

#[derive(Default)]
struct Config {
    input_files: Vec<String>,
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
    let temp_file = std::env::temp_dir().join("cwr_validate_stdin.tmp");
    if let Err(e) = fs::write(&temp_file, &buffer) {
        eprintln!("Error writing temporary file: {}", e);
        process::exit(1);
    }

    let temp_path = temp_file.to_string_lossy();
    let result = allegro_cwr_validate::check_roundtrip_integrity(&temp_path, config.cwr_version);
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
        "Successfully checked {} CWR records from stdin in {:.2?}",
        allegro_cwr::format_int_with_commas(count as i64),
        elapsed_time
    );
}

fn process_files(config: &Config, start_time: Instant) {
    let mut total_count = 0u64;
    let mut processed_files = 0usize;
    let mut failed_files = Vec::new();

    for filename in &config.input_files {
        println!("Validating CWR file: {}", filename);

        let result = allegro_cwr_validate::check_roundtrip_integrity(filename, config.cwr_version);

        match result {
            Ok(count) => {
                total_count += count as u64;
                processed_files += 1;
                if config.input_files.len() > 1 {
                    println!("{}: {} records", filename, allegro_cwr::format_int_with_commas(count as i64));
                }
            }
            Err(e) => {
                eprintln!("Error processing file '{}': {}", filename, e);
                failed_files.push(filename.clone());
            }
        }

        println!();
    }

    let elapsed_time = start_time.elapsed();

    if config.input_files.len() == 1 {
        if !failed_files.is_empty() {
            eprintln!("Failed to process {} file(s): {}", failed_files.len(), failed_files.join(", "));
            process::exit(1);
        }

        println!(
            "Checked {} CWR records from '{}' in {:.2?}",
            allegro_cwr::format_int_with_commas(total_count as i64),
            &config.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Checked {} CWR records from {} files in {:.2?}",
            allegro_cwr::format_int_with_commas(total_count as i64),
            processed_files,
            elapsed_time
        );
    }
}

fn print_help() {
    eprintln!("Usage: cwr-validate [OPTIONS] [FILES...]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  [FILES...]          CWR files to check for validity. If no files specified, reads from stdin");
    eprintln!();
    eprintln!("Options:");
    eprintln!("      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cwr-validate input.cwr                    # Check single CWR file");
    eprintln!("  cwr-validate *.cwr                        # Check multiple CWR files");
    eprintln!("  cwr-validate --cwr 2.2 input.cwr          # Force CWR version 2.2");
    eprintln!("  cat input.cwr | cwr-validate              # Process CWR data from stdin");
    eprintln!("  find . -name '*.cwr' | xargs cwr-validate # Process all CWR files recursively");
}
