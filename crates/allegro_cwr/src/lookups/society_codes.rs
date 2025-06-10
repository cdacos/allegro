//! Society codes lookup table from CWR specification

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Society codes mapping from society name to numeric code
pub static SOCIETY_CODES: Lazy<HashMap<&'static str, u16>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ACUM", 1); // Israel
    m.insert("ADDAF", 2); // Brazil
    m.insert("AEPI", 3); // Greece
    m.insert("AGADU", 4); // Uruguay
    m.insert("AKM", 5); // Austria
    m.insert("BUCADA", 6); // Central African Republic
    m.insert("APDAYC", 7); // Peru
    m.insert("APRA", 8); // Australia
    m.insert("ARTISJUS", 9); // Hungary
    m.insert("ASCAP", 10); // United States
    m.insert("AUSTRO-MECHANA (AUME)", 11); // Austria
    m.insert("AMCOS", 12); // Australia
    m.insert("AWA", 13); // Germany
    m.insert("ARGENTORES", 14); // Argentina
    m.insert("APA", 15); // Paraguay
    m.insert("BUMDA", 16); // Mali
    m.insert("AMRA", 17); // United States
    m.insert("BGDA", 18); // Guinea
    m.insert("BMDAV", 19); // Morocco
    m.insert("SOCAN RR", 20); // Canada
    m.insert("BMI", 21); // United States
    m.insert("MCSN", 22); // Nigeria
    m.insert("BUMA", 23); // Netherlands
    m.insert("BURIDA", 24); // Cote D'Ivoire
    m.insert("SODAV", 25); // Senegal
    m.insert("CASH", 26); // Hong Kong
    m.insert("CAPAC", 27); // Canada
    m.insert("LITA", 28); // Slovakia
    m.insert("SCD", 29); // Chile
    m.insert("AMAR", 30); // Brazil
    m.insert("DILIA", 31); // Czech Republic
    m.insert("FILSCAP", 32); // Philippines
    m.insert("OMDA", 33); // Madagascar
    m.insert("HFA", 34); // United States
    m.insert("GEMA", 35); // Germany
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
    m.insert("NCB", 48); // Denmark
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
    m.insert("SOKOJ", 64); // Serbia And Montenegro
    m.insert("SAYCE", 65); // Ecuador
    m.insert("SBACEM", 66); // Brazil
    m.insert("SBAT", 67); // Brazil
    m.insert("SDRM", 68); // France
    m.insert("SPA", 69); // Portugal
    m.insert("SOGEM", 70); // Mexico
    m.insert("SESAC Inc.", 71); // United States
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
    m.insert("OTDAV", 82); // Tunisia
    m.insert("SONECA", 83); // Congo, The Democratic Republic Of The
    m.insert("SAYCO", 84); // Colombia
    m.insert("SOZA", 85); // Slovakia
    m.insert("SICAM", 86); // Brazil
    m.insert("SPACEM", 87); // France (Tahiti)
    m.insert("CMRRA", 88); // Canada
    m.insert("TEOSTO", 89); // Finland
    m.insert("TONO", 90); // Norway
    m.insert("SSA", 91); // Switzerland
    m.insert("SOCINADA", 92); // Cameroon
    m.insert("UBC", 93); // Brazil
    m.insert("RAO", 94); // Russian Federation
    m.insert("VG WORT", 95); // Germany
    m.insert("COTT", 96); // Trinidad And Tobago
    m.insert("ZAIKS", 97); // Poland
    m.insert("ZIMURA", 98); // Zimbabwe
    m.insert("SOCAN", 101); // Canada
    m.insert("NASCAM", 102); // Namibia
    m.insert("ACDAM", 103); // Cuba
    m.insert("MACP", 104); // Malaysia
    m.insert("MASA", 105); // Mauritius
    m.insert("COMPASS", 106); // Singapore
    m.insert("ACAM", 107); // Costa Rica
    m.insert("CHA", 108); // Taiwan, Chinese Taipei
    m.insert("KCI", 109); // Indonesia
    m.insert("LATGA", 110); // Lithuania
    m.insert("HDS-ZAMP", 111); // Croatia
    m.insert("SAZAS", 112); // Slovenia
    m.insert("LAA", 113); // Latvia
    m.insert("AGAYC", 114); // Guatemala
    m.insert("UCMR-ADA", 115); // Romania
    m.insert("EAU", 116); // Estonia
    m.insert("MESAM", 117); // Turkey
    m.insert("KOMCA", 118); // Korea, Republic Of
    m.insert("MCSC", 119); // China
    m.insert("LIRA", 120); // Netherlands
    m.insert("VDFS", 121); // Austria
    m.insert("AKKA-LAA", 122); // Latvia
    m.insert("COSGA", 123); // Ghana
    m.insert("COSOMA", 124); // Malawi
    m.insert("BNDA", 125); // Niger
    m.insert("MCT", 126); // Thailand
    m.insert("ALBAUTOR", 127); // Albania
    m.insert("IMRO", 128); // Ireland
    m.insert("SOBODAYCOM", 129); // Bolivia
    m.insert("BUTODRA", 130); // Togo
    m.insert("ATHINA-SADA", 131); // Greece
    m.insert("VG BILD-KUNST", 132); // Germany
    m.insert("ZAMCOPS", 133); // Zambia
    m.insert("SLPRS", 134); // Sri Lanka
    m.insert("SADH", 135); // Greece
    m.insert("ZAMP - Macédoine", 136); // Macedonia, The Former Yugoslav Republic Of
    m.insert("SOFAM", 137); // Belgium
    m.insert("KOPIOSTO", 138); // Finland
    m.insert("VISDA", 139); // Denmark
    m.insert("NGO-UACRR", 140); // Ukraine
    m.insert("ATN", 141); // Chile
    m.insert("DALRO", 142); // South Africa
    m.insert("TEATERAUTOR", 143); // Bulgaria
    m.insert("HAA", 144); // Croatia
    m.insert("DIRECTORS UK", 145); // United Kingdom
    m.insert("SPAC", 146); // Panama
    m.insert("FILMAUTOR", 147); // Bulgaria
    m.insert("ADAGP", 148); // France
    m.insert("ARS", 149); // United States
    m.insert("BEELDRECHT", 150); // Netherlands
    m.insert("BONO", 151); // Norway
    m.insert("Bildupphovsrätt (Visual Copyright Society)", 152); // Sweden
    m.insert("DACS", 153); // United Kingdom
    m.insert("HUNGART", 154); // Hungary
    m.insert("SOMAAP", 155); // Mexico
    m.insert("VAGA", 156); // United States
    m.insert("BILDRECHT GmbH", 157); // Austria
    m.insert("VEGAP", 158); // Spain
    m.insert("VISCOPY", 159); // Australia
    m.insert("NCIP", 160); // Belarus
    m.insert("MÜST", 161); // Taiwan, Chinese Taipei
    m.insert("AMPAL", 162); // Australia
    m.insert("APG-Japan", 163); // Japan
    m.insert("APSAV", 164); // Peru
    m.insert("AUTORARTE", 166); // Venezuela
    m.insert("BURAFO", 167); // Netherlands
    m.insert("Copyright Agency", 168); // Australia
    m.insert("COSCAP", 169); // Barbados
    m.insert("CPSN", 170); // Nepal
    m.insert("CREAIMAGEN", 171); // Chile
    m.insert("DGA", 172); // United States
    m.insert("DIRECTORES", 173); // Mexico
    m.insert("FILMJUS", 174); // Hungary
    m.insert("CopyRo", 175); // Romania
    m.insert("JACAP", 176); // Jamaica
    m.insert("KazAK", 177); // Kazakstan
    m.insert("KOSA", 178); // Korea, Republic Of
    m.insert("KUVASTO", 179); // Finland
    m.insert("MUSIKEDITION", 180); // Austria
    m.insert("NMPA", 181); // United States
    m.insert("PAPPRI", 182); // Indonesia
    m.insert("SACK", 183); // Korea, Republic Of
    m.insert("SARTEC", 184); // Canada
    m.insert("SESAM", 185); // France
    m.insert("SGDL", 186); // France
    m.insert("SNAC", 187); // France
    m.insert("Société de l'Image", 188); // France
    m.insert("SOCINPRO", 189); // Brazil
    m.insert("SOPE", 190); // Greece
    m.insert("SPACQ-AE", 191); // Canada
    m.insert("SFF", 192); // Sweden
    m.insert("The Society of Authors (SOA)", 193); // United Kingdom
    m.insert("UFFICIO GIURIDICO", 194); // Holy See (Vatican City State)
    m.insert("VEVAM", 195); // Netherlands
    m.insert("WGAW", 196); // United States
    m.insert("WGJ", 197); // Japan
    m.insert("ZAMP Association of Slovenia", 198); // Slovenia
    m.insert("SFP-ZAPA", 199); // Poland
    m.insert("MSG", 200); // Turkey
    m.insert("ABRAMUS", 201); // Brazil
    m.insert("AsDAC", 202); // Moldova, Republic Of
    m.insert("AWGACS", 203); // Australia
    m.insert("GCA", 204); // Georgia
    m.insert("SODART", 205); // Canada
    m.insert("UFW", 206); // Finland
    m.insert("The Author's Registry Inc.", 207); // United States
    m.insert("SGA", 208); // Guinea-Bissau
    m.insert("ARMAUTHOR NGO", 209); // Armenia
    m.insert("ACCESS COPYRIGHT", 210); // Canada
    m.insert("CSCS", 212); // Canada
    m.insert("DRCC", 213); // Canada
    m.insert("ECCO", 214); // Saint Lucia
    m.insert("Kyrgyzpatent", 215); // Kyrgyzstan
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
    m.insert("UPRS", 234); // Uganda
    m.insert("SACENC", 235); // France
    m.insert("ARTEGESTION", 236); // Ecuador
    m.insert("TALI", 237); // Israel
    m.insert("BSCAP", 238); // Belize
    m.insert("CMC", 239); // Cameroon
    m.insert("DAMA", 240); // Spain
    m.insert("NICAUTOR", 241); // Nicaragua
    m.insert("SACIM", 242); // El Salvador
    m.insert("SADIA", 243); // Angola
    m.insert("SASUR", 244); // Suriname
    m.insert("SETEM", 245); // Turkey
    m.insert("VCPMC", 246); // Viet Nam
    m.insert("IVARO", 247); // Ireland
    m.insert("DAC", 248); // Argentina
    m.insert("PAM CG", 249); // Montenegro
    m.insert("AEI-GUATEMALA", 250); // Guatemala
    m.insert("ASDACS", 251); // Australia
    m.insert("COLCCMA", 252); // Taiwan, Chinese Taipei
    m.insert("AAS", 253); // Azerbaijan
    m.insert("SOCILADRA", 254); // Cameroon
    m.insert("SODOMAPLA", 255); // Dominican Republic
    m.insert("PICTORIGHT", 256); // Netherlands
    m.insert("SAVA", 257); // Argentina
    m.insert("MRCSN", 258); // Nepal
    m.insert("SDCSI", 259); // Ireland
    m.insert("ACS", 260); // United Kingdom
    m.insert("GAI Uz", 261); // Uzbekistan
    m.insert("SINEBIR", 262); // Turkey
    m.insert("SACS", 263); // Seychelles
    m.insert("CARCC", 264); // Canada
    m.insert("MACA", 265); // Macau
    m.insert("BeAT", 266); // Brunei Darussalam
    m.insert("UPRAVIS", 267); // Russian Federation
    m.insert("COSON", 268); // Nigeria
    m.insert("WAMI", 269); // Indonesia
    m.insert("JASPAR", 270); // Japan
    m.insert("DHFA", 271); // Croatia
    m.insert("MOSCAP", 272); // Mongolia
    m.insert("AMUS", 273); // Bosnia And Herzegovina
    m.insert("AuPO CINEMA", 274); // Ukraine
    m.insert("AUTODIA", 275); // Greece
    m.insert("DASC", 276); // Colombia
    m.insert("RSAU", 277); // Rwanda
    m.insert("RUR", 278); // Russian Federation
    m.insert("SDADV", 279); // Andorra
    m.insert("SANASTO", 280); // Finland
    m.insert("UNAC-SA", 282); // Angola
    m.insert("CAPASSO", 283); // South Africa
    m.insert("COSOZA", 284); // Tanzania, United Republic Of
    m.insert("GHAMRO", 285); // Ghana
    m.insert("ODDA", 286); // Djibouti
    m.insert("KOLAA", 287); // Korea
    m.insert("ABYROY", 288); // Kazakhstan
    m.insert("AIPA", 289); // Slovenia
    m.insert("AZDG", 290); // Azerbaijan
    m.insert("OFA", 291); // Serbia
    m.insert("ZPAP", 292); // Poland
    m.insert("DBCA", 293); // Brazil
    m.insert("REDES SGC", 294); // Colombia
    m.insert("SAGCRYT", 295); // Mexico
    m.insert("DACIN-SARA", 296); // Romania
    m.insert("GEDAR", 297); // Brazil
    m.insert("OOA-S", 298); // Czech Republic
    m.insert("SCM-COOPERATIVA", 299); // Cape Verde
    m.insert("WID Centre", 300); // United States
    m.insert("GESAC", 301); // Belgium
    m.insert("LATINAUTOR", 302); // Uruguay
    m.insert("NORD-DOC", 303); // Sweden
    m.insert("SONGCODE", 304); // United States
    m.insert("IMJV", 305); // Netherlands
    m.insert("ACCS", 306); // Trinidad And Tobago
    m.insert("MIS@ASIA", 307); // Singapore
    m.insert("ECAD", 308); // Brazil
    m.insert("LatinNet", 309); // Spain
    m.insert("DIVA", 310); // Hong Kong
    m.insert("MCPS-PRS Alliance", 311); // United Kingdom
    m.insert("CISAC", 312); // France
    m.insert("FastTrack DCN", 313); // France
    m.insert("IDA", 314); // France
    m.insert("CSI", 315); // France
    m.insert("CIS-Net AVI", 316); // France
    m.insert("INTL-REP", 317); // France
    m.insert("SGS", 318); // France
    m.insert("ICE Services AB", 319); // Sweden
    m.insert("ARMONIA", 320); // France
    m.insert("PUBLISHERS", 321); // 
    m.insert("EVA", 322); // Belgium
    m.insert("ANCO", 323); // Moldova
    m.insert("CNRCMSE", 324); // Ethiopia
    m.insert("CRSEA", 325); // Russia
    m.insert("IMPF", 326); // Belgium
    m.insert("OAZA", 327); // Czech Republic
    m.insert("SAA", 328); // Belgium
    m.insert("ICSC", 329); // China
    m.insert("SINGCAPS", 330); // Singapore
    m.insert("COSBOTS", 331); // Botswana
    m.insert("DEGANZ", 332); // New Zealand
    m.insert("DGK", 333); // Republic Of Korea
    m.insert("ASCRL", 334); // USA
    m.insert("ISOCRATIS", 335); // Greece
    m.insert("KOSCAP", 336); // Republic Of Korea
    m.insert("COPYSWEDE", 337); // Sweden
    m.insert("EDEM", 338); // Greece
    m.insert("EKKI", 339); // Spain
    m.insert("MYNDSTEF", 340); // Iceland
    m.insert("AVTE", 341); // France
    m.insert("DGJ", 342); // Japan
    m.insert("Mali Maliki Institute", 343); // Ghana
    m.insert("SEDA", 344); // Spain
    m.insert("SIIP", 345); // Uzbekistan
    m.insert("TAMRISO", 346); // Tanzania, United Republic Of
    m.insert("IAF", 347); // United Kingdom
    m.insert("AVRS", 348); // Nigeria
    m.insert("DYGA", 349); // Chile
    m.insert("LESCOSAA", 350); // Lesotho
    m.insert("SEF", 351); // Turkey
    m.insert("VISARTA", 352); // Romania
    m.insert("RRA", 353); // New Zealand
    m.insert("ArmCinemaAuthor", 354); // Armenia
    m.insert("Asocijacija Autora", 355); // Serbia
    m.insert("KTRWA", 356); // Republic Of Korea
    m.insert("MRIGHTS", 357); // Italy
    m.insert("OSDEETE", 358); // Greece
    m.insert("SRAI", 359); // India
    m.insert("VAPIK", 360); // Kosovo
    m.insert("GRD", 555); // Fasttrack/GRD
    m.insert("SACEM Deal Multi territorial", 590); // Sacem-France
    m.insert("SACEM Deal", 591); // Sacem-France
    m.insert("BACKOFFICE", 592); // 
    m.insert("GEMA-US", 635); // Additional CIS-Net Node
    m.insert("SACEM-US", 658); // Additional CIS-Net Node
    m.insert("SGAE-NY", 672); // Additional CIS-Net Node
    m.insert("WIPO", 700); // Code used for the Deployment of the WIPO test CIS-Net node
    m.insert("SONY", 701); // 
    m.insert("BMG", 702); // 
    m.insert("UNIVERSAL", 703); // 
    m.insert("DISCOVERY", 704); // 
    m.insert("KOBALT", 705); // 
    m.insert("MusicMark", 707); // USA
    m.insert("The MLC", 708); // USA
    m.insert("ISAN", 710); // Switzerland
    m.insert("SACEM-LIBAN", 758); // Additional CIS-Net Node
    m.insert("Solar EMI", 775); // Germany/UK
    m.insert("Solar Sony", 776); // Germany/UK
    m.insert("CELAS", 777); // Germany/UK
    m.insert("GMR", 778); // United States
    m.insert("Polaris Nordic", 779); // Scandinavia
    m.insert("UNISON", 780); // Spain
    m.insert("SOUNDREEF", 781); // England and Wales
    m.insert("NexTone", 782); // Japan
    m.insert("HEXACORP LTD", 783); // USA
    m.insert("ESMAA", 784); // United Arab Emirates
    m.insert("LEA", 785); // Italy
    m.insert("ALLTRACK", 786); // USA
    m.insert("ORFIUM Greece", 787); // Greece
    m.insert("MINT", 788); // Hub of 16 Societies established by SESAC and SUISA
    m.insert("GDSDX", 789); // Asia Pacific
    m.insert("MESAM / MSG", 790); // Turkey
    m.insert("ATLAS", 791); // Asia Pacific
    m.insert("BRIDGER", 792); // Luxembourg
    m.insert("NMP", 793); // Sweden
    m.insert("PAECOL", 888); // Additional CIS-Net Node
    m
});

/// Reverse lookup from numeric code to society name
pub static SOCIETY_CODES_BY_NUMBER: Lazy<HashMap<u16, &'static str>> =
    Lazy::new(|| SOCIETY_CODES.iter().map(|(name, &code)| (code, *name)).collect());

/// Validates a society code exists in the lookup table
/// Accepts both society name strings and numeric codes (as strings with leading zeros)
pub fn is_valid_society_code(code: &str) -> bool {
    // First try as society name
    if SOCIETY_CODES.contains_key(code) {
        return true;
    }

    // Try as numeric code (parse and look up in reverse mapping)
    if let Ok(numeric_code) = code.parse::<u16>() {
        return SOCIETY_CODES_BY_NUMBER.contains_key(&numeric_code);
    }

    false
}

/// Gets the numeric code for a society name
pub fn get_society_numeric_code(society_name: &str) -> Option<u16> {
    SOCIETY_CODES.get(society_name).copied()
}

/// Gets all valid society codes
pub fn get_all_society_codes() -> Vec<&'static str> {
    SOCIETY_CODES.keys().copied().collect()
}
