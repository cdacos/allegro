//! Sender type for HDR record

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Sender type for HDR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum SenderType {
    #[default]
    Publisher,
    Society,
    Writer,
    AdministrativeAgency,
    /// For IPNN > 9 digits, this contains the leading digits
    NumericPrefix(String),
}

impl SenderType {
    pub fn as_str(&self) -> &str {
        match self {
            SenderType::Publisher => "PB",
            SenderType::Society => "SO",
            SenderType::Writer => "WR",
            SenderType::AdministrativeAgency => "AA",
            SenderType::NumericPrefix(s) => s,
        }
    }
}

impl CwrFieldWrite for SenderType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for SenderType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "PB" => (SenderType::Publisher, vec![]),
            "SO" => (SenderType::Society, vec![]),
            "WR" => (SenderType::Writer, vec![]),
            "AA" => (SenderType::AdministrativeAgency, vec![]),
            s if s.chars().all(|c| c.is_ascii_digit()) && s.len() <= 2 => {
                (SenderType::NumericPrefix(s.to_string()), vec![])
            }
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!(
                        "Invalid sender type '{}', must be PB, SO, WR, AA, or 2-digit numeric prefix",
                        trimmed
                    ),
                }];
                (SenderType::Publisher, warnings)
            }
        }
    }
}
