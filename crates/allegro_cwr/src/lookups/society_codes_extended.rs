//! Society codes lookup table

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Society codes mapping from numeric code to society name
pub static SOCIETY_CODES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("0", "DP"); // Public Domain
    m.insert("1", "ACUM"); // Israel
    m.insert("2", "ADDAF"); // Brazil
    m.insert("3", "AEPI"); // Greece
    m.insert("4", "AGADU"); // Uruguay
    m.insert("5", "AKM"); // Austria
    m.insert("6", "BUCUDA"); // Central Afr Rep
    m.insert("7", "APDAYC"); // Peru
    m.insert("8", "APRA"); // Australia
    m.insert("9", "ARTISJUS"); // Hungary
    m.insert("10", "ASCAP"); // USA
    m.insert("11", "AUSTRO-MECHANA"); // Austria
    m.insert("12", "AMCOS"); // Australia
    m.insert("13", "AWA"); // German Dem Rep
    m.insert("14", "ARGENTORES"); // Argentina
    m.insert("15", "APA"); // Paraguay
    m.insert("16", "BUMDA"); // Mali
    m.insert("17", "AMRA"); // USA
    m.insert("18", "BGDA"); // Guinea
    m.insert("19", "BMDA"); // Morocco
    m.insert("20", "SODRAC"); // Canada
    m.insert("21", "BMI"); // USA
    m.insert("22", "MCSN"); // Nigeria
    m.insert("23", "BUMA"); // Netherlands
    m.insert("24", "BURIDA"); // Ivory Coast
    m.insert("25", "BSDA"); // Senegal
    m.insert("26", "CASH"); // Hong Kong
    m.insert("27", "CAPAC"); // Canada
    m.insert("28", "LITA"); // Slovakia
    m.insert("29", "CELAS"); // Mexico
    m.insert("30", "CMRRA"); // Canada
    m.insert("31", "COMPASS"); // Singapore
    m.insert("32", "COSOMA"); // Botswana
    m.insert("33", "DALRO"); // South Africa
    m.insert("34", "EAU"); // Kenya
    m.insert("35", "ECAD"); // Brazil
    m.insert("36", "FILSCAP"); // Philippines
    m.insert("37", "FODEM"); // Congo
    m.insert("38", "GEMA"); // Germany
    m.insert("39", "GESAC"); // Venezuela
    m.insert("40", "HDS"); // Croatia
    m.insert("41", "HFA"); // USA
    m.insert("42", "IMRO"); // Ireland
    m.insert("43", "IPRS"); // India
    m.insert("44", "JASRAC"); // Japan
    m.insert("45", "KODA"); // Denmark
    m.insert("46", "KOMCA"); // South Korea
    m.insert("47", "LATGA-A"); // Latvia
    m.insert("48", "MACP"); // Malaysia
    m.insert("49", "MCPS"); // United Kingdom
    m.insert("50", "MECL"); // Canada
    m.insert("51", "MESAM"); // Turkey
    m.insert("52", "MUSICAUTOR"); // Bulgaria
    m.insert("53", "NCB"); // Denmark
    m.insert("54", "ONDA"); // Algeria
    m.insert("55", "OSA"); // Czech Republic
    m.insert("56", "PRS"); // United Kingdom
    m.insert("57", "RAO"); // Russia
    m.insert("58", "RTVS"); // Slovenia
    m.insert("59", "SABAM"); // Belgium
    m.insert("60", "SACEM"); // France
    m.insert("61", "SACM"); // Mexico
    m.insert("62", "SACVEN"); // Venezuela
    m.insert("63", "SADAIC"); // Argentina
    m.insert("64", "SAMRO"); // South Africa
    m.insert("65", "SAYCO"); // Colombia
    m.insert("66", "SESAC"); // USA
    m.insert("67", "SGAE"); // Spain
    m.insert("68", "SIA"); // Algeria
    m.insert("69", "SIAE"); // Italy
    m.insert("70", "SOBODAYCOM"); // Bolivia
    m.insert("71", "SOCAN"); // Canada
    m.insert("72", "SOCINPRO"); // Brazil
    m.insert("73", "SODRE"); // Uruguay
    m.insert("74", "SOGEM"); // Mexico
    m.insert("75", "SOZA"); // Slovakia
    m.insert("76", "SPA"); // Portugal
    m.insert("77", "STEF"); // Iceland
    m.insert("78", "STEMRA"); // Netherlands
    m.insert("79", "SUISA"); // Switzerland
    m.insert("80", "TEOSTO"); // Finland
    m.insert("81", "TONO"); // Norway
    m.insert("82", "UBC"); // Brazil
    m.insert("83", "UCMR-ADA"); // Romania
    m.insert("84", "VAAP"); // Estonia
    m.insert("85", "VCPMC"); // Vietnam
    m.insert("86", "ZAIKS"); // Poland
    m.insert("87", "ZAiKS"); // Poland
    m.insert("88", "ABRAMUS"); // Brazil
    m.insert("89", "ACAM"); // Chile
    m.insert("90", "ACDAN"); // Dominican Republic
    m.insert("91", "ACTRAV"); // Chile
    m.insert("92", "AHANONM"); // Lebanon
    m.insert("93", "ALBAUTOR"); // Albania
    m.insert("94", "AMUS"); // Uruguay
    m.insert("95", "APDIF"); // France
    m.insert("96", "ARDEM"); // Morocco
    m.insert("97", "ARMAUTEUR"); // Armenia
    m.insert("98", "ARTEX"); // Cuba
    m.insert("99", "AUTODIA"); // Greece
    m.insert("100", "BARCLAY"); // France
    m.insert("101", "BIEM"); // International
    m.insert("102", "BOURDA"); // Burkina Faso
    m.insert("103", "CAPASSO"); // South Africa
    m.insert("104", "CIAM"); // Chile
    m.insert("105", "COPYDAN"); // Denmark
    m.insert("106", "COSBIEU"); // Benin
    m.insert("107", "COSGA"); // Gabon
    m.insert("108", "COTT"); // Trinidad and Tobago
    m.insert("109", "CREDIAUTOR"); // Brazil
    m.insert("110", "DACS"); // United Kingdom
    m.insert("111", "DDDA"); // Chad
    m.insert("112", "DGA"); // USA
    m.insert("113", "DMCS"); // Mauritius
    m.insert("114", "DRAMST"); // Poland
    m.insert("115", "DROIT"); // Brazil
    m.insert("116", "EMI"); // International
    m.insert("117", "FESPA"); // Paraguay
    m.insert("118", "GCA"); // Ghana
    m.insert("119", "GESTI"); // Spain
    m.insert("120", "GVL"); // Germany
    m.insert("121", "HZSU"); // Croatia
    m.insert("122", "IPA"); // Jamaica
    m.insert("123", "ITAS"); // Slovenia
    m.insert("124", "JACAP"); // Jamaica
    m.insert("125", "JCAP"); // Jamaica
    m.insert("126", "KAMRA"); // Kazakhstan
    m.insert("127", "LATGA"); // Latvia
    m.insert("128", "LEVY"); // Canada
    m.insert("129", "LIRA"); // Lebanon
    m.insert("130", "LSG"); // Germany
    m.insert("131", "MASA"); // Malaysia
    m.insert("132", "MUSICMARK"); // South Africa
    m.insert("133", "NORM"); // Norway
    m.insert("134", "NORWACO"); // Norway
    m.insert("135", "OESR"); // Austria
    m.insert("136", "OHIM"); // Spain
    m.insert("137", "OKANO"); // Slovenia
    m.insert("138", "OMDA"); // Cameroon
    m.insert("139", "PACRA"); // Panama
    m.insert("140", "PAMRA"); // South Africa
    m.insert("141", "PAYOLA"); // Poland
    m.insert("142", "PHONORIGHTS"); // United Kingdom
    m.insert("143", "PPL"); // United Kingdom
    m.insert("144", "PROLITTERIS"); // Switzerland
    m.insert("145", "PROPHON"); // Switzerland
    m.insert("146", "RAPI"); // South Africa
    m.insert("147", "REPRO"); // South Africa
    m.insert("148", "RICORDI"); // Italy
    m.insert("149", "RTVS"); // Slovenia
    m.insert("150", "SACD"); // France
    m.insert("151", "SACDA"); // Chile
    m.insert("152", "SACEM"); // France
    m.insert("153", "SACEM/SDRM"); // France - use SACEM instead
    m.insert("154", "SADAIC"); // Argentina
    m.insert("155", "SAYCE"); // Ecuador
    m.insert("156", "SCD"); // Chile
    m.insert("157", "SCPP"); // France
    m.insert("158", "SDRM"); // France - deprecated, use SACEM
    m.insert("159", "SIMAEF"); // Chile
    m.insert("160", "SNAC"); // Senegal
    m.insert("161", "SOBODAYCOM"); // Bolivia
    m.insert("162", "SOMACOS"); // Morocco
    m.insert("163", "SORAF"); // Chad
    m.insert("164", "SOZA"); // Czech Republic
    m.insert("165", "SPEDIDAM"); // France
    m.insert("166", "SPI"); // Portugal
    m.insert("167", "SPRELUDIA"); // Slovenia
    m.insert("168", "STIM"); // Sweden
    m.insert("169", "SUISA"); // Switzerland
    m.insert("170", "SWISSPERFORM"); // Switzerland
    m.insert("171", "TEOSTO"); // Finland
    m.insert("172", "TEVA"); // Lithuania
    m.insert("173", "TONO"); // Norway
    m.insert("174", "UBC"); // Brazil
    m.insert("175", "UPFI"); // France
    m.insert("176", "VG WORT"); // Germany
    m.insert("177", "WAMI"); // Nigeria
    m.insert("178", "WIPO"); // International
    m.insert("179", "ZAIKS"); // Poland
    m.insert("180", "ZAiKS"); // Poland
    m.insert("181", "ZAMP"); // Croatia
    m.insert("182", "ZAPA"); // South Africa
    m.insert("183", "ZIMURA"); // Zimbabwe
    m.insert("184", "AEI"); // Spain
    m.insert("185", "AIE"); // Spain
    m.insert("186", "AKKA-LAA"); // Latvia
    m.insert("187", "ALCS"); // United Kingdom
    m.insert("188", "ALIA"); // Lithuania
    m.insert("189", "AMACOS"); // Australia
    m.insert("190", "AMRA"); // USA
    m.insert("191", "APA"); // Austria
    m.insert("192", "APDAYC"); // Peru
    m.insert("193", "APRA"); // Australia
    m.insert("194", "ARTISJUS"); // Hungary
    m.insert("195", "ASCAP"); // USA
    m.insert("196", "AUSTRO-MECHANA"); // Austria
    m.insert("197", "AUTODIA"); // Greece
    m.insert("198", "BIEM"); // International
    m.insert("199", "BMI"); // USA
    m.insert("200", "BUMA"); // Netherlands
    m.insert("201", "CASH"); // Hong Kong
    m.insert("202", "CISAC"); // International
    m.insert("203", "COMPASS"); // Singapore
    m.insert("204", "COPYDAN"); // Denmark
    m.insert("205", "GEMA"); // Germany
    m.insert("206", "HFA"); // USA
    m.insert("207", "IMRO"); // Ireland
    m.insert("208", "JASRAC"); // Japan
    m.insert("209", "KODA"); // Denmark
    m.insert("210", "KOMCA"); // South Korea
    m.insert("211", ""); // No Longer In Use
    m.insert("212", "MCPS"); // United Kingdom
    m.insert("213", "MESAM"); // Turkey
    m.insert("214", "OSA"); // Czech Republic
    m.insert("215", "PRS"); // United Kingdom
    m.insert("216", "SABAM"); // Belgium
    m.insert("217", "SACEM"); // France
    m.insert("218", "SAMRO"); // South Africa
    m.insert("219", "SESAC"); // USA
    m.insert("220", "SGAE"); // Spain
    m.insert("221", "SIAE"); // Italy
    m.insert("222", "SOCAN"); // Canada
    m.insert("223", "SPA"); // Portugal
    m.insert("224", "STEMRA"); // Netherlands
    m.insert("225", "STIM"); // Sweden
    m.insert("226", "SUISA"); // Switzerland
    m.insert("227", "TEOSTO"); // Finland
    m.insert("228", "TONO"); // Norway
    m.insert("229", "ZAIKS"); // Poland
    m.insert("230", "ABRAMUS"); // Brazil
    m.insert("231", "ACINPRO"); // Brazil
    m.insert("232", "ADDAF"); // Brazil
    m.insert("233", "APDIF"); // France
    m.insert("234", "ARTISTI"); // Finland
    m.insert("235", "CMRRA"); // Canada
    m.insert("236", "COPYDAN"); // Denmark
    m.insert("237", "ECAD"); // Brazil
    m.insert("238", "GVL"); // Germany
    m.insert("239", "LIRA"); // Lebanon
    m.insert("240", "LSG"); // Germany
    m.insert("241", "MECL"); // Canada
    m.insert("242", "NORM"); // Norway
    m.insert("243", "NORWACO"); // Norway
    m.insert("244", "PHONORIGHTS"); // United Kingdom
    m.insert("245", "PPL"); // United Kingdom
    m.insert("246", "PROLITTERIS"); // Switzerland
    m.insert("247", "PROPHON"); // Switzerland
    m.insert("248", "SACD"); // France
    m.insert("249", "SCPP"); // France
    m.insert("250", "SDRM"); // France
    m.insert("251", "SOCINPRO"); // Brazil
    m.insert("252", "SODRAC"); // Canada
    m.insert("253", "SPEDIDAM"); // France
    m.insert("254", "SWISSPERFORM"); // Switzerland
    m.insert("255", "UBC"); // Brazil
    m.insert("256", "UPFI"); // France
    m.insert("257", "VG WORT"); // Germany
    m.insert("301", "AACIMH"); // Hungary
    m.insert("302", "ADAGP"); // France
    m.insert("303", "AKM"); // Austria
    m.insert("304", "COPYDAN"); // Denmark
    m.insert("305", "DACS"); // United Kingdom
    m.insert("306", "HUNGART"); // Hungary
    m.insert("307", "LIRA"); // Lebanon
    m.insert("308", "SABAM"); // Belgium
    m.insert("309", "SACD"); // France
    m.insert("310", "VG WORT"); // Germany
    m
});

/// Validates a society code
#[must_use]
pub fn is_valid_society_code(code: &str) -> bool {
    SOCIETY_CODES.contains_key(code)
}

/// Gets the society name for a society code
#[must_use]
pub fn get_society_name(code: &str) -> Option<&'static str> {
    SOCIETY_CODES.get(code).copied()
}

/// Gets all valid society codes
#[must_use]
pub fn get_all_society_codes() -> Vec<&'static str> {
    SOCIETY_CODES.keys().copied().collect()
}
