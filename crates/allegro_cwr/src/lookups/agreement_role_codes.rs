//! Agreement role codes lookup table

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Agreement role codes mapping
pub static AGREEMENT_ROLE_CODES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AS", "Assignor");
    m.insert("AC", "Acquirer");
    m
});

/// Validates an agreement role code
pub fn is_valid_agreement_role_code(code: &str) -> bool {
    AGREEMENT_ROLE_CODES.contains_key(code)
}

/// Gets the description for an agreement role code
pub fn get_agreement_role_description(code: &str) -> Option<&'static str> {
    AGREEMENT_ROLE_CODES.get(code).copied()
}

/// Gets all valid agreement role codes
pub fn get_all_agreement_role_codes() -> Vec<&'static str> {
    AGREEMENT_ROLE_CODES.keys().copied().collect()
}