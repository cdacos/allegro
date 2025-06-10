//! CWR version number for GRH record

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// CWR version number for GRH record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersionNumber(pub String);

impl CwrVersionNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for CwrVersionNumber {
    fn to_cwr_str(&self, width: usize) -> String {
        format_text(self.as_str(), width)
    }
}

impl CwrFieldParse for CwrVersionNumber {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        // Validate version format (should be like "02.10", "02.20", etc.)
        if !trimmed.matches('.').count() == 1 || trimmed.len() != 5 {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Invalid version format '{}', expected format like '02.10'", trimmed),
            });
        }

        (CwrVersionNumber(trimmed.to_string()), warnings)
    }
}
