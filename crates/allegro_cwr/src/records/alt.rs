//! ALT - Alternate Title Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// ALT - Alternate Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AltRecord {
    /// Always "ALT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Alternate title (60 chars)
    pub alternate_title: String,

    /// Title type (2 chars)
    pub title_type: String,

    /// Language code (2 chars, conditional)
    pub language_code: Option<String>,
}

impl AltRecord {
    fn post_process_fields(_record: &mut AltRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for ALT
    }
}

impl_cwr_parsing! {
    AltRecord {
        record_type: (0, 3, required, one_of(&["ALT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        alternate_title: (19, 79, required),
        title_type: (79, 81, required),
        language_code: (81, 83, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(AltRecord, "ALT0000000100000001ALTERNATE TITLE                                          ATEN");
