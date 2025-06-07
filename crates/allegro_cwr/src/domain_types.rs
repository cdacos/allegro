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
