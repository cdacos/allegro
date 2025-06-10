//! Time and duration types for CWR parsing

use super::common::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use chrono::NaiveTime;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Time(pub Option<NaiveTime>);

impl Time {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(time) => time.format("%H%M%S").to_string(),
            None => "000000".to_string(),
        }
    }
}

impl CwrFieldWrite for Time {
    fn to_cwr_str(&self) -> String {
        self.as_str()
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

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Duration(pub Option<NaiveTime>);

impl Duration {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(time) => time.format("%H%M%S").to_string(),
            None => "000000".to_string(),
        }
    }
}

impl CwrFieldWrite for Duration {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for Option<Duration> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "000000" {
            (None, vec![])
        } else {
            if trimmed.len() != 6 {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Duration should be 6 characters HHMMSS, got {}", trimmed.len()) }];
                return (Some(Duration(None)), warnings);
            }

            match NaiveTime::parse_from_str(trimmed, "%H%M%S") {
                Ok(time) => (Some(Duration(Some(time))), vec![]),
                Err(_) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid duration format: {}", trimmed) }];
                    (Some(Duration(None)), warnings)
                }
            }
        }
    }
}
