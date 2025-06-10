//! CIS Language codes lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// CIS Language codes mapping
pub static LANGUAGE_CODES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("AB", "Abkhazian");
    m.insert("AA", "Afar");
    m.insert("AF", "Afrikaans");
    m.insert("SQ", "Albanian");
    m.insert("AM", "Amharic");
    m.insert("AR", "Arabic");
    m.insert("HY", "Armenian");
    m.insert("AS", "Assamese");
    m.insert("AY", "Aymara");
    m.insert("AZ", "Azerbaijani");
    m.insert("BA", "Bashkir");
    m.insert("EU", "Basque");
    m.insert("BN", "Bengali");
    m.insert("DZ", "Bhutani");
    m.insert("BH", "Bihari");
    m.insert("BI", "Bislama");
    m.insert("BR", "Breton");
    m.insert("BG", "Bulgarian");
    m.insert("MY", "Burmese");
    m.insert("BE", "Byelorussian");
    m.insert("KM", "Cambodian");
    m.insert("CA", "Catalan");
    m.insert("ZH", "Chinese");
    m.insert("CO", "Corsican");
    m.insert("HR", "Croatian");
    m.insert("CS", "Czech");
    m.insert("DA", "Danish");
    m.insert("NL", "Dutch");
    m.insert("EN", "English");
    m.insert("EO", "Esperanto");
    m.insert("ET", "Estonian");
    m.insert("FO", "Faeroese");
    m.insert("FA", "Farsi");
    m.insert("FJ", "Fiji");
    m.insert("FI", "Finnish");
    m.insert("FR", "French");
    m.insert("FY", "Frisian");
    m.insert("GL", "Galician");
    m.insert("KA", "Georgian");
    m.insert("DE", "German");
    m.insert("EL", "Greek");
    m.insert("KL", "Greenlandic");
    m.insert("GN", "Guarani");
    m.insert("GU", "Gujarati");
    m.insert("HA", "Hausa");
    m.insert("HW", "Hawaii");
    m.insert("IW", "Hebrew");
    m.insert("HI", "Hindi");
    m.insert("HU", "Hungarian");
    m.insert("IS", "Icelandic");
    m.insert("IN", "Indonesian");
    m.insert("IA", "Interlingua");
    m.insert("IE", "Interlingue");
    m.insert("IK", "Inupiak");
    m.insert("GA", "Irish");
    m.insert("IT", "Italian");
    m.insert("JA", "Japanese");
    m.insert("JW", "Javanese");
    m.insert("KN", "Kannada");
    m.insert("KS", "Kashmiri");
    m.insert("KK", "Kazakh");
    m.insert("RW", "Kinyarwanda");
    m.insert("KY", "Kirghiz");
    m.insert("RN", "Kirundi");
    m.insert("KO", "Korean");
    m.insert("KU", "Kurdish");
    m.insert("LO", "Laothian");
    m.insert("LA", "Latin");
    m.insert("LV", "Latvian");
    m.insert("LN", "Lingala");
    m.insert("LT", "Lithuanian");
    m.insert("MK", "Macedonian");
    m.insert("MG", "Malagasy");
    m.insert("MS", "Malay");
    m.insert("ML", "Malayalam");
    m.insert("MT", "Maltese");
    m.insert("MI", "Maori");
    m.insert("MR", "Marathi");
    m.insert("MO", "Moldavian");
    m.insert("MN", "Mongolian");
    m.insert("NA", "Nauru");
    m.insert("ND", "Ndebele");
    m.insert("NE", "Nepali");
    m.insert("NS", "North Sotho");
    m.insert("NO", "Norwegian");
    m.insert("OC", "Occitan");
    m.insert("OR", "Oriya");
    m.insert("OM", "Oromo");
    m.insert("PM", "Papiamento");
    m.insert("PS", "Pashto");
    m.insert("PL", "Polish");
    m.insert("PT", "Portuguese");
    m.insert("PA", "Punjabi");
    m.insert("QU", "Quechua");
    m.insert("RM", "Rhaeto-Romance");
    m.insert("RO", "Romanian");
    m.insert("RU", "Russian");
    m.insert("SM", "Samoan");
    m.insert("SG", "Sangro");
    m.insert("SA", "Sanskrit");
    m.insert("GD", "Scots Gaelic");
    m.insert("SR", "Serbian");
    m.insert("SH", "Serbo-Croatian");
    m.insert("ST", "Sesotho");
    m.insert("TN", "Setswana");
    m.insert("SN", "Shona");
    m.insert("SD", "Sindhi");
    m.insert("SI", "Singhalese");
    m.insert("SS", "Siswati");
    m.insert("SK", "Slovak");
    m.insert("SL", "Slovenian");
    m.insert("SO", "Somali");
    m.insert("ES", "Spanish");
    m.insert("SU", "Sudanese");
    m.insert("SW", "Swahili");
    m.insert("SV", "Swedish");
    m.insert("TL", "Tagalog");
    m.insert("TG", "Tajik");
    m.insert("TA", "Tamil");
    m.insert("TT", "Tatar");
    m.insert("TE", "Telugu");
    m.insert("TH", "Thai");
    m.insert("BO", "Tibetan");
    m.insert("TI", "Tigrinya");
    m.insert("TO", "Tonga");
    m.insert("TS", "Tsonga");
    m.insert("TR", "Turkish");
    m.insert("TK", "Turkmen");
    m.insert("TW", "Twi");
    m.insert("UK", "Ukranian");
    m.insert("UR", "Urdu");
    m.insert("UZ", "Uzbek");
    m.insert("VE", "Venda");
    m.insert("VI", "Vietnamese");
    m.insert("VO", "Volapuk");
    m.insert("CY", "Welsh");
    m.insert("WO", "Wolof");
    m.insert("XH", "Xhosa");
    m.insert("JI", "Yiddish");
    m.insert("YO", "Yoruba");
    m.insert("ZU", "Zulu");
    m
});

/// Validates a CIS language code
pub fn is_valid_language_code(code: &str) -> bool {
    LANGUAGE_CODES.contains_key(code)
}

/// Gets the description for a CIS language code
pub fn get_language_description(code: &str) -> Option<&'static str> {
    LANGUAGE_CODES.get(code).copied()
}

/// Gets all valid CIS language codes
pub fn get_all_language_codes() -> Vec<&'static str> {
    LANGUAGE_CODES.keys().copied().collect()
}
