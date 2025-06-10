//! ISRC (International Standard Recording Code)

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;
use std::ops::Deref;

/// ISRC (International Standard Recording Code) - 12 characters
/// Format: CCXXXYYNNNNN where:
/// - CC: Country code (2 letters)
/// - XXX: Registrant code (3 alphanumeric)
/// - YY: Year (2 digits)
/// - NNNNN: Designation code (5 digits)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Isrc(pub String);

impl Isrc {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validates ISRC format (12 alphanumeric characters)
    pub fn is_valid_format(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.len() == 12 && trimmed.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

impl Deref for Isrc {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for Isrc {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for Isrc {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !Isrc::is_valid_format(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!(
                    "ISRC '{}' should be exactly 12 alphanumeric characters (CCXXXYYNNNNN format)",
                    trimmed
                ),
            });
        }

        (Isrc(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<Isrc> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (isrc, warnings) = Isrc::parse_cwr_field(source, field_name, field_title);
            (Some(isrc), warnings)
        }
    }
}
