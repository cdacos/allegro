//! Society Code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Society Code (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SocietyCode(pub String);

impl SocietyCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for SocietyCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for SocietyCode {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for SocietyCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::society_codes::is_valid_society_code;

        let trimmed = source.trim();
        let mut warnings = vec![];

        if !is_valid_society_code(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Society Code '{}' not found in lookup table", trimmed),
            });
        }

        (SocietyCode(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<SocietyCode> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (society_code, warnings) = SocietyCode::parse_cwr_field(source, field_name, field_title);
            (Some(society_code), warnings)
        }
    }
}
