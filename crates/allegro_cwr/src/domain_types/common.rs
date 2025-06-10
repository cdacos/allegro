//! Common types and traits for CWR field parsing

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

/// Trait for parsing CWR fields with warnings
pub trait CwrFieldParse: Sized + Default {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>);
}

/// Trait for converting CWR fields to their string representation for writing
pub trait CwrFieldWrite {
    fn to_cwr_str(&self) -> String;
}

// Implementations for basic types
impl CwrFieldWrite for String {
    fn to_cwr_str(&self) -> String {
        self.clone()
    }
}

impl<T: CwrFieldWrite> CwrFieldWrite for Option<T> {
    fn to_cwr_str(&self) -> String {
        match self {
            Some(val) => val.to_cwr_str(),
            None => String::new(),
        }
    }
}

impl CwrFieldParse for String {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        // For fixed-width CWR fields, preserve exact spacing to maintain round-trip integrity
        // Only trim if the field is completely empty or whitespace-only
        let trimmed = source.trim();
        if trimmed.is_empty() { (String::new(), vec![]) } else { (source.to_string(), vec![]) }
    }
}

impl CwrFieldParse for Option<String> {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            // For fixed-width CWR fields, preserve exact spacing to maintain round-trip integrity
            (Some(source.to_string()), vec![])
        }
    }
}
