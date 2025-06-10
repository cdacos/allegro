use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// IND - Instrumentation Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ind_custom_validate, test_data = "IND0000000100000001PNO004")]
pub struct IndRecord {
    #[cwr(title = "Always 'IND'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Instrument code", start = 19, len = 3)]
    pub instrument_code: InstrumentCode,

    #[cwr(title = "Number of players (optional)", start = 22, len = 3)]
    pub number_of_players: Option<Number>,
}

// Custom validation function for IND record
fn ind_custom_validate(record: &mut IndRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "IND" {
        warnings.push(CwrWarning {
            field_name: "record_type",
            field_title: "Always 'IND'",
            source_str: std::borrow::Cow::Owned(record.record_type.clone()),
            level: WarningLevel::Critical,
            description: "Record type must be 'IND'".to_string(),
        });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Instrument code validation is now handled by the InstrumentCode domain type

    // Validate number of players if present
    if let Some(ref players) = record.number_of_players {
        if players.0 == 0 {
            warnings.push(CwrWarning {
                field_name: "number_of_players",
                field_title: "Number of players (optional)",
                source_str: std::borrow::Cow::Owned(players.to_string()),
                level: WarningLevel::Warning,
                description: "Number of players should be greater than 0 if specified".to_string(),
            });
        }
    }

    warnings
}
