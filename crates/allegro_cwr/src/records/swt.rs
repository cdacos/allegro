//! SWT - Writer Territory of Control Record / OWT - Other Writer Territory Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// SWT - Writer Territory of Control Record (also OWT - Other Writer Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwtRecord {
    /// "SWT" or "OWT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// PR collection share (5 chars, optional)
    pub pr_collection_share: Option<String>,

    /// MR collection share (5 chars, optional)
    pub mr_collection_share: Option<String>,

    /// SR collection share (5 chars, optional)
    pub sr_collection_share: Option<String>,

    /// Inclusion/Exclusion indicator (1 char)
    pub inclusion_exclusion_indicator: String,

    /// TIS numeric code (4 chars)
    pub tis_numeric_code: String,

    /// Shares change (1 char, optional)
    pub shares_change: Option<String>,

    /// Sequence number (3 chars, v2.1+)
    pub sequence_num: Option<String>,
}

impl SwtRecord {
    fn post_process_fields(_record: &mut SwtRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for SWT
    }
}

impl_cwr_parsing! {
    SwtRecord {
        record_type: (0, 3, required, one_of(&["SWT", "OWT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        interested_party_num: (19, 28, optional),
        pr_collection_share: (28, 33, optional),
        mr_collection_share: (33, 38, optional),
        sr_collection_share: (38, 43, optional),
        inclusion_exclusion_indicator: (43, 44, required),
        tis_numeric_code: (44, 48, required),
        shares_change: (48, 49, optional),
        sequence_num: (49, 52, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(SwtRecord, "SWT0000000100000001         I2840 ");
