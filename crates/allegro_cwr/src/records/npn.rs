//! NPN - Non-Roman Alphabet Publisher Name Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPN - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    test_data = "NPN0000000100000002011234567890PLACEHOLDER PUBLISHER NAME                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          EN"
)]
pub struct NpnRecord {
    #[cwr(title = "Always 'NPN'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Publisher sequence number", start = 19, len = 2)]
    pub publisher_sequence_num: String,

    #[cwr(title = "Interested party number", start = 21, len = 9)]
    pub interested_party_num: String,

    #[cwr(title = "Publisher name", start = 30, len = 480)]
    pub publisher_name: String,

    #[cwr(title = "Language code (optional)", start = 510, len = 2)]
    pub language_code: Option<String>,
}
