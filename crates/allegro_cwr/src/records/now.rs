use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NOW - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = now_custom_validate,
    test_data = "NOW0000000100000002PLACEHOLDER WRITER NAME                                                                                                                                         PLACEHOLDER FIRST NAME                                                                                                                                          EN1"
)]
pub struct NowRecord {
    #[cwr(title = "Always 'NOW'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Writer name", start = 19, len = 160)]
    pub writer_name: NonRomanAlphabet,

    #[cwr(title = "Writer first name", start = 179, len = 160)]
    pub writer_first_name: NonRomanAlphabet,

    #[cwr(title = "Language code (optional)", start = 339, len = 2)]
    pub language_code: Option<LanguageCode>,

    #[cwr(title = "Writer position (1 char, optional)", start = 341, len = 1)]
    pub writer_position: Option<LookupPlaceholder>,
}

// Custom validation function for NOW record
fn now_custom_validate(record: &mut NowRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NOW" {
        warnings.push(CwrWarning {
            field_name: "record_type",
            field_title: "Always 'NOW'",
            source_str: std::borrow::Cow::Owned(record.record_type.clone()),
            level: WarningLevel::Critical,
            description: "Record type must be 'NOW'".to_string(),
        });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate writer name is not empty
    if record.writer_name.as_str().trim().is_empty() {
        warnings.push(CwrWarning {
            field_name: "writer_name",
            field_title: "Writer name",
            source_str: std::borrow::Cow::Owned(record.writer_name.as_str().to_string()),
            level: WarningLevel::Critical,
            description: "Writer name cannot be empty".to_string(),
        });
    }

    // Validate language code format if present (ISO 639-1)
    if let Some(ref lang_code) = record.language_code {
        if !lang_code.as_str().trim().is_empty() && lang_code.as_str().len() != 2 {
            warnings.push(CwrWarning {
                field_name: "language_code",
                field_title: "Language code (optional)",
                source_str: std::borrow::Cow::Owned(lang_code.as_str().to_string()),
                level: WarningLevel::Warning,
                description: "Language code should be 2 characters (ISO 639-1)".to_string(),
            });
        }
    }

    // Validate writer position if present
    if let Some(ref position) = record.writer_position {
        if !position.as_str().trim().is_empty() && position.as_str().len() != 1 {
            warnings.push(CwrWarning {
                field_name: "writer_position",
                field_title: "Writer position (1 char, optional)",
                source_str: std::borrow::Cow::Owned(position.as_str().to_string()),
                level: WarningLevel::Warning,
                description: "Writer position must be exactly 1 character if specified".to_string(),
            });
        }
    }

    warnings
}
