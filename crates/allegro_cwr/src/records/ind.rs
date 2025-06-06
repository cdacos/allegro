//! IND - Instrumentation Detail Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// IND - Instrumentation Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "IND0000000100000001PNO004")]
pub struct IndRecord {
    #[cwr(title = "Always 'IND'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Instrument code", start = 19, len = 3)]
    pub instrument_code: String,

    #[cwr(title = "Number of players (optional)", start = 22, len = 3)]
    pub number_of_players: Option<String>,
}
