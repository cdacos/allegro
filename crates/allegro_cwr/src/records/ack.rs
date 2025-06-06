//! ACK - Acknowledgement of Transaction Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ACK - Acknowledgement of Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "ACK0000000100000001200501011200000000100000001NWR                                                                                                    20050102AS")]
pub struct AckRecord {
    #[cwr(title = "Always 'ACK'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Creation date of original file YYYYMMDD", start = 19, len = 8)]
    pub creation_date: String,

    #[cwr(title = "Creation time of original file HHMMSS", start = 27, len = 6)]
    pub creation_time: String,

    #[cwr(title = "Original group ID", start = 33, len = 5)]
    pub original_group_id: String,

    #[cwr(title = "Original transaction sequence number", start = 38, len = 8)]
    pub original_transaction_sequence_num: String,

    #[cwr(title = "Original transaction type", start = 46, len = 3)]
    pub original_transaction_type: String,

    #[cwr(title = "Creation title (conditional)", start = 49, len = 60)]
    pub creation_title: Option<String>,

    #[cwr(title = "Submitter creation number (conditional)", start = 109, len = 20)]
    pub submitter_creation_num: Option<String>,

    #[cwr(title = "Recipient creation number (conditional)", start = 129, len = 20)]
    pub recipient_creation_num: Option<String>,

    #[cwr(title = "Processing date YYYYMMDD", start = 149, len = 8)]
    pub processing_date: String,

    #[cwr(title = "Transaction status", start = 157, len = 2)]
    pub transaction_status: String,
}
