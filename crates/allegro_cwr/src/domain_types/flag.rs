use crate::domain_types::CharacterSet;
use crate::domain_types::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use crate::parsing::format_text_to_cwr_bytes;
use std::borrow::Cow;

/// Flag with Yes/No/Unknown values
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum Flag {
    Yes,
    No,
    #[default]
    Unknown,
}

impl Flag {
    pub fn as_str(&self) -> &str {
        match self {
            Flag::Yes => "Y",
            Flag::No => "N",
            Flag::Unknown => "U",
        }
    }
}

impl CwrFieldWrite for Flag {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for Flag {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (Flag::Yes, vec![]),
            "N" => (Flag::No, vec![]),
            "U" => (Flag::Unknown, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid Y/N/U flag value '{}', defaulting to No", trimmed),
                }];
                (Flag::No, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Flag> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (flag, warnings) = Flag::parse_cwr_field(source, field_name, field_title);
            (Some(flag), warnings)
        }
    }
}
