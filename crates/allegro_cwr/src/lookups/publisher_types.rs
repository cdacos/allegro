//! Publisher types lookup table

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Publisher type codes mapping
pub static PUBLISHER_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AQ", "Acquirer");
    m.insert("AM", "Administrator");
    m.insert("PA", "Income Participant");
    m.insert("E", "Original Publisher");
    m.insert("ES", "Substituted Publisher");
    m.insert("SE", "Sub Publisher");
    m
});

/// Validates a publisher type code
pub fn is_valid_publisher_type(code: &str) -> bool {
    PUBLISHER_TYPES.contains_key(code)
}

/// Gets the description for a publisher type code
pub fn get_publisher_type_description(code: &str) -> Option<&'static str> {
    PUBLISHER_TYPES.get(code).copied()
}

/// Gets all valid publisher type codes
pub fn get_all_publisher_type_codes() -> Vec<&'static str> {
    PUBLISHER_TYPES.keys().copied().collect()
}