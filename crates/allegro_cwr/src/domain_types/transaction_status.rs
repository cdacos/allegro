//! Transaction Status

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Transaction Status (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TransactionStatus(pub String);

impl TransactionStatus {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for TransactionStatus {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for TransactionStatus {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::transaction_statuses::is_valid_transaction_status;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_transaction_status(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Transaction Status '{}' not found in lookup table", trimmed),
            });
        }

        (TransactionStatus(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<TransactionStatus> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (transaction_status, warnings) = TransactionStatus::parse_cwr_field(source, field_name, field_title);
            (Some(transaction_status), warnings)
        }
    }
}
