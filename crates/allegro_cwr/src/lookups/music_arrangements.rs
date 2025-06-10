//! Music Arrangement lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Music Arrangement codes mapping
pub static MUSIC_ARRANGEMENTS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("NEW", "New");
    m.insert("ARR", "Arrangement");
    m.insert("ADM", "Addition");
    m.insert("UNS", "Unspecified arrangement");
    m.insert("ORI", "Original");
    m
});

/// Validates a music arrangement code
pub fn is_valid_music_arrangement(code: &str) -> bool {
    MUSIC_ARRANGEMENTS.contains_key(code)
}

/// Gets the description for a music arrangement code
pub fn get_music_arrangement_description(code: &str) -> Option<&'static str> {
    MUSIC_ARRANGEMENTS.get(code).copied()
}

/// Gets all valid music arrangement codes
pub fn get_all_music_arrangements() -> Vec<&'static str> {
    MUSIC_ARRANGEMENTS.keys().copied().collect()
}
