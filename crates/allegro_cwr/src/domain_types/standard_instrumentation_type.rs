//! Standard Instrumentation Type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;
use std::ops::Deref;

/// Standard Instrumentation Type (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct StandardInstrumentationType(pub String);

impl StandardInstrumentationType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for StandardInstrumentationType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for StandardInstrumentationType {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for StandardInstrumentationType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::standard_instrumentations::is_valid_standard_instrumentation;

        let trimmed = source.trim();
        let mut warnings = vec![];

        if !trimmed.is_empty() && !is_valid_standard_instrumentation(trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Standard Instrumentation Type '{}' not found in lookup table", trimmed),
            });
        }

        (StandardInstrumentationType(trimmed.to_string()), warnings)
    }
}

impl CwrFieldParse for Option<StandardInstrumentationType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (instrumentation_type, warnings) =
                StandardInstrumentationType::parse_cwr_field(source, field_name, field_title);
            (Some(instrumentation_type), warnings)
        }
    }
}
