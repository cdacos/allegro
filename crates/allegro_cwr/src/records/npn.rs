//! NPN - Non-Roman Alphabet Publisher Name Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// NPN - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpnRecord {
    /// Always "NPN"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher sequence number (2 chars)
    pub publisher_sequence_num: String,

    /// Interested party number (9 chars)
    pub interested_party_num: String,

    /// Publisher name (480 chars)
    pub publisher_name: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NpnRecord {
    fn post_process_fields(_record: &mut NpnRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for NPN
    }
}

impl_cwr_parsing! {
    NpnRecord {
        record_type: (0, 3, required, one_of(&["NPN"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        publisher_sequence_num: (19, 21, required),
        interested_party_num: (21, 30, required),
        publisher_name: (30, 510, required),
        language_code: (510, 512, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(
    NpnRecord,
    [
        "NPN0000000100000001011234567890PUBLISHER NAME                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          EN"
    ]
);
