//! VER - Original Work Title for Versions Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// VER - Original Work Title for Versions Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerRecord {
    /// Always "VER"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Original work title (60 chars)
    pub original_work_title: String,

    /// ISWC of original work (11 chars, optional)
    pub iswc_of_original_work: Option<String>,

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

impl VerRecord {
    fn post_process_fields(_record: &mut VerRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for VER
    }
}

impl_cwr_parsing! {
    VerRecord {
        record_type: (0, 3, required, one_of(&["VER"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        original_work_title: (19, 79, required),
        iswc_of_original_work: (79, 90, optional),
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

impl_cwr_parsing_test_roundtrip!(VerRecord, ["VER0000000100000001ORIGINAL WORK TITLE                                 T-987654321ENWRITER 1 LAST NAME                     WRITER 1 FIRST NAME        SOURCE                                                      98765432109876543210987WRITER 2 LAST NAME                     WRITER 2 FIRST NAME        87654321098765432109876ORIGWORK123   "]);
