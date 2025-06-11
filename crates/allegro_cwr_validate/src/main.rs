use std::process;
use std::time::Instant;

use allegro_cwr_cli::{get_value, init_logging_and_parse_args, process_stdin_with_temp_file, BaseConfig};

#[derive(Default)]
struct Config {
    base: BaseConfig,
    charset_override: Option<String>,
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
            lexopt::Arg::Long("charset") => {
                let charset_str = get_value(&mut parser, "charset")?;
                config.charset_override = Some(charset_str);
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
        "cwr_validate_stdin",
        |temp_path, start_time| {
            let result = allegro_cwr_validate::check_roundtrip_integrity_with_charset(
                temp_path,
                config.base.cwr_version,
                config.charset_override.as_deref(),
            );
            let elapsed_time = start_time.elapsed();

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
        },
        start_time,
    );
}

fn process_files(config: &Config, start_time: Instant) {
    let mut total_count = 0;
    let mut processed_files = 0;
    let mut failed_files = Vec::new();

    for filename in &config.base.input_files {
        println!("Validating CWR file: {}", filename);

        let result = allegro_cwr_validate::check_roundtrip_integrity_with_charset(
            filename,
            config.base.cwr_version,
            config.charset_override.as_deref(),
        );

        match result {
            Ok(count) => {
                total_count += count;
                processed_files += 1;
                if config.base.input_files.len() > 1 {
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

    if config.base.input_files.len() == 1 {
        if !failed_files.is_empty() {
            eprintln!("Failed to process {} file(s): {}", failed_files.len(), failed_files.join(", "));
            process::exit(1);
        }

        println!(
            "Validated {} CWR records from '{}' in {:.2?}",
            allegro_cwr::format_int_with_commas(total_count as i64),
            &config.base.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Validated {} CWR records from {} files in {:.2?}",
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
    eprintln!("      --charset <charset>  Override character set when missing in HDR record (e.g., UTF-8, ASCII)");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cwr-validate input.cwr                    # Check single CWR file");
    eprintln!("  cwr-validate *.cwr                        # Check multiple CWR files");
    eprintln!("  cwr-validate --cwr 2.2 input.cwr          # Force CWR version 2.2");
    eprintln!("  cat input.cwr | cwr-validate              # Process CWR data from stdin");
    eprintln!("  find . -name '*.cwr' | xargs cwr-validate # Process all CWR files recursively");
}
