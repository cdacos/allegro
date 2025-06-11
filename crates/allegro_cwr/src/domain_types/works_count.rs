//! Works count for AGR record

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Works count for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct WorksCount(pub u32);

impl WorksCount {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldWrite for WorksCount {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), _width, _character_set)
    }
}

impl CwrFieldParse for WorksCount {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (WorksCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid works count format: {}", trimmed),
                }];
                (WorksCount(0), warnings)
            }
        }
    }
}
