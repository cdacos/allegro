//! Intended Purpose

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Intended Purpose (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum IntendedPurpose {
    /// Commercial / Jingle / Trailer
    Commercial,
    /// Film
    Film,
    /// General Usage
    #[default]
    General,
    /// Library Work
    Library,
    /// Multimedia
    Multimedia,
    /// Radio
    Radio,
    /// Television
    Television,
    /// Theatre
    Theatre,
    /// Video
    Video,
}

impl IntendedPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            IntendedPurpose::Commercial => "COM",
            IntendedPurpose::Film => "FIL",
            IntendedPurpose::General => "GEN",
            IntendedPurpose::Library => "LIB",
            IntendedPurpose::Multimedia => "MUL",
            IntendedPurpose::Radio => "RAD",
            IntendedPurpose::Television => "TEL",
            IntendedPurpose::Theatre => "THR",
            IntendedPurpose::Video => "VID",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "COM" => Some(IntendedPurpose::Commercial),
            "FIL" => Some(IntendedPurpose::Film),
            "GEN" => Some(IntendedPurpose::General),
            "LIB" => Some(IntendedPurpose::Library),
            "MUL" => Some(IntendedPurpose::Multimedia),
            "RAD" => Some(IntendedPurpose::Radio),
            "TEL" => Some(IntendedPurpose::Television),
            "THR" => Some(IntendedPurpose::Theatre),
            "VID" => Some(IntendedPurpose::Video),
            _ => None,
        }
    }
}

impl CwrFieldWrite for IntendedPurpose {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for IntendedPurpose {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match IntendedPurpose::from_str(trimmed) {
            Some(intended_purpose) => (intended_purpose, warnings),
            None => {
                warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Intended Purpose '{}' not found in lookup table. Expected: COM, FIL, GEN, LIB, MUL, RAD, TEL, THR, VID", trimmed) });
                (IntendedPurpose::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<IntendedPurpose> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (intended_purpose, warnings) = IntendedPurpose::parse_cwr_field(source, field_name, field_title);
            (Some(intended_purpose), warnings)
        }
    }
}
