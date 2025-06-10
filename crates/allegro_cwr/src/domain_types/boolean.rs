use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
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
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for Boolean {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (Boolean::Yes, vec![]),
            "N" => (Boolean::No, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid Yes/No value '{}', defaulting to No", trimmed) }];
                (Boolean::No, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Boolean> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (yes_no, warnings) = Boolean::parse_cwr_field(source, field_name, field_title);
            (Some(yes_no), warnings)
        }
    }
}
