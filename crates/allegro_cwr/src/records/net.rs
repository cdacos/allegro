use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Also handles NCT (Non-Roman Alphabet Title for Components) and NVT (Non-Roman Alphabet Original Title for Versions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = net_custom_validate,
    codes = ["NET", "NCT", "NVT"],
    test_data = "NET0000000100000002PLACEHOLDER TITLE                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               EN"
)]
pub struct NetRecord {
    #[cwr(title = "'NET', 'NCT', or 'NVT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Title", start = 19, len = 640)]
    pub title: String,

    #[cwr(title = "Language code (optional)", start = 659, len = 2)]
    pub language_code: Option<LanguageCode>,
}

// Custom validation function for NET/NCT/NVT records
fn net_custom_validate(record: &mut NetRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    match record.record_type.as_str() {
        "NET" | "NCT" | "NVT" => {}
        _ => {
            warnings.push(CwrWarning {
                field_name: "record_type",
                field_title: "'NET', 'NCT', or 'NVT'",
                source_str: std::borrow::Cow::Owned(record.record_type.clone()),
                level: WarningLevel::Critical,
                description: "Record type must be 'NET', 'NCT', or 'NVT'".to_string(),
            });
        }
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate title is not empty
    if record.title.trim().is_empty() {
        warnings.push(CwrWarning {
            field_name: "title",
            field_title: "Title",
            source_str: std::borrow::Cow::Owned(record.title.clone()),
            level: WarningLevel::Critical,
            description: "Title cannot be empty".to_string(),
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

    warnings
}
