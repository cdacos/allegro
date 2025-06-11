//! Type of Right

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Type of Right (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TypeOfRight {
    /// Mechanical Right - The right to record, reproduce and distribute a work on a carrier
    #[default]
    Mechanical,
    /// Performing Right - The entitlement to perform musical, literary or dramatic works live or by mechanical means
    Performing,
    /// Synchronisation Right - The right to include and combine a work in timed relation with other works
    Synchronisation,
    /// All Rights
    All,
}

impl TypeOfRight {
    pub fn as_str(&self) -> &'static str {
        match self {
            TypeOfRight::Mechanical => "MEC",
            TypeOfRight::Performing => "PER",
            TypeOfRight::Synchronisation => "SYN",
            TypeOfRight::All => "ALL",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "MEC" => Some(TypeOfRight::Mechanical),
            "PER" => Some(TypeOfRight::Performing),
            "SYN" => Some(TypeOfRight::Synchronisation),
            "ALL" => Some(TypeOfRight::All),
            _ => None,
        }
    }
}

impl CwrFieldWrite for TypeOfRight {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for TypeOfRight {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match TypeOfRight::from_str(trimmed) {
            Some(type_of_right) => (type_of_right, warnings),
            None => {
                warnings.push(CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!(
                        "Type of Right '{}' not found in lookup table. Expected: MEC, PER, SYN",
                        trimmed
                    ),
                });
                (TypeOfRight::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<TypeOfRight> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (type_of_right, warnings) = TypeOfRight::parse_cwr_field(source, field_name, field_title);
            (Some(type_of_right), warnings)
        }
    }
}
