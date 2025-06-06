//! NWN - Non-Roman Alphabet Writer Name Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NWN - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "NWN0000000100000001123456789WRITER LAST NAME                                                                                                                                                                EN")]
pub struct NwnRecord {
    #[cwr(title = "Always 'NWN'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number (9 chars, conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Writer last name", start = 28, len = 160)]
    pub writer_last_name: String,

    #[cwr(title = "Writer first name (160 chars, optional)", start = 188, len = 160)]
    pub writer_first_name: Option<String>,

    #[cwr(title = "Language code (2 chars, optional)", start = 348, len = 2)]
    pub language_code: Option<String>,

}