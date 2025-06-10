//! Date types for CWR parsing

use super::common::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use chrono::NaiveDate;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Date(pub Option<NaiveDate>);

impl Date {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(date) => date.format("%Y%m%d").to_string(),
            None => "00000000".to_string(),
        }
    }
}

impl CwrFieldWrite for Date {
    fn to_cwr_str(&self) -> String {
        self.as_str()
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
        if trimmed.is_empty() {
            (None, vec![])
        } else if trimmed == "00000000" {
            (Some(Date(None)), vec![])
        } else {
            let (date, warnings) = Date::parse_cwr_field(source, field_name, field_title);
            (Some(date), warnings)
        }
    }
}
