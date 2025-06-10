use std::process;
use std::time::Instant;

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

    info!("Obfuscating CWR file: {}", input_filename);

    let start_time = Instant::now();

    let result = allegro_cwr_obfuscate::process_cwr_obfuscation(&input_filename, config.output_filename.as_deref(), config.cwr_version);

    let elapsed_time = start_time.elapsed();

    info!("Processing completed");

    let count = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error processing file '{}' after {:.2?}: {}", &input_filename, elapsed_time, e);
            process::exit(1);
        }
    };

    println!("Successfully obfuscated {} CWR records from '{}' in {:.2?}", allegro_cwr::format_int_with_commas(count as i64), &input_filename, elapsed_time);
}

fn print_help() {
    eprintln!("Usage: cwr-obfuscate [OPTIONS] <input_filename>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_filename>    CWR file to obfuscate");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output file path (defaults to <input>.obfuscated)");
    eprintln!("      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Obfuscates sensitive information in CWR files while maintaining referential integrity.");
    eprintln!("Names, titles, IPIs, and work numbers are consistently mapped throughout the file.");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  cwr-obfuscate input.cwr");
    eprintln!("  cwr-obfuscate -o obfuscated.cwr input.cwr");
}
