use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// PWR - Publisher for Writer Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = pwr_custom_validate, test_data = "PWR0000000000000325ABKC     ABKCO MUSIC INC.                                                         WOMA     01")]
pub struct PwrRecord {
    #[cwr(title = "Always 'PWR'", start = 0, len = 3)]
    pub record_type: String,

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
    pub publisher_sequence_num: Option<PublisherSequenceNumber>,
}

// Custom validation function for PWR record
fn pwr_custom_validate(record: &mut PwrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Publisher identification - either Publisher IP Number or Publisher Name required
    if (record.publisher_ip_num.is_none() || record.publisher_ip_num.as_ref().is_none_or(|s| s.trim().is_empty())) && (record.publisher_name.is_none() || record.publisher_name.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "publisher_ip_num", field_title: "Publisher IP number (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Either Publisher IP Number or Publisher Name must be provided".to_string() });
    }

    // Business rule: Publisher sequence numbers must be sequential within a work
    // This requires cross-record validation which would be implemented in a post-processing step

    // TODO: Additional business rules requiring broader context:
    // - Must follow a SWR record (requires parsing context)
    // - Publisher IP numbers must be valid IPI numbers (requires IPI lookup)
    // - Agreement numbers must follow proper format (submitter vs society-assigned)
    // - Writer IP number cross-validation with preceding SWR record
    // - Publisher sequence numbers must be sequential starting from 01

    warnings
}
