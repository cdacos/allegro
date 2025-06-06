//! TER - Territory in Agreement Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// TER - Territory in Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerRecord {
    /// Always "TER"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Inclusion/Exclusion indicator (1 char)
    pub inclusion_exclusion_indicator: String,

    /// TIS Numeric Code (4 chars)
    pub tis_numeric_code: String,
}


impl_cwr_parsing! {
    TerRecord {
        record_type: (0, 3, required, one_of(&["TER"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        inclusion_exclusion_indicator: (19, 20, required),
        tis_numeric_code: (20, 24, required),
    }
    with_tests ["TER0000000100000001I2840"]
}

