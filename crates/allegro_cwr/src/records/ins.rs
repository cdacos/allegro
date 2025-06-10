use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// INS - Instrumentation Summary Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ins_custom_validate, test_data = "INS000000010000000104 ORCHFULL ORCHESTRA WITH STRINGS AND BRASS SECTION    ")]
pub struct InsRecord {
    #[cwr(title = "Always 'INS'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Number of voices (optional)", start = 19, len = 3)]
    pub number_of_voices: Option<Number>,

    #[cwr(title = "Standard instrumentation type (conditional)", start = 22, len = 3)]
    pub standard_instrumentation_type: Option<LookupPlaceholder>,

    #[cwr(title = "Instrumentation description (conditional)", start = 25, len = 50)]
    pub instrumentation_description: Option<String>,
}

// Custom validation function for INS record
fn ins_custom_validate(record: &mut InsRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "INS" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'INS'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'INS'".to_string() });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate number of voices if present
    if let Some(ref voices) = record.number_of_voices {
        if voices.0 == 0 {
            warnings.push(CwrWarning { field_name: "number_of_voices", field_title: "Number of voices (optional)", source_str: std::borrow::Cow::Owned(voices.to_string()), level: WarningLevel::Warning, description: "Number of voices should be greater than 0 if specified".to_string() });
        }
    }

    // Validate standard instrumentation type if present
    if let Some(ref inst_type) = record.standard_instrumentation_type {
        if !inst_type.as_str().trim().is_empty() && inst_type.as_str().len() != 3 {
            warnings.push(CwrWarning {
                field_name: "standard_instrumentation_type",
                field_title: "Standard instrumentation type (conditional)",
                source_str: std::borrow::Cow::Owned(inst_type.as_str().to_string()),
                level: WarningLevel::Critical,
                description: "Standard instrumentation type must be exactly 3 characters if specified".to_string(),
            });
        }
        // TODO: Validate against standard instrumentation type lookup table
    }

    // Conditional validation: at least one of standard_instrumentation_type or instrumentation_description must be present
    if record.standard_instrumentation_type.as_ref().map(|s| s.as_str().trim().is_empty()).unwrap_or(true) && record.instrumentation_description.as_ref().map(|s| s.trim().is_empty()).unwrap_or(true) {
        warnings.push(CwrWarning {
            field_name: "standard_instrumentation_type",
            field_title: "Standard instrumentation type (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Either standard instrumentation type or instrumentation description must be provided".to_string(),
        });
    }

    warnings
}
