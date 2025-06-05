/// Macro to generate from_cwr_line_v2 method for CWR record parsing
/// 
/// Usage:
/// ```
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
    (
        $struct_name:ident {
            $(
                $field_name:ident: ($start:expr, $end:expr, $field_type:ident $(, $validator:expr)?)
            ),* $(,)?
        }
    ) => {
        impl $struct_name {
            /// Parse a CWR line into a record (v2 with validation and warnings)
            pub fn from_cwr_line_v2(line: &str) -> Result<$crate::error::CwrParseResult<Self>, $crate::error::CwrParseError> {
                use $crate::util::{extract_required_validated, extract_optional_validated};
                let mut warnings = Vec::new();

                $(
                    let $field_name = impl_cwr_parsing!(@extract_field line, $start, $end, stringify!($field_name), $field_type, warnings $(, $validator)?);
                )*

                let record = Self {
                    $(
                        $field_name: impl_cwr_parsing!(@handle_result $field_name, $field_type),
                    )*
                };

                Ok($crate::error::CwrParseResult { record, warnings })
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
    }

    #[test]
    fn test_macro_generated_parsing() {
        let line = "TST12345ABCDE";
        let result = TestRecord::from_cwr_line_v2(line).unwrap();
        
        assert_eq!(result.record.record_type, "TST");
        assert_eq!(result.record.required_field, "12345");
        assert_eq!(result.record.optional_field, Some("ABCDE".to_string()));
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_macro_with_validation_error() {
        let line = "XYZ12345ABCDE";
        let result = TestRecord::from_cwr_line_v2(line);
        
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
        let result = TestRecord::from_cwr_line_v2(line).unwrap();
        
        assert_eq!(result.record.record_type, "TST");
        assert_eq!(result.record.required_field, "12345");
        assert_eq!(result.record.optional_field, None);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("Line too short for optional field"));
    }
}