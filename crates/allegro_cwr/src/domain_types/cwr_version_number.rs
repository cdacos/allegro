//! CWR version number for GRH record

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
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
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
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
