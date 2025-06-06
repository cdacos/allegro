//! COM - Composite Component Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// COM - Composite Component Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComRecord {
    /// Always "COM"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Title (60 chars)
    pub title: String,

    /// ISWC of component (11 chars, optional)
    pub iswc_of_component: Option<String>,

    /// Submitter work number (14 chars, optional)
    pub submitter_work_num: Option<String>,

    /// Duration HHMMSS (6 chars, optional)
    pub duration: Option<String>,

    /// Writer 1 last name (45 chars)
    pub writer_1_last_name: String,

    /// Writer 1 first name (30 chars, optional)
    pub writer_1_first_name: Option<String>,

    /// Writer 1 IPI name number (11 chars, optional)
    pub writer_1_ipi_name_num: Option<String>,

    /// Writer 2 last name (45 chars, optional)
    pub writer_2_last_name: Option<String>,

    /// Writer 2 first name (30 chars, optional)
    pub writer_2_first_name: Option<String>,

    /// Writer 2 IPI name number (11 chars, optional)
    pub writer_2_ipi_name_num: Option<String>,

    /// Writer 1 IPI base number (13 chars, optional)
    pub writer_1_ipi_base_number: Option<String>,

    /// Writer 2 IPI base number (13 chars, optional)
    pub writer_2_ipi_base_number: Option<String>,
}

impl ComRecord {
    fn post_process_fields(_record: &mut ComRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for COM
    }
}

impl_cwr_parsing! {
    ComRecord {
        record_type: (0, 3, required, one_of(&["COM"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        title: (19, 79, required),
        iswc_of_component: (79, 90, optional),
        submitter_work_num: (90, 104, optional),
        duration: (104, 110, optional),
        writer_1_last_name: (110, 155, required),
        writer_1_first_name: (155, 185, optional),
        writer_1_ipi_name_num: (185, 196, optional),
        writer_2_last_name: (196, 241, optional),
        writer_2_first_name: (241, 271, optional),
        writer_2_ipi_name_num: (271, 282, optional),
        writer_1_ipi_base_number: (282, 295, optional),
        writer_2_ipi_base_number: (295, 308, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(ComRecord, ["COM0000000100000001Component Title                                          T-123456789SW1234567890123120000COMPOSER                                     FIRST                         12345678901WRITER2                                      FIRST2                        12345678901123456789012312345678901231"]);
