//! Media Type

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;
use std::ops::Deref;

/// Media Type (variable length, typically 1-3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct MediaType(pub String);

impl MediaType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for MediaType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for MediaType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for MediaType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::media_types::is_valid_media_type;

        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !is_valid_media_type(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Media Type '{}' not found in lookup table", trimmed),
            });
        }

        (MediaType(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<MediaType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (media_type, warnings) = MediaType::parse_cwr_field(source, field_name, field_title);
            (Some(media_type), warnings)
        }
    }
}
