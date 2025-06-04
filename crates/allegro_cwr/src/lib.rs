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
    use std::fs;
    use std::collections::HashMap;
    use allegro_cwr_sqlite::count_records_by_type;

    #[test]
    fn test_integration_record_counts() {
        // Configuration - easily changeable test file path
        let test_file_path = "../../.me/TestSample.V21";  // Adjust this path as needed
        
        // Check if test file exists, if not fail test
        if !std::path::Path::new(test_file_path).exists() {
            panic!("Test file {} not found - integration test failed", test_file_path);
        }
        
        // Count records by manually parsing the file first
        let expected_counts = count_records_in_cwr_file(test_file_path)
            .expect("Failed to count records in CWR file");
        
        // Process the file through database
        let output_db = "/tmp/integration_test.db";
        let _ = fs::remove_file(output_db); // Clean up any existing file
        
        let result = process_cwr_file_with_output(test_file_path, Some(output_db), OutputFormat::Default);
        assert!(result.is_ok(), "Processing failed: {:?}", result.err());
        
        let (db_path, total_count) = result.unwrap();
        assert_eq!(db_path, output_db);
        
        // Verify total count matches expected
        let expected_total: i32 = expected_counts.values().sum();
        assert_eq!(total_count, expected_total as usize, 
                   "Total record count mismatch: expected {}, got {}", expected_total, total_count);
        
        // Query database for actual counts
        let actual_counts = count_records_by_type(&db_path)
            .expect("Failed to count records in database");
        
        // Compare expected vs actual counts
        for (record_type, expected_count) in &expected_counts {
            let actual_count = actual_counts.get(record_type).unwrap_or(&0);
            assert_eq!(
                *actual_count, *expected_count,
                "Count mismatch for {}: expected {}, got {}",
                record_type, expected_count, actual_count
            );
        }
        
        // Ensure no unexpected record types in database
        for (record_type, actual_count) in &actual_counts {
            let expected_count = expected_counts.get(record_type).unwrap_or(&0);
            assert_eq!(
                *actual_count, *expected_count,
                "Unexpected records found for {}: expected {}, got {}",
                record_type, expected_count, actual_count
            );
        }
        
        println!("✅ Integration test passed:");
        println!("   Total records: {}", total_count);
        println!("   Record type counts:");
        for (record_type, count) in &expected_counts {
            println!("     {}: {}", record_type, count);
        }
        
        // Clean up
        let _ = fs::remove_file(output_db);
    }

    /// Count record types by parsing CWR file manually
    /// Maps file record types to database table types
    fn count_records_in_cwr_file(file_path: &str) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut counts = HashMap::new();
        
        for line in content.lines() {
            if line.len() >= 3 {
                let file_record_type = &line[0..3];
                
                // Map file record types to database table types
                let db_record_type = match file_record_type {
                    "OPU" => "SPU", // OPU records are stored in SPU table
                    "OWR" => "SWR", // OWR records are stored in SWR table  
                    "OWT" => "SWT", // OWT records are stored in SWT table
                    other => other,
                };
                
                *counts.entry(db_record_type.to_string()).or_insert(0) += 1;
            }
        }
        
        Ok(counts)
    }
}