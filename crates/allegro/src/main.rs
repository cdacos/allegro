use std::process;
use std::time::Instant;

use allegro_cwr::{process_cwr_file_with_output, format_int_with_commas, OutputFormat};

fn main() {
    let mut output_path = None;
    let mut input_filename = None;
    let mut format = OutputFormat::Default;
    
    let mut parser = lexopt::Parser::from_env();
    while let Ok(arg) = parser.next() {
        match arg {
            Some(lexopt::Arg::Short('o')) | Some(lexopt::Arg::Long("output")) => {
                match parser.value() {
                    Ok(val) => output_path = Some(val.to_string_lossy().to_string()),
                    Err(e) => {
                        eprintln!("Error: Missing value for --output: {}", e);
                        process::exit(1);
                    }
                }
            }
            Some(lexopt::Arg::Short('f')) | Some(lexopt::Arg::Long("format")) => {
                match parser.value() {
                    Ok(val) => {
                        let format_str = val.to_string_lossy().to_lowercase();
                        format = match format_str.as_str() {
                            "sql" => OutputFormat::Sql,
                            "json" => OutputFormat::Json,
                            _ => {
                                eprintln!("Error: Invalid format '{}'. Valid formats are: sql, json", format_str);
                                process::exit(1);
                            }
                        };
                    }
                    Err(e) => {
                        eprintln!("Error: Missing value for --format: {}", e);
                        process::exit(1);
                    }
                }
            }
            Some(lexopt::Arg::Value(val)) => {
                if input_filename.is_some() {
                    eprintln!("Error: Multiple input files specified");
                    process::exit(1);
                }
                input_filename = Some(val.to_string_lossy().to_string());
            }
            Some(lexopt::Arg::Short('h')) | Some(lexopt::Arg::Long("help")) => {
                print_help();
                process::exit(0);
            }
            Some(_) => {
                eprintln!("Error: Unknown argument");
                print_help();
                process::exit(1);
            }
            None => break,
        }
    }
    
    let input_filename = match input_filename {
        Some(filename) => filename,
        None => {
            eprintln!("Error: No input file specified");
            print_help();
            process::exit(1);
        }
    };

    println!("Processing input file: {}...", input_filename);

    let start_time = Instant::now();

    let result = process_cwr_file_with_output(&input_filename, output_path.as_deref(), format);

    let elapsed_time = start_time.elapsed();

    println!("Done!");

    // Handle the result of file processing
    let (db_filename, count) = match result {
        Ok((db, c)) => (db, c),
        Err(e) => {
            eprintln!("Error processing file '{}' after {:.2?}: {}", &input_filename, elapsed_time, e);
            process::exit(1);
        }
    };

    println!("Successfully processed {} CWR records from '{}' into '{}' in {:.2?}.", format_int_with_commas(count as i64), &input_filename, db_filename, elapsed_time);
}

fn print_help() {
    eprintln!("Usage: allegro [OPTIONS] <input_filename>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_filename>    CWR file to process");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output database file path");
    eprintln!("  -f, --format <format>    Output format (sql, json)");
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("By default, creates <input_filename>.db, or numbered variants if it exists");
    eprintln!("(.1.db, .2.db, etc.). Use --output to specify exact file. Multiple files");
    eprintln!("can be imported into the same database.");
}