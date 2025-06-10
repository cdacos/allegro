//! EAN (European Article Number / International Article Number)

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;
use std::ops::Deref;

/// EAN (European Article Number) - 13 digits
/// Also known as International Article Number or UPC-A barcode.
/// Used for commercial product identification.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Ean(pub String);

impl Ean {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validates EAN format (13 digits)
    pub fn is_valid_format(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.len() == 13 && trimmed.chars().all(|c| c.is_ascii_digit())
    }
}

impl Deref for Ean {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for Ean {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for Ean {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !Ean::is_valid_format(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("EAN '{}' should be exactly 13 digits", trimmed),
            });
        }

        (Ean(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<Ean> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (ean, warnings) = Ean::parse_cwr_field(source, field_name, field_title);
            (Some(ean), warnings)
        }
    }
}
