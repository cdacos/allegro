//! Musical Work Distribution Category lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Musical Work Distribution Category mapping
pub static MUSICAL_WORK_DISTRIBUTION_CATEGORIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("JAZ", "Jazz");
    m.insert("POP", "Popular");
    m.insert("SER", "Serious");
    m.insert("UNC", "Unclassified Distribution Category");
    m
});

/// Validates a musical work distribution category code
#[must_use]
pub fn is_valid_musical_work_distribution_category(code: &str) -> bool {
    MUSICAL_WORK_DISTRIBUTION_CATEGORIES.contains_key(code)
}

/// Gets the description for a musical work distribution category code
#[must_use]
pub fn get_musical_work_distribution_category_description(code: &str) -> Option<&'static str> {
    MUSICAL_WORK_DISTRIBUTION_CATEGORIES.get(code).copied()
}

/// Gets all valid musical work distribution category codes
#[must_use]
pub fn get_all_musical_work_distribution_categories() -> Vec<&'static str> {
    MUSICAL_WORK_DISTRIBUTION_CATEGORIES.keys().copied().collect()
}
