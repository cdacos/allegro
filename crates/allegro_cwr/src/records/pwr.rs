//! PWR - Publisher for Writer Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// PWR - Publisher for Writer Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "PWR0000000100000001123456789PUBLISHER NAME                         SUBAGR        SOCAGR        12345678901")]
pub struct PwrRecord {
    #[cwr(title = "Always 'PWR'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Publisher IP number (conditional)", start = 19, len = 9)]
    pub publisher_ip_num: Option<String>,

    #[cwr(title = "Publisher name (conditional)", start = 28, len = 45)]
    pub publisher_name: Option<String>,

    #[cwr(title = "Submitter agreement number (optional)", start = 73, len = 14)]
    pub submitter_agreement_number: Option<String>,

    #[cwr(title = "Society-assigned agreement number (optional)", start = 87, len = 14)]
    pub society_assigned_agreement_number: Option<String>,

    #[cwr(title = "Writer IP number (conditional, v2.1+)", start = 101, len = 9)]
    pub writer_ip_num: Option<String>,

    #[cwr(title = "Publisher sequence number (v2.2+)", start = 110, len = 2)]
    pub publisher_sequence_num: Option<String>,
}
