//! GRT - Group Trailer Record
//!
//! The Group Trailer record marks the end of a group and contains summary counts for that group.

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// GRT - Group Trailer Record
///
/// Marks the end of a group and contains summary counts for that group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrtRecord {
    /// Always "GRT"
    pub record_type: String,

    /// Group ID (5 chars)
    pub group_id: String,

    /// Transaction count (8 chars)
    pub transaction_count: String,

    /// Record count (8 chars)
    pub record_count: String,

    /// Currency indicator (3 chars, conditional)
    pub currency_indicator: Option<String>,

    /// Total monetary value (10 chars, optional)
    pub total_monetary_value: Option<String>,
}


impl_cwr_parsing! {
    GrtRecord {
        record_type: (0, 3, required, one_of(&["GRT"])),
        group_id: (3, 8, required),
        transaction_count: (8, 16, required),
        record_count: (16, 24, required),
        currency_indicator: (24, 27, optional),
        total_monetary_value: (27, 37, optional),
    }
    with_tests ["GRT000010000001400000365             "]
}

