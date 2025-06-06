//! Domain types for CWR field parsing

use chrono::NaiveDate;
use std::borrow::Cow;

/// Warning levels for CWR parsing
#[derive(Debug, Clone, PartialEq)]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// Warning generated during CWR parsing
#[derive(Debug, Clone, PartialEq)]
pub struct CwrWarning<'a> {
    pub field_name: &'static str,
    pub field_title: &'static str,
    pub source_str: Cow<'a, str>,
    pub level: WarningLevel,
    pub description: String,
}

impl CwrWarning<'_> {
    pub fn is_critical(&self) -> bool {
        matches!(self.level, WarningLevel::Critical)
    }
}

/// Trait for parsing CWR fields with warnings
pub trait CwrFieldParse: Sized + Default {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>);
}

// Domain types

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct WorksCount(pub u32);

impl WorksCount {
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordType {
    Agr,
    Ack,
    Alt,
    Ari,
    Com,
    Ewt,
    Grh,
    Grt,
    #[default]
    Hdr,
    Ind,
    Ins,
    Ipa,
    Msg,
    Nat,
    Net,
    Now,
    Npa,
    Npn,
    Npr,
    Nwr,
    Nwn,
    Orn,
    Per,
    Pwr,
    Rec,
    Spt,
    Spu,
    Swr,
    Swt,
    Ter,
    Trl,
    Ver,
    Xrf,
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RecordType::Agr => "AGR",
            RecordType::Ack => "ACK",
            RecordType::Alt => "ALT",
            RecordType::Ari => "ARI",
            RecordType::Com => "COM",
            RecordType::Ewt => "EWT",
            RecordType::Grh => "GRH",
            RecordType::Grt => "GRT",
            RecordType::Hdr => "HDR",
            RecordType::Ind => "IND",
            RecordType::Ins => "INS",
            RecordType::Ipa => "IPA",
            RecordType::Msg => "MSG",
            RecordType::Nat => "NAT",
            RecordType::Net => "NET",
            RecordType::Now => "NOW",
            RecordType::Npa => "NPA",
            RecordType::Npn => "NPN",
            RecordType::Npr => "NPR",
            RecordType::Nwr => "NWR",
            RecordType::Nwn => "NWN",
            RecordType::Orn => "ORN",
            RecordType::Per => "PER",
            RecordType::Pwr => "PWR",
            RecordType::Rec => "REC",
            RecordType::Spt => "SPT",
            RecordType::Spu => "SPU",
            RecordType::Swr => "SWR",
            RecordType::Swt => "SWT",
            RecordType::Ter => "TER",
            RecordType::Trl => "TRL",
            RecordType::Ver => "VER",
            RecordType::Xrf => "XRF",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum YesNo {
    Yes,
    #[default]
    No,
}

impl YesNo {
    pub fn as_str(&self) -> &str {
        match self {
            YesNo::Yes => "Y",
            YesNo::No => "N",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Date(pub Option<NaiveDate>);

impl Date {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(date) => date.format("%Y%m%d").to_string(),
            None => String::new(),
        }
    }
}


// Implement CwrFieldParse for String (default case)
impl CwrFieldParse for String {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        (source.trim().to_string(), vec![])
    }
}

// Implement for Option<String>
impl CwrFieldParse for Option<String> {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() { (None, vec![]) } else { (Some(trimmed.to_string()), vec![]) }
    }
}

// Implement CwrFieldParse for RecordType
impl CwrFieldParse for RecordType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let record_type = match trimmed {
            "AGR" => RecordType::Agr,
            "ACK" => RecordType::Ack,
            "ALT" => RecordType::Alt,
            "ARI" => RecordType::Ari,
            "COM" => RecordType::Com,
            "EWT" => RecordType::Ewt,
            "GRH" => RecordType::Grh,
            "GRT" => RecordType::Grt,
            "HDR" => RecordType::Hdr,
            "IND" => RecordType::Ind,
            "INS" => RecordType::Ins,
            "IPA" => RecordType::Ipa,
            "MSG" => RecordType::Msg,
            "NAT" => RecordType::Nat,
            "NET" | "NCT" | "NVT" => RecordType::Net,
            "NOW" => RecordType::Now,
            "NPA" => RecordType::Npa,
            "NPN" => RecordType::Npn,
            "NPR" => RecordType::Npr,
            "NWR" | "REV" | "ISW" | "EXC" => RecordType::Nwr,
            "NWN" => RecordType::Nwn,
            "ORN" => RecordType::Orn,
            "PER" => RecordType::Per,
            "PWR" => RecordType::Pwr,
            "REC" => RecordType::Rec,
            "SPT" | "OPT" => RecordType::Spt,
            "SPU" | "OPU" => RecordType::Spu,
            "SWR" | "OWR" => RecordType::Swr,
            "SWT" | "OWT" => RecordType::Swt,
            "TER" => RecordType::Ter,
            "TRL" => RecordType::Trl,
            "VER" => RecordType::Ver,
            "XRF" => RecordType::Xrf,
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Unknown record type: {}", trimmed) }];
                return (RecordType::default(), warnings);
            }
        };
        (record_type, vec![])
    }
}

// Implement CwrFieldParse for YesNo
impl CwrFieldParse for YesNo {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (YesNo::Yes, vec![]),
            "N" => (YesNo::No, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid Yes/No value '{}', defaulting to No", trimmed) }];
                (YesNo::No, warnings)
            }
        }
    }
}

// Implement CwrFieldParse for Date
impl CwrFieldParse for Date {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.len() != 8 {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Date should be 8 characters YYYYMMDD, got {}", trimmed.len()) }];
            return (Date(None), warnings);
        }

        match NaiveDate::parse_from_str(trimmed, "%Y%m%d") {
            Ok(date) => (Date(Some(date)), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid date format: {}", trimmed) }];
                (Date(None), warnings)
            }
        }
    }
}

// Implement CwrFieldParse for Option<Date>
impl CwrFieldParse for Option<Date> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000000" {
            (None, vec![])
        } else {
            let (date, warnings) = Date::parse_cwr_field(source, field_name, field_title);
            (Some(date), warnings)
        }
    }
}

// Implement CwrFieldParse for WorksCount
impl CwrFieldParse for WorksCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(count) if (1..=99999).contains(&count) => (WorksCount(count), vec![]),
            Ok(count) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Works count {} outside valid range 1-99999", count) }];
                (WorksCount(count.clamp(1, 99999)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid number format: {}", trimmed) }];
                (WorksCount(0), warnings)
            }
        }
    }
}
