//! IPI Base Number

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;
use std::ops::Deref;

/// IPI Base Number (13 alphanumeric characters)
///
/// International Publisher Identification base numbers are 13-character alphanumeric codes
/// used to uniquely identify interested parties in the music industry.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct IpiBaseNumber(pub String);

impl IpiBaseNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validates IPI Base Number format (13 alphanumeric characters)
    pub fn is_valid_format(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.len() == 13 && trimmed.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

impl Deref for IpiBaseNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for IpiBaseNumber {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for IpiBaseNumber {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !IpiBaseNumber::is_valid_format(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("IPI Base Number '{}' should be exactly 13 alphanumeric characters", trimmed),
            });
        }

        (IpiBaseNumber(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<IpiBaseNumber> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (ipi_base_number, warnings) = IpiBaseNumber::parse_cwr_field(source, field_name, field_title);
            (Some(ipi_base_number), warnings)
        }
    }
}
