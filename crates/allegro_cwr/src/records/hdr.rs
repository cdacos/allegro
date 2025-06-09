use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Contains information about the sender and the transmission itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = hdr_custom_validate, test_data = "HDRPB123456789BMI MUSIC                                    01.1020050101120000200501010              2.2  1DEV MUSIC SOFTWARE VERSION 1.0  MUSIC PACKAGE VERSION 2.0   ")]
pub struct HdrRecord {
    #[cwr(title = "Always 'HDR'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Sender type", start = 3, len = 2)]
    pub sender_type: SenderType,

    #[cwr(title = "Sender ID", start = 5, len = 9)]
    pub sender_id: SenderId,

    #[cwr(title = "Sender name", start = 14, len = 45)]
    pub sender_name: SenderName,

    #[cwr(title = "EDI standard version number", start = 59, len = 5)]
    pub edi_standard_version_number: EdiStandardVersion,

    #[cwr(title = "Creation date YYYYMMDD", start = 64, len = 8)]
    pub creation_date: Date,

    #[cwr(title = "Creation time HHMMSS", start = 72, len = 6)]
    pub creation_time: Time,

    #[cwr(title = "Transmission date YYYYMMDD", start = 78, len = 8)]
    pub transmission_date: Date,

    #[cwr(title = "Character set (v2.1+)", start = 86, len = 15, min_version = 2.1)]
    pub character_set: Option<CharacterSet>,

    #[cwr(title = "Version (v2.2+)", start = 101, len = 3, min_version = 2.2)]
    pub version: Option<CwrVersion>,

    #[cwr(title = "Revision (v2.2+)", start = 104, len = 3, min_version = 2.2)]
    pub revision: Option<CwrRevision>,

    #[cwr(title = "Software package (v2.2+)", start = 107, len = 30, min_version = 2.2)]
    pub software_package: Option<String>,

    #[cwr(title = "Software package version (v2.2+)", start = 137, len = 30, min_version = 2.2)]
    pub software_package_version: Option<String>,
}

// Custom validation function for HDR record
fn hdr_custom_validate(record: &mut HdrRecord) -> Vec<CwrWarning<'static>> {
    use crate::lookups::society_codes::is_valid_society_code;
    use crate::lookups::society_members::{get_society_name_for_transmitter, is_valid_transmitter_code};

    let mut warnings = Vec::new();

    // Complex Sender Type + Sender ID + Sender Name cross-validation based on CWR 2.2 spec
    match &record.sender_type {
        SenderType::Publisher | SenderType::AdministrativeAgency | SenderType::Writer => {
            // For PB, AA, WR: sender_id should be numeric IPI and sender_name should match
            let sender_id_str = record.sender_id.as_str();
            let sender_name_str = record.sender_name.as_str();

            // Validate IPI format (should be numeric, 9-11 digits)
            if !sender_id_str.chars().all(|c| c.is_ascii_digit()) {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(sender_id_str.to_string()), level: WarningLevel::Warning, description: format!("Sender ID should be numeric IPI for sender type {}", record.sender_type.as_str()) });
            } else if sender_id_str.len() < 9 || sender_id_str.len() > 11 {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(sender_id_str.to_string()), level: WarningLevel::Warning, description: format!("IPI should be 9-11 digits, got {} digits", sender_id_str.len()) });
            }

            // Note: Full validation against CWR Sender ID and Codes Table would require
            // additional lookup tables not currently available in the CSV files
            if sender_name_str.is_empty() {
                warnings.push(CwrWarning { field_name: "sender_name", field_title: "Sender name", source_str: std::borrow::Cow::Owned(sender_name_str.to_string()), level: WarningLevel::Critical, description: "Sender name is required for Publisher/Administrative Agency/Writer".to_string() });
            }
        }
        SenderType::Society => {
            // For SO: sender_id should be society code and sender_name should match society name
            let sender_id_str = record.sender_id.as_str();
            let sender_name_str = record.sender_name.as_str();

            // Check if sender_id is a valid society code
            if !is_valid_society_code(sender_id_str) {
                // Maybe it's a transmitter code?
                if !is_valid_transmitter_code(sender_id_str) {
                    warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(sender_id_str.to_string()), level: WarningLevel::Warning, description: format!("Sender ID '{}' not found in society codes or transmitter codes tables", sender_id_str) });
                } else {
                    // It's a valid transmitter code, check if name matches
                    if let Some(expected_name) = get_society_name_for_transmitter(sender_id_str) {
                        if !sender_name_str.eq_ignore_ascii_case(expected_name) && !sender_name_str.is_empty() {
                            warnings.push(CwrWarning {
                                field_name: "sender_name",
                                field_title: "Sender name",
                                source_str: std::borrow::Cow::Owned(sender_name_str.to_string()),
                                level: WarningLevel::Warning,
                                description: format!("Sender name '{}' does not match expected name '{}' for transmitter code '{}'", sender_name_str, expected_name, sender_id_str),
                            });
                        }
                    }
                }
            } else {
                // It's a valid society code, validate sender_name consistency
                if sender_name_str.is_empty() {
                    warnings.push(CwrWarning { field_name: "sender_name", field_title: "Sender name", source_str: std::borrow::Cow::Owned(sender_name_str.to_string()), level: WarningLevel::Warning, description: "Sender name should be provided for society sender type".to_string() });
                } else {
                    // Check if sender_name matches the society code
                    if !is_valid_society_code(sender_name_str) {
                        warnings.push(CwrWarning {
                            field_name: "sender_name",
                            field_title: "Sender name",
                            source_str: std::borrow::Cow::Owned(sender_name_str.to_string()),
                            level: WarningLevel::Info,
                            description: format!("Sender name '{}' does not match society code format - may be organization display name", sender_name_str),
                        });
                    }
                }
            }
        }
        SenderType::NumericPrefix(prefix) => {
            // Handle IPNN > 9 digits case: prefix + sender_id should form valid IPI
            let sender_id_str = record.sender_id.as_str();
            let sender_name_str = record.sender_name.as_str();
            let combined_id = format!("{}{}", prefix, sender_id_str);

            if !combined_id.chars().all(|c| c.is_ascii_digit()) {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(combined_id.clone()), level: WarningLevel::Critical, description: "Combined sender type prefix + sender ID must be numeric for IPNN > 9 digits".to_string() });
            } else if combined_id.len() < 10 || combined_id.len() > 12 {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(combined_id), level: WarningLevel::Warning, description: "Combined IPI should be 10-12 digits for numeric prefix case".to_string() });
            }

            if sender_name_str.is_empty() {
                warnings.push(CwrWarning { field_name: "sender_name", field_title: "Sender name", source_str: std::borrow::Cow::Owned(sender_name_str.to_string()), level: WarningLevel::Critical, description: "Sender name is required for numeric prefix IPI".to_string() });
            }
        }
    }

    warnings
}
