//! Title type for CWR alternate title (ALT) records
//!
//! Indicates the type of alternate title being provided for a musical work.

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Title type for ALT record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TitleType {
    AlternateTitle,
    FormalTitle,
    #[default]
    OriginalTitle,
    TransliteratedTitle,
    AbbreviatedTitle,
    SearchTitle,
    TranslatedTitle,
    TransliterationTrans,
    TransliterationAlt,
    // Add more as needed
}

impl TitleType {
    pub fn as_str(&self) -> &str {
        match self {
            TitleType::AlternateTitle => "AT",
            TitleType::FormalTitle => "FT",
            TitleType::OriginalTitle => "OT",
            TitleType::TransliteratedTitle => "TR",
            TitleType::AbbreviatedTitle => "AB",
            TitleType::SearchTitle => "ST",
            TitleType::TranslatedTitle => "TT",
            TitleType::TransliterationTrans => "LT",
            TitleType::TransliterationAlt => "LA",
        }
    }
}

impl CwrFieldWrite for TitleType {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for TitleType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "AT" => (TitleType::AlternateTitle, vec![]),
            "FT" => (TitleType::FormalTitle, vec![]),
            "OT" => (TitleType::OriginalTitle, vec![]),
            "TR" => (TitleType::TransliteratedTitle, vec![]),
            "LT" => (TitleType::TransliterationTrans, vec![]),
            "LA" => (TitleType::TransliterationAlt, vec![]),
            "AB" => (TitleType::AbbreviatedTitle, vec![]),
            "ST" => (TitleType::SearchTitle, vec![]),
            "TT" => (TitleType::TranslatedTitle, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Unknown title type '{}', defaulting to AT", trimmed),
                }];
                (TitleType::AlternateTitle, warnings)
            }
        }
    }
}
