//! XRF - Work ID Cross Reference Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// XRF - Work ID Cross Reference Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XrfRecord {
    /// Always "XRF"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Organisation code (3 chars)
    pub organisation_code: String,

    /// Identifier (14 chars)
    pub identifier: String,

    /// Identifier type (1 char)
    pub identifier_type: String,

    /// Validity (1 char)
    pub validity: String,
}


impl_cwr_parsing! {
    XrfRecord {
        record_type: (0, 3, required, one_of(&["XRF"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        organisation_code: (19, 22, required),
        identifier: (22, 36, required),
        identifier_type: (36, 37, required),
        validity: (37, 38, required),
    }
}

impl_cwr_parsing_test_roundtrip!(XrfRecord, ["XRF0000000100000001ISWT1234567890123WY"]);
