use std::process;
use std::time::Instant;

use allegro_cwr::{OutputFormat, format_int_with_commas, process_cwr_file_json};

struct Config {
    output_path: Option<String>,
    input_filename: Option<String>,
    format: OutputFormat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_path: None,
            input_filename: None,
            format: OutputFormat::Default,
        }
    }
}

fn parse_args() -> Result<Config, String> {
    let mut config = Config::default();
    let mut parser = lexopt::Parser::from_env();
    
    while let Ok(Some(arg)) = parser.next() {
        match arg {
            lexopt::Arg::Short('o') | lexopt::Arg::Long("output") => {
                config.output_path = Some(get_value(&mut parser, "output")?);
            }
            lexopt::Arg::Short('f') | lexopt::Arg::Long("format") => {
                let format_str = get_value(&mut parser, "format")?;
                config.format = format_str.parse()?;
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
    parser.value()
        .map(|val| val.to_string_lossy().to_string())
        .map_err(|e| format!("Missing value for --{}: {}", arg_name, e))
}

fn main() {
    let config = match parse_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_help();
            process::exit(1);
        }
    };

    let input_filename = config.input_filename.expect("input_filename already validated during parsing");

    println!("Processing input file: {}...", input_filename);

    let start_time = Instant::now();

    let result = match config.format {
        OutputFormat::Json => {
            match process_cwr_file_json(&input_filename) {
                Ok(count) => Ok(("".to_string(), count)),
                Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
            }
        }
        OutputFormat::Default | OutputFormat::Sql => {
            let db_filename = allegro_cwr_sqlite::determine_db_filename(&input_filename, config.output_path.as_deref());
            println!("Using database filename: '{}'", db_filename);
            
            match allegro_cwr_sqlite::process_cwr_to_sqlite(&input_filename, &db_filename) {
                Ok((file_id, count, report)) => {
                    println!("{}", report);
                    Ok((db_filename, count))
                }
                Err(e) => Err(e),
            }
        }
    };

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

    println!("Successfully processed {} CWR records from '{}'{} in {:.2?}.", 
        format_int_with_commas(count as i64), 
        &input_filename,
        if !db_filename.is_empty() { format!(" into '{}'", db_filename) } else { String::new() },
        elapsed_time);
}

fn print_help() {
    eprintln!("Usage: allegro [OPTIONS] <input_filename>");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <input_filename>    CWR file to process");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -o, --output <file>      Output database file path");
    eprintln!("  -f, --format <format>    Output format ({})", OutputFormat::valid_formats());
    eprintln!("  -h, --help               Show this help message");
    eprintln!();
    eprintln!("By default, creates <input_filename>.db, or numbered variants if it exists");
    eprintln!("(.1.db, .2.db, etc.). Use --output to specify exact file. Multiple files");
    eprintln!("can be imported into the same database.");
}
