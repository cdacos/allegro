//! EWT - Entire Work Title for Excerpts Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// EWT - Entire Work Title for Excerpts Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
pub struct EwtRecord {
    #[cwr(title = "Always 'EWT'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Entire work title", start = 19, len = 60)]
    pub entire_work_title: String,

    #[cwr(title = "ISWC of entire work (optional)", start = 79, len = 11)]
    pub iswc_of_entire_work: Option<String>,

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
