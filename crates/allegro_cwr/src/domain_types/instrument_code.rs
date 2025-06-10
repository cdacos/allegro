//! Instrument Code

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Instrument Code (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct InstrumentCode(pub String);

impl InstrumentCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for InstrumentCode {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for InstrumentCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::instrument_codes::is_valid_instrument_code;

        let trimmed = source.trim();
        let mut warnings = vec![];

        if !is_valid_instrument_code(trimmed) {
            warnings.push(CwrWarning { 
                field_name, 
                field_title, 
                source_str: Cow::Owned(source.to_string()), 
                level: WarningLevel::Warning, 
                description: format!("Instrument Code '{}' not found in lookup table", trimmed) 
            });
        }

        (InstrumentCode(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<InstrumentCode> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (instrument_code, warnings) = InstrumentCode::parse_cwr_field(source, field_name, field_title);
            (Some(instrument_code), warnings)
        }
    }
}