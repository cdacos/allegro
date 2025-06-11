//! Inclusion/Exclusion indicator for CWR territory records
//!
//! Indicates whether a territory is included or excluded from an agreement or right.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Inclusion/Exclusion indicator for territory records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum InclusionExclusionIndicator {
    #[default]
    Included,
    Excluded,
}

impl InclusionExclusionIndicator {
    pub fn as_str(&self) -> &str {
        match self {
            InclusionExclusionIndicator::Included => "I",
            InclusionExclusionIndicator::Excluded => "E",
        }
    }
}

impl CwrFieldWrite for InclusionExclusionIndicator {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for InclusionExclusionIndicator {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "I" => (InclusionExclusionIndicator::Included, vec![]),
            "E" => (InclusionExclusionIndicator::Excluded, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!("Invalid inclusion/exclusion indicator '{}', must be I or E", trimmed),
                }];
                (InclusionExclusionIndicator::Included, warnings)
            }
        }
    }
}
