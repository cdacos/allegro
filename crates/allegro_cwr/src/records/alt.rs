//! ALT - Alternate Title Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ALT - Alternate Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "ALT0000000100000001ALTERNATE TITLE                                          ATEN")]
pub struct AltRecord {
    #[cwr(title = "Always 'ALT'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Alternate title", start = 19, len = 60)]
    pub alternate_title: String,

    #[cwr(title = "Title type", start = 79, len = 2)]
    pub title_type: String,

    #[cwr(title = "Language code (conditional)", start = 81, len = 2)]
    pub language_code: Option<String>,

}