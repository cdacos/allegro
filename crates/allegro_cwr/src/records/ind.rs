//! IND - Instrumentation Detail Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// IND - Instrumentation Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndRecord {
    /// Always "IND"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Instrument code (3 chars)
    pub instrument_code: String,

    /// Number of players (3 chars, optional)
    pub number_of_players: Option<String>,
}

impl IndRecord {
    fn post_process_fields(_record: &mut IndRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for IND
    }
}

impl_cwr_parsing! {
    IndRecord {
        record_type: (0, 3, required, one_of(&["IND"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        instrument_code: (19, 22, required),
        number_of_players: (22, 25, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(IndRecord, ["IND0000000100000001PNO004"]);
