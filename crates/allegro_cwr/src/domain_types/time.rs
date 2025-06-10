//! Time types for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use chrono::{NaiveTime, Timelike};
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

    pub fn duration_since_midnight(&self) -> Option<f32> {
        self.0.map(|time| (time.hour() * 3600 + time.minute() * 60 + time.second()) as f32)
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

impl CwrFieldParse for Option<Time> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "000000" {
            (None, vec![])
        } else {
            let (parsed_time, warnings) = Time::parse_cwr_field(source, field_name, field_title);
            (Some(parsed_time), warnings)
        }
    }
}
