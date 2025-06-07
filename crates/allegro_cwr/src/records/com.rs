use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// COM - Composite Component Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    test_data = "COM0000000100000002PLACEHOLDER TITLE                                    12345678901234567890PLACEHOLDER WRITER                      FIRSTNAME           12345678901PLACEHOLDER WRITER 2                     FIRSTNAME 2         123456789011234567890123456789012345                                                                                        "
)]
pub struct ComRecord {
    #[cwr(title = "Always 'COM'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Title", start = 19, len = 60)]
    pub title: String,

    #[cwr(title = "ISWC of component (optional)", start = 79, len = 11)]
    pub iswc_of_component: Option<String>,

    #[cwr(title = "Submitter work number (optional)", start = 90, len = 14)]
    pub submitter_work_num: Option<String>,

    #[cwr(title = "Duration HHMMSS (optional)", start = 104, len = 6)]
    pub duration: Option<String>,

    #[cwr(title = "Writer 1 last name", start = 110, len = 45)]
    pub writer_1_last_name: String,

    #[cwr(title = "Writer 1 first name (optional)", start = 155, len = 30)]
    pub writer_1_first_name: Option<String>,

    #[cwr(title = "Writer 1 IPI name number (optional)", start = 185, len = 11)]
    pub writer_1_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 2 last name (optional)", start = 196, len = 45)]
    pub writer_2_last_name: Option<String>,

    #[cwr(title = "Writer 2 first name (optional)", start = 241, len = 30)]
    pub writer_2_first_name: Option<String>,

    #[cwr(title = "Writer 2 IPI name number (optional)", start = 271, len = 11)]
    pub writer_2_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 1 IPI base number (optional)", start = 282, len = 13)]
    pub writer_1_ipi_base_number: Option<String>,

    #[cwr(title = "Writer 2 IPI base number (optional)", start = 295, len = 13)]
    pub writer_2_ipi_base_number: Option<String>,
}
