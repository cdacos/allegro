use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Starts a new group of transactions within a CWR transmission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = grh_custom_validate, test_data = "GRHAGR0000102.20            ")]
pub struct GrhRecord {
    #[cwr(title = "Always 'GRH'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction type code", start = 3, len = 3)]
    pub transaction_type: TransactionType,

    #[cwr(title = "Group identifier within the transmission", start = 6, len = 5)]
    pub group_id: GroupId,

    #[cwr(title = "Version number for this transaction type", start = 11, len = 5)]
    pub version_number: CwrVersionNumber,

    #[cwr(title = "Optional batch request identifier", start = 16, len = 10)]
    pub batch_request: Option<Number>,

    #[cwr(title = "Optional submission/distribution type (blank for CWR)", start = 26, len = 2)]
    pub submission_distribution_type: Option<LookupPlaceholder>,
}

// Custom validation function for GRH record
fn grh_custom_validate(record: &mut GrhRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Cross-field validation: Version Number should match expected values
    let version_str = record.version_number.as_str();
    let is_valid_version = matches!(version_str, "02.00" | "02.10" | "02.20");

    if !is_valid_version {
        warnings.push(CwrWarning {
            field_name: "version_number",
            field_title: "Version number for this transaction type",
            source_str: std::borrow::Cow::Owned(version_str.to_string()),
            level: WarningLevel::Warning,
            description: format!("Version number '{}' is not a valid CWR version (expected: 02.00, 02.10, or 02.20)", version_str),
        });
    }

    // Business rule: Group ID should start at 1 and increment sequentially
    // Note: Full validation requires context of previous groups in file
    if record.group_id.0 == 0 {
        warnings.push(CwrWarning { field_name: "group_id", field_title: "Group identifier within the transmission", source_str: std::borrow::Cow::Owned(record.group_id.as_str()), level: WarningLevel::Critical, description: "Group ID must start at 1, not 0".to_string() });
    }

    warnings
}
