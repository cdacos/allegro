//! SWT - Writer Territory of Control Record (also OWT - Other Writer Territory)

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SWT - Writer Territory of Control Record (also OWT - Other Writer Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "SWT0000000100000001         I2840 ")]
pub struct SwtRecord {
    #[cwr(title = "'SWT' or 'OWT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number (9 chars, conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "PR collection share (5 chars, optional)", start = 28, len = 5)]
    pub pr_collection_share: Option<String>,

    #[cwr(title = "MR collection share (5 chars, optional)", start = 33, len = 5)]
    pub mr_collection_share: Option<String>,

    #[cwr(title = "SR collection share (5 chars, optional)", start = 38, len = 5)]
    pub sr_collection_share: Option<String>,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 43, len = 1)]
    pub inclusion_exclusion_indicator: String,

    #[cwr(title = "TIS numeric code", start = 44, len = 4)]
    pub tis_numeric_code: String,

    #[cwr(title = "Shares change (1 char, optional)", start = 48, len = 1)]
    pub shares_change: Option<String>,

    #[cwr(title = "Sequence number (3 chars, v2.1+)", start = 49, len = 3)]
    pub sequence_num: Option<String>,

}