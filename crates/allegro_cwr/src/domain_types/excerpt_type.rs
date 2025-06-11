//! Excerpt Type code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;
use std::ops::Deref;

/// Excerpt Type code (MOV, UEX, or blank)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct ExcerptType(pub String);

impl ExcerptType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for ExcerptType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for ExcerptType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for ExcerptType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::excerpt_types::is_valid_excerpt_type;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_excerpt_type(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Excerpt Type code '{}' not found in lookup table", trimmed),
            });
        }

        (ExcerptType(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<ExcerptType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (code, warnings) = ExcerptType::parse_cwr_field(source, field_name, field_title);
            (Some(code), warnings)
        }
    }
}
