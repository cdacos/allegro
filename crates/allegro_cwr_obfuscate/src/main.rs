use std::process;
use std::time::Instant;

use allegro_cwr_cli::{
    get_output_filename_with_default_extension, get_value, init_logging_and_parse_args, process_stdin_with_temp_file,
    BaseConfig,
};
use log::info;

#[derive(Default)]
struct Config {
    base: BaseConfig,
    output_filename: Option<String>,
}

fn parse_args() -> Result<Config, String> {
    let mut config = Config::default();
    let mut parser = lexopt::Parser::from_env();

    while let Ok(Some(arg)) = parser.next() {
        match arg {
            lexopt::Arg::Long("cwr") => {
                let version_str = get_value(&mut parser, "cwr")?;
                config.base.set_cwr_version(&version_str)?;
            }
            lexopt::Arg::Short('o') | lexopt::Arg::Long("output") => {
                let output_filename = get_value(&mut parser, "output")?;
                config.output_filename = Some(output_filename);
            }
            lexopt::Arg::Value(val) => {
                config.base.add_input_file(val.to_string_lossy().to_string());
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

    config.base.finalize();
    Ok(config)
}

fn main() {
    let config = init_logging_and_parse_args(|| {
        parse_args().inspect_err(|_| {
            print_help();
        })
    });

    let start_time = Instant::now();

    if config.base.read_stdin {
        process_stdin(&config, start_time);
    } else {
        process_files(&config, start_time);
    }
}

fn process_stdin(config: &Config, start_time: Instant) {
    process_stdin_with_temp_file(
        "cwr_obfuscate_stdin",
        |temp_path, start_time| {
            let result = if let Some(output_file) = &config.output_filename {
                allegro_cwr_obfuscate::process_cwr_obfuscation(temp_path, Some(output_file), config.base.cwr_version)
            } else {
                use std::io;
                allegro_cwr_obfuscate::process_cwr_obfuscation_to_writer(
                    temp_path,
                    io::stdout(),
                    config.base.cwr_version,
                )
            };
            let elapsed_time = start_time.elapsed();

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
        },
        start_time,
    );
}

fn process_files(config: &Config, start_time: Instant) {
    let mut total_records = 0;
    let mut files_processed = 0;
    let mut failed_files = Vec::new();

    for input_filename in &config.base.input_files {
        info!("Obfuscating CWR file: {}", input_filename);

        let output_filename = get_output_filename_with_default_extension(
            config.output_filename.as_deref(),
            input_filename,
            config.base.input_files.len(),
            files_processed,
            "obfuscated",
        );

        let result = allegro_cwr_obfuscate::process_cwr_obfuscation(
            input_filename,
            output_filename.as_deref(),
            config.base.cwr_version,
        );

        match result {
            Ok(count) => {
                total_records += count;
                files_processed += 1;
                info!("Processed {} records from '{}'", count, input_filename);
                if config.base.input_files.len() > 1 {
                    println!("{}: {} records", input_filename, allegro_cwr::format_int_with_commas(count as i64));
                }
            }
            Err(e) => {
                eprintln!("Error processing file '{}': {}", input_filename, e);
                failed_files.push(input_filename.clone());
            }
        }

        println!();
    }

    let elapsed_time = start_time.elapsed();
    info!("Processing completed");

    if config.base.input_files.len() == 1 {
        if !failed_files.is_empty() {
            eprintln!("Failed to process {} file(s): {}", failed_files.len(), failed_files.join(", "));
            process::exit(1);
        }

        println!(
            "Obfuscated {} CWR records from '{}' in {:.2?}",
            allegro_cwr::format_int_with_commas(total_records as i64),
            &config.base.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Obfuscated {} CWR records from {} files in {:.2?}",
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
