//! Time types for CWR parsing

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use chrono::{NaiveTime, Timelike};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Time(pub NaiveTime);

impl Default for Time {
    fn default() -> Self {
        Time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }
}

impl Time {
    pub fn as_str(&self) -> String {
        self.0.format("%H%M%S").to_string()
    }

    pub fn duration_since_midnight(&self) -> f32 {
        (self.0.hour() * 3600 + self.0.minute() * 60 + self.0.second()) as f32
    }
}

impl CwrFieldWrite for Time {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for Time {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.len() != 6 {
            let warnings = vec![CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Time should be 6 characters HHMMSS, got {}", trimmed.len()),
            }];
            return (Time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()), warnings);
        }

        match NaiveTime::parse_from_str(trimmed, "%H%M%S") {
            Ok(time) => (Time(time), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid time format: {}", trimmed),
                }];
                (Time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Time> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed.chars().all(|c| c.is_whitespace()) {
            (None, vec![])
        } else {
            let (parsed_time, warnings) = Time::parse_cwr_field(source, field_name, field_title);
            (Some(parsed_time), warnings)
        }
    }
}
