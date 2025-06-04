//! CWR (Common Works Registration) file parser library
//! 
//! This library provides functionality to parse CWR files and load them into SQLite databases.

pub mod error;
pub mod parser;
pub mod record_handlers;
pub mod report;
pub mod util;

// Re-export commonly used items
pub use allegro_cwr_sqlite::{determine_db_filename, setup_database};
pub use crate::error::CwrParseError;
pub use crate::parser::{process_and_load_file, ParsingContext};
pub use crate::report::report_summary;
pub use crate::util::format_int_with_commas;

/// Main processing function that combines parsing and reporting
pub fn process_cwr_file(input_filename: &str) -> Result<(String, usize), CwrParseError> {
    process_cwr_file_with_output(input_filename, None)
}

/// Main processing function with optional output path
pub fn process_cwr_file_with_output(input_filename: &str, output_path: Option<&str>) -> Result<(String, usize), CwrParseError> {
    let db_filename = determine_db_filename(input_filename, output_path);
    
    println!("Using database filename: '{}'", db_filename);
    
    setup_database(&db_filename).map_err(|e| CwrParseError::BadFormat(format!("Database setup error: {}", e)))?;
    
    let (file_id, count) = process_and_load_file(input_filename, &db_filename)?;
    
    report_summary(&db_filename, file_id).map_err(|e| CwrParseError::BadFormat(format!("Report generation error: {}", e)))?;
    
    Ok((db_filename, count))
}