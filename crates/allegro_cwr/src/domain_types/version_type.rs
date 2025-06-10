//! Version Type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Version Type (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct VersionType(pub String);

impl VersionType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for VersionType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for VersionType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::version_types::is_valid_version_type;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_version_type(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Version Type '{}' not found in lookup table", trimmed),
            });
        }

        (VersionType(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<VersionType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (version_type, warnings) = VersionType::parse_cwr_field(source, field_name, field_title);
            (Some(version_type), warnings)
        }
    }
}
