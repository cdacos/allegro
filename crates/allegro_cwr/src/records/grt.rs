use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Marks the end of a group and contains summary counts for that group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = grt_custom_validate, test_data = "GRT000010000001400000365             ")]
pub struct GrtRecord {
    #[cwr(title = "Always 'GRT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Group ID", start = 3, len = 5)]
    pub group_id: GroupId,

    #[cwr(title = "Transaction count", start = 8, len = 8)]
    pub transaction_count: TransactionCount,

    #[cwr(title = "Record count", start = 16, len = 8)]
    pub record_count: RecordCount,

    #[cwr(title = "Currency indicator (conditional)", start = 24, len = 3)]
    pub currency_indicator: CurrencyCode,

    #[cwr(title = "Total monetary value (optional)", start = 27, len = 10)]
    pub total_monetary_value: Option<String>,
}

// Custom validation function for GRT record
fn grt_custom_validate(record: &mut GrtRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Currency Indicator is mandatory if Total Monetary Value is provided
    if record.total_monetary_value.is_some() && record.total_monetary_value.as_ref().map_or(false, |v| !v.trim().is_empty()) {
        if record.currency_indicator.0.is_none() {
            warnings.push(CwrWarning { field_name: "currency_indicator", field_title: "Currency indicator (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Currency Indicator is mandatory when Total Monetary Value is provided".to_string() });
        }
    }

    // Business rule: Group ID must match the preceding GRH record
    // TODO: This requires context of the preceding GRH record to validate properly

    // Business rule: Transaction count must equal actual transaction count in group
    // TODO: This requires context of all transactions in the group to validate properly

    // Business rule: Record count must equal actual record count in group (including GRH and GRT)
    // TODO: This requires context of all records in the group to validate properly

    warnings
}
