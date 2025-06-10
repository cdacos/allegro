//! Identifier and code types for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Sender ID with validation based on sender type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SenderId(pub String);

impl SenderId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for SenderId {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for SenderId {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::society_codes::is_valid_society_code;
        use crate::lookups::society_members::is_valid_transmitter_code;

        let trimmed = source.trim();

        if trimmed.is_empty() {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: "Sender ID is required".to_string() }];
            return (SenderId(String::new()), warnings);
        }

        let mut warnings = vec![];

        // Basic validation - for full validation we need SenderType context
        // This is a preliminary validation, full validation happens in post_process

        // Check if it looks like a society code (alpha characters)
        if trimmed.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()) {
            if !is_valid_society_code(trimmed) {
                warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Sender ID '{}' not found in society codes table - may be invalid for SO sender type", trimmed) });
            }
        }
        // Check if it looks like a transmitter code (alphanumeric, typically 3-4 chars)
        else if trimmed.len() <= 4 && trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
            if !is_valid_transmitter_code(trimmed) {
                warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Info, description: format!("Sender ID '{}' not found in transmitter codes table - may be a custom code", trimmed) });
            }
        }
        // Check if it looks like an IPI number (9+ digits)
        else if trimmed.len() >= 9 && trimmed.chars().all(|c| c.is_ascii_digit()) {
            // IPI number format validation - should be 9-11 digits
            if trimmed.len() > 11 {
                warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("IPI number '{}' is longer than standard 11 digits", trimmed) });
            }
        }

        (SenderId(trimmed.to_string()), warnings)
    }
}

/// Sender name with validation
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct SenderName(pub String);

impl SenderName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for SenderName {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for SenderName {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();

        if trimmed.is_empty() {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: "Sender name is required".to_string() }];
            return (SenderName(String::new()), warnings);
        }

        // Basic length validation
        let mut warnings = vec![];
        if trimmed.len() > 45 {
            warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Sender name '{}' exceeds maximum length of 45 characters", trimmed) });
        }

        (SenderName(trimmed.to_string()), warnings)
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

impl CwrFieldWrite for EdiStandardVersion {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for EdiStandardVersion {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed == "01.10" {
            (EdiStandardVersion(trimmed.to_string()), vec![])
        } else {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid EDI standard version '{}', must be '01.10'", trimmed) }];
            (EdiStandardVersion("01.10".to_string()), warnings)
        }
    }
}
