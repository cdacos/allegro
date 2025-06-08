use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use allegro_cwr::format_int_with_commas;
use log::{error, info};

#[derive(Default)]
struct Config {
    input_filename: Option<String>,
    output_filename: Option<String>,
    cwr_version: Option<f32>,
}

fn parse_args() -> Result<Config, String> {
    let mut config = Config::default();
    let mut parser = lexopt::Parser::from_env();

    while let Ok(Some(arg)) = parser.next() {
        match arg {
            lexopt::Arg::Long("cwr") => {
                let version_str = get_value(&mut parser, "cwr")?;
                let version: f32 = version_str.parse().map_err(|_| format!("Invalid CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version_str))?;

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
                if config.input_filename.is_some() {
                    return Err("Multiple input files specified".to_string());
                }
                config.input_filename = Some(val.to_string_lossy().to_string());
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

    if config.input_filename.is_none() {
        return Err("No input file specified".to_string());
    }

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
        Err(format!("Cannot determine input format for '{}'. Expected CWR file (starting with 'HDR') or JSON file (starting with '{{')", filename))
    }
}

fn get_value(parser: &mut lexopt::Parser, arg_name: &str) -> Result<String, String> {
    parser.value().map(|val| val.to_string_lossy().to_string()).map_err(|e| format!("Missing value for --{}: {}", arg_name, e))
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

    let input_filename = config.input_filename.expect("input_filename already validated during parsing");

    // Detect input format
    let input_format = match detect_input_format(&input_filename) {
        Ok(format) => format,
        Err(e) => {
            error!("Format detection error: {}", e);
            process::exit(1);
        }
    };

    println!("Processing input file: {} (detected format: {:?})", input_filename, input_format);

    let start_time = Instant::now();

    let result = match input_format {
        InputFormat::Cwr => {
            // CWR -> JSON (existing functionality)
            allegro_cwr_json::process_cwr_to_json_with_version_and_output(&input_filename, config.cwr_version, config.output_filename.as_deref())
        }
        InputFormat::Json => {
            // JSON -> CWR (new functionality)
            allegro_cwr_json::process_json_to_cwr_with_version_and_output(&input_filename, config.cwr_version, config.output_filename.as_deref())
        }
    };

    let elapsed_time = start_time.elapsed();

    info!("Processing completed");

    let count = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error processing file '{}' after {:.2?}: {}", &input_filename, elapsed_time, e);
            process::exit(1);
        }
    };

    println!("Successfully processed {} CWR records from '{}' in {:.2?}", format_int_with_commas(count as i64), &input_filename, elapsed_time);
}

fn print_help() {
    eprintln!("Usage: cwr-json [OPTIONS] <input_filename>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_filename>    CWR or JSON file to process");
    eprintln!();
    eprintln!("Options:");
    eprintln!("      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified");
    eprintln!("  -o, --output <file>      Output file (format determined by input: CWR→JSON or JSON→CWR)");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Bidirectional converter:");
    eprintln!("  CWR → JSON: cwr-json file.cwr [-o output.json]");
    eprintln!("  JSON → CWR: cwr-json file.json [-o output.cwr]");
    eprintln!();
    eprintln!("Input format auto-detected by content (CWR starts with 'HDR', JSON starts with '{{')");
}
