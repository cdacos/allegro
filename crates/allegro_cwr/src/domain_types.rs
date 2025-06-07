//! Domain types for CWR field parsing

use chrono::{NaiveDate, NaiveTime};
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

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct WorksCount(pub u32);

impl WorksCount {
    pub fn as_str(&self) -> String {
        self.0.to_string()
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

impl CwrFieldParse for String {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        (source.trim().to_string(), vec![])
    }
}

impl CwrFieldParse for Option<String> {
    fn parse_cwr_field(source: &str, _field_name: &'static str, _field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() { (None, vec![]) } else { (Some(trimmed.to_string()), vec![]) }
    }
}

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

/// Sender type for HDR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum SenderType {
    #[default]
    Publisher,
    Society,
    Writer,
    AdministrativeAgency,
    /// For IPNN > 9 digits, this contains the leading digits
    NumericPrefix(String),
}

impl SenderType {
    pub fn as_str(&self) -> &str {
        match self {
            SenderType::Publisher => "PB",
            SenderType::Society => "SO",
            SenderType::Writer => "WR",
            SenderType::AdministrativeAgency => "AA",
            SenderType::NumericPrefix(s) => s,
        }
    }
}

impl CwrFieldParse for SenderType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "PB" => (SenderType::Publisher, vec![]),
            "SO" => (SenderType::Society, vec![]),
            "WR" => (SenderType::Writer, vec![]),
            "AA" => (SenderType::AdministrativeAgency, vec![]),
            s if s.chars().all(|c| c.is_ascii_digit()) && s.len() <= 2 => (SenderType::NumericPrefix(s.to_string()), vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid sender type '{}', must be PB, SO, WR, AA, or 2-digit numeric prefix", trimmed) }];
                (SenderType::Publisher, warnings)
            }
        }
    }
}

/// Sender ID with validation based on sender type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SenderId(pub String);

impl SenderId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldParse for SenderId {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        // Basic format validation - must be 9 characters numeric for IPI or alphanumeric for society codes
        // TODO: Implement table-based validation for:
        // - CWR Sender ID and Codes Table (for PB, AA, WR sender types)
        // - Society Code Table (for SO sender type)
        // This requires cross-field validation with SenderType in post_process step

        if trimmed.is_empty() {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: "Sender ID is required".to_string() }];
            return (SenderId(String::new()), warnings);
        }

        (SenderId(trimmed.to_string()), vec![])
    }
}

/// EDI Standard Version Number
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct EdiStandardVersion(pub String);

impl EdiStandardVersion {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldParse for EdiStandardVersion {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed == "01.10" {
            (EdiStandardVersion(trimmed.to_string()), vec![])
        } else {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("EDI Standard Version must be '01.10', got '{}'", trimmed) }];
            (EdiStandardVersion("01.10".to_string()), warnings)
        }
    }
}

/// CWR Version (2.2)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersion(pub Option<String>);

impl CwrVersion {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(v) => v.clone(),
            None => String::new(),
        }
    }
}

impl CwrFieldParse for CwrVersion {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (CwrVersion(None), vec![])
        } else if trimmed == "2.2" {
            (CwrVersion(Some(trimmed.to_string())), vec![])
        } else {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("CWR Version must be '2.2' if specified, got '{}'", trimmed) }];
            (CwrVersion(Some("2.2".to_string())), warnings)
        }
    }
}

/// CWR Revision Number
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrRevision(pub Option<u32>);

impl CwrRevision {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(r) => r.to_string(),
            None => String::new(),
        }
    }
}

impl CwrFieldParse for CwrRevision {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (CwrRevision(None), vec![])
        } else {
            match trimmed.parse::<u32>() {
                Ok(1) => (CwrRevision(Some(1)), vec![]),
                Ok(rev) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("CWR Revision number {} may not be valid, current valid value is 1", rev) }];
                    (CwrRevision(Some(rev)), warnings)
                }
                Err(_) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid revision number format: {}", trimmed) }];
                    (CwrRevision(Some(1)), warnings)
                }
            }
        }
    }
}

/// Time in HHMMSS format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Time(pub Option<NaiveTime>);

impl Time {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(time) => time.format("%H%M%S").to_string(),
            None => String::new(),
        }
    }
}

impl CwrFieldParse for Time {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.len() != 6 {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Time should be 6 characters HHMMSS, got {}", trimmed.len()) }];
            return (Time(None), warnings);
        }

        match NaiveTime::parse_from_str(trimmed, "%H%M%S") {
            Ok(time) => (Time(Some(time)), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid time format: {}", trimmed) }];
                (Time(None), warnings)
            }
        }
    }
}

// TODO: Implement additional domain types for table-based validations:
//
// 1. CharacterSet enum with validation against character set lookup table:
//    - Traditional [Big5], Simplified [GB], UTF-8, and Unicode values
//    - Reference: http://www.unicode.org/charts
//
// 2. SenderName with cross-validation against lookup tables:
//    - For PB (Publisher): must match name in CWR Sender ID and Codes Table
//    - For SO (Society): must match name in Society Code Table
//    - For AA (Administrative Agency): must match name in Publisher Code Table
//
// 3. Complex Sender validation requiring post_process step:
//    - Cross-validate SenderType + SenderId + SenderName combination
//    - Handle IPNN > 9 digits case (SenderType numeric prefix + SenderId)
//    - Validate against appropriate lookup tables based on sender type
//
// 4. CwrRevision enum with validation against version number lookup table:
//    - Current valid values for CWR 2.2 revision numbers
//    - Should be extensible for future revisions
