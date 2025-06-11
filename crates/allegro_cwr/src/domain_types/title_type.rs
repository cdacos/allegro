//! Title type for CWR alternate title (ALT) records
//!
//! Indicates the type of alternate title being provided for a musical work.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Title type for ALT record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TitleType {
    /// An alternative to an original title
    AlternativeTitle,
    /// The beginning of a text
    FirstLineOfText,
    /// A standardised title in which the elements are arranged in a pre-determined order. Normally created for classical works
    FormalTitle,
    /// A spurious or unacceptable title sometimes mistakenly used for identification
    IncorrectTitle,
    /// A title given to the work by its creator(s) shown in its original language
    #[default]
    OriginalTitle,
    /// An original title translated into a different language
    OriginalTitleTranslated,
    /// A section of a work which is not recognized as an excerpt in its own right and does not have its own ISWC
    PartTitle,
    /// A title from which all initial articles and punctuation have been removed
    RestrictedTitle,
    /// An alternate title created to aid database searching (e.g. where special characters, puns, or slang have been replaced by standardized elements)
    ExtraSearchTitle,
    /// The Original title of the Work in it's original language, using 'accented' National characters
    OriginalTitleWithNationalCharacters,
    /// An alternatice work title in it's original language, using 'accented ' National characters
    AlternativeTitleWithNationalCharacters,
}

impl TitleType {
    pub fn as_str(&self) -> &str {
        match self {
            TitleType::AlternativeTitle => "AT",
            TitleType::FirstLineOfText => "TE",
            TitleType::FormalTitle => "FT",
            TitleType::IncorrectTitle => "IT",
            TitleType::OriginalTitle => "OT",
            TitleType::OriginalTitleTranslated => "TT",
            TitleType::PartTitle => "PT",
            TitleType::RestrictedTitle => "RT",
            TitleType::ExtraSearchTitle => "ET",
            TitleType::OriginalTitleWithNationalCharacters => "OL",
            TitleType::AlternativeTitleWithNationalCharacters => "AL",
        }
    }
}

impl CwrFieldWrite for TitleType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for TitleType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "AT" => (TitleType::AlternativeTitle, vec![]),
            "TE" => (TitleType::FirstLineOfText, vec![]),
            "FT" => (TitleType::FormalTitle, vec![]),
            "IT" => (TitleType::IncorrectTitle, vec![]),
            "OT" => (TitleType::OriginalTitle, vec![]),
            "TT" => (TitleType::OriginalTitleTranslated, vec![]),
            "PT" => (TitleType::PartTitle, vec![]),
            "RT" => (TitleType::RestrictedTitle, vec![]),
            "ET" => (TitleType::ExtraSearchTitle, vec![]),
            "OL" => (TitleType::OriginalTitleWithNationalCharacters, vec![]),
            "AL" => (TitleType::AlternativeTitleWithNationalCharacters, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Unknown title type '{}', defaulting to OT", trimmed),
                }];
                (TitleType::OriginalTitle, warnings)
            }
        }
    }
}
