use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPA - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = npa_custom_validate,
    test_data = "NPA000000010000000212345678 PLACEHOLDER INTERESTED PARTY NAME                                                                                                                               PLACEHOLDER FIRST NAME                                                                                                                                          EN"
)]
pub struct NpaRecord {
    #[cwr(title = "Always 'NPA'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Interested party number (conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Interested party name", start = 28, len = 160)]
    pub interested_party_name: String,

    #[cwr(title = "Interested party writer first name", start = 188, len = 160)]
    pub interested_party_writer_first_name: String,

    #[cwr(title = "Language code (optional)", start = 348, len = 2)]
    pub language_code: Option<LookupPlaceholder>,
}

// Custom validation function for NPA record
fn npa_custom_validate(record: &mut NpaRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NPA" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'NPA'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'NPA'".to_string() });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate interested party number format if present
    if let Some(ref ip_num) = record.interested_party_num {
        if !ip_num.trim().is_empty() {
            if ip_num.len() != 9 {
                warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number (conditional)", source_str: std::borrow::Cow::Owned(ip_num.clone()), level: WarningLevel::Warning, description: "Interested party number should be 9 characters if specified".to_string() });
            }
            // Basic IPI format validation (usually numeric)
            if !ip_num.chars().all(|c| c.is_ascii_digit() || c.is_ascii_whitespace()) {
                warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number (conditional)", source_str: std::borrow::Cow::Owned(ip_num.clone()), level: WarningLevel::Warning, description: "Interested party number should be numeric".to_string() });
            }
        }
    }

    // Validate interested party name is not empty
    if record.interested_party_name.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "interested_party_name", field_title: "Interested party name", source_str: std::borrow::Cow::Owned(record.interested_party_name.clone()), level: WarningLevel::Critical, description: "Interested party name cannot be empty".to_string() });
    }

    // Validate language code format if present (ISO 639-1)
    if let Some(ref lang_code) = record.language_code {
        if !lang_code.as_str().trim().is_empty() && lang_code.as_str().len() != 2 {
            warnings.push(CwrWarning { field_name: "language_code", field_title: "Language code (optional)", source_str: std::borrow::Cow::Owned(lang_code.as_str().to_string()), level: WarningLevel::Warning, description: "Language code should be 2 characters (ISO 639-1)".to_string() });
        }
    }

    warnings
}
