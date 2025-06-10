use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// MSG - Message Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = msg_custom_validate, test_data = "MSG0000000100000001E00000002NWRR001Record rejected due to invalid format                                                                                                                            ")]
pub struct MsgRecord {
    #[cwr(title = "Always 'MSG'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Message type (1 char)", start = 19, len = 1)]
    pub message_type: MessageType,

    #[cwr(title = "Original record sequence number", start = 20, len = 8)]
    pub original_record_sequence_num: Number,

    #[cwr(title = "Record type", start = 28, len = 3)]
    pub record_type_field: String,

    #[cwr(title = "Message level (1 char)", start = 31, len = 1)]
    pub message_level: MessageLevel,

    #[cwr(title = "Validation number", start = 32, len = 3)]
    pub validation_number: String,

    #[cwr(title = "Message text", start = 35, len = 150)]
    pub message_text: String,
}

// Custom validation function for MSG record
fn msg_custom_validate(record: &mut MsgRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "MSG" {
        warnings.push(CwrWarning { field_name: "record_type", field_title: "Always 'MSG'", source_str: std::borrow::Cow::Owned(record.record_type.clone()), level: WarningLevel::Critical, description: "Record type must be 'MSG'".to_string() });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Message type validation is now handled by the MessageType domain type

    // Validate original record sequence number is numeric
    // Validate record type field (3 characters, uppercase)
    if record.record_type_field.len() != 3 {
        warnings.push(CwrWarning { field_name: "record_type_field", field_title: "Record type", source_str: std::borrow::Cow::Owned(record.record_type_field.clone()), level: WarningLevel::Critical, description: "Record type field must be exactly 3 characters".to_string() });
    }
    // TODO: Validate against known CWR record types

    // Message level validation is now handled by the MessageLevel domain type

    // Validate validation number is 3 characters
    if record.validation_number.len() != 3 {
        warnings.push(CwrWarning { field_name: "validation_number", field_title: "Validation number", source_str: std::borrow::Cow::Owned(record.validation_number.clone()), level: WarningLevel::Critical, description: "Validation number must be exactly 3 characters".to_string() });
    }

    // Validate message text is not empty
    if record.message_text.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "message_text", field_title: "Message text", source_str: std::borrow::Cow::Owned(record.message_text.clone()), level: WarningLevel::Critical, description: "Message text cannot be empty".to_string() });
    }

    warnings
}
