//! Music Arrangement code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;
use std::ops::Deref;

/// Music Arrangement code (NEW, ARR, ADM, UNS, ORI)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct MusicArrangement(pub String);

impl MusicArrangement {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for MusicArrangement {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for MusicArrangement {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for MusicArrangement {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::music_arrangements::is_valid_music_arrangement;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_music_arrangement(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Music Arrangement code '{}' not found in lookup table", trimmed),
            });
        }

        (MusicArrangement(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<MusicArrangement> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (code, warnings) = MusicArrangement::parse_cwr_field(source, field_name, field_title);
            (Some(code), warnings)
        }
    }
}
