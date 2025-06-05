use crate::error::CwrParseError;

/// Test utilities for CWR record testing
pub mod record_test_utils {
    use super::*;

    /// Trait for testing CWR record parsing and serialization
    pub trait CwrRecordTestable {
        /// The expected record type (e.g., "ACK", "HDR", etc.)
        fn expected_record_type() -> &'static str;
        
        /// The expected minimum line length for this record type
        fn expected_line_length() -> usize;
        
        /// Parse a CWR line into this record type
        fn from_cwr_line(line: &str) -> Result<Self, CwrParseError>
        where
            Self: Sized;
        
        /// Convert this record to a CWR line
        fn to_cwr_line(&self) -> String;
        
        /// Create a valid test instance of this record
        fn create_test_record() -> Self
        where
            Self: Sized;
    }

    /// Generate test cases for a record type
    pub struct RecordTestGenerator;

    impl RecordTestGenerator {
        /// Test that a record type rejects lines that are too short
        pub fn test_line_too_short<T: CwrRecordTestable>() {
            let short_line = "X".repeat(T::expected_line_length() - 1);
            let result = T::from_cwr_line(&short_line);
            assert!(result.is_err(), "Should reject line that is too short");
            
            if let Err(CwrParseError::BadFormat(msg)) = result {
                assert!(msg.contains("too short") || msg.contains("short"), 
                    "Error message should mention line is too short: {}", msg);
            } else {
                panic!("Expected BadFormat error for short line");
            }
        }

        /// Test that a record type rejects wrong record type prefix
        pub fn test_wrong_record_type<T: CwrRecordTestable>() {
            let test_record = T::create_test_record();
            let mut line = test_record.to_cwr_line();
            
            // Replace the first 3 characters with "XYZ"
            line.replace_range(0..3, "XYZ");
            
            let result = T::from_cwr_line(&line);
            assert!(result.is_err(), "Should reject wrong record type");
            
            if let Err(CwrParseError::BadFormat(msg)) = result {
                assert!(msg.contains("Expected") && msg.contains(T::expected_record_type()), 
                    "Error message should mention expected record type: {}", msg);
            } else {
                panic!("Expected BadFormat error for wrong record type");
            }
        }

        /// Test basic round-trip serialization/deserialization
        pub fn test_round_trip<T: CwrRecordTestable + PartialEq + std::fmt::Debug>() {
            let original = T::create_test_record();
            let line = original.to_cwr_line();
            
            // Verify line length
            assert_eq!(line.len(), T::expected_line_length(), 
                "Generated line should match expected length");
            
            // Verify it starts with correct record type
            assert!(line.starts_with(T::expected_record_type()), 
                "Generated line should start with correct record type");
            
            // Test parsing
            let parsed = T::from_cwr_line(&line).expect("Should parse generated line");
            assert_eq!(original, parsed, "Round-trip should preserve all data");
        }

        /// Test that exactly minimum length line is accepted
        pub fn test_exact_minimum_length<T: CwrRecordTestable>() {
            let test_record = T::create_test_record();
            let line = test_record.to_cwr_line();
            
            assert_eq!(line.len(), T::expected_line_length(), 
                "Test record should generate exact minimum length");
            
            let result = T::from_cwr_line(&line);
            assert!(result.is_ok(), "Should accept line of exact minimum length");
        }

        /// Test that one character too short is rejected
        pub fn test_one_char_too_short<T: CwrRecordTestable>() {
            let test_record = T::create_test_record();
            let mut line = test_record.to_cwr_line();
            
            // Remove last character
            line.pop();
            assert_eq!(line.len(), T::expected_line_length() - 1);
            
            let result = T::from_cwr_line(&line);
            assert!(result.is_err(), "Should reject line that is one character too short");
        }

        /// Run all standard tests for a record type
        pub fn run_all_tests<T: CwrRecordTestable + PartialEq + std::fmt::Debug>() {
            Self::test_round_trip::<T>();
            Self::test_line_too_short::<T>();
            Self::test_wrong_record_type::<T>();
            Self::test_exact_minimum_length::<T>();
            Self::test_one_char_too_short::<T>();
        }
    }

    /// Utilities for creating test lines with specific properties
    pub struct TestLineBuilder {
        content: String,
    }

    impl TestLineBuilder {
        /// Create a new test line builder starting with a record type
        pub fn new(record_type: &str) -> Self {
            Self {
                content: record_type.to_string(),
            }
        }

        /// Add a field with specific width, padding with spaces if needed
        pub fn add_field(mut self, value: &str, width: usize) -> Self {
            let formatted = format!("{:width$}", value, width = width);
            self.content.push_str(&formatted);
            self
        }

        /// Add an optional field (empty if None, or the value if Some)
        pub fn add_optional_field(self, value: Option<&str>, width: usize) -> Self {
            self.add_field(value.unwrap_or(""), width)
        }

        /// Build the final test line
        pub fn build(self) -> String {
            self.content
        }

        /// Build a line with the specified total length, padding with spaces
        pub fn build_with_length(mut self, total_length: usize) -> String {
            while self.content.len() < total_length {
                self.content.push(' ');
            }
            self.content.truncate(total_length);
            self.content
        }
    }

    /// Macro to generate standard test cases for a record type
    #[macro_export]
    macro_rules! generate_record_tests {
        ($record_type:ty) => {
            #[cfg(test)]
            mod generated_tests {
                use super::*;
                use crate::test_utils::record_test_utils::{CwrRecordTestable, RecordTestGenerator};

                #[test]
                fn test_round_trip_generated() {
                    RecordTestGenerator::test_round_trip::<$record_type>();
                }

                #[test]
                fn test_line_too_short_generated() {
                    RecordTestGenerator::test_line_too_short::<$record_type>();
                }

                #[test]
                fn test_wrong_record_type_generated() {
                    RecordTestGenerator::test_wrong_record_type::<$record_type>();
                }

                #[test]
                fn test_exact_minimum_length_generated() {
                    RecordTestGenerator::test_exact_minimum_length::<$record_type>();
                }

                #[test]
                fn test_one_char_too_short_generated() {
                    RecordTestGenerator::test_one_char_too_short::<$record_type>();
                }
            }
        };
    }
}

/// Parser-level test utilities
pub mod parser_test_utils {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    /// Create a temporary CWR file for testing
    pub fn create_temp_cwr_file(content: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file_path = temp_dir.join(format!("test_cwr_{}.cwr", timestamp));
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        file_path
    }

    /// Clean up a temporary file
    pub fn cleanup_temp_file(path: &PathBuf) {
        std::fs::remove_file(path).ok();
    }

    /// Generate a valid HDR line for testing
    pub fn create_test_hdr_line(version: f32) -> String {
        match version as i32 {
            2 => "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101".to_string(),
            _ => "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    2.2".to_string(),
        }
    }

    /// Generate a valid TRL line for testing
    pub fn create_test_trl_line() -> String {
        "TRL00000001000000012005010100                                                                                                                                                                                                                                                                                                                                                                                   ".to_string()
    }
}

/// Field validation test utilities
pub mod validation_test_utils {
    /// Test utilities for date validation
    pub mod date_tests {
        pub fn invalid_date_formats() -> Vec<&'static str> {
            vec![
                "2005013",   // Too short
                "200501011", // Too long
                "20051301",  // Invalid month
                "20050132",  // Invalid day
                "abcd0101",  // Non-numeric year
                "2005ab01",  // Non-numeric month
                "200501ab",  // Non-numeric day
                "",          // Empty
                "        ",  // Spaces only
            ]
        }

        pub fn valid_date_formats() -> Vec<&'static str> {
            vec![
                "20050101",  // Valid date
                "20051231",  // Valid end of year
                "20050228",  // Valid February date
                "20040229",  // Valid leap year date
            ]
        }
    }

    /// Test utilities for time validation
    pub mod time_tests {
        pub fn invalid_time_formats() -> Vec<&'static str> {
            vec![
                "12000",    // Too short
                "1200000",  // Too long
                "250000",   // Invalid hour
                "126000",   // Invalid minute
                "120060",   // Invalid second
                "ab0000",   // Non-numeric hour
                "12ab00",   // Non-numeric minute
                "1200ab",   // Non-numeric second
                "",         // Empty
                "      ",   // Spaces only
            ]
        }

        pub fn valid_time_formats() -> Vec<&'static str> {
            vec![
                "120000",   // Valid time
                "000000",   // Midnight
                "235959",   // End of day
                "123456",   // Random valid time
            ]
        }
    }
}