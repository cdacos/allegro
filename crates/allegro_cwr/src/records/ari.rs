//! ARI - Additional Related Information Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ARI - Additional Related Information Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "ARI0000000100000001021              ALL  Additional related information note for the work                                                                                                    ")]
pub struct AriRecord {
    #[cwr(title = "Always 'ARI'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Society number", start = 19, len = 3)]
    pub society_num: String,

    #[cwr(title = "Work number (conditional)", start = 22, len = 14)]
    pub work_num: Option<String>,

    #[cwr(title = "Type of right", start = 36, len = 3)]
    pub type_of_right: String,

    #[cwr(title = "Subject code (conditional)", start = 39, len = 2)]
    pub subject_code: Option<String>,

    #[cwr(title = "Note (conditional)", start = 41, len = 160)]
    pub note: Option<String>,

}