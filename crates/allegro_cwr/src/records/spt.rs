//! SPT - Publisher Territory of Control Record (also OPT - Other Publisher Territory)

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SPT - Publisher Territory of Control Record (also OPT - Other Publisher Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "SPT0000000100000001123456789                  I2840 ")]
pub struct SptRecord {
    #[cwr(title = "'SPT' or 'OPT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number", start = 19, len = 9)]
    pub interested_party_num: String,

    #[cwr(title = "Constant - spaces", start = 28, len = 6)]
    pub constant: String,

    #[cwr(title = "PR collection share (conditional)", start = 34, len = 5)]
    pub pr_collection_share: Option<String>,

    #[cwr(title = "MR collection share (conditional)", start = 39, len = 5)]
    pub mr_collection_share: Option<String>,

    #[cwr(title = "SR collection share (conditional)", start = 44, len = 5)]
    pub sr_collection_share: Option<String>,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 49, len = 1)]
    pub inclusion_exclusion_indicator: String,

    #[cwr(title = "TIS numeric code", start = 50, len = 4)]
    pub tis_numeric_code: String,

    #[cwr(title = "Shares change (1 char, optional)", start = 54, len = 1)]
    pub shares_change: Option<String>,

    #[cwr(title = "Sequence number (v2.1+)", start = 55, len = 3)]
    pub sequence_num: Option<String>,
}
