//! Text Music Relationship lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Text Music Relationship codes mapping
pub static TEXT_MUSIC_RELATIONSHIPS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("MUS", "Music");
    m.insert("MTX", "Music and Text");
    m.insert("TXT", "Text");
    m.insert("MTN", "Music and Text (separate creation)");
    m
});

/// Validates a text music relationship code
pub fn is_valid_text_music_relationship(code: &str) -> bool {
    TEXT_MUSIC_RELATIONSHIPS.contains_key(code)
}

/// Gets the description for a text music relationship code
pub fn get_text_music_relationship_description(code: &str) -> Option<&'static str> {
    TEXT_MUSIC_RELATIONSHIPS.get(code).copied()
}

/// Gets all valid text music relationship codes
pub fn get_all_text_music_relationships() -> Vec<&'static str> {
    TEXT_MUSIC_RELATIONSHIPS.keys().copied().collect()
}