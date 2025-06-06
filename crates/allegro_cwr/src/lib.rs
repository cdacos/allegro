//! CWR (Common Works Registration) file parser library
//!
//! This library provides core functionality to parse CWR files. For database storage,
//! see the `allegro_cwr_sqlite` crate. For JSON output, see the `allegro_cwr_json` crate.

pub mod domain_types;
pub mod error;
pub mod parser;
pub mod records;
pub mod util;

#[cfg(test)]
pub mod test_utils;

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
pub use crate::parser::{ParsedRecord, ParsingContext, process_cwr_stream, process_cwr_stream_with_version};
pub use crate::records::*;
pub use crate::util::{extract_version_from_filename, format_int_with_commas};

use log::info;

/// Trait for handling CWR records during processing
pub trait CwrHandler {
    type Error: std::error::Error;

    /// Process a single parsed CWR record
    fn process_record(&mut self, record: ParsedRecord) -> Result<(), Self::Error>;

    /// Handle a parsing error (e.g., log it, count it, etc.)
    fn handle_parse_error(&mut self, line_number: usize, error: &CwrParseError) -> Result<(), Self::Error>;

    /// Handle warnings from a successfully parsed record (optional override)
    fn handle_warnings(&mut self, line_number: usize, record_type: &str, warnings: &[String]) -> Result<(), Self::Error> {
        // Default implementation does nothing - handlers can override to store warnings
        let _ = (line_number, record_type, warnings);
        Ok(())
    }

    /// Finalize processing (e.g., commit transaction, close files, etc.)
    fn finalize(&mut self) -> Result<(), Self::Error>;

    /// Generate a report of the processing results
    fn get_report(&self) -> String;
}

/// Generic function to process CWR file with any handler that implements CwrHandler trait
pub fn process_cwr_with_handler<H: CwrHandler>(input_filename: &str, handler: H) -> Result<String, Box<dyn std::error::Error>>
where
    H::Error: 'static,
{
    process_cwr_with_handler_and_version(input_filename, handler, None)
}

/// Generic function to process CWR file with any handler that implements CwrHandler trait and optional version hint
pub fn process_cwr_with_handler_and_version<H: CwrHandler>(input_filename: &str, mut handler: H, version_hint: Option<f32>) -> Result<String, Box<dyn std::error::Error>>
where
    H::Error: 'static,
{
    let mut processed_count = 0;
    let mut error_count = 0;

    for result in process_cwr_stream_with_version(input_filename, version_hint)? {
        match result {
            Ok(parsed_record) => {
                // Handle warnings if any
                if !parsed_record.warnings.is_empty() {
                    handler.handle_warnings(parsed_record.line_number, parsed_record.record.record_type(), &parsed_record.warnings)?;
                }
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

    info!("Processing complete: {} records processed, {} errors", processed_count, error_count);
    Ok(handler.get_report())
}
