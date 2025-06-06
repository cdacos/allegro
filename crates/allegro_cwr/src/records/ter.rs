//! TER - Territory in Agreement Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// TER - Territory in Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "TER0000000100000001I2840")]
pub struct TerRecord {
    #[cwr(title = "Always 'TER'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 19, len = 1)]
    pub inclusion_exclusion_indicator: String,

    #[cwr(title = "TIS Numeric Code", start = 20, len = 4)]
    pub tis_numeric_code: String,
}
