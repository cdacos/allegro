//! NPA - Non-Roman Alphabet Publisher Name Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// NPA - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpaRecord {
    /// Always "NPA"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Interested party name (160 chars)
    pub interested_party_name: String,

    /// Interested party writer first name (160 chars)
    pub interested_party_writer_first_name: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}


impl_cwr_parsing! {
    NpaRecord {
        record_type: (0, 3, required, one_of(&["NPA"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        interested_party_num: (19, 28, optional),
        interested_party_name: (28, 188, required),
        interested_party_writer_first_name: (188, 348, required),
        language_code: (348, 350, optional),
    }
}
