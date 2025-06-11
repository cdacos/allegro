//! CWR revision number type

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// CWR revision number (v2.2+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrRevision(pub u32);

impl CwrRevision {
    pub fn as_str(&self) -> String {
        format!("{:03}", self.0)
    }
}

impl CwrFieldWrite for CwrRevision {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for CwrRevision {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (CwrRevision(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid revision number format: {}", trimmed),
                }];
                (CwrRevision(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CwrRevision> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (revision, warnings) = CwrRevision::parse_cwr_field(source, field_name, field_title);
            (Some(revision), warnings)
        }
    }
}
