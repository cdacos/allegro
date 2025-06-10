//! USA License Indicator lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// USA License Indicator mapping
pub static USA_LICENSE_INDICATORS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("A", "ASCAP");
    m.insert("B", "BMI");
    m.insert("S", "SESAC");
    m.insert("M", "AMRA");
    m.insert("G", "GMR");
    m
});

/// Validates a USA license indicator code
#[must_use]
pub fn is_valid_usa_license_indicator(code: &str) -> bool {
    USA_LICENSE_INDICATORS.contains_key(code)
}

/// Gets the description for a USA license indicator code
#[must_use]
pub fn get_usa_license_indicator_description(code: &str) -> Option<&'static str> {
    USA_LICENSE_INDICATORS.get(code).copied()
}

/// Gets all valid USA license indicator codes
#[must_use]
pub fn get_all_usa_license_indicators() -> Vec<&'static str> {
    USA_LICENSE_INDICATORS.keys().copied().collect()
}
