//! NAT - Non-Roman Alphabet Title Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// NAT - Non-Roman Alphabet Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NatRecord {
    /// Always "NAT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Title (640 chars)
    pub title: String,

    /// Title type (2 chars)
    pub title_type: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NatRecord {
    fn post_process_fields(_record: &mut NatRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for NAT
    }
}

impl_cwr_parsing! {
    NatRecord {
        record_type: (0, 3, required, one_of(&["NAT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        title: (19, 659, required),
        title_type: (659, 661, required),
        language_code: (661, 663, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(NatRecord, ["NAT0000000100000001NON-ROMAN TITLE                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  OTEN"]);
