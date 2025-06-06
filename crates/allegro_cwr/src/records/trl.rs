//! /// Marks the end of a CWR transmission and contains summary counts.

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// /// Marks the end of a CWR transmission and contains summary counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "TRL000010000001400000367")]
pub struct TrlRecord {
    #[cwr(title = "Always 'TRL'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Group count", start = 3, len = 5)]
    pub group_count: String,

    #[cwr(title = "Transaction count", start = 8, len = 8)]
    pub transaction_count: String,

    #[cwr(title = "Record count", start = 16, len = 8)]
    pub record_count: String,
}
