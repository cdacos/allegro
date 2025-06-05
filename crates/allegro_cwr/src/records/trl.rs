//! TRL - Transmission Trailer Record
//!
//! The Transmission Trailer record marks the end of a CWR transmission and contains summary counts.

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// TRL - Transmission Trailer Record
///
/// Marks the end of a CWR transmission and contains summary counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrlRecord {
    /// Always "TRL"
    pub record_type: String,

    /// Group count (5 chars)
    pub group_count: String,

    /// Transaction count (8 chars)
    pub transaction_count: String,

    /// Record count (8 chars)
    pub record_count: String,
}

impl TrlRecord {
    fn post_process_fields(_record: &mut TrlRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for TRL
    }
}

impl_cwr_parsing! {
    TrlRecord {
        record_type: (0, 3, required, one_of(&["TRL"])),
        group_count: (3, 8, required),
        transaction_count: (8, 16, required),
        record_count: (16, 24, required),
    }
}

impl_cwr_parsing_test_roundtrip!(TrlRecord, "TRL000010000001400000367");
