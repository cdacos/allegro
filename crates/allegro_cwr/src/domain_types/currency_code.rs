//! ISO 4217 currency code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
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
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
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
