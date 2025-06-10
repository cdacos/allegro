//! Intended Purpose lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Intended Purpose codes mapping
pub static INTENDED_PURPOSES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("COM", "Commercial / Jingle / Trailer");
    m.insert("FIL", "Film");
    m.insert("GEN", "General Usage");
    m.insert("LIB", "Library Work");
    m.insert("MUL", "Multimedia");
    m.insert("RAD", "Radio");
    m.insert("TEL", "Television");
    m.insert("THR", "Theatre");
    m.insert("VID", "Video");
    m
});

/// Validates an intended purpose code
pub fn is_valid_intended_purpose(code: &str) -> bool {
    INTENDED_PURPOSES.contains_key(code)
}

/// Gets the description for an intended purpose code
pub fn get_intended_purpose_description(code: &str) -> Option<&'static str> {
    INTENDED_PURPOSES.get(code).copied()
}

/// Gets all valid intended purpose codes
pub fn get_all_intended_purposes() -> Vec<&'static str> {
    INTENDED_PURPOSES.keys().copied().collect()
}
