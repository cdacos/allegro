//! SPT - Publisher Territory of Control Record / OPT - Other Publisher Territory Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// SPT - Publisher Territory of Control Record (also OPT - Other Publisher Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SptRecord {
    /// "SPT" or "OPT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars)
    pub interested_party_num: String,

    /// Constant - spaces (6 chars)
    pub constant: String,

    /// PR collection share (5 chars, conditional)
    pub pr_collection_share: Option<String>,

    /// MR collection share (5 chars, conditional)
    pub mr_collection_share: Option<String>,

    /// SR collection share (5 chars, conditional)
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


impl_cwr_parsing! {
    SptRecord {
        record_type: (0, 3, required, one_of(&["SPT", "OPT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        interested_party_num: (19, 28, required),
        constant: (28, 34, required),
        pr_collection_share: (34, 39, optional),
        mr_collection_share: (39, 44, optional),
        sr_collection_share: (44, 49, optional),
        inclusion_exclusion_indicator: (49, 50, required),
        tis_numeric_code: (50, 54, required),
        shares_change: (54, 55, optional),
        sequence_num: (55, 58, optional),
    }
    with_test_data ["SPT0000000100000001123456789                  I2840 "]
}

