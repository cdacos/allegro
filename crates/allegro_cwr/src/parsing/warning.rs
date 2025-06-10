//! Warning types for CWR parsing

use std::borrow::Cow;

/// Warning levels for CWR parsing
#[derive(Debug, Clone, PartialEq)]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// Warning generated during CWR parsing
#[derive(Debug, Clone, PartialEq)]
pub struct CwrWarning<'a> {
    pub field_name: &'static str,
    pub field_title: &'static str,
    pub source_str: Cow<'a, str>,
    pub level: WarningLevel,
    pub description: String,
}

impl CwrWarning<'_> {
    pub fn is_critical(&self) -> bool {
        matches!(self.level, WarningLevel::Critical)
    }
}