use std::fs::File;
use std::io::Read;
use std::process;
use std::time::Instant;

use allegro_cwr::{OutputFormat, format_int_with_commas};
use log::{error, info};

#[derive(Default)]
struct Config {
    output_path: Option<String>,
    input_filename: Option<String>,
    cwr_version: Option<f32>,
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
            lexopt::Arg::Short('o') | lexopt::Arg::Long("output") => {
                config.output_path = Some(get_value(&mut parser, "output")?);
            }
            lexopt::Arg::Long("cwr") => {
                let version_str = get_value(&mut parser, "cwr")?;
                let version: f32 = version_str.parse().map_err(|_| format!("Invalid CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version_str))?;

                if ![2.0, 2.1, 2.2].contains(&version) {
                    return Err(format!("Unsupported CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version));
                }

                config.cwr_version = Some(version);
            }
            lexopt::Arg::Long("file-id") => {
                let file_id_str = get_value(&mut parser, "file-id")?;
                let file_id: i64 = file_id_str.parse().map_err(|_| format!("Invalid file ID '{}'. Must be a positive integer", file_id_str))?;

                if file_id <= 0 {
                    return Err(format!("Invalid file ID '{}'. Must be a positive integer", file_id));
                }

                config.file_id = Some(file_id);
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

    let file_id: i64 = conn.query_row("SELECT file_id FROM file ORDER BY imported_on DESC LIMIT 1", [], |row| row.get(0)).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => "No files found in database".to_string(),
        _ => format!("Database error: {}", e),
    })?;

    Ok(file_id)
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
            // CWR -> SQLite (existing functionality)
            let db_filename = allegro_cwr_sqlite::determine_db_filename(&input_filename, config.output_path.as_deref());
            info!("Using database filename: '{}'", db_filename);

            match allegro_cwr_sqlite::process_cwr_to_sqlite_with_version(&input_filename, &db_filename, config.cwr_version) {
                Ok((file_id, count, report)) => {
                    println!("{}", report);
                    if let Err(e) = allegro_cwr_sqlite::report::report_summary(&db_filename, file_id, OutputFormat::Sql) {
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
                    get_most_recent_file_id(&input_filename)
                }
            };

            match file_id {
                Ok(id) => allegro_cwr_sqlite::process_sqlite_to_cwr_with_version_and_output(&input_filename, id, config.cwr_version, config.output_path.as_deref()),
                Err(e) => Err(e),
            }
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
    eprintln!("Usage: cwr-sqlite [OPTIONS] <input_filename>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_filename>    CWR or SQLite database file to process");
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
}
