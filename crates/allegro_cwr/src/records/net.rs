//! NET - Non-Roman Alphabet Entire Work Title for Excerpts/Components/Versions Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// NET - Non-Roman Alphabet Entire Work Title for Excerpts/Components/Versions Record
/// Also handles NCT (Non-Roman Alphabet Title for Components) and NVT (Non-Roman Alphabet Original Title for Versions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetRecord {
    /// "NET", "NCT", or "NVT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Title (640 chars)
    pub title: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NetRecord {
    fn post_process_fields(_record: &mut NetRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for NET
    }
}

impl_cwr_parsing! {
    NetRecord {
        record_type: (0, 3, required, one_of(&["NET", "NCT", "NVT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        title: (19, 659, required),
        language_code: (659, 661, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(NetRecord, ["NET0000000100000001Non-Roman Entire Work Title                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            EN"]);
