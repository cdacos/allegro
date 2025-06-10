//! Ownership share type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Ownership share (0-100.00% represented as 0-10000)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct OwnershipShare(pub u16);

impl OwnershipShare {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }

    pub fn as_percentage(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}

impl CwrFieldWrite for OwnershipShare {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for OwnershipShare {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) if num <= 10000 => (OwnershipShare(num), vec![]),
            Ok(num) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Ownership share {} exceeds maximum 10000 (100.00%)", num),
                }];
                (OwnershipShare(0), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid ownership share format: {}", trimmed),
                }];
                (OwnershipShare(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<OwnershipShare> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000" {
            (None, vec![])
        } else {
            let (share, warnings) = OwnershipShare::parse_cwr_field(source, field_name, field_title);
            (Some(share), warnings)
        }
    }
}
