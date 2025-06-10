//! General numeric field for sequence numbers and counts

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// General numeric field for sequence numbers and counts
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Number(pub u32);

impl Number {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CwrFieldWrite for Number {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for Number {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (Number(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid number format: {}", trimmed),
                }];
                (Number(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Number> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000000" {
            (None, vec![])
        } else {
            let (number, warnings) = Number::parse_cwr_field(source, field_name, field_title);
            (Some(number), warnings)
        }
    }
}
