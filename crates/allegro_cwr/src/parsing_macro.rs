#![allow(unused_mut)]

/// Macro to generate from_cwr_line method for CWR record parsing
///
/// Usage:
/// ```ignore
/// use allegro_cwr::impl_cwr_parsing;
/// use allegro_cwr::validators::one_of;
///
/// impl_cwr_parsing! {
///     AgrRecord {
///         record_type: (0, 3, required, one_of(&["AGR"])),
///         transaction_sequence_num: (3, 11, required),
///         record_sequence_num: (11, 19, required),
///         field_name: (start, end, optional, validator),
///         // ... more fields
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_cwr_parsing {
    // Pattern with tests first, then post-process
    (
        $struct_name:ident {
            $(
                $field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)
            ),* $(,)?
        }
        with_test_data [$($test_line:expr),+ $(,)?]
        with_post_process $post_process_fn:expr
    ) => {
        impl_cwr_parsing!(@impl $struct_name { $($field_name: ($start, $end, $field_type $(, $validator)?)),* } with_post_process $post_process_fn);
        impl_cwr_parsing!(@generate_tests $struct_name, [$($test_line),+]);
    };

    // Pattern with tests only
    (
        $struct_name:ident {
            $(
                $field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)
            ),* $(,)?
        }
        with_test_data [$($test_line:expr),+ $(,)?]
    ) => {
        impl_cwr_parsing!(@impl $struct_name { $($field_name: ($start, $end, $field_type $(, $validator)?)),* });
        impl_cwr_parsing!(@generate_tests $struct_name, [$($test_line),+]);
    };



    // Pattern with only post-process
    (
        $struct_name:ident {
            $(
                $field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)
            ),* $(,)?
        }
        with_post_process $post_process_fn:expr
    ) => {
        impl_cwr_parsing!(@impl $struct_name { $($field_name: ($start, $end, $field_type $(, $validator)?)),* } with_post_process $post_process_fn);
    };

    // Pattern with neither (basic case)
    (
        $struct_name:ident {
            $(
                $field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)
            ),* $(,)?
        }
    ) => {
        impl_cwr_parsing!(@impl $struct_name { $($field_name: ($start, $end, $field_type $(, $validator)?)),* });
    };

    // Internal implementation helper
    (@impl $struct_name:ident { $($field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)),* } $(with_post_process $post_process_fn:expr)?) => {
        impl $struct_name {
            /// Parse a CWR line into a record (v2 with validation and warnings)
            pub fn from_cwr_line(line: &str) -> Result<$crate::error::CwrParseResult<Self>, $crate::error::CwrParseError> {
                use $crate::util::extract_required_validated;
                #[allow(unused_imports)]
                use $crate::util::extract_optional_validated;
                let mut warnings = Vec::new();

                $(
                    let $field_name = impl_cwr_parsing!(@extract_field line, $start, $end, stringify!($field_name), $field_type, warnings $(, $validator)?);
                )*

                #[allow(unused_mut)]
                let mut record = Self {
                    $(
                        $field_name: impl_cwr_parsing!(@handle_result $field_name, $field_type),
                    )*
                };

                // Call post-processing - use custom or default
                impl_cwr_parsing!(@post_process $struct_name, record, warnings $(, $post_process_fn)?);

                Ok($crate::error::CwrParseResult { record, warnings })
            }

            /// Convert this record to a CWR format line
            pub fn to_cwr_line(&self) -> String {
                let mut fields = Vec::new();

                $(
                    let field_value = impl_cwr_parsing!(@format_field self.$field_name, $field_type);
                    let field_width = $end - $start;

                    // Format field to exact width (left-aligned, space-padded)
                    let formatted = format!("{:<width$}", field_value, width = field_width);
                    fields.push(formatted);
                )*

                fields.join("")
            }
        }
    };

    // Helper rule for extracting fields
    (@extract_field $line:expr, $start:expr, $end:expr, $field_name:expr, required, $warnings:expr, $validator:expr) => {
        extract_required_validated($line, $start, $end, $field_name, Some(&$validator), &mut $warnings)
    };

    (@extract_field $line:expr, $start:expr, $end:expr, $field_name:expr, required, $warnings:expr) => {
        extract_required_validated($line, $start, $end, $field_name, None, &mut $warnings)
    };

    (@extract_field $line:expr, $start:expr, $end:expr, $field_name:expr, optional, $warnings:expr, $validator:expr) => {
        extract_optional_validated($line, $start, $end, $field_name, Some(&$validator), &mut $warnings)
    };

    (@extract_field $line:expr, $start:expr, $end:expr, $field_name:expr, optional, $warnings:expr) => {
        extract_optional_validated($line, $start, $end, $field_name, None, &mut $warnings)
    };

    // Helper rule for handling results
    (@handle_result $field:expr, required) => {
        $field?
    };

    (@handle_result $field:expr, optional) => {
        $field
    };

    // Helper rule for formatting fields
    (@format_field $field:expr, required) => {
        &$field
    };

    (@format_field $field:expr, optional) => {
        $field.as_deref().unwrap_or("")
    };


    // Helper: Custom post-processing
    (@post_process $struct_name:ident, $record:ident, $warnings:ident, $post_process_fn:expr) => {
        let post_process = $post_process_fn;
        post_process(&mut $record, &mut $warnings);
    };

    // Helper: Default post-processing (no-op)
    (@post_process $struct_name:ident, $record:ident, $warnings:ident) => {
        // Default: do nothing
    };

    // Helper: Generate tests when provided
    (@generate_tests $struct_name:ident, [$($test_line:expr),+]) => {
        #[cfg(test)]
        mod roundtrip_tests {
            use super::*;

            #[test]
            fn test_roundtrip_parsing() {
                let test_lines = [$($test_line),+];

                for (i, original_line) in test_lines.iter().enumerate() {
                    // Parse the line
                    let parse_result = $struct_name::from_cwr_line(original_line)
                        .unwrap_or_else(|e| panic!("Failed to parse test case {}: {}", i, e));
                    let record = parse_result.record;

                    // Convert back to CWR line
                    let regenerated_line = record.to_cwr_line();

                    // Should be able to round-trip the data
                    assert_eq!(*original_line, regenerated_line,
                        "Round-trip failed for test case {}: original line and regenerated line don't match", i);

                    // Parse the regenerated line to ensure it's still valid
                    let reparse_result = $struct_name::from_cwr_line(&regenerated_line)
                        .unwrap_or_else(|e| panic!("Failed to reparse test case {}: {}", i, e));

                    // The records should be identical
                    assert_eq!(record, reparse_result.record,
                        "Records differ after round-trip parsing for test case {}", i);
                }
            }
        }
    };

    // Helper: No tests to generate
    (@generate_tests $struct_name:ident) => {
        // No tests to generate
    };

}

/// Macro to generate round-trip tests for CWR record parsing
///
/// Usage:
/// ```ignore
/// use allegro_cwr::impl_cwr_parsing_test_roundtrip;
///
/// impl_cwr_parsing_test_roundtrip!(AgrRecord, [
///     "AGR00000001000000011234567890123...",
///     "AGR00000002000000021234567890456..."
/// ]);
/// ```
#[macro_export]
macro_rules! impl_cwr_parsing_test_roundtrip {
    ($struct_name:ident, [$($test_line:expr),+ $(,)?]) => {
        #[cfg(test)]
        mod roundtrip_tests {
            use super::*;

            #[test]
            fn test_roundtrip_parsing() {
                let test_lines = [$($test_line),+];

                for (i, original_line) in test_lines.iter().enumerate() {
                    // Parse the line
                    let parse_result = $struct_name::from_cwr_line(original_line)
                        .unwrap_or_else(|e| panic!("Failed to parse test case {}: {}", i, e));
                    let record = parse_result.record;

                    // Convert back to CWR line
                    let regenerated_line = record.to_cwr_line();

                    // Should be able to round-trip the data
                    assert_eq!(*original_line, regenerated_line,
                        "Round-trip failed for test case {}: original line and regenerated line don't match", i);

                    // Parse the regenerated line to ensure it's still valid
                    let reparse_result = $struct_name::from_cwr_line(&regenerated_line)
                        .unwrap_or_else(|e| panic!("Failed to reparse test case {}: {}", i, e));

                    // The records should be identical
                    assert_eq!(record, reparse_result.record,
                        "Records differ after round-trip parsing for test case {}", i);
                }
            }
        }
    };

}

#[cfg(test)]
mod tests {
    use crate::error::CwrParseError;
    use crate::validators::one_of;

    #[derive(Debug, Clone, PartialEq)]
    struct TestRecord {
        record_type: String,
        required_field: String,
        optional_field: Option<String>,
    }


    impl_cwr_parsing! {
        TestRecord {
            record_type: (0, 3, required, one_of(&["TST"])),
            required_field: (3, 8, required),
            optional_field: (8, 13, optional),
        }
        with_test_data ["TST12345ABCDE"]
    }

    #[test]
    fn test_macro_generated_parsing() {
        let line = "TST12345ABCDE";
        let result = TestRecord::from_cwr_line(line).unwrap();

        assert_eq!(result.record.record_type, "TST");
        assert_eq!(result.record.required_field, "12345");
        assert_eq!(result.record.optional_field, Some("ABCDE".to_string()));
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_macro_with_validation_error() {
        let line = "XYZ12345ABCDE";
        let result = TestRecord::from_cwr_line(line);

        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert!(msg.contains("Validation failed for field record_type"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_macro_with_optional_field_missing() {
        let line = "TST12345";
        let result = TestRecord::from_cwr_line(line).unwrap();

        assert_eq!(result.record.record_type, "TST");
        assert_eq!(result.record.required_field, "12345");
        assert_eq!(result.record.optional_field, None);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("Line too short for optional field"));
    }

    #[test]
    fn test_macro_generated_to_cwr_line() {
        let line = "TST12345ABCDE";
        let result = TestRecord::from_cwr_line(line).unwrap();

        // Test that the generated to_cwr_line method works
        let output = result.record.to_cwr_line();
        assert_eq!(output, "TST12345ABCDE");
    }

    #[test]
    fn test_macro_round_trip() {
        let original_line = "TST12345ABCDE";
        let parsed = TestRecord::from_cwr_line(original_line).unwrap();
        let regenerated_line = parsed.record.to_cwr_line();

        // Should be able to round-trip the data
        assert_eq!(original_line, regenerated_line);
    }

    #[test]
    fn test_struct_literal_creation() {
        // Test that struct literals work
        let record = TestRecord {
            record_type: "TST".to_string(),
            required_field: "12345".to_string(),
            optional_field: Some("ABCDE".to_string()),
        };

        assert_eq!(record.record_type, "TST");
        assert_eq!(record.required_field, "12345");
        assert_eq!(record.optional_field, Some("ABCDE".to_string()));

        // Test that to_cwr_line works with struct literal created record
        let output = record.to_cwr_line();
        assert_eq!(output, "TST12345ABCDE");
    }
}
