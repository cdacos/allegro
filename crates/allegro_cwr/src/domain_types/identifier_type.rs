//! Identifier Type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Identifier Type (1 character)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub enum IdentifierType {
    /// Title identifier
    #[default]
    Title,
    /// Work identifier
    Work,
    /// Version identifier
    Version,
}

impl IdentifierType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IdentifierType::Title => "T",
            IdentifierType::Work => "W",
            IdentifierType::Version => "V",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "T" => Some(IdentifierType::Title),
            "W" => Some(IdentifierType::Work),
            "V" => Some(IdentifierType::Version),
            _ => None,
        }
    }
}

impl CwrFieldWrite for IdentifierType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for IdentifierType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match IdentifierType::from_str(trimmed) {
            Some(identifier_type) => (identifier_type, warnings),
            None => {
                warnings.push(CwrWarning { 
                    field_name, 
                    field_title, 
                    source_str: Cow::Owned(source.to_string()), 
                    level: WarningLevel::Critical, 
                    description: format!("Identifier Type '{}' not valid. Expected: T (Title), W (Work), V (Version)", trimmed) 
                });
                (IdentifierType::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<IdentifierType> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (identifier_type, warnings) = IdentifierType::parse_cwr_field(source, field_name, field_title);
            (Some(identifier_type), warnings)
        }
    }
}