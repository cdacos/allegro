//! Work Type

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Work Type (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct WorkType(pub String);

impl WorkType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for WorkType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for WorkType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::work_types::is_valid_work_type;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_work_type(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Work Type '{}' not found in lookup table", trimmed),
            });
        }

        (WorkType(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<WorkType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (work_type, warnings) = WorkType::parse_cwr_field(source, field_name, field_title);
            (Some(work_type), warnings)
        }
    }
}
