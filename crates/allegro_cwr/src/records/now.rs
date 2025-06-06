//! NOW - Non-Roman Alphabet Writer Name Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NOW - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "NOW0000000100000002PLACEHOLDER WRITER NAME                                                                                                                                         PLACEHOLDER FIRST NAME                                                                                                                                          EN1")]
pub struct NowRecord {
    #[cwr(title = "Always 'NOW'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Writer name", start = 19, len = 160)]
    pub writer_name: String,

    #[cwr(title = "Writer first name", start = 179, len = 160)]
    pub writer_first_name: String,

    #[cwr(title = "Language code (optional)", start = 339, len = 2)]
    pub language_code: Option<String>,

    #[cwr(title = "Writer position (1 char, optional)", start = 341, len = 1)]
    pub writer_position: Option<String>,
}
