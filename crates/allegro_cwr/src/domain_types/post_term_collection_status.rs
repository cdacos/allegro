//! Post-term collection status for CWR agreement (AGR) records
//!
//! Indicates the post-term collection status for an agreement.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Post-term collection status for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PostTermCollectionStatus {
    #[default]
    None,
    Original,
    Designated,
}

impl PostTermCollectionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PostTermCollectionStatus::None => "N",
            PostTermCollectionStatus::Original => "O",
            PostTermCollectionStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PostTermCollectionStatus {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for PostTermCollectionStatus {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PostTermCollectionStatus::None, vec![]),
            "O" => (PostTermCollectionStatus::Original, vec![]),
            "D" => (PostTermCollectionStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!("Invalid post-term collection status '{}', must be N, O, or D", trimmed),
                }];
                (PostTermCollectionStatus::None, warnings)
            }
        }
    }
}
