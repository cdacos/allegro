//! Media Type lookup table (BIEM or CISAC)

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Media Type codes mapping
pub static MEDIA_TYPES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("S", "45 rpm 17 cm Single");
    m.insert("EP", "45 rpm 17 cm EP");
    m.insert("DS", "45 rpm (12 inches) Maxi Single");
    m.insert("RDS", "Disco Single Remix (12 inches)");
    m.insert("RMS", "Maxi Single Remix (7 inches)");
    m.insert("EPM", "LP 33 rpm 17cm EP");
    m.insert("MLP", "LP 33 rpm 25 cm");
    m.insert("LP", "LP 33 rpm 30 cm");
    m.insert("LP2", "LP Compilation");
    m.insert("LP3", "LP identical to a CD");
    m.insert("LP4", "LP Compilation identical to a CD Compilation");
    m.insert("SCD", "CD Single 2 tracks");
    m.insert("CDS", "CD Singles 3&5 inches");
    m.insert("CDM", "CD Maxi-single");
    m.insert("RCD", "CD Maxi single remix");
    m.insert("CD", "CD-LP (5 inches)");
    m.insert("CD2", "CD Compilation");
    m.insert("SA", "SACD");
    m.insert("SA2", "SACD Compilation");
    m.insert("CES", "CD Extra Single 2 tracks");
    m.insert("CXS", "CD Extra Single");
    m.insert("CXM", "CD Extra Maxi Single");
    m.insert("RCE", "CD Extra Maxi Remix");
    m.insert("CEP", "CD Extra EP");
    m.insert("CE", "CD Extra LP");
    m.insert("CE2", "CD Extra Compilation");
    m.insert("SMC", "MC single");
    m.insert("SM2", "MC single identical to a CDS");
    m.insert("MMC", "MC maxi");
    m.insert("EMC", "MC EP");
    m.insert("RMC", "MC Remix");
    m.insert("MCP", "MC");
    m.insert("MC", "MC LP");
    m.insert("MC2", "MC Compilation");
    m.insert("MC3", "MC identical to a CD");
    m.insert("MC4", "MC Compilation identical to a CD compilation");
    m.insert("DMC", "Double MC");
    m.insert("MDS", "MD Single/ Maxi Single");
    m.insert("MDR", "MD Maxi Remix");
    m.insert("MDP", "MD EP");
    m.insert("MD", "MD");
    m.insert("MD2", "MD Compilation");
    m.insert("DC", "DCC");
    m.insert("DC2", "DCC Compilation");
    m.insert("DV1", "DVD-Audio");
    m.insert("DV2", "DVD-Video");
    m.insert("DV3", "DVD-Rom");
    m.insert("DV4", "DVD-Single");
    m.insert("DW", "Downloading of a title");
    m.insert("DM", "Downloading of a Single/Maxi Single");
    m.insert("DL", "Downloading of LP");
    m
});

/// Validates a media type code
pub fn is_valid_media_type(code: &str) -> bool {
    MEDIA_TYPES.contains_key(code)
}

/// Gets the description for a media type code
pub fn get_media_type_description(code: &str) -> Option<&'static str> {
    MEDIA_TYPES.get(code).copied()
}

/// Gets all valid media type codes
pub fn get_all_media_types() -> Vec<&'static str> {
    MEDIA_TYPES.keys().copied().collect()
}