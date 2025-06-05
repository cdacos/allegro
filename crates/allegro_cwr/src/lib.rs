//! CWR (Common Works Registration) file parser library
//!
//! This library provides functionality to parse CWR files and load them into SQLite databases.

pub mod error;
pub mod parser;
pub mod records;
pub mod util;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Default,
    Sql,
    Json,
}

impl OutputFormat {
    pub fn valid_formats() -> &'static str {
        "sql, json"
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sql" => Ok(OutputFormat::Sql),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Invalid format '{}'. Valid formats are: {}", s, Self::valid_formats())),
        }
    }
}

// Re-export commonly used items
pub use crate::error::CwrParseError;
pub use crate::parser::{ParsingContext, process_cwr_stream, ParsedRecord};
pub use crate::records::*;
pub use crate::util::format_int_with_commas;

/// Trait for handling CWR records during processing
pub trait CwrHandler {
    type Error: std::error::Error;
    
    /// Process a single parsed CWR record
    fn process_record(&mut self, record: ParsedRecord) -> Result<(), Self::Error>;
    
    /// Handle a parsing error (e.g., log it, count it, etc.)
    fn handle_parse_error(&mut self, line_number: usize, error: &CwrParseError) -> Result<(), Self::Error>;
    
    /// Finalize processing (e.g., commit transaction, close files, etc.)
    fn finalize(&mut self) -> Result<(), Self::Error>;
    
    /// Generate a report of the processing results
    fn get_report(&self) -> String;
}

/// Generic function to process CWR file with any handler that implements CwrHandler trait
pub fn process_cwr_with_handler<H: CwrHandler>(
    input_filename: &str, 
    mut handler: H
) -> Result<String, Box<dyn std::error::Error>> 
where 
    H::Error: 'static,
{
    let mut processed_count = 0;
    let mut error_count = 0;
    
    for result in process_cwr_stream(input_filename)? {
        match result {
            Ok(parsed_record) => {
                handler.process_record(parsed_record)?;
                processed_count += 1;
            }
            Err(parse_error) => {
                handler.handle_parse_error(processed_count + error_count + 1, &parse_error)?;
                error_count += 1;
            }
        }
    }
    
    handler.finalize()?;
    
    println!("Processing complete: {} records processed, {} errors", processed_count, error_count);
    Ok(handler.get_report())
}


/// Streams JSON output for each CWR record without using a database
pub fn process_and_stream_json(input_filename: &str) -> Result<usize, CwrParseError> {
    println!("[");

    let mut line_count = 0;
    let mut first_record = true;

    for result in process_cwr_stream(input_filename)? {
        line_count += 1;
        match result {
            Ok(parsed_record) => {
                if !first_record {
                    println!(",");
                }
                
                // Output JSON representation of the parsed record
                println!("  {{");
                println!("    \"line_number\": {},", parsed_record.line_number);
                println!("    \"record_type\": \"{}\",", parsed_record.record.record_type());
                println!("    \"cwr_version\": {},", parsed_record.context.cwr_version);
                println!("    \"status\": \"parsed\"");
                println!("  }}");
                
                first_record = false;
            }
            Err(e) => {
                if !first_record {
                    println!(",");
                }
                println!("  {{");
                println!("    \"line_number\": {},", line_count);
                println!("    \"status\": \"error\",");
                println!("    \"error_message\": \"{}\"", e.to_string().replace('"', "\\\""));
                println!("  }}");
                first_record = false;
            }
        }
    }

    println!();
    println!("]");
    Ok(line_count)
}

/// Main processing function for JSON output
pub fn process_cwr_file_json(input_filename: &str) -> Result<usize, CwrParseError> {
    process_and_stream_json(input_filename)
}

