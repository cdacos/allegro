//! Writer designation lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Writer designation mapping
pub static WRITER_DESIGNATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AD", "Adaptor");
    m.insert("AR", "Arranger");
    m.insert("A", "Author, Writer, Author of Lyrics");
    m.insert("C", "Composer, Writer");
    m.insert("CA", "Composer/Author");
    m.insert("SR", "Sub Arranger");
    m.insert("SA", "Sub Author");
    m.insert("TR", "Translator");
    m.insert("PA", "Income Participant");
    m
});

/// Validates a writer designation code
#[must_use]
pub fn is_valid_writer_designation(code: &str) -> bool {
    WRITER_DESIGNATIONS.contains_key(code)
}

/// Gets the description for a writer designation code
#[must_use]
pub fn get_writer_designation_description(code: &str) -> Option<&'static str> {
    WRITER_DESIGNATIONS.get(code).copied()
}

/// Gets all valid writer designation codes
#[must_use]
pub fn get_all_writer_designations() -> Vec<&'static str> {
    WRITER_DESIGNATIONS.keys().copied().collect()
}
