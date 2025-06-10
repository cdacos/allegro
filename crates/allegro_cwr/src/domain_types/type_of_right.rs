//! Type of Right

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Type of Right (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub enum TypeOfRight {
    /// Mechanical Right - The right to record, reproduce and distribute a work on a carrier
    #[default]
    Mechanical,
    /// Performing Right - The entitlement to perform musical, literary or dramatic works live or by mechanical means
    Performing, 
    /// Synchronisation Right - The right to include and combine a work in timed relation with other works
    Synchronisation,
}

impl TypeOfRight {
    pub fn as_str(&self) -> &'static str {
        match self {
            TypeOfRight::Mechanical => "MEC",
            TypeOfRight::Performing => "PER", 
            TypeOfRight::Synchronisation => "SYN",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "MEC" => Some(TypeOfRight::Mechanical),
            "PER" => Some(TypeOfRight::Performing),
            "SYN" => Some(TypeOfRight::Synchronisation),
            _ => None,
        }
    }
}


impl CwrFieldWrite for TypeOfRight {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for TypeOfRight {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
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
                    description: format!("Type of Right '{}' not found in lookup table. Expected: MEC, PER, SYN", trimmed) 
                });
                (TypeOfRight::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<TypeOfRight> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (type_of_right, warnings) = TypeOfRight::parse_cwr_field(source, field_name, field_title);
            (Some(type_of_right), warnings)
        }
    }
}