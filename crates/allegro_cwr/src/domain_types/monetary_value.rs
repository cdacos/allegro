//! Monetary value for GRT trailer records
//!
//! Represents monetary values in GRT records that should be space-padded when None

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Monetary value for GRT trailer records (space-padded when None)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct MonetaryValue(pub u64);

impl MonetaryValue {
    pub fn as_str(&self) -> String {
        format!("{:010}", self.0)
    }

    /// Parse from SQL string - needed for SQLite integration
    pub fn from_sql_string(s: &str) -> Result<Self, String> {
        s.parse::<u64>().map(MonetaryValue).map_err(|e| format!("Invalid monetary value: {}", e))
    }
}

impl std::fmt::Display for MonetaryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CwrFieldWrite for MonetaryValue {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(format!("{:0width$}", self.0, width = width).as_str(), width, character_set)
    }
}

impl CwrFieldParse for MonetaryValue {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u64>() {
            Ok(num) => (MonetaryValue(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid monetary value format: {}", trimmed),
                }];
                (MonetaryValue(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<MonetaryValue> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed.chars().all(|c| c == ' ') {
            (None, vec![])
        } else {
            let (monetary_value, warnings) = MonetaryValue::parse_cwr_field(source, field_name, field_title);
            (Some(monetary_value), warnings)
        }
    }
}
