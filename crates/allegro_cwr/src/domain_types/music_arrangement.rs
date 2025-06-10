//! Music Arrangement code

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
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
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
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
