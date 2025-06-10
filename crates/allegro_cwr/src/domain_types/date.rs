//! Date types for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use chrono::NaiveDate;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Date(pub NaiveDate);

impl Default for Date {
    fn default() -> Self {
        Date(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
    }
}

impl Date {
    pub fn as_str(&self) -> String {
        self.0.format("%Y%m%d").to_string()
    }

    /// Convert to Unix timestamp (seconds since epoch) at midnight UTC
    pub fn to_timestamp(&self) -> i64 {
        self.0.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()
    }
}

impl CwrFieldWrite for Date {
    fn to_cwr_str(&self, _width: usize) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for Date {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.len() != 8 {
            let warnings = vec![CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Date should be 8 characters YYYYMMDD, got {}", trimmed.len()),
            }];
            return (Date(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()), warnings);
        }

        match NaiveDate::parse_from_str(trimmed, "%Y%m%d") {
            Ok(date) => (Date(date), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid date format: {}", trimmed),
                }];
                (Date(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Date> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000000" {
            (None, vec![])
        } else {
            let (date, warnings) = Date::parse_cwr_field(source, field_name, field_title);
            (Some(date), warnings)
        }
    }
}
