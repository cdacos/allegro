//! IPI Name Number

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;
use std::ops::Deref;

/// IPI Name Number (11 digits)
///
/// International Publisher Identification name numbers are 11-digit codes
/// used to uniquely identify interested parties in the music industry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct IpiNameNumber(pub String);

impl IpiNameNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validates IPI Name Number format (11 digits)
    pub fn is_valid_format(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.len() == 11 && trimmed.chars().all(|c| c.is_ascii_digit())
    }
}

impl Deref for IpiNameNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for IpiNameNumber {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for IpiNameNumber {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !IpiNameNumber::is_valid_format(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("IPI Name Number '{}' should be exactly 11 digits", trimmed),
            });
        }

        (IpiNameNumber(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<IpiNameNumber> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (ipi_name_number, warnings) = IpiNameNumber::parse_cwr_field(source, field_name, field_title);
            (Some(ipi_name_number), warnings)
        }
    }
}
