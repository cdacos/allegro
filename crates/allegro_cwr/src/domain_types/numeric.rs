//! Numeric types for CWR parsing

use super::common::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// General numeric field for sequence numbers and counts
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Number(pub u32);

impl Number {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CwrFieldWrite for Number {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for Number {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (Number(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid number format: {}", trimmed) }];
                (Number(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<Number> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000000" {
            (None, vec![])
        } else {
            let (number, warnings) = Number::parse_cwr_field(source, field_name, field_title);
            (Some(number), warnings)
        }
    }
}

/// CWR revision number (v2.2+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrRevision(pub u32);

impl CwrRevision {
    pub fn as_str(&self) -> String {
        format!("{:03}", self.0)
    }
}

impl CwrFieldWrite for CwrRevision {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for CwrRevision {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (CwrRevision(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid revision number format: {}", trimmed) }];
                (CwrRevision(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CwrRevision> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (revision, warnings) = CwrRevision::parse_cwr_field(source, field_name, field_title);
            (Some(revision), warnings)
        }
    }
}

/// CWR version number (v2.2+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersion(pub f32);

impl CwrVersion {
    pub fn as_str(&self) -> String {
        format!("{:.1}", self.0)
    }

    pub fn supports_version(&self, min_version: f32) -> bool {
        self.0 >= min_version
    }
}

impl CwrFieldWrite for CwrVersion {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for CwrVersion {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<f32>() {
            Ok(version) => (CwrVersion(version), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid version number format: {}", trimmed) }];
                (CwrVersion(2.1), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CwrVersion> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (version, warnings) = CwrVersion::parse_cwr_field(source, field_name, field_title);
            (Some(version), warnings)
        }
    }
}

/// CWR version number for GRH record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersionNumber(pub String);

impl CwrVersionNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for CwrVersionNumber {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for CwrVersionNumber {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        // Validate version format (should be like "02.10", "02.20", etc.)
        if !trimmed.matches('.').count() == 1 || trimmed.len() != 5 {
            warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid version format '{}', expected format like '02.10'", trimmed) });
        }

        (CwrVersionNumber(trimmed.to_string()), warnings)
    }
}

/// Group ID for GRH/GRT records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupId(pub u32);

impl GroupId {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldWrite for GroupId {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for GroupId {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (GroupId(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid group ID format: {}", trimmed) }];
                (GroupId(0), warnings)
            }
        }
    }
}

/// Publisher sequence number
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PublisherSequenceNumber(pub u8);

impl PublisherSequenceNumber {
    pub fn as_str(&self) -> String {
        format!("{:02}", self.0)
    }
}

impl CwrFieldWrite for PublisherSequenceNumber {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for PublisherSequenceNumber {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u8>() {
            Ok(num) if num > 0 && num <= 99 => (PublisherSequenceNumber(num), vec![]),
            Ok(num) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Publisher sequence number {} out of valid range 1-99", num) }];
                (PublisherSequenceNumber(1), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid publisher sequence number format: {}", trimmed) }];
                (PublisherSequenceNumber(1), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<PublisherSequenceNumber> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00" {
            (None, vec![])
        } else {
            let (seq_num, warnings) = PublisherSequenceNumber::parse_cwr_field(source, field_name, field_title);
            (Some(seq_num), warnings)
        }
    }
}

/// Ownership share (0-100.00% represented as 0-10000)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct OwnershipShare(pub u16);

impl OwnershipShare {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }

    pub fn as_percentage(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}

impl CwrFieldWrite for OwnershipShare {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for OwnershipShare {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) if num <= 10000 => (OwnershipShare(num), vec![]),
            Ok(num) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Ownership share {} exceeds maximum 10000 (100.00%)", num) }];
                (OwnershipShare(0), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid ownership share format: {}", trimmed) }];
                (OwnershipShare(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<OwnershipShare> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000" {
            (None, vec![])
        } else {
            let (share, warnings) = OwnershipShare::parse_cwr_field(source, field_name, field_title);
            (Some(share), warnings)
        }
    }
}

/// Transaction count for GRT/TRL records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TransactionCount(pub u32);

impl TransactionCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldWrite for TransactionCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for TransactionCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (TransactionCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid transaction count format: {}", trimmed) }];
                (TransactionCount(0), warnings)
            }
        }
    }
}

/// Record count for GRT/TRL records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordCount(pub u32);

impl RecordCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldWrite for RecordCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for RecordCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (RecordCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid record count format: {}", trimmed) }];
                (RecordCount(0), warnings)
            }
        }
    }
}

/// ISO 4217 currency code
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for CurrencyCode {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for CurrencyCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::currency_codes::is_valid_currency_code;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_currency_code(&trimmed) {
            warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Currency code '{}' not found in ISO 4217 table", trimmed) });
        }

        (CurrencyCode(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<CurrencyCode> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (currency, warnings) = CurrencyCode::parse_cwr_field(source, field_name, field_title);
            (Some(currency), warnings)
        }
    }
}

/// Group count for TRL record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupCount(pub u32);

impl GroupCount {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldWrite for GroupCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for GroupCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (GroupCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid group count format: {}", trimmed) }];
                (GroupCount(0), warnings)
            }
        }
    }
}

/// Works count for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct WorksCount(pub u32);

impl WorksCount {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldWrite for WorksCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for WorksCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(num) => (WorksCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid works count format: {}", trimmed) }];
                (WorksCount(0), warnings)
            }
        }
    }
}

/// Composite component count for NWR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CompositeComponentCount(pub u16);

impl CompositeComponentCount {
    pub fn as_str(&self) -> String {
        format!("{:03}", self.0)
    }
}

impl CwrFieldWrite for CompositeComponentCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for CompositeComponentCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) => (CompositeComponentCount(num), vec![]),
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid composite component count format: {}", trimmed) }];
                (CompositeComponentCount(0), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CompositeComponentCount> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "000" {
            (None, vec![])
        } else {
            let (count, warnings) = CompositeComponentCount::parse_cwr_field(source, field_name, field_title);
            (Some(count), warnings)
        }
    }
}

/// TIS numeric code for territory records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TisNumericCode(pub u16);

impl TisNumericCode {
    pub fn as_str(&self) -> String {
        format!("{:04}", self.0)
    }
}

impl CwrFieldWrite for TisNumericCode {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for TisNumericCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::tis_codes::is_valid_tis_code;

        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) => {
                let code_str = format!("{:04}", num);
                let mut warnings = vec![];

                if !is_valid_tis_code(num) {
                    warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("TIS code '{}' not found in territory table", code_str) });
                }

                (TisNumericCode(num), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid TIS numeric code format: {}", trimmed) }];
                (TisNumericCode(0), warnings)
            }
        }
    }
}
