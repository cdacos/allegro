use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// MSG - Message Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "MSG0000000100000001E00000002NWRR001Record rejected due to invalid format                                                                                                                            ")]
pub struct MsgRecord {
    #[cwr(title = "Always 'MSG'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Message type (1 char)", start = 19, len = 1)]
    pub message_type: String,

    #[cwr(title = "Original record sequence number", start = 20, len = 8)]
    pub original_record_sequence_num: String,

    #[cwr(title = "Record type", start = 28, len = 3)]
    pub record_type_field: String,

    #[cwr(title = "Message level (1 char)", start = 31, len = 1)]
    pub message_level: String,

    #[cwr(title = "Validation number", start = 32, len = 3)]
    pub validation_number: String,

    #[cwr(title = "Message text", start = 35, len = 150)]
    pub message_text: String,
}
