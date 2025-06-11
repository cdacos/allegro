//! Lyric Adaptation code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;
use std::ops::Deref;

/// Lyric Adaptation code (NEW, MOD, NON, ORI, REP, ADL, UNS, TRA)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LyricAdaptation(pub String);

impl LyricAdaptation {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for LyricAdaptation {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for LyricAdaptation {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for LyricAdaptation {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::lyric_adaptations::is_valid_lyric_adaptation;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_lyric_adaptation(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Lyric Adaptation code '{}' not found in lookup table", trimmed),
            });
        }

        (LyricAdaptation(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<LyricAdaptation> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (code, warnings) = LyricAdaptation::parse_cwr_field(source, field_name, field_title);
            (Some(code), warnings)
        }
    }
}
