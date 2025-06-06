//! NWN - Non-Roman Alphabet Writer Name Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// NWN - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NwnRecord {
    /// Always "NWN"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Writer last name (160 chars)
    pub writer_last_name: String,

    /// Writer first name (160 chars, optional)
    pub writer_first_name: Option<String>,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}


impl_cwr_parsing! {
    NwnRecord {
        record_type: (0, 3, required, one_of(&["NWN"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        interested_party_num: (19, 28, optional),
        writer_last_name: (28, 188, required),
        writer_first_name: (188, 348, optional),
        language_code: (348, 350, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(NwnRecord, ["NWN0000000100000001123456789WRITER LAST NAME                                                                                                                                                                EN"]);
