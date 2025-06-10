//! Composite type lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Composite type mapping
pub static COMPOSITE_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("COS", "Composite of Samples");
    m.insert("MED", "Medley");
    m.insert("POT", "Potpourri");
    m.insert("UCO", "Unspecified Composite");
    m.insert("", "Non-Composite"); // blank value for non-composite works
    m
});

/// Validates a composite type code
#[must_use]
pub fn is_valid_composite_type(code: &str) -> bool {
    COMPOSITE_TYPES.contains_key(code)
}

/// Gets the description for a composite type code
#[must_use]
pub fn get_composite_type_description(code: &str) -> Option<&'static str> {
    COMPOSITE_TYPES.get(code).copied()
}

/// Gets all valid composite type codes
#[must_use]
pub fn get_all_composite_types() -> Vec<&'static str> {
    COMPOSITE_TYPES.keys().copied().collect()
}
