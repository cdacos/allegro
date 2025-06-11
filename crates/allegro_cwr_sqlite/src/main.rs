use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use allegro_cwr::{OutputFormat, format_int_with_commas};
use allegro_cwr_cli::{BaseConfig, get_value, process_stdin_with_temp_file, init_logging_and_parse_args};
use log::{error, info};

#[derive(Default)]
struct Config {
    base: BaseConfig,
    output_filename: Option<String>,
    file_id: Option<i64>,
}

#[derive(Debug, PartialEq)]
enum InputFormat {
    Cwr,
    Sqlite,
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
            lexopt::Arg::Long("file-id") => {
                let file_id_str = get_value(&mut parser, "file-id")?;
                let file_id: i64 = file_id_str
                    .parse()
                    .map_err(|_| format!("Invalid file ID '{}'. Must be a positive integer", file_id_str))?;

                if file_id <= 0 {
                    return Err(format!("Invalid file ID '{}'. Must be a positive integer", file_id));
                }

                config.file_id = Some(file_id);
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

    // Everything else is treated as SQLite database
    Ok(InputFormat::Sqlite)
}

fn get_most_recent_file_id(db_filename: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(db_filename)?;

    let file_id: i64 = conn
        .query_row("SELECT file_id FROM file ORDER BY imported_on DESC LIMIT 1", [], |row| row.get(0))
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => "No files found in database".to_string(),
            _ => format!("Database error: {}", e),
        })?;

    Ok(file_id)
}


fn main() {
    let config = init_logging_and_parse_args(|| {
        parse_args().map_err(|e| {
            print_help();
            e
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
        "cwr_sqlite_stdin",
        |temp_path, start_time| {
            let input_format = match detect_input_format(temp_path) {
                Ok(format) => format,
                Err(e) => {
                    eprintln!("Format detection error: {}", e);
                    process::exit(1);
                }
            };

            let result = process_file(config, temp_path, input_format);
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
                format_int_with_commas(count as i64),
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
        info!("Processing CWR file: {}", input_filename);

        let input_format = match detect_input_format(input_filename) {
            Ok(format) => format,
            Err(e) => {
                eprintln!("Format detection error for '{}': {}", input_filename, e);
                failed_files.push(input_filename.clone());
                continue;
            }
        };

        println!("Processing input file: {} (detected format: {:?})", input_filename, input_format);

        let result = process_file(config, input_filename, input_format);

        match result {
            Ok(count) => {
                total_records += count;
                files_processed += 1;
                info!("Processed {} records from '{}'", count, input_filename);
                if config.base.input_files.len() > 1 {
                    println!("{}: {} records", input_filename, format_int_with_commas(count as i64));
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
            "Successfully processed {} CWR records from '{}' in {:.2?}",
            format_int_with_commas(total_records as i64),
            &config.base.input_files[0],
            elapsed_time
        );
    } else {
        println!(
            "Successfully processed {} CWR records from {} files in {:.2?}",
            format_int_with_commas(total_records as i64),
            files_processed,
            elapsed_time
        );
    }
}

fn process_file(config: &Config, input_filename: &str, input_format: InputFormat) -> Result<usize, Box<dyn std::error::Error>> {
    match input_format {
        InputFormat::Cwr => {
            // CWR -> SQLite (existing functionality)
            let db_filename = allegro_cwr_sqlite::determine_db_filename(input_filename, config.output_filename.as_deref());
            info!("Using database filename: '{}'", db_filename);

            match allegro_cwr_sqlite::process_cwr_to_sqlite_with_version(
                input_filename,
                &db_filename,
                config.base.cwr_version,
            ) {
                Ok((file_id, count, report)) => {
                    println!("{}", report);
                    if let Err(e) = allegro_cwr_sqlite::report::report_summary(&db_filename, file_id, OutputFormat::Sql)
                    {
                        eprintln!("Warning: Could not generate detailed report: {}", e);
                    }
                    Ok(count)
                }
                Err(e) => Err(e),
            }
        }
        InputFormat::Sqlite => {
            // SQLite -> CWR (new functionality)
            // Use specified file_id or get the most recent one
            let file_id = match config.file_id {
                Some(id) => {
                    info!("Using specified file ID: {}", id);
                    Ok(id)
                }
                None => {
                    info!("No file ID specified, using most recent file from database");
                    get_most_recent_file_id(input_filename)
                }
            };

            match file_id {
                Ok(id) => allegro_cwr_sqlite::process_sqlite_to_cwr_with_version_and_output(
                    input_filename,
                    id,
                    config.base.cwr_version,
                    config.output_filename.as_deref(),
                ),
                Err(e) => Err(e),
            }
        }
    }
}

fn print_help() {
    eprintln!("Usage: cwr-sqlite [OPTIONS] [FILES...]");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  [FILES...]          CWR or SQLite database files to process. If no files specified, reads from stdin");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output file path (SQLite database or CWR file)");
    eprintln!("      --cwr <version>      CWR version (2.0, 2.1, 2.2). Auto-detected from filename (.Vxx) or file content if not specified");
    eprintln!("      --file-id <id>       File ID to export from SQLite database (defaults to most recent)");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("Bidirectional converter:");
    eprintln!("  CWR → SQLite: cwr-sqlite file.cwr [-o output.db]");
    eprintln!("  SQLite → CWR: cwr-sqlite file.db [-o output.cwr]");
    eprintln!();
    eprintln!("Input format auto-detected by content (CWR starts with 'HDR')");
    eprintln!("For CWR → SQLite: creates <input_filename>.db by default, or numbered variants if it exists");
    eprintln!("(.1.db, .2.db, etc.). Multiple files can be imported into the same database.");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cwr-sqlite input.cwr                         # Convert CWR to SQLite");
    eprintln!("  cwr-sqlite *.cwr                             # Convert multiple CWR files");
    eprintln!("  cwr-sqlite -o output.db input.cwr            # Specify output database");
    eprintln!("  cwr-sqlite input.db                          # Convert SQLite to CWR");
    eprintln!("  cwr-sqlite --file-id 123 input.db           # Convert specific file ID from SQLite");
    eprintln!("  cat input.cwr | cwr-sqlite                   # Process CWR data from stdin");
}
