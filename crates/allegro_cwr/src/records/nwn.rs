use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NWN - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = nwn_custom_validate,
    test_data = "NWN0000000100000001123456789WRITER LAST NAME                                                                                                                                                                                                                                                                                                                                                                                                     EN  "
)]
pub struct NwnRecord {
    #[cwr(title = "Always 'NWN'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Interested party number (conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Writer last name", start = 28, len = 160)]
    pub writer_last_name: NonRomanAlphabet,

    #[cwr(title = "Writer first name (optional)", start = 188, len = 160)]
    pub writer_first_name: Option<NonRomanAlphabet>,

    #[cwr(title = "Language code (optional)", start = 348, len = 2)]
    pub language_code: Option<LanguageCode>,
}

// Custom validation function for NWN record
fn nwn_custom_validate(record: &mut NwnRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NWN" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'NWN'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'NWN'".to_string() });
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

    // Validate writer last name is not empty
    if record.writer_last_name.as_str().trim().is_empty() {
        warnings.push(CwrWarning { field_name: "writer_last_name", field_title: "Writer last name", source_str: std::borrow::Cow::Owned(record.writer_last_name.as_str().to_string()), level: WarningLevel::Critical, description: "Writer last name cannot be empty".to_string() });
    }

    // Validate language code format if present (ISO 639-1)
    if let Some(ref lang_code) = record.language_code {
        if !lang_code.as_str().trim().is_empty() && lang_code.as_str().len() != 2 {
            warnings.push(CwrWarning { field_name: "language_code", field_title: "Language code (optional)", source_str: std::borrow::Cow::Owned(lang_code.as_str().to_string()), level: WarningLevel::Warning, description: "Language code should be 2 characters (ISO 639-1)".to_string() });
        }
    }

    warnings
}
