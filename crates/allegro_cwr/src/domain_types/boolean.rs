use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum Boolean {
    Yes,
    #[default]
    No,
}

impl Boolean {
    pub fn as_str(&self) -> &str {
        match self {
            Boolean::Yes => "Y",
            Boolean::No => "N",
        }
    }
}

impl CwrFieldWrite for Boolean {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for Boolean {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (Boolean::Yes, vec![]),
            "N" => (Boolean::No, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid Yes/No value '{}', defaulting to No", trimmed),
                }];
                (Boolean::No, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Boolean> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (yes_no, warnings) = Boolean::parse_cwr_field(source, field_name, field_title);
            (Some(yes_no), warnings)
        }
    }
}
