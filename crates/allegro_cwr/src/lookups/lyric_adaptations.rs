//! Lyric Adaptation lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Lyric Adaptation codes mapping
pub static LYRIC_ADAPTATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("NEW", "New");
    m.insert("MOD", "Modification");
    m.insert("NON", "None");
    m.insert("ORI", "Original");
    m.insert("REP", "Replacement L");
    m.insert("ADL", "Addition");
    m.insert("UNS", "Unspecified");
    m.insert("TRA", "Translation");
    m
});

/// Validates a lyric adaptation code
pub fn is_valid_lyric_adaptation(code: &str) -> bool {
    LYRIC_ADAPTATIONS.contains_key(code)
}

/// Gets the description for a lyric adaptation code
pub fn get_lyric_adaptation_description(code: &str) -> Option<&'static str> {
    LYRIC_ADAPTATIONS.get(code).copied()
}

/// Gets all valid lyric adaptation codes
pub fn get_all_lyric_adaptations() -> Vec<&'static str> {
    LYRIC_ADAPTATIONS.keys().copied().collect()
}