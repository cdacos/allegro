//! Agreement types lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Agreement types mapping
pub static AGREEMENT_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("OS", "Original Specific");
    m.insert("PS", "Sub-publishing Specific");
    m.insert("PG", "Sub-publishing General");
    m.insert("OG", "Original General");
    m
});

/// Validates an agreement type code
pub fn is_valid_agreement_type(code: &str) -> bool {
    AGREEMENT_TYPES.contains_key(code)
}

/// Gets the description for an agreement type code
pub fn get_agreement_type_description(code: &str) -> Option<&'static str> {
    AGREEMENT_TYPES.get(code).copied()
}

/// Gets all valid agreement type codes
pub fn get_all_agreement_types() -> Vec<&'static str> {
    AGREEMENT_TYPES.keys().copied().collect()
}
