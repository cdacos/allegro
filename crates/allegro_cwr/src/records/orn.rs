use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ORN - Work Origin Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = orn_custom_validate,
    test_data = "ORN0000000100000002LSAMPLE PRODUCTION                                                                                                                                                                                    2022123456789012345678901234567890123456789012345612345678901234561234567890123456ABC123456789012345678912345678901234567890123456701234567890123456789 1"
)]
pub struct OrnRecord {
    #[cwr(title = "Always 'ORN'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Intended purpose", start = 19, len = 3)]
    pub intended_purpose: IntendedPurpose,

    #[cwr(title = "Production title (conditional)", start = 22, len = 60)]
    pub production_title: Option<String>,

    #[cwr(title = "CD identifier (conditional)", start = 82, len = 15)]
    pub cd_identifier: Option<String>,

    #[cwr(title = "Cut number (optional)", start = 97, len = 4)]
    pub cut_number: Option<Number>,

    #[cwr(title = "Library (conditional, v2.1+)", start = 101, len = 60, min_version = 2.1)]
    pub library: Option<String>,

    #[cwr(title = "BLTVR (1 char, optional, v2.1+)", start = 161, len = 1, min_version = 2.1)]
    pub bltvr: Option<String>,

    #[cwr(title = "Filler (optional, v2.1+)", start = 162, len = 25, min_version = 2.1)]
    pub filler: Option<Number>,

    #[cwr(title = "Production number (optional, v2.1+)", start = 187, len = 12, min_version = 2.1)]
    pub production_num: Option<String>,

    #[cwr(title = "Episode title (optional, v2.1+)", start = 199, len = 60, min_version = 2.1)]
    pub episode_title: Option<String>,

    #[cwr(title = "Episode number (optional, v2.1+)", start = 259, len = 20, min_version = 2.1)]
    pub episode_num: Option<String>,

    #[cwr(title = "Year of production (optional, v2.1+)", start = 279, len = 4, min_version = 2.1)]
    pub year_of_production: Option<Number>,

    #[cwr(title = "AVI society code (optional, v2.1+)", start = 283, len = 3, min_version = 2.1)]
    pub avi_society_code: Option<Number>,

    #[cwr(title = "Audio-visual number (optional, v2.1+)", start = 286, len = 15, min_version = 2.1)]
    pub audio_visual_number: Option<String>,

    #[cwr(title = "V-ISAN/ISAN (optional, v2.2+)", start = 301, len = 12, min_version = 2.2)]
    pub v_isan_isan: Option<String>,

    #[cwr(title = "V-ISAN/Episode (optional, v2.2+)", start = 313, len = 4, min_version = 2.2)]
    pub v_isan_episode: Option<String>,

    #[cwr(title = "V-ISAN/Check Digit 1 (1 char, optional, v2.2+)", start = 317, len = 1, min_version = 2.2)]
    pub v_isan_check_digit_1: Option<String>,

    #[cwr(title = "V-ISAN/Version (optional, v2.2+)", start = 318, len = 8, min_version = 2.2)]
    pub v_isan_version: Option<String>,

    #[cwr(title = "V-ISAN/Check Digit 2 (1 char, optional, v2.2+)", start = 326, len = 1, min_version = 2.2)]
    pub v_isan_check_digit_2: Option<String>,

    #[cwr(title = "EIDR (optional, v2.2+)", start = 327, len = 20, min_version = 2.2)]
    pub eidr: Option<String>,

    #[cwr(title = "EIDR/Check Digit (1 char, optional, v2.2+)", start = 347, len = 1, min_version = 2.2)]
    pub eidr_check_digit: Option<String>,
}

// Custom validation function for ORN record
fn orn_custom_validate(record: &mut OrnRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "ORN" {
        warnings.push(CwrWarning {
            field_name: "record_type",
            field_title: "Always 'ORN'",
            source_str: std::borrow::Cow::Owned(record.record_type.clone()),
            level: WarningLevel::Critical,
            description: "Record type must be 'ORN'".to_string(),
        });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate intended purpose is 3 characters
    if record.intended_purpose.as_str().len() != 3 {
        warnings.push(CwrWarning {
            field_name: "intended_purpose",
            field_title: "Intended purpose",
            source_str: std::borrow::Cow::Owned(record.intended_purpose.as_str().to_string()),
            level: WarningLevel::Critical,
            description: "Intended purpose must be exactly 3 characters".to_string(),
        });
    }
    // TODO: Validate intended_purpose against lookup table (e.g., "L" for Library, etc.)

    // Validate cut number is reasonable if present
    if let Some(ref cut_num) = record.cut_number {
        if cut_num.0 > 9999 {
            warnings.push(CwrWarning {
                field_name: "cut_number",
                field_title: "Cut number (optional)",
                source_str: std::borrow::Cow::Owned(cut_num.to_string()),
                level: WarningLevel::Warning,
                description: "Cut number should be a 4-digit number (0000-9999)".to_string(),
            });
        }
    }

    // Validate BLTVR is single character if present
    if let Some(ref bltvr) = record.bltvr {
        if !bltvr.trim().is_empty() && bltvr.len() != 1 {
            warnings.push(CwrWarning {
                field_name: "bltvr",
                field_title: "BLTVR (1 char, optional, v2.1+)",
                source_str: std::borrow::Cow::Owned(bltvr.clone()),
                level: WarningLevel::Warning,
                description: "BLTVR must be exactly 1 character if specified".to_string(),
            });
        }
    }

    // Validate year of production is reasonable if present
    if let Some(ref year) = record.year_of_production {
        if year.0 < 1900 || year.0 > 2100 {
            warnings.push(CwrWarning {
                field_name: "year_of_production",
                field_title: "Year of production (optional, v2.1+)",
                source_str: std::borrow::Cow::Owned(year.to_string()),
                level: WarningLevel::Warning,
                description: "Year of production should be a reasonable year (1900-2100)".to_string(),
            });
        }
    }

    // Validate AVI society code is reasonable if present
    if let Some(ref avi_code) = record.avi_society_code {
        if avi_code.0 > 999 {
            warnings.push(CwrWarning {
                field_name: "avi_society_code",
                field_title: "AVI society code (optional, v2.1+)",
                source_str: std::borrow::Cow::Owned(avi_code.to_string()),
                level: WarningLevel::Warning,
                description: "AVI society code should be a 3-digit number (000-999)".to_string(),
            });
        }
        // TODO: Validate against AVI society code lookup table
    }

    // Validate V-ISAN check digits are single characters if present
    if let Some(ref check_digit) = record.v_isan_check_digit_1 {
        if !check_digit.trim().is_empty() && check_digit.len() != 1 {
            warnings.push(CwrWarning {
                field_name: "v_isan_check_digit_1",
                field_title: "V-ISAN/Check Digit 1 (1 char, optional, v2.2+)",
                source_str: std::borrow::Cow::Owned(check_digit.clone()),
                level: WarningLevel::Warning,
                description: "V-ISAN check digit 1 must be exactly 1 character if specified".to_string(),
            });
        }
    }

    if let Some(ref check_digit) = record.v_isan_check_digit_2 {
        if !check_digit.trim().is_empty() && check_digit.len() != 1 {
            warnings.push(CwrWarning {
                field_name: "v_isan_check_digit_2",
                field_title: "V-ISAN/Check Digit 2 (1 char, optional, v2.2+)",
                source_str: std::borrow::Cow::Owned(check_digit.clone()),
                level: WarningLevel::Warning,
                description: "V-ISAN check digit 2 must be exactly 1 character if specified".to_string(),
            });
        }
    }

    // Validate EIDR check digit is single character if present
    if let Some(ref eidr_check) = record.eidr_check_digit {
        if !eidr_check.trim().is_empty() && eidr_check.len() != 1 {
            warnings.push(CwrWarning {
                field_name: "eidr_check_digit",
                field_title: "EIDR/Check Digit (1 char, optional, v2.2+)",
                source_str: std::borrow::Cow::Owned(eidr_check.clone()),
                level: WarningLevel::Warning,
                description: "EIDR check digit must be exactly 1 character if specified".to_string(),
            });
        }
    }

    // TODO: Add cross-field validation logic for V-ISAN and EIDR complete identifier validation
    // TODO: Validate CD identifier format if present
    // TODO: Validate episode number format if present

    warnings
}
