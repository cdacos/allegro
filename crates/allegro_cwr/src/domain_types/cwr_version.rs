//! CWR version number type

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// CWR version number (v2.2+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersion(pub f32);

impl CwrVersion {
    pub fn as_str(&self) -> String {
        format!("{:.1}", self.0)
    }

    pub fn supports_version(&self, min_version: f32) -> bool {
        self.0 >= min_version
    }
}

impl CwrFieldWrite for CwrVersion {
    fn to_cwr_str(&self, _width: usize) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for CwrVersion {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<f32>() {
            Ok(version) => (CwrVersion(version), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid version number format: {}", trimmed),
                }];
                (CwrVersion(2.1), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CwrVersion> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (version, warnings) = CwrVersion::parse_cwr_field(source, field_name, field_title);
            (Some(version), warnings)
        }
    }
}
