use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ACK - Acknowledgement of Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ack_custom_validate, test_data = "ACK0000000100000001200501011200000000100000001NWRTEST WORK TITLE                                          SW123456789012345678                    20050102AS   ")]
pub struct AckRecord {
    #[cwr(title = "Always 'ACK'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Creation date of original file YYYYMMDD", start = 19, len = 8)]
    pub creation_date: Date,

    #[cwr(title = "Creation time of original file HHMMSS", start = 27, len = 6)]
    pub creation_time: Time,

    #[cwr(title = "Original group ID", start = 33, len = 5)]
    pub original_group_id: GroupId,

    #[cwr(title = "Original transaction sequence number", start = 38, len = 8)]
    pub original_transaction_sequence_num: TransactionCount,

    #[cwr(title = "Original transaction type", start = 46, len = 3)]
    pub original_transaction_type: TransactionType,

    #[cwr(title = "Creation title (conditional)", start = 49, len = 60)]
    pub creation_title: Option<String>,

    #[cwr(title = "Submitter creation number (conditional)", start = 109, len = 20)]
    pub submitter_creation_num: Option<String>,

    #[cwr(title = "Recipient creation number (conditional)", start = 129, len = 20)]
    pub recipient_creation_num: Option<String>,

    #[cwr(title = "Processing date YYYYMMDD", start = 149, len = 8)]
    pub processing_date: Date,

    #[cwr(title = "Transaction status", start = 157, len = 2)]
    pub transaction_status: TransactionStatus,
}

// Custom validation function for ACK record
fn ack_custom_validate(record: &mut AckRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Creation Title required if ACK responds to NWR or REV transaction
    if matches!(record.original_transaction_type, TransactionType::NWR | TransactionType::REV)
        && (record.creation_title.is_none() || record.creation_title.as_ref().is_none_or(|s| s.trim().is_empty()))
    {
        warnings.push(CwrWarning {
            field_name: "creation_title",
            field_title: "Creation title (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Creation Title is required when ACK responds to NWR or REV transaction".to_string(),
        });
    }

    // Business rule: Submitter Creation # required if ACK is response to a transaction
    if !matches!(record.original_transaction_type, TransactionType::AGR)
        && (record.submitter_creation_num.is_none()
            || record.submitter_creation_num.as_ref().is_none_or(|s| s.trim().is_empty()))
    {
        warnings.push(CwrWarning {
            field_name: "submitter_creation_num",
            field_title: "Submitter creation number (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Submitter Creation Number is required when ACK responds to a transaction".to_string(),
        });
    }

    // Special case validation for HDR/TRL problems
    let is_hdr_trl_problem = record.original_group_id.0 == 0 && record.original_transaction_sequence_num.0 == 0;
    if is_hdr_trl_problem {
        // For HDR/TRL problems, Original Transaction Type should be HDR or TRL
        // Note: TransactionType enum doesn't include HDR/TRL, so this would need extension
        // TODO: Extend TransactionType enum to include HDR/TRL for ACK records
    }

    // TODO: Additional business rules requiring broader context:
    // - Creation Date/Time must match HDR record of referenced file
    // - Original Group ID + Transaction Sequence # must be valid within referenced file
    // - Original Transaction Type must match transaction referred to by Creation Date/Time/Group/Sequence combination
    // - Creation Title must match title associated with Submitter Creation #
    // - Transaction Status must match Transaction Status table entry
    // - Only one ACK allowed per transaction (requires context)
    // - Validation of referenced transaction existence

    warnings
}
