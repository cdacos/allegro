//! Character set indicator for HDR record

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Character set indicator for HDR record (v2.1+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum CharacterSet {
    #[default]
    ASCII,
    TraditionalBig5,
    SimplifiedGb,
    UTF8,
    Unicode,
    Unknown(String),
}

impl CharacterSet {
    pub fn as_str(&self) -> &str {
        match self {
            CharacterSet::ASCII => "ASCII",
            CharacterSet::TraditionalBig5 => "Traditional Big5",
            CharacterSet::SimplifiedGb => "Simplified GB",
            CharacterSet::UTF8 => "UTF-8",
            CharacterSet::Unicode => "Unicode",
            CharacterSet::Unknown(s) => s,
        }
    }
}

impl CwrFieldWrite for CharacterSet {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for CharacterSet {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "ASCII" => (CharacterSet::ASCII, vec![]),
            "Traditional Big5" => (CharacterSet::TraditionalBig5, vec![]),
            "Simplified GB" => (CharacterSet::SimplifiedGb, vec![]),
            "UTF-8" => (CharacterSet::UTF8, vec![]),
            "Unicode" => (CharacterSet::Unicode, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Info, description: format!("Unknown character set '{}', treating as custom", trimmed) }];
                (CharacterSet::Unknown(trimmed.to_string()), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CharacterSet> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (charset, warnings) = CharacterSet::parse_cwr_field(source, field_name, field_title);
            (Some(charset), warnings)
        }
    }
}
