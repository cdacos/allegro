//! ISRC Validity Indicator

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// ISRC Validity Indicator (1 character)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub enum IsrcValidityIndicator {
    /// The link is valid
    #[default]
    Valid,
    /// The link is invalid
    Invalid,
    /// The ISRC is invalid
    IsrcInvalid,
}

impl IsrcValidityIndicator {
    pub fn as_str(&self) -> &'static str {
        match self {
            IsrcValidityIndicator::Valid => "Y",
            IsrcValidityIndicator::Invalid => "U",
            IsrcValidityIndicator::IsrcInvalid => "N",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "Y" => Some(IsrcValidityIndicator::Valid),
            "U" => Some(IsrcValidityIndicator::Invalid),
            "N" => Some(IsrcValidityIndicator::IsrcInvalid),
            _ => None,
        }
    }
}


impl CwrFieldWrite for IsrcValidityIndicator {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for IsrcValidityIndicator {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match IsrcValidityIndicator::from_str(trimmed) {
            Some(indicator) => (indicator, warnings),
            None => {
                warnings.push(CwrWarning { 
                    field_name, 
                    field_title, 
                    source_str: Cow::Owned(source.to_string()), 
                    level: WarningLevel::Warning, 
                    description: format!("ISRC Validity Indicator '{}' not found in lookup table. Expected: Y, U, N", trimmed) 
                });
                (IsrcValidityIndicator::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<IsrcValidityIndicator> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (indicator, warnings) = IsrcValidityIndicator::parse_cwr_field(source, field_name, field_title);
            (Some(indicator), warnings)
        }
    }
}