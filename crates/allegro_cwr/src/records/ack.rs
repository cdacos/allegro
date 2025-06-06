//! ACK - Acknowledgement of Transaction Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// ACK - Acknowledgement of Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AckRecord {
    /// Always "ACK"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Creation date of original file YYYYMMDD (8 chars)
    pub creation_date: String,

    /// Creation time of original file HHMMSS (6 chars)
    pub creation_time: String,

    /// Original group ID (5 chars)
    pub original_group_id: String,

    /// Original transaction sequence number (8 chars)
    pub original_transaction_sequence_num: String,

    /// Original transaction type (3 chars)
    pub original_transaction_type: String,

    /// Creation title (60 chars, conditional)
    pub creation_title: Option<String>,

    /// Submitter creation number (20 chars, conditional)
    pub submitter_creation_num: Option<String>,

    /// Recipient creation number (20 chars, conditional)
    pub recipient_creation_num: Option<String>,

    /// Processing date YYYYMMDD (8 chars)
    pub processing_date: String,

    /// Transaction status (2 chars)
    pub transaction_status: String,
}


impl_cwr_parsing! {
    AckRecord {
        record_type: (0, 3, required, one_of(&["ACK"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        creation_date: (19, 27, required),
        creation_time: (27, 33, required),
        original_group_id: (33, 38, required),
        original_transaction_sequence_num: (38, 46, required),
        original_transaction_type: (46, 49, required),
        creation_title: (49, 109, optional),
        submitter_creation_num: (109, 129, optional),
        recipient_creation_num: (129, 149, optional),
        processing_date: (149, 157, required),
        transaction_status: (157, 159, required),
    }
    with_tests ["ACK0000000100000001200501011200000000100000001NWR                                                                                                    20050102AS"]
}

