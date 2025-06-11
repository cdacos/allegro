use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use allegro_cwr_cli::{BaseConfig, get_value, init_logging_and_parse_args, process_stdin_with_temp_file};
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

#[derive(Debug, PartialEq)]
enum InputFormat {
    Cwr,
    Json,
}

fn detect_input_format(filename: &str) -> Result<InputFormat, String> {
    let mut file = File::open(filename).map_err(|e| format!("Cannot open file '{}': {}", filename, e))?;

    let mut buffer = [0u8; 16];
    let bytes_read = file.read(&mut buffer).map_err(|e| format!("Cannot read file '{}': {}", filename, e))?;

    if bytes_read == 0 {
        return Err("File is empty".to_string());
    }

    // Convert to string for easier analysis, handling non-UTF8 gracefully
    let content = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Check for CWR format: starts with "HDR"
    if content.starts_with("HDR") {
        return Ok(InputFormat::Cwr);
    }

    // Check for JSON format: starts with '{' (possibly after whitespace)
    let trimmed = content.trim_start();
    if trimmed.starts_with('{') {
        return Ok(InputFormat::Json);
    }

    // Fallback: try file extension
    if filename.to_lowercase().ends_with(".json") {
        Ok(InputFormat::Json)
    } else if filename.to_lowercase().ends_with(".cwr") {
        Ok(InputFormat::Cwr)
    } else {
        Err(format!(
            "Cannot determine input format for '{}'. Expected CWR file (starting with 'HDR') or JSON file (starting with '{{')",
            filename
        ))
    }
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
        "cwr_json_stdin",
        |temp_path, start_time| {
            let input_format = match detect_input_format(temp_path) {
                Ok(format) => format,
                Err(e) => {
                    eprintln!("Format detection error: {}", e);
                    process::exit(1);
                }
            };

            let result = match input_format {
                InputFormat::Cwr => allegro_cwr_json::process_cwr_to_json_with_version_and_output(
                    temp_path,
                    config.base.cwr_version,
                    config.output_filename.as_deref(),
                ),
                InputFormat::Json => allegro_cwr_json::process_json_to_cwr_with_version_and_output(
                    temp_path,
                    config.base.cwr_version,
                    config.output_filename.as_deref(),
                ),
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
                "Successfully processed {} CWR records from stdin in {:.2?}",
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
        info!("Processing CWR/JSON file: {}", input_filename);

        let input_format = match detect_input_format(input_filename) {
            Ok(format) => format,
            Err(e) => {
                eprintln!("Format detection error for '{}': {}", input_filename, e);
                failed_files.push(input_filename.clone());
                continue;
            }
        };

        let output_filename = if config.base.input_files.len() > 1 && config.output_filename.is_some() {
            let base_name = config.output_filename.as_ref().unwrap();
            Some(format!("{}.{}", base_name, files_processed + 1))
        } else {
            config.output_filename.clone()
        };

        let result = match input_format {
            InputFormat::Cwr => allegro_cwr_json::process_cwr_to_json_with_version_and_output(
                input_filename,
                config.base.cwr_version,
                output_filename.as_deref(),
            ),
            InputFormat::Json => allegro_cwr_json::process_json_to_cwr_with_version_and_output(
                input_filename,
                config.base.cwr_version,
                output_filename.as_deref(),
            ),
        };

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
            "Processed {} CWR records from '{}' in {:.2?}",
            allegro_cwr::format_int_with_commas(total_records as i64),
            &config.base.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Processed {} CWR records from {} files in {:.2?}",
            allegro_cwr::format_int_with_commas(total_records as i64),
            files_processed,
            elapsed_time
        );
    }
}

fn print_help() {
    eprintln!("Usage: cwr-json [OPTIONS] [FILES...]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  [FILES...]          CWR or JSON files to process. If no files specified, reads from stdin");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output file path (format auto-detected or stdout for stdin)");
    eprintln!(
        "      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified"
    );
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Bidirectional converter between CWR and JSON formats.");
    eprintln!("Input format auto-detected by content (CWR starts with 'HDR', JSON starts with '{{')");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cwr-json file.cwr                            # Convert CWR to JSON");
    eprintln!("  cwr-json file.json                           # Convert JSON to CWR");
    eprintln!("  cwr-json -o output.json input.cwr            # Specify output file");
    eprintln!("  cwr-json *.cwr *.json                        # Process multiple files");
    eprintln!("  cat input.cwr | cwr-json                     # Process from stdin");
}
