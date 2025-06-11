//! Transaction type for CWR group header (GRH) records
//!
//! Represents the type of transaction contained within a CWR transmission.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Transaction type for GRH record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TransactionType {
    #[default]
    NWR,
    REV,
    AGR,
    ACK,
    ISW,
    EXC,
}

impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionType::NWR => "NWR",
            TransactionType::REV => "REV",
            TransactionType::AGR => "AGR",
            TransactionType::ACK => "ACK",
            TransactionType::ISW => "ISW",
            TransactionType::EXC => "EXC",
        }
    }
}

impl CwrFieldWrite for TransactionType {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for TransactionType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "NWR" => (TransactionType::NWR, vec![]),
            "REV" => (TransactionType::REV, vec![]),
            "AGR" => (TransactionType::AGR, vec![]),
            "ACK" => (TransactionType::ACK, vec![]),
            "ISW" => (TransactionType::ISW, vec![]),
            "EXC" => (TransactionType::EXC, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!(
                        "Invalid transaction type '{}', must be NWR, REV, AGR, ACK, ISW, or EXC",
                        trimmed
                    ),
                }];
                (TransactionType::NWR, warnings)
            }
        }
    }
}
