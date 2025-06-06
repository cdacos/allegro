//! /// Starts a new group of transactions within a CWR transmission.

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// /// Starts a new group of transactions within a CWR transmission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "GRHAGR0000102.10            ")]
pub struct GrhRecord {
    #[cwr(title = "Always 'GRH'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction type code", start = 3, len = 3)]
    pub transaction_type: String,

    #[cwr(title = "Group identifier within the transmission", start = 6, len = 5)]
    pub group_id: String,

    #[cwr(title = "Version number for this transaction type", start = 11, len = 5)]
    pub version_number: String,

    #[cwr(title = "Optional batch request identifier", start = 16, len = 10)]
    pub batch_request: Option<String>,

    #[cwr(title = "Optional submission/distribution type (blank for CWR)", start = 26, len = 2)]
    pub submission_distribution_type: Option<String>,
}
