//! Society codes lookup table from CWR specification

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Society codes mapping from society name to numeric code
pub static SOCIETY_CODES: Lazy<HashMap<&'static str, u16>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("DP", 0); // Public Domain
    m.insert("ACUM", 1); // Israel
    m.insert("ADDAF", 2); // Brazil
    m.insert("AEPI", 3); // Greece
    m.insert("AGADU", 4); // Uruguay
    m.insert("AKM", 5); // Austria
    m.insert("BUCUDA", 6); // Central Afr Rep
    m.insert("APDAYC", 7); // Peru
    m.insert("APRA", 8); // Australia
    m.insert("ARTISJUS", 9); // Hungary
    m.insert("ASCAP", 10); // USA
    m.insert("AUSTRO-MECHANA", 11); // Austria
    m.insert("AMCOS", 12); // Australia
    m.insert("AWA", 13); // German Dem Rep
    m.insert("ARGENTORES", 14); // Argentina
    m.insert("APA", 15); // Paraguay
    m.insert("BUMDA", 16); // Mali
    m.insert("AMRA", 17); // USA
    m.insert("BGDA", 18); // Guinea
    m.insert("BMDA", 19); // Morocco
    m.insert("SODRAC", 20); // Canada
    m.insert("BMI", 21); // USA
    m.insert("MCSN", 22); // Nigeria
    m.insert("BUMA", 23); // Netherlands
    m.insert("BURIDA", 24); // Ivory Coast
    m.insert("BSDA", 25); // Senegal
    m.insert("CASH", 26); // Hong Kong
    m.insert("CAPAC", 27); // Canada
    m.insert("LITA", 28); // Slovakia
    m.insert("SCD", 29); // Chile
    m.insert("AMAR", 30); // Brazil
    m.insert("DILIA", 31); // Czech Republic
    m.insert("FILSCAP", 32); // Philippines
    m.insert("OMDA", 33); // Madagascar
    m.insert("HARRY FOX AGENCY", 34); // USA
    m.insert("GEMA", 35); // German Fed Rep
    m.insert("IPRS", 36); // India
    m.insert("BUBEDRA", 37); // Benin
    m.insert("JASRAC", 38); // Japan
    m.insert("MUSICAUTOR", 39); // Bulgaria
    m.insert("KODA", 40); // Denmark
    m.insert("LITERAR-MECHANA", 41); // Austria
    m.insert("LVG", 42); // Austria
    m.insert("MCSK", 43); // Kenya
    m.insert("MCPS", 44); // United Kingdom
    m.insert("BBDA", 45); // Burkina Faso
    m.insert("MRS", 46); // United Kingdom
    m.insert("BCDA", 47); // Congo
    m.insert("NCB", 48); // Scandinavia
    m.insert("ONDA", 49); // Algeria
    m.insert("OSA", 50); // Czech Republic
    m.insert("PROLITTERIS", 51); // Switzerland
    m.insert("PRS", 52); // United Kingdom
    m.insert("PROCAN", 53); // Canada
    m.insert("ALCS", 54); // United Kingdom
    m.insert("SABAM", 55); // Belgium
    m.insert("SACD", 56); // France
    m.insert("SACERAU", 57); // Egypt
    m.insert("SACEM", 58); // France
    m.insert("SACM", 59); // Mexico
    m.insert("SACVEN", 60); // Venezuela
    m.insert("SADAIC", 61); // Argentina
    m.insert("SADEMBRA", 62); // Brazil
    m.insert("SAMRO", 63); // South Africa
    m.insert("SOKOJ", 64); // Yugoslavia
    m.insert("SAYCE", 65); // Ecuador
    m.insert("SBACEM", 66); // Brazil
    m.insert("SBAT", 67); // Brazil
    m.insert("SDRM", 68); // France (deprecated)
    m.insert("SPA", 69); // Portugal
    m.insert("SOGEM", 70); // Mexico
    m.insert("SESAC", 71); // USA
    m.insert("SGAE", 72); // Spain
    m.insert("SCAM", 73); // France
    m.insert("SIAE", 74); // Italy
    m.insert("SUISSIMAGE", 75); // Switzerland
    m.insert("ACEMLA", 76); // Puerto Rico
    m.insert("STEF", 77); // Iceland
    m.insert("STEMRA", 78); // Netherlands
    m.insert("STIM", 79); // Sweden
    m.insert("SUISA", 80); // Switzerland
    m.insert("SARRAL", 81); // South Africa
    m.insert("OTPDA", 82); // Tunisia
    m.insert("SONECA", 83); // Zaire
    m.insert("SAYCO", 84); // Columbia
    m.insert("SOZA", 85); // Slovakia
    m.insert("SICAM", 86); // Brazil
    m.insert("SPACEMF", 87); // French Polynesia
    m.insert("CMRRA", 88); // Canada
    m.insert("TEOSTO", 89); // Finland
    m.insert("TONO", 90); // Norway
    m.insert("SSA", 91); // Switzerland
    m.insert("SOCINADA", 92); // Cameroon Rep
    m.insert("UBC", 93); // Brazil
    m.insert("RAO", 94); // Russia
    m.insert("VG WORT", 95); // German
    m.insert("COTT", 96); // Trinidad & Tobago
    m.insert("ZAIKS", 97); // Poland
    m.insert("ZIMRA", 98); // Zimbabwe
    m.insert("NS", 99); // (blank/not specified)
    m.insert("SOCAN", 101); // Canada
    m.insert("NASCAM", 102); // Namibia
    m.insert("ACDAM", 103); // Cuba
    m.insert("MACP", 104); // Malaysia
    m.insert("MASA", 105); // Mauritius
    m.insert("COMPASS", 106); // Singapore
    m.insert("ACAM", 107); // Costa Rica
    m.insert("CHA", 108); // Taiwan
    m.insert("KCI", 109); // Indonesia
    m.insert("LATGA-A", 110); // Lithuania
    m.insert("HDS-ZAMP", 111); // Croatia
    m.insert("SAZAS", 112); // Slovenia
    m.insert("LAA", 113); // Latvia
    m.insert("AGAYC", 114); // Guatemala
    m.insert("UCMR-ADA", 115); // Romania
    m.insert("EAU", 116); // Estonia
    m.insert("MESAM", 117); // Turkey
    m.insert("KOMCA", 118); // South Korea
    m.insert("MCSC", 119); // China
    m.insert("LIRA", 120); // Netherlands
    m.insert("VDFS", 121); // Austria
    m.insert("AKKA/LAA", 122); // Latvia
    m.insert("COSGA", 123); // Ghana
    m.insert("COSOMA", 124); // Malawi
    m.insert("BNDA", 125); // Niger
    m.insert("MCT", 126); // Thailand
    m.insert("ALBAUTOR", 127); // Albania
    m.insert("IMRO", 128); // Ireland
    m.insert("SOBODAYCOM", 129); // Bolivia
    m.insert("BUTODRA", 130); // Togo
    m.insert("SADA", 131); // Greece
    m.insert("BILD-KUNST", 132); // German Federal Republic
    m.insert("ZAMCOPS", 133); // Zambia
    m.insert("SLPRS", 134); // Sri Lanka
    m.insert("SADH", 135); // Greece
    m.insert("ZAMP", 136); // Macedonia
    m.insert("SOFAM", 137); // Belgium
    m.insert("KOPIOSTO", 138); // Finland
    m.insert("COPY-DAN BILLEDKUNST", 139); // Denmark
    m.insert("UACRR", 140); // Ukraine
    m.insert("ATN", 141); // Chile (formerly known as GESATCH)
    m.insert("DALRO", 142); // South Africa
    m.insert("TEATERAUTOR", 143); // Bulgaria
    m.insert("HAA", 144); // Croatia
    m.insert("DPRS", 145); // United Kingdom
    m.insert("SPAC", 146); // Panama
    m.insert("FILMAUTOR", 147); // Bulgaria
    m.insert("ADAGP", 148); // France
    m.insert("ARS", 149); // USA
    m.insert("BEELDRECHT", 150); // Netherlands
    m.insert("BONO", 151); // Norway
    m.insert("BUS", 152); // Sweden
    m.insert("DACS", 153); // United Kingdom
    m.insert("HUNGART", 154); // Hungary
    m.insert("SOMAAP", 155); // Mexico
    m.insert("VAGA", 156); // USA
    m.insert("VBK", 157); // Austria
    m.insert("VEGAP", 158); // Spain
    m.insert("VISCOPY", 159); // Australia
    m.insert("RUPIS", 160); // Belarus
    m.insert("MUST", 161); // Taiwan (Province Of China)
    m.insert("AMPAL", 162); // Australia
    m.insert("APG-JAPAN", 163); // Japan
    m.insert("APSAV", 164); // Peru
    m.insert("ATN", 165); // Chile
    m.insert("AUTORARTE", 166); // Venezuela
    m.insert("BURAFO", 167); // Netherlands
    m.insert("CAL", 168); // Australia
    m.insert("COSCAP", 169); // Barbados
    m.insert("CPSN", 170); // Nepal
    m.insert("CREAIMAGEN", 171); // Chile
    m.insert("DGA", 172); // United States
    m.insert("DIRECTORES", 173); // Mexico
    m.insert("FLIM JUS", 174); // Hungary
    m.insert("COPYRO", 175); // Romania
    m.insert("JACAP", 176); // Jamaica
    m.insert("KAZAK", 177); // Kazakhstan
    m.insert("KOSA", 178); // Korea Republic Of
    m.insert("KUVASTO", 179); // Finland
    m.insert("MUSIKEDITION", 180); // Austria
    m.insert("NMPA", 181); // United States
    m.insert("PAPPRI", 182); // Indonesia
    m.insert("SACK", 183); // Korea Republic Of
    m.insert("SARTEC", 184); // Canada
    m.insert("SESAM", 185); // France
    m.insert("SGDL", 186); // France
    m.insert("SNAC", 187); // France
    m.insert("SOCIETE DE L'IMAGE", 188); // France
    m.insert("SOCINPRO", 189); // Brazil
    m.insert("SOPE", 190); // Greece
    m.insert("SPACQ", 191); // Canada
    m.insert("SFF", 192); // Sweden
    m.insert("THE SOCIETY OF AUTHORS", 193); // United Kingdom
    m.insert("UFFICIO LEGALE", 194); // Holy See (Vatican City State)
    m.insert("VEVAM", 195); // Netherlands
    m.insert("WGA", 196); // United States
    m.insert("WGJ", 197); // Japan
    m.insert("ZAMP", 198); // Slovenia
    m.insert("ZAPA", 199); // Poland
    m.insert("MSG", 200); // Turkey
    m.insert("ABRAMUS", 201); // Brazil
    m.insert("ASDAC", 202); // Moldova - Republic Of
    m.insert("AWGACS", 203); // Australia
    m.insert("SAS", 204); // Georgia
    m.insert("SODART", 205); // Canada
    m.insert("SUOMEN KIRJAILIJALIITTO", 206); // Finland
    m.insert("THE AUTHOR'S REGISTRY INC.", 207); // United States
    m.insert("SGA", 208); // Guinea-Bissau
    m.insert("ARMAUTHOR", 209); // Armenia
    m.insert("ACCESS", 210); // Canada (Formerly CANCOPY)
    // Note: 211 is "No Longer In Use"
    m.insert("CSCS", 212); // Canada
    m.insert("DRCC", 213); // Canada
    m.insert("HMS", 214); // Saint Lucia
    m.insert("KYRGYZPATENT", 215); // Kyrgyzstan
    m.insert("SQN", 216); // Bosnia And Herzegovina
    m.insert("ABRAC", 217); // Brazil
    m.insert("ANACIM", 218); // Brazil
    m.insert("ASSIM", 219); // Brazil
    m.insert("ATIDA", 220); // Brazil
    m.insert("SABEM", 221); // Brazil
    m.insert("FONOPERU", 222); // Peru
    m.insert("COSOTA", 223); // Tanzania, United Republic Of
    m.insert("SOMAS", 224); // Mozambique
    m.insert("SAIF", 225); // France
    m.insert("AACIMH", 226); // Honduras
    m.insert("SGACEDOM", 227); // Dominican Republic
    m.insert("ROMS", 228); // Russian Federation
    m.insert("ICG", 229); // United States
    m.insert("ADAVIS", 230); // Cuba
    m.insert("AUTVIS", 231); // Brazil
    m.insert("GESTOR", 232); // Czech Republic
    m.insert("SACEMLUXEMBOURG", 233); // Luxembourg
    m.insert("UCOSO", 234); // Uganda
    m.insert("SACENC", 235); // France
    m.insert("WID CENTRE", 300); // United States
    m.insert("GESAC", 301); // Belgium
    m.insert("LATINAUTOR", 302); // Uruguay
    m.insert("NORD-DOC", 303); // Sweden
    m.insert("SONGCODE", 304); // United States
    m.insert("IMJV", 305); // Netherlands
    m.insert("CCL", 306); // Trinidad And Tobago
    m.insert("MISASIA", 307); // Singapore
    m.insert("ECAD", 308); // Brazil
    m.insert("LATINNET", 309); // Spain
    m.insert("DIVA", 310); // Hong Kong
    m
});

/// Validates a society code exists in the lookup table
pub fn is_valid_society_code(code: &str) -> bool {
    SOCIETY_CODES.contains_key(code)
}

/// Gets the numeric code for a society name
pub fn get_society_numeric_code(society_name: &str) -> Option<u16> {
    SOCIETY_CODES.get(society_name).copied()
}

/// Gets all valid society codes
pub fn get_all_society_codes() -> Vec<&'static str> {
    SOCIETY_CODES.keys().copied().collect()
}