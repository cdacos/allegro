use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ARI - Additional Related Information Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ari_custom_validate, test_data = "ARI0000000100000001021              ALL  Additional related information note for the work                                                                                                                ")]
pub struct AriRecord {
    #[cwr(title = "Always 'ARI'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Society number", start = 19, len = 3)]
    pub society_num: SocietyCode,

    #[cwr(title = "Work number (conditional)", start = 22, len = 14)]
    pub work_num: Option<String>,

    #[cwr(title = "Type of right", start = 36, len = 3)]
    pub type_of_right: TypeOfRight,

    #[cwr(title = "Subject code (conditional)", start = 39, len = 2)]
    pub subject_code: Option<SubjectCode>,

    #[cwr(title = "Note (conditional)", start = 41, len = 160)]
    pub note: Option<String>,
}

// Custom validation function for ARI record
fn ari_custom_validate(record: &mut AriRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "ARI" {
        warnings.push(CwrWarning {
            field_name: "record_type",
            field_title: "Always 'ARI'",
            source_str: std::borrow::Cow::Owned(record.record_type.clone()),
            level: WarningLevel::Critical,
            description: "Record type must be 'ARI'".to_string(),
        });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate society number is numeric (3 digits)
    if !record.society_num.as_str().chars().all(|c| c.is_ascii_digit()) || record.society_num.as_str().len() != 3 {
        warnings.push(CwrWarning {
            field_name: "society_num",
            field_title: "Society number",
            source_str: std::borrow::Cow::Owned(record.society_num.as_str().to_string()),
            level: WarningLevel::Critical,
            description: "Society number must be 3 numeric digits".to_string(),
        });
    }

    // TODO: Validate society_num against society lookup table

    // Type of right validation is now handled by the TypeOfRight domain type

    // Conditional validation: at least one of work_num, subject_code, or note must be present
    if record.work_num.is_none() && record.subject_code.is_none() && record.note.is_none() {
        warnings.push(CwrWarning {
            field_name: "work_num",
            field_title: "Work number (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "At least one of work number, subject code, or note must be provided".to_string(),
        });
    }

    warnings
}
