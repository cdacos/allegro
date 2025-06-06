//! MSG - Message Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// MSG - Message Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgRecord {
    /// Always "MSG"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Message type (1 char)
    pub message_type: String,

    /// Original record sequence number (8 chars)
    pub original_record_sequence_num: String,

    /// Record type (3 chars)
    pub record_type_field: String,

    /// Message level (1 char)
    pub message_level: String,

    /// Validation number (3 chars)
    pub validation_number: String,

    /// Message text (150 chars)
    pub message_text: String,
}


impl_cwr_parsing! {
    MsgRecord {
        record_type: (0, 3, required, one_of(&["MSG"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        message_type: (19, 20, required),
        original_record_sequence_num: (20, 28, required),
        record_type_field: (28, 31, required),
        message_level: (31, 32, required),
        validation_number: (32, 35, required),
        message_text: (35, 185, required),
    }
}

impl_cwr_parsing_test_roundtrip!(MsgRecord, ["MSG0000000100000001E00000002NWRR001Record rejected due to invalid format                                                                                                                            "]);
