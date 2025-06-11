//! Prior royalty status for CWR agreement (AGR) records
//!
//! Indicates the prior royalty status for an agreement.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Prior royalty status for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PriorRoyaltyStatus {
    #[default]
    None,
    Acquired,
    Designated,
}

impl PriorRoyaltyStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PriorRoyaltyStatus::None => "N",
            PriorRoyaltyStatus::Acquired => "A",
            PriorRoyaltyStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PriorRoyaltyStatus {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for PriorRoyaltyStatus {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PriorRoyaltyStatus::None, vec![]),
            "A" => (PriorRoyaltyStatus::Acquired, vec![]),
            "D" => (PriorRoyaltyStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!("Invalid prior royalty status '{}', must be N, A, or D", trimmed),
                }];
                (PriorRoyaltyStatus::None, warnings)
            }
        }
    }
}
