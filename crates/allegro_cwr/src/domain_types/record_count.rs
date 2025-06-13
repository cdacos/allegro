//! Record count for GRT/TRL records

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Record count for GRT/TRL records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordCount(pub u32);

impl RecordCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldWrite for RecordCount {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for RecordCount {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (RecordCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid record count format: {}", trimmed),
                }];
                (RecordCount(0), warnings)
            }
        }
    }
}
