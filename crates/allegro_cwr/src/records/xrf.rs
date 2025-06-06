//! XRF - Work ID Cross Reference Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// XRF - Work ID Cross Reference Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "XRF0000000100000001ISWT1234567890123WY")]
pub struct XrfRecord {
    #[cwr(title = "Always 'XRF'", start = 0, len = 3)]
    pub record_type: &'static str,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Organisation code", start = 19, len = 3)]
    pub organisation_code: String,

    #[cwr(title = "Identifier", start = 22, len = 14)]
    pub identifier: String,

    #[cwr(title = "Identifier type (1 char)", start = 36, len = 1)]
    pub identifier_type: String,

    #[cwr(title = "Validity (1 char)", start = 37, len = 1)]
    pub validity: String,
}
