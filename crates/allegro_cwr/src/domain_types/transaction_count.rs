//! Transaction count for GRT/TRL records

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Transaction count for GRT/TRL records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TransactionCount(pub u32);

impl TransactionCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldWrite for TransactionCount {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), _width, _character_set)
    }
}

impl CwrFieldParse for TransactionCount {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (TransactionCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid transaction count format: {}", trimmed),
                }];
                (TransactionCount(0), warnings)
            }
        }
    }
}
