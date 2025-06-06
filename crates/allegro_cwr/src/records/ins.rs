//! INS - Instrumentation Summary Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// INS - Instrumentation Summary Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsRecord {
    /// Always "INS"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Number of voices (3 chars, optional)
    pub number_of_voices: Option<String>,

    /// Standard instrumentation type (3 chars, conditional)
    pub standard_instrumentation_type: Option<String>,

    /// Instrumentation description (50 chars, conditional)
    pub instrumentation_description: Option<String>,
}

impl InsRecord {
    fn post_process_fields(_record: &mut InsRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for INS
    }
}

impl_cwr_parsing! {
    InsRecord {
        record_type: (0, 3, required, one_of(&["INS"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        number_of_voices: (19, 22, optional),
        standard_instrumentation_type: (22, 25, optional),
        instrumentation_description: (25, 75, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(InsRecord, "INS000000010000000104 ORCHFULL ORCHESTRA WITH STRINGS AND BRASS SECTION   ");
