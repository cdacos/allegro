//! ISO 4217 currency codes lookup table

use std::collections::HashSet;
use once_cell::sync::Lazy;

/// ISO 4217 currency codes (common subset)
pub static CURRENCY_CODES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("USD"); // US Dollar
    s.insert("EUR"); // Euro
    s.insert("GBP"); // British Pound
    s.insert("JPY"); // Japanese Yen
    s.insert("CHF"); // Swiss Franc
    s.insert("CAD"); // Canadian Dollar
    s.insert("AUD"); // Australian Dollar
    s.insert("SEK"); // Swedish Krona
    s.insert("NOK"); // Norwegian Krone
    s.insert("DKK"); // Danish Krone
    s.insert("PLN"); // Polish Zloty
    s.insert("CZK"); // Czech Koruna
    s.insert("HUF"); // Hungarian Forint
    s.insert("RON"); // Romanian Leu
    s.insert("BGN"); // Bulgarian Lev
    s.insert("HRK"); // Croatian Kuna
    s.insert("RUB"); // Russian Ruble
    s.insert("TRY"); // Turkish Lira
    s.insert("BRL"); // Brazilian Real
    s.insert("MXN"); // Mexican Peso
    s.insert("ARS"); // Argentine Peso
    s.insert("CLP"); // Chilean Peso
    s.insert("CNY"); // Chinese Yuan
    s.insert("HKD"); // Hong Kong Dollar
    s.insert("SGD"); // Singapore Dollar
    s.insert("KRW"); // South Korean Won
    s.insert("INR"); // Indian Rupee
    s.insert("THB"); // Thai Baht
    s.insert("MYR"); // Malaysian Ringgit
    s.insert("IDR"); // Indonesian Rupiah
    s.insert("PHP"); // Philippine Peso
    s.insert("ZAR"); // South African Rand
    s.insert("EGP"); // Egyptian Pound
    s.insert("NGN"); // Nigerian Naira
    s.insert("KES"); // Kenyan Shilling
    s.insert("MAD"); // Moroccan Dirham
    s.insert("TND"); // Tunisian Dinar
    s.insert("ILS"); // Israeli New Shekel
    s.insert("SAR"); // Saudi Riyal
    s.insert("AED"); // UAE Dirham
    s.insert("QAR"); // Qatari Riyal
    s.insert("KWD"); // Kuwaiti Dinar
    s.insert("BHD"); // Bahraini Dinar
    s.insert("OMR"); // Omani Rial
    s.insert("JOD"); // Jordanian Dinar
    s.insert("LBP"); // Lebanese Pound
    s.insert("NZD"); // New Zealand Dollar
    s.insert("FJD"); // Fijian Dollar
    s.insert("TOP"); // Tongan Pa'anga
    s.insert("WST"); // Samoan Tala
    s.insert("VUV"); // Vanuatu Vatu
    s.insert("SBD"); // Solomon Islands Dollar
    s.insert("PGK"); // Papua New Guinea Kina
    s.insert("ISK"); // Icelandic Krona
    s.insert("FOK"); // Faroese Króna
    s.insert("ALL"); // Albanian Lek
    s.insert("MKD"); // North Macedonian Denar
    s.insert("RSD"); // Serbian Dinar
    s.insert("BAM"); // Bosnia and Herzegovina Convertible Mark
    s.insert("MDL"); // Moldovan Leu
    s.insert("UAH"); // Ukrainian Hryvnia
    s.insert("BYN"); // Belarusian Ruble
    s.insert("GEL"); // Georgian Lari
    s.insert("AMD"); // Armenian Dram
    s.insert("AZN"); // Azerbaijani Manat
    s.insert("KZT"); // Kazakhstani Tenge
    s.insert("KGS"); // Kyrgyzstani Som
    s.insert("UZS"); // Uzbekistani So'm
    s.insert("TJS"); // Tajikistani Somoni
    s.insert("TMT"); // Turkmenistani Manat
    s.insert("AFN"); // Afghan Afghani
    s.insert("PKR"); // Pakistani Rupee
    s.insert("LKR"); // Sri Lankan Rupee
    s.insert("BDT"); // Bangladeshi Taka
    s.insert("BTN"); // Bhutanese Ngultrum
    s.insert("MVR"); // Maldivian Rufiyaa
    s.insert("NPR"); // Nepalese Rupee
    s.insert("MMK"); // Myanmar Kyat
    s.insert("LAK"); // Lao Kip
    s.insert("KHR"); // Cambodian Riel
    s.insert("VND"); // Vietnamese Dong
    s.insert("BND"); // Brunei Dollar
    s.insert("TWD"); // New Taiwan Dollar
    s.insert("MNT"); // Mongolian Tugrik
    s.insert("KPW"); // North Korean Won
    // Add more as needed...
    s
});

/// Validates an ISO 4217 currency code
pub fn is_valid_currency_code(code: &str) -> bool {
    CURRENCY_CODES.contains(code)
}

/// Gets all valid currency codes
pub fn get_all_currency_codes() -> Vec<&'static str> {
    CURRENCY_CODES.iter().copied().collect()
}