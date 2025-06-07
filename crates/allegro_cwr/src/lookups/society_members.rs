//! Society members lookup table with transmitter codes

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Society member transmitter codes mapping
pub static SOCIETY_TRANSMITTER_CODES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("0000", "ALL RECIPIENTS");
    m.insert("THI", "13 Music Limited");
    m.insert("DUC", "1830 Music");
    m.insert("DNM", "22d Music Netherlands");
    m.insert("TTD", "22D MUSIC US1");
    m.insert("FCM", "411 Publishing Co 10");
    m.insert("FCM", "411 Publishing Company 20");
    m.insert("FOO", "411 Publishing Company 20");
    m.insert("4AD", "4AD Music Limited");
    m.insert("AE", "A AND E TELEVISION NETWORKS");
    m.insert("226", "AACIMH");
    m.insert("ABO", "ABOOD MUSIC LIMITED");
    m.insert("166", "ABRAC");
    m.insert("167", "ABRAMUS");
    m.insert("107", "ACAM");
    m.insert("168", "ACCESS");
    m.insert("ACC", "Accorder Music Publishing America");
    m.insert("ACC", "Accorder Music Publishing Ltd");
    m.insert("103", "ACDAM");
    m.insert("76", "ACEMLA");
    m.insert("ACT", "ACTIVE MUSIC PUBLISHING PTY L");
    m.insert("1", "ACUM");
    m.insert("148", "ADAGP");
    m.insert("230", "ADAVIS");
    m.insert("2", "ADDAF");
    m.insert("3", "AEPI");
    m.insert("AE", "AETN MUSIC PUBLISHING");
    m.insert("AE", "AETN UK MUSIC PUBLISHING");
    m.insert("AFR", "Africori Limited");
    m.insert("MLG", "AFSG");
    m.insert("4", "AGADU");
    m.insert("114", "AGAYC");
    m.insert("AAL", "AIR EDEL ASSOCIATES LTD");
    m.insert("AIR", "AIRSTATE LTD");
    m.insert("122", "AKKA/LAA");
    m.insert("5", "AKM");
    m.insert("127", "ALBAUTOR");
    m.insert("54", "ALCS");
    m.insert("AML", "Alibi Music Library Limited");
    m.insert("ALL", "All Music Publishing");
    m.insert("ALF", "Allforone Music Sarl");
    m.insert("AX", "Alliance Audiosparx");
    m.insert("AM", "Almo Music Corporation");
    m.insert("AC", "Almo Music of Canada");
    m.insert("ALP", "Alpha Music Publishing Uk Limited");
    m.insert("AB", "Altitude Balloon");
    m.insert("ALV", "Alvaro Music Bv");
    m.insert("30", "AMAR");
    m.insert("12", "AMCOS");
    m.insert("AME", "AME MUSIKVERLAG EDWARD KASSNER GMBH");
    m.insert("162", "AMPAL");
    m.insert("17", "USA");
    m.insert("AUN", "A-music eurl");
    m.insert("169", "ANACIM");
    m.insert("ARM", "Annie Reed Music Ltd");
    m.insert("AVI", "Another Victory Inc");
    m.insert("15", "APA");
    m.insert("7", "APDAYC");
    m.insert("163", "APG-JAPAN");
    m.insert("APL", "Apollo Music");
    m.insert("APO", "Aporai Records Inc");
    m.insert("APO", "Aporetic Music Publishing");
    m.insert("8", "APRA");
    m.insert("164", "APSAV");
    m.insert("ARC", "Arcadia (Msi)");
    m.insert("AEM", "Arcadia Entertainment Muzik Ve Belgesel Pazarlama Ltd Sti");
    m.insert("14", "ARGENTORES");
    m.insert("170", "ARMAUTHOR");
    m.insert("149", "ARS");
    m.insert("9", "ARTISJUS");
    m.insert("10", "ASCAP");
    m.insert("171", "ASDAC");
    m.insert("172", "ASSIM");
    m.insert("AP", "Associated Production Music");
    m.insert("ATP", "At Publishing");
    m.insert("173", "ATIDA");
    m.insert("AMG", "Atlas Music Group");
    m.insert("141", "ATN");
    m.insert("174", "ATN");
    m.insert("AUN", "Audio Network (Holland) B.V.");
    m.insert("AUN", "Audio Network Gmbh");
    m.insert("AUN", "Audio Network Plc");
    m.insert("AUM", "Audiomicro Publishing");
    m.insert("11", "AUSTRO-MECHANA");
    m.insert("175", "AUTORARTE");
    m.insert("231", "AUTVIS");
    m.insert("AVE", "Avenue Editorial Suisse");
    m.insert("13", "AWA");
    m.insert("176", "AWGACS");
    // Add more entries as needed from the full CSV...
    m
});

/// Validates a society transmitter code exists in the lookup table
pub fn is_valid_transmitter_code(code: &str) -> bool {
    SOCIETY_TRANSMITTER_CODES.contains_key(code)
}

/// Gets the society name for a transmitter code
pub fn get_society_name_for_transmitter(transmitter_code: &str) -> Option<&'static str> {
    SOCIETY_TRANSMITTER_CODES.get(transmitter_code).copied()
}

/// Gets all valid transmitter codes
pub fn get_all_transmitter_codes() -> Vec<&'static str> {
    SOCIETY_TRANSMITTER_CODES.keys().copied().collect()
}