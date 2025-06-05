//! GRH - Group Header Record
//!
//! The Group Header record starts a new group of transactions within a CWR transmission.

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// GRH - Group Header Record
///
/// Starts a new group of transactions within a CWR transmission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrhRecord {
    /// Always "GRH"
    pub record_type: String,

    /// Transaction type code (3 chars)
    pub transaction_type: String,

    /// Group identifier within the transmission (5 chars)
    pub group_id: String,

    /// Version number for this transaction type (5 chars)
    pub version_number: String,

    /// Optional batch request identifier (10 chars)
    pub batch_request: Option<String>,

    /// Optional submission/distribution type (2 chars, blank for CWR)
    pub submission_distribution_type: Option<String>,
}

impl GrhRecord {
    fn post_process_fields(_record: &mut GrhRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for GRH
    }
}

impl_cwr_parsing! {
    GrhRecord {
        record_type: (0, 3, required, one_of(&["GRH"])),
        transaction_type: (3, 6, required),
        group_id: (6, 11, required),
        version_number: (11, 16, required),
        batch_request: (16, 26, optional),
        submission_distribution_type: (26, 28, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(GrhRecord, "GRHAGR0000102.10            ");
