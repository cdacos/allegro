//! INS - Instrumentation Summary Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// INS - Instrumentation Summary Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "INS000000010000000104 ORCHFULL ORCHESTRA WITH STRINGS AND BRASS SECTION    ")]
pub struct InsRecord {
    #[cwr(title = "Always 'INS'", start = 0, len = 3)]
    pub record_type: &'static str,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Number of voices (optional)", start = 19, len = 3)]
    pub number_of_voices: Option<String>,

    #[cwr(title = "Standard instrumentation type (conditional)", start = 22, len = 3)]
    pub standard_instrumentation_type: Option<String>,

    #[cwr(title = "Instrumentation description (conditional)", start = 25, len = 50)]
    pub instrumentation_description: Option<String>,
}
