//! Trait for converting CWR fields to their string representation for writing

use crate::domain_types::CharacterSet;

/// Trait for converting CWR fields to their byte representation for writing
pub trait CwrFieldWrite {
    /// Convert field to bytes using the specified character set
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8>;
}

/// Helper function for formatting numeric values with zero-padding
pub fn format_number(value: impl std::fmt::Display, width: usize) -> String {
    format!("{:0width$}", value, width = width)
}

/// Helper function for formatting text values with space-padding on the right
pub fn format_text(value: &str, width: usize) -> String {
    format!("{:width$}", value, width = width)
}

/// Convert a string to bytes according to the specified character set
/// This is the core function that ensures proper encoding for CWR output
pub fn string_to_cwr_bytes(value: &str, character_set: &CharacterSet) -> Vec<u8> {
    match character_set {
        CharacterSet::ASCII => {
            // For ASCII, convert each character to ASCII bytes, replacing non-ASCII with '?'
            let mut bytes = Vec::new();
            for ch in value.chars() {
                if ch.is_ascii() {
                    bytes.push(ch as u8);
                } else {
                    // Replace non-ASCII characters with '?' for ASCII compatibility
                    bytes.push(b'?');
                }
            }
            bytes
        }
        CharacterSet::UTF8 => {
            // For UTF-8, use the string's natural UTF-8 encoding
            value.as_bytes().to_vec()
        }
        CharacterSet::TraditionalBig5 | CharacterSet::SimplifiedGb | CharacterSet::Unicode => {
            // For other character sets, fall back to UTF-8 for now
            // In a real implementation, you'd use proper encoding libraries
            string_to_cwr_bytes(value, &CharacterSet::UTF8)
        }
        CharacterSet::Unknown(_) => {
            // For unknown character sets, default to UTF-8
            string_to_cwr_bytes(value, &CharacterSet::UTF8)
        }
    }
}

/// Convert a formatted text field to CWR bytes with proper width and character set handling
pub fn format_text_to_cwr_bytes(value: &str, width: usize, character_set: &CharacterSet) -> Vec<u8> {
    let formatted_string = format_text(value, width);
    string_to_cwr_bytes(&formatted_string, character_set)
}

/// Trait for numeric fields that need zero-padding to a specific width
pub trait CwrNumericField {
    /// Returns the unpadded numeric value as a string (e.g., "5" instead of "00000005")
    fn to_numeric_str(&self) -> String;

    /// Returns true if this is an empty/None value that should be rendered as "0"
    fn is_empty_numeric(&self) -> bool {
        false
    }
}

impl CwrFieldWrite for String {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self, width, character_set)
    }
}

// Option<T> fields: Always space-padded when None, regardless of T's type
impl<T: CwrFieldWrite> CwrFieldWrite for Option<T> {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        match self {
            Some(val) => val.to_cwr_field_bytes(width, character_set),
            None => {
                let empty_string = format!("{:width$}", "", width = width);
                string_to_cwr_bytes(&empty_string, character_set)
            }
        }
    }
}
