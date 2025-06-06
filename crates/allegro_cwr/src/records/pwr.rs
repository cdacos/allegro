//! PWR - Publisher for Writer Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// PWR - Publisher for Writer Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PwrRecord {
    /// Always "PWR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher IP number (9 chars, conditional)
    pub publisher_ip_num: Option<String>,

    /// Publisher name (45 chars, conditional)
    pub publisher_name: Option<String>,

    /// Submitter agreement number (14 chars, optional)
    pub submitter_agreement_number: Option<String>,

    /// Society-assigned agreement number (14 chars, optional)
    pub society_assigned_agreement_number: Option<String>,

    /// Writer IP number (9 chars, conditional, v2.1+)
    pub writer_ip_num: Option<String>,

    /// Publisher sequence number (2 chars, v2.2+)
    pub publisher_sequence_num: Option<String>,
}

impl PwrRecord {
    fn post_process_fields(_record: &mut PwrRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for PWR
    }
}

impl_cwr_parsing! {
    PwrRecord {
        record_type: (0, 3, required, one_of(&["PWR"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        publisher_ip_num: (19, 28, optional),
        publisher_name: (28, 73, optional),
        submitter_agreement_number: (73, 87, optional),
        society_assigned_agreement_number: (87, 101, optional),
        writer_ip_num: (101, 110, optional),
        publisher_sequence_num: (110, 112, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(PwrRecord, ["PWR0000000100000001123456789PUBLISHER NAME                         SUBAGR        SOCAGR        12345678901"]);
