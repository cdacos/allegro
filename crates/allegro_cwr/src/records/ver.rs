//! VER - Original Work Title for Versions Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// VER - Original Work Title for Versions Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    test_data = "VER0000000100000002PLACEHOLDER ORIGINAL WORK TITLE                       1234567890 EN PLACEHOLDER WRITER 1                      FIRSTNAME 1         PLACEHOLDER SOURCE                                      12345678901123456789012PLACEHOLDER WRITER 2                     FIRSTNAME 2         123456789011234567890123456789012345                                        "
)]
pub struct VerRecord {
    #[cwr(title = "Always 'VER'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Original work title", start = 19, len = 60)]
    pub original_work_title: String,

    #[cwr(title = "ISWC of original work (optional)", start = 79, len = 11)]
    pub iswc_of_original_work: Option<String>,

    #[cwr(title = "Language code (optional)", start = 90, len = 2)]
    pub language_code: Option<String>,

    #[cwr(title = "Writer 1 last name (optional)", start = 92, len = 45)]
    pub writer_1_last_name: Option<String>,

    #[cwr(title = "Writer 1 first name (optional)", start = 137, len = 30)]
    pub writer_1_first_name: Option<String>,

    #[cwr(title = "Source (optional)", start = 167, len = 60)]
    pub source: Option<String>,

    #[cwr(title = "Writer 1 IPI name number (optional)", start = 227, len = 11)]
    pub writer_1_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 1 IPI base number (optional)", start = 238, len = 13)]
    pub writer_1_ipi_base_number: Option<String>,

    #[cwr(title = "Writer 2 last name (optional)", start = 251, len = 45)]
    pub writer_2_last_name: Option<String>,

    #[cwr(title = "Writer 2 first name (optional)", start = 296, len = 30)]
    pub writer_2_first_name: Option<String>,

    #[cwr(title = "Writer 2 IPI name number (optional)", start = 326, len = 11)]
    pub writer_2_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 2 IPI base number (optional)", start = 337, len = 13)]
    pub writer_2_ipi_base_number: Option<String>,

    #[cwr(title = "Submitter work number (optional)", start = 350, len = 14)]
    pub submitter_work_num: Option<String>,
}
