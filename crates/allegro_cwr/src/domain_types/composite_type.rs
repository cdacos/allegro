//! Composite Type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// Composite Type (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CompositeType(pub String);

impl CompositeType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for CompositeType {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for CompositeType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::composite_types::is_valid_composite_type;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_composite_type(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Composite Type '{}' not found in lookup table", trimmed),
            });
        }

        (CompositeType(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<CompositeType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (composite_type, warnings) = CompositeType::parse_cwr_field(source, field_name, field_title);
            (Some(composite_type), warnings)
        }
    }
}
