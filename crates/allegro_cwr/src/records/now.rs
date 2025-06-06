//! NOW - Non-Roman Alphabet Writer Name Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// NOW - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NowRecord {
    /// Always "NOW"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Writer name (160 chars)
    pub writer_name: String,

    /// Writer first name (160 chars)
    pub writer_first_name: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Writer position (1 char, optional)
    pub writer_position: Option<String>,
}


impl_cwr_parsing! {
    NowRecord {
        record_type: (0, 3, required, one_of(&["NOW"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        writer_name: (19, 179, required),
        writer_first_name: (179, 339, required),
        language_code: (339, 341, optional),
        writer_position: (341, 342, optional),
    }
}
