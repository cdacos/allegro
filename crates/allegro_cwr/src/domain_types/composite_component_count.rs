//! Composite component count for NWR record

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Composite component count for NWR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CompositeComponentCount(pub u16);

impl CompositeComponentCount {
    pub fn as_str(&self) -> String {
        format!("{:03}", self.0)
    }
}

impl CwrFieldWrite for CompositeComponentCount {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for CompositeComponentCount {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) => (CompositeComponentCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid composite component count format: {}", trimmed),
                }];
                (CompositeComponentCount(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CompositeComponentCount> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "000" {
            (None, vec![])
        } else {
            let (count, warnings) = CompositeComponentCount::parse_cwr_field(source, field_name, field_title);
            (Some(count), warnings)
        }
    }
}
