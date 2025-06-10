//! CIS Language code

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// CIS Language code (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LanguageCode(pub String);

impl LanguageCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for LanguageCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for LanguageCode {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for LanguageCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::language_codes::is_valid_language_code;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_language_code(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Language code '{}' not found in CIS Language Code table", trimmed),
            });
        }

        (LanguageCode(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<LanguageCode> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (language_code, warnings) = LanguageCode::parse_cwr_field(source, field_name, field_title);
            (Some(language_code), warnings)
        }
    }
}
