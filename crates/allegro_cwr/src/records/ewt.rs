//! EWT - Entire Work Title for Excerpts Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// EWT - Entire Work Title for Excerpts Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EwtRecord {
    /// Always "EWT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Entire work title (60 chars)
    pub entire_work_title: String,

    /// ISWC of entire work (11 chars, optional)
    pub iswc_of_entire_work: Option<String>,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Writer 1 last name (45 chars, optional)
    pub writer_1_last_name: Option<String>,

    /// Writer 1 first name (30 chars, optional)
    pub writer_1_first_name: Option<String>,

    /// Source (60 chars, optional)
    pub source: Option<String>,

    /// Writer 1 IPI name number (11 chars, optional)
    pub writer_1_ipi_name_num: Option<String>,

    /// Writer 1 IPI base number (13 chars, optional)
    pub writer_1_ipi_base_number: Option<String>,

    /// Writer 2 last name (45 chars, optional)
    pub writer_2_last_name: Option<String>,

    /// Writer 2 first name (30 chars, optional)
    pub writer_2_first_name: Option<String>,

    /// Writer 2 IPI name number (11 chars, optional)
    pub writer_2_ipi_name_num: Option<String>,

    /// Writer 2 IPI base number (13 chars, optional)
    pub writer_2_ipi_base_number: Option<String>,

    /// Submitter work number (14 chars, optional)
    pub submitter_work_num: Option<String>,
}

impl EwtRecord {
    fn post_process_fields(_record: &mut EwtRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for EWT
    }
}

impl_cwr_parsing! {
    EwtRecord {
        record_type: (0, 3, required, one_of(&["EWT"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        entire_work_title: (19, 79, required),
        iswc_of_entire_work: (79, 90, optional),
        language_code: (90, 92, optional),
        writer_1_last_name: (92, 137, optional),
        writer_1_first_name: (137, 167, optional),
        source: (167, 227, optional),
        writer_1_ipi_name_num: (227, 238, optional),
        writer_1_ipi_base_number: (238, 251, optional),
        writer_2_last_name: (251, 296, optional),
        writer_2_first_name: (296, 326, optional),
        writer_2_ipi_name_num: (326, 337, optional),
        writer_2_ipi_base_number: (337, 350, optional),
        submitter_work_num: (350, 364, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(EwtRecord, "EWT0000000100000001ENTIRE WORK TITLE                                   T-123456789ENWRITER 1 LAST NAME                     WRITER 1 FIRST NAME        SOURCE                                                      01234567890123456789012WRITER 2 LAST NAME                     WRITER 2 FIRST NAME        09876543210987654321098SUBWORK123    ");
