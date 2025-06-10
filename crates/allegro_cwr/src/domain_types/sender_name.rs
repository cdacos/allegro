//! Sender name type for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// Sender name with validation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SenderName(pub String);

impl SenderName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for SenderName {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for SenderName {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();

        if trimmed.is_empty() {
            let warnings = vec![CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Critical,
                description: "Sender name is required".to_string(),
            }];
            return (SenderName(String::new()), warnings);
        }

        // Basic length validation
        let mut warnings = vec![];
        if trimmed.len() > 45 {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Sender name '{}' exceeds maximum length of 45 characters", trimmed),
            });
        }

        (SenderName(trimmed.to_string()), warnings)
    }
}
