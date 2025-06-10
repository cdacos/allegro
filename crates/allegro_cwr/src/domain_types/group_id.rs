//! Group ID for GRH/GRT records

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// Group ID for GRH/GRT records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupId(pub u32);

impl GroupId {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldWrite for GroupId {
    fn to_cwr_str(&self, _width: usize) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for GroupId {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (GroupId(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid group ID format: {}", trimmed),
                }];
                (GroupId(0), warnings)
            }
        }
    }
}
