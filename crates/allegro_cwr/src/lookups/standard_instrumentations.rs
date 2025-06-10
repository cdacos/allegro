//! Standard Instrumentation lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Standard Instrumentation codes mapping
pub static STANDARD_INSTRUMENTATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AUD", "Audience");
    m.insert("BND", "Band");
    m.insert("BBA", "Brass Band");
    m.insert("BRC", "Brass Choir");
    m.insert("BQU", "Brass Quartet");
    m.insert("BQN", "Brass Quintet");
    m.insert("BXT", "Brass Sextet");
    m.insert("BTR", "Brass Trio");
    m.insert("CEN", "Chamber Ensemble");
    m.insert("CHO", "Chorus");
    m.insert("CAO", "Chorus and Orchestra");
    m.insert("BCH", "Chorus, Boys'");
    m.insert("CCH", "Chorus, Children's");
    m.insert("DCH", "Chorus, Double");
    m.insert("MCH", "Chorus, Men's");
    m.insert("MXC", "Chorus, Mixed");
    m.insert("TCH", "Chorus, Treble");
    m.insert("UCH", "Chorus, Unison");
    m.insert("WCH", "Chorus, Women's");
    m.insert("YCH", "Chorus, Youth");
    m.insert("CLC", "Clarinet Choir");
    m.insert("CQN", "Clarinet Quintet");
    m.insert("CBA", "Concert Band");
    m.insert("FLC", "Flute Choir");
    m.insert("GML", "Gamelan");
    m.insert("GQT", "Guitar Quartet");
    m.insert("HNC", "Horn Choir");
    m.insert("JZE", "Jazz Ensemble");
    m.insert("OQU", "Oboe Quartet");
    m.insert("COR", "Orchestra, Chamber");
    m.insert("ORC", "Orchestra, Full");
    m.insert("SOR", "Orchestra, String");
    m.insert("PCE", "Percussion Ensemble");
    m.insert("PDU", "Piano Duo");
    m.insert("PFH", "Piano Four Hands");
    m.insert("PQU", "Piano Quartet");
    m.insert("PQN", "Piano Quintet");
    m.insert("PTR", "Piano Trio");
    m.insert("SQT", "Saxophone Quartet");
    m.insert("SOC", "String Octet");
    m.insert("SQU", "String Quartet");
    m.insert("SQN", "String Quintet");
    m.insert("SGT", "String Trio");
    m.insert("SBA", "Symphonic Band");
    m.insert("TBC", "Trombone Choir");
    m.insert("TPC", "Trumpet Choir");
    m.insert("TUC", "Tuba Choir");
    m.insert("WEN", "Wind Ensemble");
    m.insert("WQR", "Woodwind Quartet");
    m.insert("WQN", "Woodwind Quintet");
    m
});

/// Validates a standard instrumentation code
pub fn is_valid_standard_instrumentation(code: &str) -> bool {
    STANDARD_INSTRUMENTATIONS.contains_key(code)
}

/// Gets the description for a standard instrumentation code
pub fn get_standard_instrumentation_description(code: &str) -> Option<&'static str> {
    STANDARD_INSTRUMENTATIONS.get(code).copied()
}

/// Gets all valid standard instrumentation codes
pub fn get_all_standard_instrumentations() -> Vec<&'static str> {
    STANDARD_INSTRUMENTATIONS.keys().copied().collect()
}
