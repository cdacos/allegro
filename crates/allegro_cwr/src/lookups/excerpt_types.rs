//! Excerpt Type lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Excerpt Type codes mapping
pub static EXCERPT_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("MOV", "Movement");
    m.insert("UEX", "Unspecified Excerpt");
    m.insert("", "Non-Excerpt");
    m
});

/// Validates an excerpt type code
pub fn is_valid_excerpt_type(code: &str) -> bool {
    EXCERPT_TYPES.contains_key(code)
}

/// Gets the description for an excerpt type code
pub fn get_excerpt_type_description(code: &str) -> Option<&'static str> {
    EXCERPT_TYPES.get(code).copied()
}

/// Gets all valid excerpt type codes
pub fn get_all_excerpt_types() -> Vec<&'static str> {
    EXCERPT_TYPES.keys().copied().collect()
}
