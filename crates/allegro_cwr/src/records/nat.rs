use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NAT - Non-Roman Alphabet Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = nat_custom_validate,
    test_data = "NAT00000455000000170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ATEN"
)]
pub struct NatRecord {
    #[cwr(title = "Always 'NAT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Title", start = 19, len = 640)]
    pub title: String,

    #[cwr(title = "Title type", start = 659, len = 2)]
    pub title_type: TitleType,

    #[cwr(title = "Language code (optional)", start = 661, len = 2)]
    pub language_code: Option<String>,
}

// Custom validation function for NAT record
fn nat_custom_validate(record: &mut NatRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NAT" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'NAT'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'NAT'".to_string() });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate title is not empty
    if record.title.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "title", field_title: "Title", source_str: std::borrow::Cow::Owned(record.title.clone()), level: WarningLevel::Critical, description: "Title cannot be empty".to_string() });
    }

    // Validate language code format if present (ISO 639-1)
    if let Some(ref lang_code) = record.language_code {
        if !lang_code.trim().is_empty() && lang_code.len() != 2 {
            warnings.push(CwrWarning { field_name: "language_code", field_title: "Language code (optional)", source_str: std::borrow::Cow::Owned(lang_code.clone()), level: WarningLevel::Warning, description: "Language code should be 2 characters (ISO 639-1)".to_string() });
        }
    }

    // Note: TitleType validation is handled by the domain type parser

    warnings
}
