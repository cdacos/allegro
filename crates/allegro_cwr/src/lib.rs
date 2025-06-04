//! CWR (Common Works Registration) file parser library
//! 
//! This library provides functionality to parse CWR files and load them into SQLite databases.

pub mod error;
pub mod parser;
pub mod record_handlers;
pub mod records;
pub mod report;
pub mod util;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Default,
    Sql,
    Json,
}

// Re-export commonly used items
pub use allegro_cwr_sqlite::{determine_db_filename, setup_database};
pub use crate::error::CwrParseError;
pub use crate::parser::{process_and_load_file, process_and_stream_json, ParsingContext};
pub use crate::records::{
    AckRecord, AgrRecord, AltRecord, AriRecord, ComRecord, EwtRecord, GrhRecord, GrtRecord, 
    HdrRecord, IndRecord, InsRecord, IpaRecord, MsgRecord, NatRecord, NetRecord, NowRecord,
    NpaRecord, NpnRecord, NprRecord, NwrRecord, NwnRecord, OrnRecord, PerRecord, PwrRecord, 
    RecRecord, SpuRecord, SptRecord, SwrRecord, SwtRecord, TerRecord, TrlRecord, VerRecord, 
    XrfRecord
};
pub use crate::report::report_summary;
pub use crate::util::format_int_with_commas;

/// Main processing function that combines parsing and reporting
pub fn process_cwr_file(input_filename: &str) -> Result<(String, usize), CwrParseError> {
    process_cwr_file_with_output(input_filename, None, OutputFormat::Default)
}

/// Main processing function with optional output path
pub fn process_cwr_file_with_output(input_filename: &str, output_path: Option<&str>, format: OutputFormat) -> Result<(String, usize), CwrParseError> {
    match format {
        OutputFormat::Json => {
            // For JSON format, stream directly without using database
            let count = process_and_stream_json(input_filename)?;
            Ok(("".to_string(), count))
        }
        OutputFormat::Default | OutputFormat::Sql => {
            // For default and sql formats, use database
            let db_filename = determine_db_filename(input_filename, output_path);
            
            println!("Using database filename: '{}'", db_filename);
            
            setup_database(&db_filename).map_err(|e| CwrParseError::BadFormat(format!("Database setup error: {}", e)))?;
            
            let (file_id, count) = process_and_load_file(input_filename, &db_filename)?;
            
            report_summary(&db_filename, file_id, format).map_err(|e| CwrParseError::BadFormat(format!("Report generation error: {}", e)))?;
            
            Ok((db_filename, count))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grh_record_import() {
        let grh = GrhRecord::new(
            "AGR".to_string(),
            "00001".to_string(),
            "02.10".to_string(),
        );
        assert_eq!(grh.record_type, "GRH");
    }
}