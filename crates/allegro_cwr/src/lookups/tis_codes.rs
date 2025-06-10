//! CISAC TIS (Territory Information System) lookup tables
//!
//! This implementation uses the official CISAC TIS data with proper modeling of:
//! - Territory validity periods (start/end dates)
//! - Territory types (country, geographical groups, etc.)
//! - English territory names only
//! - Usability indicators
//! - Territory hierarchy relationships

use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Territory type classification from CISAC TIS
#[derive(Debug, Clone, PartialEq)]
pub enum TerritoryType {
    Country,
    EconomicalCountryGroup,
    GeographicalCountryGroup,
    GeographicalCountrySubdivision,
    PoliticalCountryGroup,
    PoliticalCountrySubdivision,
}

/// CISAC TIS Territory information
#[derive(Debug, Clone)]
pub struct TisTerritory {
    pub code: u16,
    pub territory_name: String,
    pub official_name: Option<String>,
    pub abbreviated_name: Option<String>,
    pub territory_type: TerritoryType,
    pub validity_start: NaiveDateTime,
    pub validity_end: NaiveDateTime,
    pub usable: bool,
    pub remarks: Option<String>,
}

impl TisTerritory {
    /// Check if territory is valid for a given date (default: current date)
    pub fn is_valid_at(&self, date: Option<NaiveDateTime>) -> bool {
        let check_date = date.unwrap_or_else(|| chrono::Utc::now().naive_utc());
        self.usable && check_date >= self.validity_start && check_date <= self.validity_end
    }

    /// Get the best display name (abbreviated > territory > official)
    pub fn display_name(&self) -> &str {
        self.abbreviated_name.as_deref().unwrap_or(&self.territory_name)
    }
}

/// CISAC TIS territories lookup table (English names only)
pub static TIS_TERRITORIES: Lazy<HashMap<u16, TisTerritory>> = Lazy::new(|| {
    let mut territories = HashMap::new();

    // Generated from official CISAC TIS CSV data - English names only

    territories.insert(
        4,
        TisTerritory {
            code: 4,
            territory_name: "AFGHANISTAN".to_string(),
            official_name: None,
            abbreviated_name: Some("AFGHANISTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        8,
        TisTerritory {
            code: 8,
            territory_name: "ALBANIA".to_string(),
            official_name: Some("REPUBLIC OF ALBANIA".to_string()),
            abbreviated_name: Some("ALBANIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        12,
        TisTerritory {
            code: 12,
            territory_name: "ALGERIA".to_string(),
            official_name: Some("PEOPLE'S DEMOCRATIC REPUBLIC OF ALGERIA".to_string()),
            abbreviated_name: Some("ALGERIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        20,
        TisTerritory {
            code: 20,
            territory_name: "ANDORRA".to_string(),
            official_name: Some("PRINCIPALITY OF ANDORRA".to_string()),
            abbreviated_name: Some("ANDORRA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: false,
            remarks: None,
        },
    );
    territories.insert(
        24,
        TisTerritory {
            code: 24,
            territory_name: "ANGOLA".to_string(),
            official_name: Some("REPUBLIC OF ANGOLA".to_string()),
            abbreviated_name: Some("ANGOLA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        28,
        TisTerritory {
            code: 28,
            territory_name: "ANTIGUA AND BARBUDA".to_string(),
            official_name: None,
            abbreviated_name: Some("ANTIGUA+BARBUDA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        31,
        TisTerritory {
            code: 31,
            territory_name: "AZERBAIJAN".to_string(),
            official_name: Some("REPUBLIC OF AZERBAIJAN".to_string()),
            abbreviated_name: Some("AZERBAIJAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        32,
        TisTerritory {
            code: 32,
            territory_name: "ARGENTINA".to_string(),
            official_name: Some("ARGENTINE REPUBLIC".to_string()),
            abbreviated_name: Some("ARGENTINA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        36,
        TisTerritory {
            code: 36,
            territory_name: "AUSTRALIA".to_string(),
            official_name: None,
            abbreviated_name: Some("AUSTRALIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        40,
        TisTerritory {
            code: 40,
            territory_name: "AUSTRIA".to_string(),
            official_name: Some("REPUBLIC OF AUSTRIA".to_string()),
            abbreviated_name: Some("AUSTRIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        44,
        TisTerritory {
            code: 44,
            territory_name: "BAHAMAS".to_string(),
            official_name: Some("COMMONWEALTH OF THE BAHAMAS".to_string()),
            abbreviated_name: Some("BAHAMAS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        48,
        TisTerritory {
            code: 48,
            territory_name: "BAHRAIN".to_string(),
            official_name: Some("KINGDOM OF BAHRAIN".to_string()),
            abbreviated_name: Some("BAHRAIN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        50,
        TisTerritory {
            code: 50,
            territory_name: "BANGLADESH".to_string(),
            official_name: Some("PEOPLE'S REPUBLIC OF BANGLADESH".to_string()),
            abbreviated_name: Some("BANGLADESH".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        51,
        TisTerritory {
            code: 51,
            territory_name: "ARMENIA".to_string(),
            official_name: Some("REPUBLIC OF ARMENIA".to_string()),
            abbreviated_name: Some("ARMENIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        52,
        TisTerritory {
            code: 52,
            territory_name: "BARBADOS".to_string(),
            official_name: None,
            abbreviated_name: Some("BARBADOS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        56,
        TisTerritory {
            code: 56,
            territory_name: "BELGIUM".to_string(),
            official_name: Some("KINGDOM OF BELGIUM".to_string()),
            abbreviated_name: Some("BELGIUM".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        64,
        TisTerritory {
            code: 64,
            territory_name: "BHUTAN".to_string(),
            official_name: Some("KINGDOM OF BHUTAN".to_string()),
            abbreviated_name: Some("BHUTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        68,
        TisTerritory {
            code: 68,
            territory_name: "BOLIVIA".to_string(),
            official_name: Some("PLURINATIONAL STATE OF BOLIVIA".to_string()),
            abbreviated_name: Some("BOLIVIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        70,
        TisTerritory {
            code: 70,
            territory_name: "BOSNIA AND HERZEGOVINA".to_string(),
            official_name: None,
            abbreviated_name: Some("BOSNIA+HERZEGO.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        72,
        TisTerritory {
            code: 72,
            territory_name: "BOTSWANA".to_string(),
            official_name: Some("REPUBLIC OF BOTSWANA".to_string()),
            abbreviated_name: Some("BOTSWANA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        76,
        TisTerritory {
            code: 76,
            territory_name: "BRAZIL".to_string(),
            official_name: Some("FEDERATIVE REPUBLIC OF BRAZIL".to_string()),
            abbreviated_name: Some("BRAZIL".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        84,
        TisTerritory {
            code: 84,
            territory_name: "BELIZE".to_string(),
            official_name: None,
            abbreviated_name: Some("BELIZE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        90,
        TisTerritory {
            code: 90,
            territory_name: "SOLOMON ISLANDS".to_string(),
            official_name: None,
            abbreviated_name: Some("SOLOMON ISLANDS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        96,
        TisTerritory {
            code: 96,
            territory_name: "BRUNEI DARUSSALAM".to_string(),
            official_name: None,
            abbreviated_name: Some("BRUNEI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        100,
        TisTerritory {
            code: 100,
            territory_name: "BULGARIA".to_string(),
            official_name: Some("REPUBLIC OF BULGARIA".to_string()),
            abbreviated_name: Some("BULGARIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        104,
        TisTerritory {
            code: 104,
            territory_name: "MYANMAR".to_string(),
            official_name: Some("REPUBLIC OF THE UNION OF MYANMAR".to_string()),
            abbreviated_name: Some("MYANMAR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        108,
        TisTerritory {
            code: 108,
            territory_name: "BURUNDI".to_string(),
            official_name: Some("REPUBLIC OF BURUNDI".to_string()),
            abbreviated_name: Some("BURUNDI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        112,
        TisTerritory {
            code: 112,
            territory_name: "BELARUS".to_string(),
            official_name: Some("REPUBLIC OF BELARUS".to_string()),
            abbreviated_name: Some("BELARUS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        116,
        TisTerritory {
            code: 116,
            territory_name: "CAMBODIA".to_string(),
            official_name: Some("KINGDOM OF CAMBODIA".to_string()),
            abbreviated_name: Some("CAMBODIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        120,
        TisTerritory {
            code: 120,
            territory_name: "CAMEROON".to_string(),
            official_name: Some("REPUBLIC OF CAMEROON".to_string()),
            abbreviated_name: Some("CAMEROON".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        124,
        TisTerritory {
            code: 124,
            territory_name: "CANADA".to_string(),
            official_name: None,
            abbreviated_name: Some("CANADA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        132,
        TisTerritory {
            code: 132,
            territory_name: "CAPE VERDE".to_string(),
            official_name: Some("REPUBLIC OF CAPE VERDE".to_string()),
            abbreviated_name: Some("CAPE VERDE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        140,
        TisTerritory {
            code: 140,
            territory_name: "CENTRAL AFRICAN REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("CENT.AFRIC.REP.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        144,
        TisTerritory {
            code: 144,
            territory_name: "SRI LANKA".to_string(),
            official_name: Some("DEMOCRATIC SOCIALIST REPUBLIC OF SRI LANKA".to_string()),
            abbreviated_name: Some("SRI LANKA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        148,
        TisTerritory {
            code: 148,
            territory_name: "CHAD".to_string(),
            official_name: Some("REPUBLIC OF CHAD".to_string()),
            abbreviated_name: Some("CHAD".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        152,
        TisTerritory {
            code: 152,
            territory_name: "CHILE".to_string(),
            official_name: Some("REPUBLIC OF CHILE".to_string()),
            abbreviated_name: Some("CHILE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        156,
        TisTerritory {
            code: 156,
            territory_name: "CHINA".to_string(),
            official_name: Some("PEOPLE'S REPUBLIC OF CHINA".to_string()),
            abbreviated_name: Some("CHINA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: Some("TIS-N 156 CHINA IN THE MEANING OF THIS STANDARD DOES NOT INCLUDE HONG KONG. CF. ENTRY TIS-N 344 HONG KONG.".to_string()),
        },
    );
    territories.insert(
        158,
        TisTerritory {
            code: 158,
            territory_name: "TAIWAN, PROVINCE OF CHINA".to_string(),
            official_name: None,
            abbreviated_name: Some("TAIWAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        170,
        TisTerritory {
            code: 170,
            territory_name: "COLOMBIA".to_string(),
            official_name: Some("REPUBLIC OF COLOMBIA".to_string()),
            abbreviated_name: Some("COLOMBIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        174,
        TisTerritory {
            code: 174,
            territory_name: "COMOROS".to_string(),
            official_name: Some("UNION OF THE COMOROS".to_string()),
            abbreviated_name: Some("COMOROS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: Some("THIS ENTRY REFERS TO THE STATE COMOROS, NOT TO THE ARCHIPELAGO WITH THAT NAME. IT COMPRISES THEREFORE ANJOUAN, GRANDE COMORE AND MOHELI.".to_string()),
        },
    );
    territories.insert(
        178,
        TisTerritory {
            code: 178,
            territory_name: "CONGO".to_string(),
            official_name: Some("REPUBLIC OF THE CONGO".to_string()),
            abbreviated_name: Some("CONGO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        180,
        TisTerritory {
            code: 180,
            territory_name: "CONGO, THE DEMOCRATIC REPUBLIC OF THE".to_string(),
            official_name: Some("THE DEMOCRATIC REPUBLIC OF THE CONGO".to_string()),
            abbreviated_name: Some("DEMOC.REP.CONGO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        188,
        TisTerritory {
            code: 188,
            territory_name: "COSTA RICA".to_string(),
            official_name: Some("REPUBLIC OF COSTA RICA".to_string()),
            abbreviated_name: Some("COSTA RICA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        191,
        TisTerritory {
            code: 191,
            territory_name: "CROATIA".to_string(),
            official_name: Some("REPUBLIC OF CROATIA".to_string()),
            abbreviated_name: Some("CROATIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        192,
        TisTerritory {
            code: 192,
            territory_name: "CUBA".to_string(),
            official_name: Some("REPUBLIC OF CUBA".to_string()),
            abbreviated_name: Some("CUBA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        196,
        TisTerritory {
            code: 196,
            territory_name: "CYPRUS".to_string(),
            official_name: Some("REPUBLIC OF CYPRUS".to_string()),
            abbreviated_name: Some("CYPRUS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        200,
        TisTerritory {
            code: 200,
            territory_name: "CZECHOSLOVAKIA".to_string(),
            official_name: None,
            abbreviated_name: Some("CZECHOSLOVAKIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1992, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        203,
        TisTerritory {
            code: 203,
            territory_name: "CZECH REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("CZECH REPUBLIC".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1993, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        204,
        TisTerritory {
            code: 204,
            territory_name: "BENIN".to_string(),
            official_name: Some("REPUBLIC OF BENIN".to_string()),
            abbreviated_name: Some("BENIN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        208,
        TisTerritory {
            code: 208,
            territory_name: "DENMARK".to_string(),
            official_name: Some("KINGDOM OF DENMARK".to_string()),
            abbreviated_name: Some("DENMARK".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        212,
        TisTerritory {
            code: 212,
            territory_name: "DOMINICA".to_string(),
            official_name: Some("COMMONWEALTH OF DOMINICA".to_string()),
            abbreviated_name: Some("DOMINICA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        214,
        TisTerritory {
            code: 214,
            territory_name: "DOMINICAN REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("DOMINICAN REP.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        218,
        TisTerritory {
            code: 218,
            territory_name: "ECUADOR".to_string(),
            official_name: Some("REPUBLIC OF ECUADOR".to_string()),
            abbreviated_name: Some("ECUADOR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        222,
        TisTerritory {
            code: 222,
            territory_name: "EL SALVADOR".to_string(),
            official_name: Some("REPUBLIC OF EL SALVADOR".to_string()),
            abbreviated_name: Some("EL SALVADOR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        226,
        TisTerritory {
            code: 226,
            territory_name: "EQUATORIAL GUINEA".to_string(),
            official_name: Some("REPUBLIC OF EQUATORIAL GUINEA".to_string()),
            abbreviated_name: Some("EQUAT.GUINEA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        230,
        TisTerritory {
            code: 230,
            territory_name: "ETHIOPIA".to_string(),
            official_name: None,
            abbreviated_name: Some("ETHIOPIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1993, 5, 23).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        231,
        TisTerritory {
            code: 231,
            territory_name: "ETHIOPIA".to_string(),
            official_name: Some("FEDERAL DEMOCRATIC REPUBLIC OF ETHIOPIA".to_string()),
            abbreviated_name: Some("ETHIOPIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1993, 5, 24).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        232,
        TisTerritory {
            code: 232,
            territory_name: "ERITREA".to_string(),
            official_name: None,
            abbreviated_name: Some("ERITREA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1993, 5, 24).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        233,
        TisTerritory {
            code: 233,
            territory_name: "ESTONIA".to_string(),
            official_name: Some("REPUBLIC OF ESTONIA".to_string()),
            abbreviated_name: Some("ESTONIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        242,
        TisTerritory {
            code: 242,
            territory_name: "FIJI".to_string(),
            official_name: Some("REPUBLIC OF FIJI".to_string()),
            abbreviated_name: Some("FIJI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        246,
        TisTerritory {
            code: 246,
            territory_name: "FINLAND".to_string(),
            official_name: Some("REPUBLIC OF FINLAND".to_string()),
            abbreviated_name: Some("FINLAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        250,
        TisTerritory {
            code: 250,
            territory_name: "FRANCE".to_string(),
            official_name: Some("FRENCH REPUBLIC".to_string()),
            abbreviated_name: Some("FRANCE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: Some("TIS-N 250 FRANCE IN THE MEANING OF THIS STANDARD DOES NOT INCLUDE FRENCH POLYNESIA. CF. ENTRY TIS-N 258 FRENCH POLYNESIA.".to_string()),
        },
    );
    territories.insert(
        258,
        TisTerritory {
            code: 258,
            territory_name: "FRENCH POLYNESIA".to_string(),
            official_name: None,
            abbreviated_name: Some("FR.POLYNESIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        262,
        TisTerritory {
            code: 262,
            territory_name: "DJIBOUTI".to_string(),
            official_name: Some("REPUBLIC OF DJIBOUTI".to_string()),
            abbreviated_name: Some("DJIBOUTI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        266,
        TisTerritory {
            code: 266,
            territory_name: "GABON".to_string(),
            official_name: Some("GABONESE REPUBLIC".to_string()),
            abbreviated_name: Some("GABON".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        268,
        TisTerritory {
            code: 268,
            territory_name: "GEORGIA".to_string(),
            official_name: None,
            abbreviated_name: Some("GEORGIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        270,
        TisTerritory {
            code: 270,
            territory_name: "GAMBIA".to_string(),
            official_name: Some("REPUBLIC OF THE GAMBIA".to_string()),
            abbreviated_name: Some("GAMBIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        276,
        TisTerritory {
            code: 276,
            territory_name: "GERMANY".to_string(),
            official_name: Some("FEDERAL REPUBLIC OF GERMANY".to_string()),
            abbreviated_name: Some("GERMANY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1990, 10, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        278,
        TisTerritory {
            code: 278,
            territory_name: "GERMAN DEMOCRATIC REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("GDR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1990, 10, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        280,
        TisTerritory {
            code: 280,
            territory_name: "GERMANY".to_string(),
            official_name: Some("FEDERAL REPUBLIC OF GERMANY".to_string()),
            abbreviated_name: Some("GERMANY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1990, 10, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        288,
        TisTerritory {
            code: 288,
            territory_name: "GHANA".to_string(),
            official_name: Some("REPUBLIC OF GHANA".to_string()),
            abbreviated_name: Some("GHANA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        296,
        TisTerritory {
            code: 296,
            territory_name: "KIRIBATI".to_string(),
            official_name: Some("REPUBLIC OF KIRIBATI".to_string()),
            abbreviated_name: Some("KIRIBATI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        300,
        TisTerritory {
            code: 300,
            territory_name: "GREECE".to_string(),
            official_name: Some("HELLENIC REPUBLIC".to_string()),
            abbreviated_name: Some("GREECE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        308,
        TisTerritory {
            code: 308,
            territory_name: "GRENADA".to_string(),
            official_name: None,
            abbreviated_name: Some("GRENADA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        320,
        TisTerritory {
            code: 320,
            territory_name: "GUATEMALA".to_string(),
            official_name: Some("REPUBLIC OF GUATEMALA".to_string()),
            abbreviated_name: Some("GUATEMALA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        324,
        TisTerritory {
            code: 324,
            territory_name: "GUINEA".to_string(),
            official_name: Some("REPUBLIC OF GUINEA".to_string()),
            abbreviated_name: Some("GUINEA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        328,
        TisTerritory {
            code: 328,
            territory_name: "GUYANA".to_string(),
            official_name: Some("REPUBLIC OF GUYANA".to_string()),
            abbreviated_name: Some("GUYANA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        332,
        TisTerritory {
            code: 332,
            territory_name: "HAITI".to_string(),
            official_name: Some("REPUBLIC OF HAITI".to_string()),
            abbreviated_name: Some("HAITI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        336,
        TisTerritory {
            code: 336,
            territory_name: "HOLY SEE (VATICAN CITY STATE)".to_string(),
            official_name: None,
            abbreviated_name: Some("VATICAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        340,
        TisTerritory {
            code: 340,
            territory_name: "HONDURAS".to_string(),
            official_name: Some("REPUBLIC OF HONDURAS".to_string()),
            abbreviated_name: Some("HONDURAS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        344,
        TisTerritory {
            code: 344,
            territory_name: "HONG KONG".to_string(),
            official_name: Some("HONG KONG SPECIAL ADMINISTRATIVE REGION OF CHINA".to_string()),
            abbreviated_name: Some("HONG KONG".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        348,
        TisTerritory {
            code: 348,
            territory_name: "HUNGARY".to_string(),
            official_name: Some("HUNGARY".to_string()),
            abbreviated_name: Some("HUNGARY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        352,
        TisTerritory {
            code: 352,
            territory_name: "ICELAND".to_string(),
            official_name: Some("REPUBLIC OF ICELAND".to_string()),
            abbreviated_name: Some("ICELAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        356,
        TisTerritory {
            code: 356,
            territory_name: "INDIA".to_string(),
            official_name: Some("REPUBLIC OF INDIA".to_string()),
            abbreviated_name: Some("INDIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        360,
        TisTerritory {
            code: 360,
            territory_name: "INDONESIA".to_string(),
            official_name: Some("REPUBLIC OF INDONESIA".to_string()),
            abbreviated_name: Some("INDONESIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        364,
        TisTerritory {
            code: 364,
            territory_name: "IRAN, ISLAMIC REPUBLIC OF".to_string(),
            official_name: Some("ISLAMIC REPUBLIC OF IRAN".to_string()),
            abbreviated_name: Some("IRAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        368,
        TisTerritory {
            code: 368,
            territory_name: "IRAQ".to_string(),
            official_name: Some("REPUBLIC OF IRAQ".to_string()),
            abbreviated_name: Some("IRAQ".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        372,
        TisTerritory {
            code: 372,
            territory_name: "IRELAND".to_string(),
            official_name: None,
            abbreviated_name: Some("IRELAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        376,
        TisTerritory {
            code: 376,
            territory_name: "ISRAEL".to_string(),
            official_name: Some("STATE OF ISRAEL".to_string()),
            abbreviated_name: Some("ISRAEL".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        380,
        TisTerritory {
            code: 380,
            territory_name: "ITALY".to_string(),
            official_name: Some("ITALIAN REPUBLIC".to_string()),
            abbreviated_name: Some("ITALY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        384,
        TisTerritory {
            code: 384,
            territory_name: "COTE D'IVOIRE".to_string(),
            official_name: Some("REPUBLIC OF COTE D'IVOIRE".to_string()),
            abbreviated_name: Some("COTE D'IVOIRE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        388,
        TisTerritory {
            code: 388,
            territory_name: "JAMAICA".to_string(),
            official_name: None,
            abbreviated_name: Some("JAMAICA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        392,
        TisTerritory {
            code: 392,
            territory_name: "JAPAN".to_string(),
            official_name: None,
            abbreviated_name: Some("JAPAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        398,
        TisTerritory {
            code: 398,
            territory_name: "KAZAKHSTAN".to_string(),
            official_name: Some("REPUBLIC OF KAZAKHSTAN".to_string()),
            abbreviated_name: Some("KAZAKHSTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        400,
        TisTerritory {
            code: 400,
            territory_name: "JORDAN".to_string(),
            official_name: Some("HASHEMITE KINGDOM OF JORDAN".to_string()),
            abbreviated_name: Some("JORDAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        404,
        TisTerritory {
            code: 404,
            territory_name: "KENYA".to_string(),
            official_name: Some("REPUBLIC OF KENYA".to_string()),
            abbreviated_name: Some("KENYA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        408,
        TisTerritory {
            code: 408,
            territory_name: "KOREA, DEMOCRATIC PEOPLE'S REPUBLIC OF".to_string(),
            official_name: Some("DEMOCRATIC PEOPLE'S REPUBLIC OF KOREA".to_string()),
            abbreviated_name: Some("NORTH KOREA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        410,
        TisTerritory {
            code: 410,
            territory_name: "KOREA, REPUBLIC OF".to_string(),
            official_name: Some("REPUBLIC OF KOREA".to_string()),
            abbreviated_name: Some("SOUTH KOREA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        414,
        TisTerritory {
            code: 414,
            territory_name: "KUWAIT".to_string(),
            official_name: Some("STATE OF KUWAIT".to_string()),
            abbreviated_name: Some("KUWAIT".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        417,
        TisTerritory {
            code: 417,
            territory_name: "KYRGYZSTAN".to_string(),
            official_name: Some("KYRGYZ REPUBLIC".to_string()),
            abbreviated_name: Some("KYRGYZSTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        418,
        TisTerritory {
            code: 418,
            territory_name: "LAO PEOPLE'S DEMOCRATIC REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("LAO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        422,
        TisTerritory {
            code: 422,
            territory_name: "LEBANON".to_string(),
            official_name: Some("LEBANESE REPUBLIC".to_string()),
            abbreviated_name: Some("LEBANON".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        426,
        TisTerritory {
            code: 426,
            territory_name: "LESOTHO".to_string(),
            official_name: Some("KINGDOM OF LESOTHO".to_string()),
            abbreviated_name: Some("LESOTHO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        428,
        TisTerritory {
            code: 428,
            territory_name: "LATVIA".to_string(),
            official_name: Some("REPUBLIC OF LATVIA".to_string()),
            abbreviated_name: Some("LATVIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        430,
        TisTerritory {
            code: 430,
            territory_name: "LIBERIA".to_string(),
            official_name: Some("REPUBLIC OF LIBERIA".to_string()),
            abbreviated_name: Some("LIBERIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        434,
        TisTerritory {
            code: 434,
            territory_name: "LIBYA".to_string(),
            official_name: Some("STATE OF LIBYA".to_string()),
            abbreviated_name: Some("LIBYA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        438,
        TisTerritory {
            code: 438,
            territory_name: "LIECHTENSTEIN".to_string(),
            official_name: Some("PRINCIPALITY OF LIECHTENSTEIN".to_string()),
            abbreviated_name: Some("LIECHTENSTEIN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        440,
        TisTerritory {
            code: 440,
            territory_name: "LITHUANIA".to_string(),
            official_name: Some("REPUBLIC OF LITHUANIA".to_string()),
            abbreviated_name: Some("LITHUANIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        442,
        TisTerritory {
            code: 442,
            territory_name: "LUXEMBOURG".to_string(),
            official_name: Some("GRAND DUCHY OF LUXEMBOURG".to_string()),
            abbreviated_name: Some("LUXEMBOURG".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        446,
        TisTerritory {
            code: 446,
            territory_name: "MACAO".to_string(),
            official_name: Some("MACAO SPECIAL ADMINISTRATIVE REGION OF CHINA".to_string()),
            abbreviated_name: Some("MACAO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1999, 12, 20).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        450,
        TisTerritory {
            code: 450,
            territory_name: "MADAGASCAR".to_string(),
            official_name: Some("REPUBLIC OF MADAGASCAR".to_string()),
            abbreviated_name: Some("MADAGASCAR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        454,
        TisTerritory {
            code: 454,
            territory_name: "MALAWI".to_string(),
            official_name: Some("REPUBLIC OF MALAWI".to_string()),
            abbreviated_name: Some("MALAWI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        458,
        TisTerritory {
            code: 458,
            territory_name: "MALAYSIA".to_string(),
            official_name: None,
            abbreviated_name: Some("MALAYSIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        462,
        TisTerritory {
            code: 462,
            territory_name: "MALDIVES".to_string(),
            official_name: Some("REPUBLIC OF MALDIVES".to_string()),
            abbreviated_name: Some("MALDIVES".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        466,
        TisTerritory {
            code: 466,
            territory_name: "MALI".to_string(),
            official_name: Some("REPUBLIC OF MALI".to_string()),
            abbreviated_name: Some("MALI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        470,
        TisTerritory {
            code: 470,
            territory_name: "MALTA".to_string(),
            official_name: Some("REPUBLIC OF MALTA".to_string()),
            abbreviated_name: Some("MALTA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        478,
        TisTerritory {
            code: 478,
            territory_name: "MAURITANIA".to_string(),
            official_name: Some("ISLAMIC REPUBLIC OF MAURITANIA".to_string()),
            abbreviated_name: Some("MAURITANIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        480,
        TisTerritory {
            code: 480,
            territory_name: "MAURITIUS".to_string(),
            official_name: Some("REPUBLIC OF MAURITIUS".to_string()),
            abbreviated_name: Some("MAURITIUS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        484,
        TisTerritory {
            code: 484,
            territory_name: "MEXICO".to_string(),
            official_name: Some("UNITED MEXICAN STATES".to_string()),
            abbreviated_name: Some("MEXICO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        492,
        TisTerritory {
            code: 492,
            territory_name: "MONACO".to_string(),
            official_name: Some("PRINCIPALITY OF MONACO".to_string()),
            abbreviated_name: Some("MONACO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        496,
        TisTerritory {
            code: 496,
            territory_name: "MONGOLIA".to_string(),
            official_name: None,
            abbreviated_name: Some("MONGOLIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        498,
        TisTerritory {
            code: 498,
            territory_name: "MOLDOVA, REPUBLIC OF".to_string(),
            official_name: Some("REPUBLIC OF MOLDOVA".to_string()),
            abbreviated_name: Some("MOLDOVA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        499,
        TisTerritory {
            code: 499,
            territory_name: "MONTENEGRO".to_string(),
            official_name: Some("MONTENEGRO".to_string()),
            abbreviated_name: Some("MONTENEGRO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(2006, 6, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        504,
        TisTerritory {
            code: 504,
            territory_name: "MOROCCO".to_string(),
            official_name: Some("KINGDOM OF MOROCCO".to_string()),
            abbreviated_name: Some("MOROCCO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        508,
        TisTerritory {
            code: 508,
            territory_name: "MOZAMBIQUE".to_string(),
            official_name: Some("REPUBLIC OF MOZAMBIQUE".to_string()),
            abbreviated_name: Some("MOZAMBIQUE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        512,
        TisTerritory {
            code: 512,
            territory_name: "OMAN".to_string(),
            official_name: Some("SULTANATE OF OMAN".to_string()),
            abbreviated_name: Some("OMAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        516,
        TisTerritory {
            code: 516,
            territory_name: "NAMIBIA".to_string(),
            official_name: Some("REPUBLIC OF NAMIBIA".to_string()),
            abbreviated_name: Some("NAMIBIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        520,
        TisTerritory {
            code: 520,
            territory_name: "NAURU".to_string(),
            official_name: Some("REPUBLIC OF NAURU".to_string()),
            abbreviated_name: Some("NAURU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        524,
        TisTerritory {
            code: 524,
            territory_name: "NEPAL".to_string(),
            official_name: Some("FEDERAL DEMOCRATIC REPUBLIC OF NEPAL".to_string()),
            abbreviated_name: Some("NEPAL".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        528,
        TisTerritory {
            code: 528,
            territory_name: "NETHERLANDS".to_string(),
            official_name: Some("KINGDOM OF THE NETHERLANDS".to_string()),
            abbreviated_name: Some("NETHERLANDS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        540,
        TisTerritory {
            code: 540,
            territory_name: "NEW CALEDONIA".to_string(),
            official_name: None,
            abbreviated_name: Some("NEW CALEDONIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        548,
        TisTerritory {
            code: 548,
            territory_name: "VANUATU".to_string(),
            official_name: Some("REPUBLIC OF VANUATU".to_string()),
            abbreviated_name: Some("VANUATU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        554,
        TisTerritory {
            code: 554,
            territory_name: "NEW ZEALAND".to_string(),
            official_name: None,
            abbreviated_name: Some("NEW ZEALAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        558,
        TisTerritory {
            code: 558,
            territory_name: "NICARAGUA".to_string(),
            official_name: Some("REPUBLIC OF NICARAGUA".to_string()),
            abbreviated_name: Some("NICARAGUA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        562,
        TisTerritory {
            code: 562,
            territory_name: "NIGER".to_string(),
            official_name: Some("REPUBLIC OF THE NIGER".to_string()),
            abbreviated_name: Some("NIGER".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        566,
        TisTerritory {
            code: 566,
            territory_name: "NIGERIA".to_string(),
            official_name: Some("FEDERAL REPUBLIC OF NIGERIA".to_string()),
            abbreviated_name: Some("NIGERIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        578,
        TisTerritory {
            code: 578,
            territory_name: "NORWAY".to_string(),
            official_name: Some("KINGDOM OF NORWAY".to_string()),
            abbreviated_name: Some("NORWAY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        583,
        TisTerritory {
            code: 583,
            territory_name: "MICRONESIA, FEDERATED STATES OF".to_string(),
            official_name: Some("FEDERATED STATES OF MICRONESIA".to_string()),
            abbreviated_name: Some("MICRONESIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        584,
        TisTerritory {
            code: 584,
            territory_name: "MARSHALL ISLANDS".to_string(),
            official_name: Some("REPUBLIC OF THE MARSHALL ISLANDS".to_string()),
            abbreviated_name: Some("MARSHALL ISL.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        585,
        TisTerritory {
            code: 585,
            territory_name: "PALAU".to_string(),
            official_name: Some("REPUBLIC OF PALAU".to_string()),
            abbreviated_name: Some("PALAU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        586,
        TisTerritory {
            code: 586,
            territory_name: "PAKISTAN".to_string(),
            official_name: Some("ISLAMIC REPUBLIC OF PAKISTAN".to_string()),
            abbreviated_name: Some("PAKISTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        591,
        TisTerritory {
            code: 591,
            territory_name: "PANAMA".to_string(),
            official_name: Some("REPUBLIC OF PANAMA".to_string()),
            abbreviated_name: Some("PANAMA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        598,
        TisTerritory {
            code: 598,
            territory_name: "PAPUA NEW GUINEA".to_string(),
            official_name: None,
            abbreviated_name: Some("PAPUA N.GUINEA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        600,
        TisTerritory {
            code: 600,
            territory_name: "PARAGUAY".to_string(),
            official_name: Some("REPUBLIC OF PARAGUAY".to_string()),
            abbreviated_name: Some("PARAGUAY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        604,
        TisTerritory {
            code: 604,
            territory_name: "PERU".to_string(),
            official_name: Some("REPUBLIC OF PERU".to_string()),
            abbreviated_name: Some("PERU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        608,
        TisTerritory {
            code: 608,
            territory_name: "PHILIPPINES".to_string(),
            official_name: Some("REPUBLIC OF THE PHILIPPINES".to_string()),
            abbreviated_name: Some("PHILIPPINES".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        616,
        TisTerritory {
            code: 616,
            territory_name: "POLAND".to_string(),
            official_name: Some("REPUBLIC OF POLAND".to_string()),
            abbreviated_name: Some("POLAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        620,
        TisTerritory {
            code: 620,
            territory_name: "PORTUGAL".to_string(),
            official_name: Some("PORTUGUESE REPUBLIC".to_string()),
            abbreviated_name: Some("PORTUGAL".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        624,
        TisTerritory {
            code: 624,
            territory_name: "GUINEA-BISSAU".to_string(),
            official_name: Some("REPUBLIC OF GUINEA-BISSAU".to_string()),
            abbreviated_name: Some("GUINEA-BISSAU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        626,
        TisTerritory {
            code: 626,
            territory_name: "TIMOR-LESTE".to_string(),
            official_name: Some("DEMOCRATIC REPUBLIC OF TIMOR-LESTE".to_string()),
            abbreviated_name: Some("TIMOR-LESTE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(2002, 5, 20).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        630,
        TisTerritory {
            code: 630,
            territory_name: "PUERTO RICO".to_string(),
            official_name: None,
            abbreviated_name: Some("PUERTO RICO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        634,
        TisTerritory {
            code: 634,
            territory_name: "QATAR".to_string(),
            official_name: Some("STATE OF QATAR".to_string()),
            abbreviated_name: Some("QATAR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        642,
        TisTerritory {
            code: 642,
            territory_name: "ROMANIA".to_string(),
            official_name: None,
            abbreviated_name: Some("ROMANIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        643,
        TisTerritory {
            code: 643,
            territory_name: "RUSSIAN FEDERATION".to_string(),
            official_name: None,
            abbreviated_name: Some("RUSSIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        646,
        TisTerritory {
            code: 646,
            territory_name: "RWANDA".to_string(),
            official_name: Some("RWANDESE RUPUBLIC".to_string()),
            abbreviated_name: Some("RWANDA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        659,
        TisTerritory {
            code: 659,
            territory_name: "SAINT KITTS AND NEVIS".to_string(),
            official_name: None,
            abbreviated_name: Some("ST.KITTS+NEVIS".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        662,
        TisTerritory {
            code: 662,
            territory_name: "SAINT LUCIA".to_string(),
            official_name: None,
            abbreviated_name: Some("ST.LUCIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        670,
        TisTerritory {
            code: 670,
            territory_name: "SAINT VINCENT AND THE GRENADINES".to_string(),
            official_name: None,
            abbreviated_name: Some("ST.VINCENT+GR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        674,
        TisTerritory {
            code: 674,
            territory_name: "SAN MARINO".to_string(),
            official_name: Some("REPUBLIC OF SAN MARINO".to_string()),
            abbreviated_name: Some("SAN MARINO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        678,
        TisTerritory {
            code: 678,
            territory_name: "SAO TOME AND PRINCIPE".to_string(),
            official_name: Some("DEMOCRATIC REPUBLIC OF SAO TOME AND PRINCIPE".to_string()),
            abbreviated_name: Some("S.TOME+PRINCIPE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        682,
        TisTerritory {
            code: 682,
            territory_name: "SAUDI ARABIA".to_string(),
            official_name: Some("KINGDOM OF SAUDI ARABIA".to_string()),
            abbreviated_name: Some("SAUDI ARABIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        686,
        TisTerritory {
            code: 686,
            territory_name: "SENEGAL".to_string(),
            official_name: Some("REPUBLIC OF SENEGAL".to_string()),
            abbreviated_name: Some("SENEGAL".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        688,
        TisTerritory {
            code: 688,
            territory_name: "SERBIA".to_string(),
            official_name: Some("REPUBLIC OF SERBIA".to_string()),
            abbreviated_name: Some("SERBIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(2006, 6, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        690,
        TisTerritory {
            code: 690,
            territory_name: "SEYCHELLES".to_string(),
            official_name: Some("REPUBLIC OF SEYCHELLES".to_string()),
            abbreviated_name: Some("SEYCHELLES".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        694,
        TisTerritory {
            code: 694,
            territory_name: "SIERRA LEONE".to_string(),
            official_name: Some("REPUBLIC OF SIERRA LEONE".to_string()),
            abbreviated_name: Some("SIERRA LEONE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        702,
        TisTerritory {
            code: 702,
            territory_name: "SINGAPORE".to_string(),
            official_name: Some("REPUBLIC OF SINGAPORE".to_string()),
            abbreviated_name: Some("SINGAPORE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        703,
        TisTerritory {
            code: 703,
            territory_name: "SLOVAKIA".to_string(),
            official_name: Some("SLOVAK REPUBLIC".to_string()),
            abbreviated_name: Some("SLOVAKIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1993, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        704,
        TisTerritory {
            code: 704,
            territory_name: "VIET NAM".to_string(),
            official_name: Some("SOCIALIST REPUBLIC OF VIET NAM".to_string()),
            abbreviated_name: Some("VIET NAM".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        705,
        TisTerritory {
            code: 705,
            territory_name: "SLOVENIA".to_string(),
            official_name: Some("REPUBLIC OF SLOVENIA".to_string()),
            abbreviated_name: Some("SLOVENIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        706,
        TisTerritory {
            code: 706,
            territory_name: "SOMALIA".to_string(),
            official_name: Some("SOMALI REPUBLIC".to_string()),
            abbreviated_name: Some("SOMALIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        710,
        TisTerritory {
            code: 710,
            territory_name: "SOUTH AFRICA".to_string(),
            official_name: Some("REPUBLIC OF SOUTH AFRICA".to_string()),
            abbreviated_name: Some("SOUTH AFRICA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        716,
        TisTerritory {
            code: 716,
            territory_name: "ZIMBABWE".to_string(),
            official_name: Some("REPUBLIC OF ZIMBABWE".to_string()),
            abbreviated_name: Some("ZIMBABWE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        720,
        TisTerritory {
            code: 720,
            territory_name: "YEMEN, DEMOCRATIC".to_string(),
            official_name: None,
            abbreviated_name: Some("DEMOCRAT.YEMEN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1990, 5, 21).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        724,
        TisTerritory {
            code: 724,
            territory_name: "SPAIN".to_string(),
            official_name: Some("KINGDOM OF SPAIN".to_string()),
            abbreviated_name: Some("SPAIN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        728,
        TisTerritory {
            code: 728,
            territory_name: "SOUTH SUDAN".to_string(),
            official_name: Some("REPUBLIC OF SOUTH SUDAN".to_string()),
            abbreviated_name: Some("SOUTH SUDAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(2011, 7, 9).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        729,
        TisTerritory {
            code: 729,
            territory_name: "SUDAN".to_string(),
            official_name: Some("REPUBLIC OF THE SUDAN".to_string()),
            abbreviated_name: Some("SUDAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(2011, 7, 9).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        732,
        TisTerritory {
            code: 732,
            territory_name: "WESTERN SAHARA".to_string(),
            official_name: None,
            abbreviated_name: Some("WESTERN SAHARA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        736,
        TisTerritory {
            code: 736,
            territory_name: "SUDAN".to_string(),
            official_name: Some("REPUBLIC OF THE SUDAN".to_string()),
            abbreviated_name: Some("SUDAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(2011, 7, 8).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        740,
        TisTerritory {
            code: 740,
            territory_name: "SURINAME".to_string(),
            official_name: Some("REPUBLIC OF SURINAME".to_string()),
            abbreviated_name: Some("SURINAME".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        748,
        TisTerritory {
            code: 748,
            territory_name: "ESWATINI".to_string(),
            official_name: Some("THE KINGDOM OF ESWATINI".to_string()),
            abbreviated_name: Some("ESWATINI".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        752,
        TisTerritory {
            code: 752,
            territory_name: "SWEDEN".to_string(),
            official_name: Some("KINGDOM OF SWEDEN".to_string()),
            abbreviated_name: Some("SWEDEN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        756,
        TisTerritory {
            code: 756,
            territory_name: "SWITZERLAND".to_string(),
            official_name: Some("SWISS CONFEDERATION".to_string()),
            abbreviated_name: Some("SWITZERLAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        760,
        TisTerritory {
            code: 760,
            territory_name: "SYRIAN ARAB REPUBLIC".to_string(),
            official_name: None,
            abbreviated_name: Some("SYRIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        762,
        TisTerritory {
            code: 762,
            territory_name: "TAJIKISTAN".to_string(),
            official_name: Some("REPUBLIC OF TAJIKISTAN".to_string()),
            abbreviated_name: Some("TAJIKISTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        764,
        TisTerritory {
            code: 764,
            territory_name: "THAILAND".to_string(),
            official_name: Some("KINGDOM OF THAILAND".to_string()),
            abbreviated_name: Some("THAILAND".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        768,
        TisTerritory {
            code: 768,
            territory_name: "TOGO".to_string(),
            official_name: Some("TOGOLESE REPUBLIC".to_string()),
            abbreviated_name: Some("TOGO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        776,
        TisTerritory {
            code: 776,
            territory_name: "TONGA".to_string(),
            official_name: Some("KINGDOM OF TONGA".to_string()),
            abbreviated_name: Some("TONGA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        780,
        TisTerritory {
            code: 780,
            territory_name: "TRINIDAD AND TOBAGO".to_string(),
            official_name: Some("REPUBLIC OF TRINIDAD AND TOBAGO".to_string()),
            abbreviated_name: Some("TRINIDAD+TOBAGO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        784,
        TisTerritory {
            code: 784,
            territory_name: "UNITED ARAB EMIRATES".to_string(),
            official_name: None,
            abbreviated_name: Some("UNIT.ARAB.EMIR.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        788,
        TisTerritory {
            code: 788,
            territory_name: "TUNISIA".to_string(),
            official_name: Some("REPUBLIC OF TUNISIA".to_string()),
            abbreviated_name: Some("TUNISIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        792,
        TisTerritory {
            code: 792,
            territory_name: "TURKEY".to_string(),
            official_name: Some("REPUBLIC OF TURKEY".to_string()),
            abbreviated_name: Some("TURKEY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        795,
        TisTerritory {
            code: 795,
            territory_name: "TURKMENISTAN".to_string(),
            official_name: None,
            abbreviated_name: Some("TURKMENISTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        798,
        TisTerritory {
            code: 798,
            territory_name: "TUVALU".to_string(),
            official_name: None,
            abbreviated_name: Some("TUVALU".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        800,
        TisTerritory {
            code: 800,
            territory_name: "UGANDA".to_string(),
            official_name: Some("REPUBLIC OF UGANDA".to_string()),
            abbreviated_name: Some("UGANDA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        804,
        TisTerritory {
            code: 804,
            territory_name: "UKRAINE".to_string(),
            official_name: None,
            abbreviated_name: Some("UKRAINE".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        807,
        TisTerritory {
            code: 807,
            territory_name: "NORTH MACEDONIA".to_string(),
            official_name: Some("THE REPUBLIC OF NORTH MACEDONIA".to_string()),
            abbreviated_name: Some("NORTH MACEDONIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        810,
        TisTerritory {
            code: 810,
            territory_name: "USSR".to_string(),
            official_name: None,
            abbreviated_name: Some("USSR".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1991, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        818,
        TisTerritory {
            code: 818,
            territory_name: "EGYPT".to_string(),
            official_name: Some("ARAB REPUBLIC OF EGYPT".to_string()),
            abbreviated_name: Some("EGYPT".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        826,
        TisTerritory {
            code: 826,
            territory_name: "UNITED KINGDOM".to_string(),
            official_name: Some("UNITED KINGDOM OF GREAT BRITAIN AND NORTHERN IRELAND".to_string()),
            abbreviated_name: Some("UNITED KINGDOM".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        834,
        TisTerritory {
            code: 834,
            territory_name: "TANZANIA, UNITED REPUBLIC OF".to_string(),
            official_name: Some("UNITED REPUBLIC OF TANZANIA".to_string()),
            abbreviated_name: Some("TANZANIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        840,
        TisTerritory {
            code: 840,
            territory_name: "UNITED STATES".to_string(),
            official_name: Some("UNITED STATES OF AMERICA".to_string()),
            abbreviated_name: Some("USA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: Some("TIS-N 840 UNITED STATES IN THE MEANING OF THIS STANDARD DOES NOT INCLUDE PUERTO RICO. CF. ENTRY TIS-N 630 PUERTO RICO.".to_string()),
        },
    );
    territories.insert(
        854,
        TisTerritory {
            code: 854,
            territory_name: "BURKINA FASO".to_string(),
            official_name: None,
            abbreviated_name: Some("BURKINA FASO".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        858,
        TisTerritory {
            code: 858,
            territory_name: "URUGUAY".to_string(),
            official_name: Some("EASTERN REPUBLIC OF URUGUAY".to_string()),
            abbreviated_name: Some("URUGUAY".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        860,
        TisTerritory {
            code: 860,
            territory_name: "UZBEKISTAN".to_string(),
            official_name: Some("REPUBLIC OF UZBEKISTAN".to_string()),
            abbreviated_name: Some("UZBEKISTAN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        862,
        TisTerritory {
            code: 862,
            territory_name: "VENEZUELA".to_string(),
            official_name: Some("BOLIVARIAN REPUBLIC OF VENEZUELA".to_string()),
            abbreviated_name: Some("VENEZUELA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        882,
        TisTerritory {
            code: 882,
            territory_name: "SAMOA".to_string(),
            official_name: Some("INDEPENDENT STATE OF SAMOA".to_string()),
            abbreviated_name: Some("SAMOA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        886,
        TisTerritory {
            code: 886,
            territory_name: "YEMEN".to_string(),
            official_name: Some("REPUBLIC OF YEMEN".to_string()),
            abbreviated_name: Some("YEMEN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1990, 5, 21).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        887,
        TisTerritory {
            code: 887,
            territory_name: "YEMEN".to_string(),
            official_name: Some("REPUBLIC OF YEMEN".to_string()),
            abbreviated_name: Some("YEMEN".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1990, 5, 22).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        890,
        TisTerritory {
            code: 890,
            territory_name: "YUGOSLAVIA".to_string(),
            official_name: Some("FEDERAL REPUBLIC OF YUGOSLAVIA".to_string()),
            abbreviated_name: Some("YUGOSLAVIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(1991, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        891,
        TisTerritory {
            code: 891,
            territory_name: "SERBIA AND MONTENEGRO".to_string(),
            official_name: Some("SERBIA AND MONTENEGRO".to_string()),
            abbreviated_name: Some("SERBIA+MONTENE.".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(2006, 6, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        894,
        TisTerritory {
            code: 894,
            territory_name: "ZAMBIA".to_string(),
            official_name: Some("REPUBLIC OF ZAMBIA".to_string()),
            abbreviated_name: Some("ZAMBIA".to_string()),
            territory_type: TerritoryType::Country,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2100,
        TisTerritory {
            code: 2100,
            territory_name: "AFRICA".to_string(),
            official_name: None,
            abbreviated_name: Some("AFRICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2101,
        TisTerritory {
            code: 2101,
            territory_name: "AMERICA".to_string(),
            official_name: None,
            abbreviated_name: Some("AMERICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2102,
        TisTerritory {
            code: 2102,
            territory_name: "AMERICAN CONTINENT".to_string(),
            official_name: None,
            abbreviated_name: Some("AMERICAN CONT.".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2103,
        TisTerritory {
            code: 2103,
            territory_name: "ANTILLES".to_string(),
            official_name: None,
            abbreviated_name: Some("ANTILLES".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2104,
        TisTerritory {
            code: 2104,
            territory_name: "APEC COUNTRIES".to_string(),
            official_name: None,
            abbreviated_name: Some("APEC".to_string()),
            territory_type: TerritoryType::EconomicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1989, 11, 7).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2105,
        TisTerritory {
            code: 2105,
            territory_name: "ASEAN COUNTRIES".to_string(),
            official_name: None,
            abbreviated_name: Some("ASEAN".to_string()),
            territory_type: TerritoryType::EconomicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1967, 8, 8).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2106,
        TisTerritory {
            code: 2106,
            territory_name: "ASIA".to_string(),
            official_name: None,
            abbreviated_name: Some("ASIA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2107,
        TisTerritory {
            code: 2107,
            territory_name: "AUSTRALASIA".to_string(),
            official_name: None,
            abbreviated_name: Some("AUSTRALASIA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2108,
        TisTerritory {
            code: 2108,
            territory_name: "BALKANS".to_string(),
            official_name: None,
            abbreviated_name: Some("BALKANS".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2109,
        TisTerritory {
            code: 2109,
            territory_name: "BALTIC STATES".to_string(),
            official_name: None,
            abbreviated_name: Some("BALTIC STATES".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2110,
        TisTerritory {
            code: 2110,
            territory_name: "BENELUX".to_string(),
            official_name: None,
            abbreviated_name: Some("BENELUX".to_string()),
            territory_type: TerritoryType::EconomicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1948, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2111,
        TisTerritory {
            code: 2111,
            territory_name: "BRITISH ISLES".to_string(),
            official_name: None,
            abbreviated_name: Some("BRITISH ISLES".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2112,
        TisTerritory {
            code: 2112,
            territory_name: "BRITISH WEST INDIES".to_string(),
            official_name: None,
            abbreviated_name: Some("BRIT.WEST IND.".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2113,
        TisTerritory {
            code: 2113,
            territory_name: "CENTRAL AMERICA".to_string(),
            official_name: None,
            abbreviated_name: Some("CENTRAL AMERICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2114,
        TisTerritory {
            code: 2114,
            territory_name: "COMMONWEALTH".to_string(),
            official_name: None,
            abbreviated_name: Some("COMMONWEALTH".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1931, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2115,
        TisTerritory {
            code: 2115,
            territory_name: "COMMONWEALTH AFRICAN TERRITORIES".to_string(),
            official_name: None,
            abbreviated_name: Some("CO AFRICA".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1931, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2116,
        TisTerritory {
            code: 2116,
            territory_name: "COMMONWEALTH ASIAN TERRITORIES".to_string(),
            official_name: None,
            abbreviated_name: Some("CO ASIA".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1931, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2117,
        TisTerritory {
            code: 2117,
            territory_name: "COMMONWEALTH AUSTRALASIAN TERRITORIES".to_string(),
            official_name: None,
            abbreviated_name: Some("CO AUSTRALASIA".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1931, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2118,
        TisTerritory {
            code: 2118,
            territory_name: "COMMONWEALTH OF INDEPENDENT STATES".to_string(),
            official_name: None,
            abbreviated_name: Some("CIS".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1992, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2119,
        TisTerritory {
            code: 2119,
            territory_name: "EASTERN EUROPE".to_string(),
            official_name: None,
            abbreviated_name: Some("EASTERN EUROPE".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2120,
        TisTerritory {
            code: 2120,
            territory_name: "EUROPE".to_string(),
            official_name: None,
            abbreviated_name: Some("EUROPE".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2121,
        TisTerritory {
            code: 2121,
            territory_name: "EUROPEAN ECONOMIC AREA".to_string(),
            official_name: None,
            abbreviated_name: Some("EUROP.ECON.AREA".to_string()),
            territory_type: TerritoryType::EconomicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1994, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2122,
        TisTerritory {
            code: 2122,
            territory_name: "EUROPEAN CONTINENT".to_string(),
            official_name: None,
            abbreviated_name: Some("EUROPEAN CONT.".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2123,
        TisTerritory {
            code: 2123,
            territory_name: "EUROPEAN UNION".to_string(),
            official_name: None,
            abbreviated_name: Some("EUROPEAN UNION".to_string()),
            territory_type: TerritoryType::PoliticalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1958, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2124,
        TisTerritory {
            code: 2124,
            territory_name: "GSA COUNTRIES".to_string(),
            official_name: None,
            abbreviated_name: Some("GSA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2125,
        TisTerritory {
            code: 2125,
            territory_name: "MIDDLE EAST".to_string(),
            official_name: None,
            abbreviated_name: Some("MIDDLE EAST".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2126,
        TisTerritory {
            code: 2126,
            territory_name: "NAFTA COUNTRIES".to_string(),
            official_name: None,
            abbreviated_name: Some("NAFTA".to_string()),
            territory_type: TerritoryType::EconomicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1994, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2127,
        TisTerritory {
            code: 2127,
            territory_name: "NORDIC COUNTRIES".to_string(),
            official_name: None,
            abbreviated_name: Some("NORDIC COUNTR.".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2128,
        TisTerritory {
            code: 2128,
            territory_name: "NORTH AFRICA".to_string(),
            official_name: None,
            abbreviated_name: Some("NORTH AFRICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2129,
        TisTerritory {
            code: 2129,
            territory_name: "NORTH AMERICA".to_string(),
            official_name: None,
            abbreviated_name: Some("NORTH AMERICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2130,
        TisTerritory {
            code: 2130,
            territory_name: "OCEANIA".to_string(),
            official_name: None,
            abbreviated_name: Some("OCEANIA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2131,
        TisTerritory {
            code: 2131,
            territory_name: "SCANDINAVIA".to_string(),
            official_name: None,
            abbreviated_name: Some("SCANDINAVIA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2132,
        TisTerritory {
            code: 2132,
            territory_name: "SOUTH AMERICA".to_string(),
            official_name: None,
            abbreviated_name: Some("SOUTH AMERICA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2133,
        TisTerritory {
            code: 2133,
            territory_name: "SOUTH EAST ASIA".to_string(),
            official_name: None,
            abbreviated_name: Some("SOUTH EAST ASIA".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2134,
        TisTerritory {
            code: 2134,
            territory_name: "WEST INDIES".to_string(),
            official_name: None,
            abbreviated_name: Some("WEST INDIES".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );
    territories.insert(
        2136,
        TisTerritory {
            code: 2136,
            territory_name: "WORLD".to_string(),
            official_name: None,
            abbreviated_name: Some("WORLD".to_string()),
            territory_type: TerritoryType::GeographicalCountryGroup,
            validity_start: NaiveDate::from_ymd_opt(1000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            validity_end: NaiveDate::from_ymd_opt(3999, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            usable: true,
            remarks: None,
        },
    );

    territories
});

/// Validates a CISAC TIS numeric code (checks existence and current validity)
pub fn is_valid_tis_code(code: u16) -> bool {
    TIS_TERRITORIES.get(&code).is_some_and(|territory| territory.is_valid_at(None))
}

/// Validates a CISAC TIS numeric code for a specific date
pub fn is_valid_tis_code_at_date(code: u16, date: NaiveDateTime) -> bool {
    TIS_TERRITORIES.get(&code).is_some_and(|territory| territory.is_valid_at(Some(date)))
}

/// Gets the territory information for a TIS code
pub fn get_territory_info(code: u16) -> Option<&'static TisTerritory> {
    TIS_TERRITORIES.get(&code)
}

/// Gets the territory name for a TIS code (preferred display name)
pub fn get_territory_name(code: u16) -> Option<&'static str> {
    TIS_TERRITORIES.get(&code).map(|t| t.display_name())
}

/// Gets all currently valid TIS codes
pub fn get_all_valid_tis_codes() -> Vec<u16> {
    TIS_TERRITORIES.values().filter(|territory| territory.is_valid_at(None)).map(|territory| territory.code).collect()
}

/// Gets all TIS codes (including expired/invalid ones)
pub fn get_all_tis_codes() -> Vec<u16> {
    TIS_TERRITORIES.keys().copied().collect()
}

/// Checks if a TIS code exists in the territory table (regardless of usability)
pub fn territory_exists(code: u16) -> bool {
    TIS_TERRITORIES.contains_key(&code)
}
