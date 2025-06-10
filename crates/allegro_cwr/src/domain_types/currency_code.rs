//! ISO 4217 currency code

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// ISO 4217 currency code
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for CurrencyCode {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for CurrencyCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::currency_codes::is_valid_currency_code;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_currency_code(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Currency code '{}' not found in ISO 4217 table", trimmed),
            });
        }

        (CurrencyCode(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<CurrencyCode> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (currency, warnings) = CurrencyCode::parse_cwr_field(source, field_name, field_title);
            (Some(currency), warnings)
        }
    }
}
