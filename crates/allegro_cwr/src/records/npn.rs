use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPN - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = npn_custom_validate,
    test_data = "NPN0000000100000002011234567890PLACEHOLDER PUBLISHER NAME                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          EN"
)]
pub struct NpnRecord {
    #[cwr(title = "Always 'NPN'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Publisher sequence number", start = 19, len = 2)]
    pub publisher_sequence_num: PublisherSequenceNumber,

    #[cwr(title = "Interested party number", start = 21, len = 9)]
    pub interested_party_num: String,

    #[cwr(title = "Publisher name", start = 30, len = 480)]
    pub publisher_name: String,

    #[cwr(title = "Language code (optional)", start = 510, len = 2)]
    pub language_code: Option<LookupPlaceholder>,
}

// Custom validation function for NPN record
fn npn_custom_validate(record: &mut NpnRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NPN" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'NPN'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'NPN'".to_string() });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate interested party number format
    if record.interested_party_num.len() != 9 {
        warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number", source_str: std::borrow::Cow::Owned(record.interested_party_num.clone()), level: WarningLevel::Critical, description: "Interested party number must be exactly 9 characters".to_string() });
    }

    // Basic IPI format validation (usually numeric)
    if !record.interested_party_num.chars().all(|c| c.is_ascii_digit() || c.is_ascii_whitespace()) {
        warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number", source_str: std::borrow::Cow::Owned(record.interested_party_num.clone()), level: WarningLevel::Warning, description: "Interested party number should be numeric".to_string() });
    }

    // Validate publisher name is not empty
    if record.publisher_name.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "publisher_name", field_title: "Publisher name", source_str: std::borrow::Cow::Owned(record.publisher_name.clone()), level: WarningLevel::Critical, description: "Publisher name cannot be empty".to_string() });
    }

    // Validate language code format if present (ISO 639-1)
    if let Some(ref lang_code) = record.language_code {
        if !lang_code.as_str().trim().is_empty() && lang_code.as_str().len() != 2 {
            warnings.push(CwrWarning { field_name: "language_code", field_title: "Language code (optional)", source_str: std::borrow::Cow::Owned(lang_code.as_str().to_string()), level: WarningLevel::Warning, description: "Language code should be 2 characters (ISO 639-1)".to_string() });
        }
    }

    // Note: PublisherSequenceNumber validation is handled by the domain type parser

    warnings
}
