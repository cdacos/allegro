//! ARI - Additional Related Information Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// ARI - Additional Related Information Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AriRecord {
    /// Always "ARI"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Society number (3 chars)
    pub society_num: String,

    /// Work number (14 chars, conditional)
    pub work_num: Option<String>,

    /// Type of right (3 chars)
    pub type_of_right: String,

    /// Subject code (2 chars, conditional)
    pub subject_code: Option<String>,

    /// Note (160 chars, conditional)
    pub note: Option<String>,
}

impl AriRecord {
    fn post_process_fields(_record: &mut AriRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for ARI
    }
}

impl_cwr_parsing! {
    AriRecord {
        record_type: (0, 3, required, one_of(&["ARI"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        society_num: (19, 22, required),
        work_num: (22, 36, optional),
        type_of_right: (36, 39, required),
        subject_code: (39, 41, optional),
        note: (41, 201, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(AriRecord, ["ARI0000000100000001021              ALL  Additional related information note for the work                                                                                                    "]);
