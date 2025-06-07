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
    pub sender_name: String,

    #[cwr(title = "EDI standard version number", start = 59, len = 5)]
    pub edi_standard_version_number: EdiStandardVersion,

    #[cwr(title = "Creation date YYYYMMDD", start = 64, len = 8)]
    pub creation_date: Date,

    #[cwr(title = "Creation time HHMMSS", start = 72, len = 6)]
    pub creation_time: Time,

    #[cwr(title = "Transmission date YYYYMMDD", start = 78, len = 8)]
    pub transmission_date: Date,

    #[cwr(title = "Character set (v2.1+)", start = 86, len = 15)]
    pub character_set: Option<CharacterSet>,

    #[cwr(title = "Version (v2.2+)", start = 101, len = 3)]
    pub version: CwrVersion,

    #[cwr(title = "Revision (v2.2+)", start = 104, len = 3)]
    pub revision: CwrRevision,

    #[cwr(title = "Software package (v2.2+)", start = 107, len = 30)]
    pub software_package: Option<String>,

    #[cwr(title = "Software package version (v2.2+)", start = 137, len = 30)]
    pub software_package_version: Option<String>,
}

// Custom validation function for HDR record
fn hdr_custom_validate(record: &mut HdrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Complex Sender Type + Sender ID validation based on CWR 2.2 spec
    match &record.sender_type {
        SenderType::Publisher | SenderType::AdministrativeAgency | SenderType::Writer => {
            // TODO: Validate sender_id against CWR Sender ID and Codes Table
            // TODO: Validate sender_name matches entry in CWR Sender ID and Codes Table

            // Basic validation: IPI should be numeric for these sender types
            let sender_id_str = record.sender_id.as_str();
            if !sender_id_str.chars().all(|c| c.is_ascii_digit()) {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(sender_id_str.to_string()), level: WarningLevel::Warning, description: format!("Sender ID should be numeric for sender type {}", record.sender_type.as_str()) });
            }
        }
        SenderType::Society => {
            // TODO: Validate sender_id against Society Code Table
            // TODO: Validate sender_name matches entry in Society Code Table
        }
        SenderType::NumericPrefix(prefix) => {
            // Handle IPNN > 9 digits case: prefix + sender_id should be valid IPI
            let combined_id = format!("{}{}", prefix, record.sender_id.as_str());
            if !combined_id.chars().all(|c| c.is_ascii_digit()) {
                warnings.push(CwrWarning { field_name: "sender_id", field_title: "Sender ID", source_str: std::borrow::Cow::Owned(combined_id), level: WarningLevel::Critical, description: "Combined sender type prefix + sender ID must be numeric for IPNN > 9 digits".to_string() });
            }
            // TODO: Validate combined_id against CWR Sender ID and Codes Table
        }
    }

    warnings
}
