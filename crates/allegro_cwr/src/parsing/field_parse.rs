//! Trait for parsing CWR fields with warnings

use crate::parsing::warning::CwrWarning;

/// Trait for parsing CWR fields with warnings
pub trait CwrFieldParse: Sized + Default {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>);
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
