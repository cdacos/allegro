//! Non-Roman Alphabet text type
//!
//! This type behaves exactly like a String but serves as a marker
//! for fields that contain text in non-Roman alphabets (e.g., Cyrillic,
//! Arabic, Chinese, Japanese, Korean, etc.).

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning};
use std::ops::Deref;

/// Non-Roman Alphabet text type
///
/// This type accepts any string value without validation.
/// It serves as a marker for fields containing text in non-Roman alphabets
/// such as writer names, performer names, and titles in languages that
/// use different writing systems.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct NonRomanAlphabet(pub String);

impl NonRomanAlphabet {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for NonRomanAlphabet {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for NonRomanAlphabet {
    fn to_cwr_str(&self, width: usize) -> String {
        // For NonRomanAlphabet fields, we need to maintain exact byte width
        // to preserve field alignment in the fixed-width CWR format
        let bytes = self.as_str().as_bytes();
        if bytes.len() >= width {
            // Truncate to exact byte width if too long
            String::from_utf8_lossy(&bytes[..width]).to_string()
        } else {
            // Pad with spaces to exact byte width if too short
            let mut result = self.as_str().to_string();
            result.push_str(&" ".repeat(width - bytes.len()));
            result
        }
    }
}

impl CwrFieldParse for NonRomanAlphabet {
    fn parse_cwr_field(
        source: &str, _field_name: &'static str, _field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        // No validation - accepts any string for non-Roman alphabet text
        // Preserve exact field width without trimming for proper field alignment
        (NonRomanAlphabet(source.to_string()), vec![])
    }
}

impl CwrFieldParse for Option<NonRomanAlphabet> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        if source.trim().is_empty() {
            (None, vec![])
        } else {
            let (text, warnings) = NonRomanAlphabet::parse_cwr_field(source, field_name, field_title);
            (Some(text), warnings)
        }
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::ToSql for NonRomanAlphabet {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.as_str().to_sql()
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::FromSql for NonRomanAlphabet {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value).map(NonRomanAlphabet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_width_preservation_with_multibyte_utf8() {
        // Test the specific bug: multi-byte UTF-8 characters in NonRomanAlphabet fields
        // should maintain exact byte width to preserve field alignment in CWR format

        // "EVIDÊNCIA" contains "Ê" which is 2 bytes in UTF-8 but 1 character
        let text_with_multibyte = NonRomanAlphabet("EVIDÊNCIA".to_string());

        // The field should be padded to exact byte width, not character width
        let width = 20;
        let formatted = text_with_multibyte.to_cwr_str(width);

        // Should be exactly 20 bytes (not 20 characters)
        assert_eq!(formatted.as_bytes().len(), width);

        // Should contain the original text plus spaces
        assert!(formatted.starts_with("EVIDÊNCIA"));
        assert!(formatted.ends_with(' '));

        // Verify the character vs byte difference: "EVIDÊNCIA" is 9 chars but 10 bytes
        assert_eq!("EVIDÊNCIA".chars().count(), 9);
        assert_eq!("EVIDÊNCIA".as_bytes().len(), 10);

        // So formatted should be "EVIDÊNCIA" (10 bytes) + 10 spaces = 20 bytes total
        assert_eq!(formatted, "EVIDÊNCIA          ");
    }

    #[test]
    fn test_byte_width_truncation_with_multibyte_utf8() {
        // Test truncation maintains byte boundaries to avoid corrupting UTF-8
        let long_text = NonRomanAlphabet("EVIDÊNCIAAAAAAAAAAAAA".to_string());

        // Truncate to 10 bytes (should not break UTF-8 sequence)
        let formatted = long_text.to_cwr_str(10);
        assert_eq!(formatted.as_bytes().len(), 10);

        // Should be valid UTF-8
        assert!(std::str::from_utf8(formatted.as_bytes()).is_ok());

        // Should contain "EVIDÊNCIA" (exactly 10 bytes)
        assert_eq!(formatted, "EVIDÊNCIA");
    }

    #[test]
    fn test_ascii_text_unchanged() {
        // ASCII text should work exactly as before
        let ascii_text = NonRomanAlphabet("HELLO".to_string());
        let formatted = ascii_text.to_cwr_str(10);

        assert_eq!(formatted.as_bytes().len(), 10);
        assert_eq!(formatted, "HELLO     ");
    }

    #[test]
    fn test_roundtrip_integrity_with_multibyte() {
        // This tests the original bug: parse and serialize should be identical
        let original_field = "EVIDÊNCIA          "; // 10 bytes + 10 spaces = 20 bytes

        // Parse the field
        let (parsed, warnings) = NonRomanAlphabet::parse_cwr_field(original_field, "test_field", "Test Field");
        assert!(warnings.is_empty());

        // Serialize back
        let serialized = parsed.to_cwr_str(20);

        // Should be identical (this was the bug - they were different before)
        assert_eq!(serialized, original_field);
        assert_eq!(serialized.as_bytes().len(), original_field.as_bytes().len());
    }
}
