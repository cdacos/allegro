//! Work types lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Work types mapping
pub static WORK_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("TA", "AAA (Triple A)");
    m.insert("AC", "Adult Contemporary");
    m.insert("AR", "Album Oriented Rock (AOR)");
    m.insert("AL", "Alternative Music");
    m.insert("AM", "Americana");
    m.insert("BD", "Band");
    m.insert("BL", "Bluegrass Music");
    m.insert("CD", "Children's Music");
    m.insert("CL", "Classical Music");
    m.insert("CC", "Contemporary Christian");
    m.insert("CT", "Country Music");
    m.insert("DN", "Dance");
    m.insert("FM", "Film/ Television Music");
    m.insert("FK", "Folk Music");
    m.insert("BG", "Gospel (Black)");
    m.insert("SG", "Gospel (Southern)");
    m.insert("JZ", "Jazz Music");
    m.insert("JG", "Jingles");
    m.insert("LN", "Latin");
    m.insert("LA", "Latina");
    m.insert("NA", "New Age");
    m.insert("OP", "Opera");
    m.insert("PK", "Polka Music");
    m.insert("PP", "Pop Music");
    m.insert("RP", "Rap Music");
    m.insert("RK", "Rock Music");
    m.insert("RB", "Rhythm and Blues");
    m.insert("SD", "Sacred");
    m.insert("SY", "Symphonic");
    m
});

/// Validates a work type code
#[must_use]
pub fn is_valid_work_type(code: &str) -> bool {
    WORK_TYPES.contains_key(code)
}

/// Gets the description for a work type code
#[must_use]
pub fn get_work_type_description(code: &str) -> Option<&'static str> {
    WORK_TYPES.get(code).copied()
}

/// Gets all valid work type codes
#[must_use]
pub fn get_all_work_types() -> Vec<&'static str> {
    WORK_TYPES.keys().copied().collect()
}
