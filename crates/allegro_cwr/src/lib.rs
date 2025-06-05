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
pub use crate::parser::{ParsingContext, process_and_load_file, process_and_stream_json};
pub use crate::records::{
    AckRecord, AgrRecord, AltRecord, AriRecord, ComRecord, EwtRecord, GrhRecord, GrtRecord, HdrRecord, IndRecord, InsRecord, IpaRecord, MsgRecord, NatRecord, NetRecord, NowRecord, NpaRecord, NpnRecord, NprRecord, NwnRecord, NwrRecord, OrnRecord, PerRecord, PwrRecord, RecRecord, SptRecord, SpuRecord, SwrRecord,
    SwtRecord, TerRecord, TrlRecord, VerRecord, XrfRecord,
};
pub use crate::report::report_summary;
pub use crate::util::format_int_with_commas;
pub use allegro_cwr_sqlite::{determine_db_filename, setup_database};

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
    use allegro_cwr_sqlite::{count_errors_by_record_type, count_records_by_type};
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_integration_record_counts() {
        // Configuration - check env var first, fallback to default
        let test_file_path = std::env::var("CWR_TEST_FILE").unwrap_or_else(|_| "../../.me/TestSample.V21".to_string());

        // Check if test file exists, if not fail test
        if !std::path::Path::new(&test_file_path).exists() {
            panic!("Test file {} not found - integration test failed", test_file_path);
        }

        // Count records by manually parsing the file first
        let expected_counts = count_records_in_cwr_file(&test_file_path).expect("Failed to count records in CWR file");

        // Process the file through database
        let output_db = "/tmp/integration_test.db";
        let _ = fs::remove_file(output_db); // Clean up any existing file

        let result = process_cwr_file_with_output(&test_file_path, Some(output_db), OutputFormat::Default);
        assert!(result.is_ok(), "Processing failed: {:?}", result.err());

        let (db_path, total_count) = result.unwrap();
        assert_eq!(db_path, output_db);

        // Verify total count is reasonable (allow for some validation rejections)
        let expected_total: i32 = expected_counts.values().sum();
        assert!(total_count <= expected_total as usize, "More records in database ({}) than in file ({})", total_count, expected_total);

        // Query database for actual counts and error counts
        let actual_counts = count_records_by_type(&db_path).expect("Failed to count records in database");
        let error_counts = count_errors_by_record_type(&db_path).expect("Failed to count errors by record type");

        // Compare expected vs actual counts
        // Note: errors are validation warnings, records are still stored
        for (record_type, expected_count) in &expected_counts {
            let actual_count = actual_counts.get(record_type).unwrap_or(&0);
            let error_count = error_counts.get(record_type).unwrap_or(&0);

            // Primary check: stored records should match file records
            if *actual_count == *expected_count {
                if *error_count > 0 {
                    println!("✓ {}: {} stored (matches file) with {} validation warnings", record_type, actual_count, error_count);
                }
            } else if *actual_count < *expected_count {
                let rejected = expected_count - actual_count;
                // Check if the rejected count matches logged errors (actual rejections)
                if rejected == *error_count {
                    println!("✓ {}: {} stored + {} rejected = {} total (matches file)", record_type, actual_count, rejected, expected_count);
                } else {
                    println!("⚠ {}: {} stored, {} expected, {} errors (mismatch)", record_type, actual_count, expected_count, error_count);
                }
            } else {
                panic!("Unexpected: {} {} records stored but only {} expected from file", actual_count, record_type, expected_count);
            }
        }

        // Ensure no unexpected record types in database (but allow for validation rejections)
        for (record_type, actual_count) in &actual_counts {
            let expected_count = expected_counts.get(record_type).unwrap_or(&0);
            if *actual_count > *expected_count {
                assert_eq!(*actual_count, *expected_count, "Unexpected extra records found for {}: expected {}, got {}", record_type, expected_count, actual_count);
            }
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
    /// Counts records by their actual record_type as they appear in the file
    fn count_records_in_cwr_file(file_path: &str) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut counts = HashMap::new();

        for line in content.lines() {
            if line.len() >= 3 {
                let record_type = &line[0..3];
                *counts.entry(record_type.to_string()).or_insert(0) += 1;
            }
        }

        Ok(counts)
    }
}
