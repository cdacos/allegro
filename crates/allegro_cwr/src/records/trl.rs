use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Marks the end of a CWR transmission and contains summary counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = trl_custom_validate, test_data = "TRL000010000001400000367")]
pub struct TrlRecord {
    #[cwr(title = "Always 'TRL'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Group count", start = 3, len = 5)]
    pub group_count: GroupCount,

    #[cwr(title = "Transaction count", start = 8, len = 8)]
    pub transaction_count: TransactionCount,

    #[cwr(title = "Record count", start = 16, len = 8)]
    pub record_count: RecordCount,
}

// Custom validation function for TRL record
fn trl_custom_validate(record: &mut TrlRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Group count must equal the number of groups within the entire file
    // TODO: This requires context of all groups in the file to validate properly

    // Business rule: Transaction count must equal the number of transactions within the entire file
    // TODO: This requires context of all transactions in the file to validate properly

    // Business rule: Record count must equal the number of physical records inclusive of HDR and TRL
    // TODO: This requires context of all records in the file to validate properly

    // Basic sanity checks
    if record.group_count.0 == 0 {
        warnings.push(CwrWarning { field_name: "group_count", field_title: "Group count", source_str: std::borrow::Cow::Owned(record.group_count.as_str()), level: WarningLevel::Warning, description: "Group count is 0, which may indicate no content in file".to_string() });
    }

    if record.transaction_count.0 == 0 {
        warnings.push(CwrWarning { field_name: "transaction_count", field_title: "Transaction count", source_str: std::borrow::Cow::Owned(record.transaction_count.as_str()), level: WarningLevel::Warning, description: "Transaction count is 0, which may indicate no content in file".to_string() });
    }

    // Record count should be at least 2 (HDR + TRL)
    if record.record_count.0 < 2 {
        warnings.push(CwrWarning { field_name: "record_count", field_title: "Record count", source_str: std::borrow::Cow::Owned(record.record_count.as_str()), level: WarningLevel::Critical, description: "Record count must be at least 2 (HDR + TRL records)".to_string() });
    }

    warnings
}
