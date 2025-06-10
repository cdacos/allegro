//! Version type lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Version type mapping
pub static VERSION_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("MOD", "Modified Version of a musical work");
    m.insert("ORI", "Original Work");
    m
});

/// Validates a version type code
#[must_use]
pub fn is_valid_version_type(code: &str) -> bool {
    VERSION_TYPES.contains_key(code)
}

/// Gets the description for a version type code
#[must_use]
pub fn get_version_type_description(code: &str) -> Option<&'static str> {
    VERSION_TYPES.get(code).copied()
}

/// Gets all valid version type codes
#[must_use]
pub fn get_all_version_types() -> Vec<&'static str> {
    VERSION_TYPES.keys().copied().collect()
}
