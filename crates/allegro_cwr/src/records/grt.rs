//! /// Marks the end of a group and contains summary counts for that group.

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// /// Marks the end of a group and contains summary counts for that group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "GRT000010000001400000365             ")]
pub struct GrtRecord {
    #[cwr(title = "Always 'GRT'", start = 0, len = 3)]
    pub record_type: &'static str,

    #[cwr(title = "Group ID", start = 3, len = 5)]
    pub group_id: String,

    #[cwr(title = "Transaction count", start = 8, len = 8)]
    pub transaction_count: String,

    #[cwr(title = "Record count", start = 16, len = 8)]
    pub record_count: String,

    #[cwr(title = "Currency indicator (conditional)", start = 24, len = 3)]
    pub currency_indicator: Option<String>,

    #[cwr(title = "Total monetary value (optional)", start = 27, len = 10)]
    pub total_monetary_value: Option<String>,
}
