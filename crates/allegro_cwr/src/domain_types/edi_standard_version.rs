//! EDI Standard Version type for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// EDI Standard Version Number
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct EdiStandardVersion(pub String);

impl EdiStandardVersion {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for EdiStandardVersion {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for EdiStandardVersion {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed == "01.10" {
            (EdiStandardVersion(trimmed.to_string()), vec![])
        } else {
            let warnings = vec![CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Critical,
                description: format!("Invalid EDI standard version '{}', must be '01.10'", trimmed),
            }];
            (EdiStandardVersion("01.10".to_string()), warnings)
        }
    }
}
