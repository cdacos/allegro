//! USA License Indicator

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// USA License Indicator (1 character)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct UsaLicenseIndicator(pub String);

impl UsaLicenseIndicator {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for UsaLicenseIndicator {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for UsaLicenseIndicator {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::usa_license_indicators::is_valid_usa_license_indicator;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_usa_license_indicator(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("USA License Indicator '{}' not found in lookup table", trimmed),
            });
        }

        (UsaLicenseIndicator(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<UsaLicenseIndicator> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (usa_license_indicator, warnings) =
                UsaLicenseIndicator::parse_cwr_field(source, field_name, field_title);
            (Some(usa_license_indicator), warnings)
        }
    }
}
