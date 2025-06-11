//! Trait for parsing CWR fields with warnings

use crate::parsing::warning::CwrWarning;

/// Trait for parsing CWR fields with warnings
pub trait CwrFieldParse: Sized + Default {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>);
}

impl CwrFieldParse for String {
    fn parse_cwr_field(
        source: &str, _field_name: &'static str, _field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        // For fixed-width CWR fields, preserve exact spacing to maintain round-trip integrity
        // Only trim if the field is completely empty or whitespace-only
        let trimmed = source.trim();
        if trimmed.is_empty() { (String::new(), vec![]) } else { (source.to_string(), vec![]) }
    }
}

impl CwrFieldParse for Option<String> {
    fn parse_cwr_field(
        source: &str, _field_name: &'static str, _field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            // For fixed-width CWR fields, preserve exact spacing to maintain round-trip integrity
            (Some(source.to_string()), vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parse_basic() {
        let source = "Test string";
        let (result, warnings) = String::parse_cwr_field(source, "test_field", "Test Field");

        assert_eq!(result, "Test string");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_string_parse_empty() {
        let source = "   ";
        let (result, warnings) = String::parse_cwr_field(source, "test_field", "Test Field");

        assert_eq!(result, "");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_option_string_parse_with_value() {
        let source = "Test value";
        let (result, warnings) = Option::<String>::parse_cwr_field(source, "test_field", "Test Field");

        assert_eq!(result, Some("Test value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_option_string_parse_empty() {
        let source = "   ";
        let (result, warnings) = Option::<String>::parse_cwr_field(source, "test_field", "Test Field");

        assert_eq!(result, None);
        assert!(warnings.is_empty());
    }
}
