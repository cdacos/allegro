//! Writer Designation

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Writer Designation (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct WriterDesignation(pub String);

impl WriterDesignation {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for WriterDesignation {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for WriterDesignation {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::writer_designations::is_valid_writer_designation;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_writer_designation(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Writer Designation '{}' not found in lookup table", trimmed),
            });
        }

        (WriterDesignation(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<WriterDesignation> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (writer_designation, warnings) = WriterDesignation::parse_cwr_field(source, field_name, field_title);
            (Some(writer_designation), warnings)
        }
    }
}
