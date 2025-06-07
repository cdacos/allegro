//! Also handles NCT (Non-Roman Alphabet Title for Components) and NVT (Non-Roman Alphabet Original Title for Versions)

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Also handles NCT (Non-Roman Alphabet Title for Components) and NVT (Non-Roman Alphabet Original Title for Versions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    codes = ["NET", "NCT", "NVT"],
    test_data = "NET0000000100000002PLACEHOLDER TITLE                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               EN"
)]
pub struct NetRecord {
    #[cwr(title = "'NET', 'NCT', or 'NVT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Title", start = 19, len = 640)]
    pub title: String,

    #[cwr(title = "Language code (optional)", start = 659, len = 2)]
    pub language_code: Option<String>,
}
