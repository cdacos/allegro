//! Text Music Relationship code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;
use std::ops::Deref;

/// Text Music Relationship code (MUS, MTX, TXT, MTN)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TextMusicRelationship(pub String);

impl TextMusicRelationship {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for TextMusicRelationship {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for TextMusicRelationship {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for TextMusicRelationship {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::text_music_relationships::is_valid_text_music_relationship;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_text_music_relationship(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Text Music Relationship code '{}' not found in lookup table", trimmed),
            });
        }

        (TextMusicRelationship(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<TextMusicRelationship> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (code, warnings) = TextMusicRelationship::parse_cwr_field(source, field_name, field_title);
            (Some(code), warnings)
        }
    }
}
