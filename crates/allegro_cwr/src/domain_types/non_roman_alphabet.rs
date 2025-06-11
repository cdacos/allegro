//! Non-Roman Alphabet text type
//!
//! This type behaves exactly like a String but serves as a marker
//! for fields that contain text in non-Roman alphabets (e.g., Cyrillic,
//! Arabic, Chinese, Japanese, Korean, etc.).

use crate::domain_types::CharacterSet;
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
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        match character_set {
            CharacterSet::ASCII => {
                // For ASCII, convert to ASCII bytes and handle non-ASCII chars
                let mut bytes = Vec::new();
                for ch in self.as_str().chars() {
                    if ch.is_ascii() {
                        bytes.push(ch as u8);
                    } else {
                        // Replace non-ASCII with '?' for ASCII compatibility
                        bytes.push(b'?');
                    }
                }

                if bytes.len() >= width {
                    bytes.truncate(width);
                } else {
                    bytes.resize(width, b' ');
                }
                bytes
            }
            CharacterSet::UTF8 => {
                // For UTF-8, use the string's natural UTF-8 encoding
                let utf8_bytes = self.as_str().as_bytes();
                if utf8_bytes.len() >= width {
                    // Truncate to exact byte width, being careful not to break UTF-8 sequences
                    let mut truncated = &utf8_bytes[..width];
                    while std::str::from_utf8(truncated).is_err() && !truncated.is_empty() {
                        truncated = &truncated[..truncated.len() - 1];
                    }
                    let mut result = truncated.to_vec();
                    result.resize(width, b' ');
                    result
                } else {
                    let mut result = utf8_bytes.to_vec();
                    result.resize(width, b' ');
                    result
                }
            }
            CharacterSet::TraditionalBig5 | CharacterSet::SimplifiedGb | CharacterSet::Unicode => {
                // TODO: For other character sets, fall back to UTF-8 for now
                // In a real implementation, you'd use proper encoding libraries
                self.to_cwr_field_bytes(width, &CharacterSet::UTF8)
            }
            CharacterSet::Unknown(_) => {
                // TODO: For unknown character sets, default to UTF-8
                self.to_cwr_field_bytes(width, &CharacterSet::UTF8)
            }
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
        let formatted = String::from_utf8(text_with_multibyte.to_cwr_field_bytes(width, &CharacterSet::UTF8)).unwrap();

        // Should be exactly 20 bytes (not 20 characters)
        assert_eq!(formatted.len(), width);

        // Should contain the original text plus spaces
        assert!(formatted.starts_with("EVIDÊNCIA"));
        assert!(formatted.ends_with(' '));

        // Verify the character vs byte difference: "EVIDÊNCIA" is 9 chars but 10 bytes
        assert_eq!("EVIDÊNCIA".chars().count(), 9);
        assert_eq!("EVIDÊNCIA".len(), 10);

        // So formatted should be "EVIDÊNCIA" (10 bytes) + 10 spaces = 20 bytes total
        assert_eq!(formatted, "EVIDÊNCIA          ");
    }

    #[test]
    fn test_byte_width_truncation_with_multibyte_utf8() {
        // Test truncation maintains byte boundaries to avoid corrupting UTF-8
        let long_text = NonRomanAlphabet("EVIDÊNCIAAAAAAAAAAAAA".to_string());

        // Truncate to 10 bytes (should not break UTF-8 sequence)
        let formatted = String::from_utf8(long_text.to_cwr_field_bytes(10, &CharacterSet::UTF8)).unwrap();
        assert_eq!(formatted.len(), 10);

        // Should be valid UTF-8
        assert!(std::str::from_utf8(formatted.as_bytes()).is_ok());

        // Should contain "EVIDÊNCIA" (exactly 10 bytes)
        assert_eq!(formatted, "EVIDÊNCIA");
    }

    #[test]
    fn test_ascii_text_unchanged() {
        // ASCII text should work exactly as before
        let ascii_text = NonRomanAlphabet("HELLO".to_string());
        let formatted = String::from_utf8(ascii_text.to_cwr_field_bytes(10, &CharacterSet::ASCII)).unwrap();

        assert_eq!(formatted.len(), 10);
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
        let serialized = String::from_utf8(parsed.to_cwr_field_bytes(20, &CharacterSet::UTF8)).unwrap();

        // Should be identical (this was the bug - they were different before)
        assert_eq!(serialized, original_field);
        assert_eq!(serialized.len(), original_field.len());
    }
}
