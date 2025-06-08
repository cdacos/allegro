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

/// Trait for converting CWR fields to their string representation for writing
pub trait CwrFieldWrite {
    fn to_cwr_str(&self) -> String;
}

// Implementations for basic types
impl CwrFieldWrite for String {
    fn to_cwr_str(&self) -> String {
        self.clone()
    }
}

impl<T: CwrFieldWrite> CwrFieldWrite for Option<T> {
    fn to_cwr_str(&self) -> String {
        match self {
            Some(val) => val.to_cwr_str(),
            None => String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct WorksCount(pub u32);

impl WorksCount {
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl CwrFieldWrite for WorksCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
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

impl CwrFieldWrite for YesNo {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
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

impl CwrFieldWrite for Date {
    fn to_cwr_str(&self) -> String {
        self.as_str()
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

impl CwrFieldWrite for SenderType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
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
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("EDI Standard Version must be '01.10', got '{}'", trimmed) }];
            (EdiStandardVersion("01.10".to_string()), warnings)
        }
    }
}

/// CWR Version (2.2)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CwrVersion(pub Option<f32>);

impl CwrVersion {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(v) => format!("{:.1}", v),
            None => String::new(),
        }
    }

    pub fn supports_version(&self, min_version: &str) -> bool {
        let current_version = self.0.unwrap_or(2.0);
        let min_version_float = min_version.parse::<f32>().unwrap_or(2.0);
        current_version >= min_version_float
    }
}

impl CwrFieldParse for CwrVersion {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (CwrVersion(None), vec![])
        } else if trimmed == "2.2" {
            (CwrVersion(Some(2.2)), vec![])
        } else {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("CWR Version must be '2.2' if specified, got '{}'", trimmed) }];
            (CwrVersion(Some(2.2)), warnings)
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

impl CwrFieldWrite for CwrRevision {
    fn to_cwr_str(&self) -> String {
        self.as_str()
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

/// Character Set for CWR files (v2.1+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum CharacterSet {
    #[default]
    Ascii,
    TraditionalBig5,
    SimplifiedGb,
    Utf8,
    Unicode,
}

impl CharacterSet {
    pub fn as_str(&self) -> &str {
        match self {
            CharacterSet::Ascii => "",
            CharacterSet::TraditionalBig5 => "Traditional [Big5]",
            CharacterSet::SimplifiedGb => "Simplified [GB]",
            CharacterSet::Utf8 => "UTF-8",
            CharacterSet::Unicode => "Unicode",
        }
    }
}

impl CwrFieldWrite for CharacterSet {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for Option<CharacterSet> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            match trimmed {
                "Traditional [Big5]" => (Some(CharacterSet::TraditionalBig5), vec![]),
                "Simplified [GB]" => (Some(CharacterSet::SimplifiedGb), vec![]),
                "UTF-8" => (Some(CharacterSet::Utf8), vec![]),
                "Unicode" => (Some(CharacterSet::Unicode), vec![]),
                _ => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid character set '{}', must be 'Traditional [Big5]', 'Simplified [GB]', 'UTF-8', or 'Unicode'", trimmed) }];
                    (Some(CharacterSet::Ascii), warnings)
                }
            }
        }
    }
}

/// Transaction Type for GRH records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TransactionType {
    #[default]
    Nwr,
    Rev,
    Agr,
    Ack,
    Isw,
    Exc,
}

impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionType::Nwr => "NWR",
            TransactionType::Rev => "REV",
            TransactionType::Agr => "AGR",
            TransactionType::Ack => "ACK",
            TransactionType::Isw => "ISW",
            TransactionType::Exc => "EXC",
        }
    }
}

impl CwrFieldWrite for TransactionType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for TransactionType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "NWR" => (TransactionType::Nwr, vec![]),
            "REV" => (TransactionType::Rev, vec![]),
            "AGR" => (TransactionType::Agr, vec![]),
            "ACK" => (TransactionType::Ack, vec![]),
            "ISW" => (TransactionType::Isw, vec![]),
            "EXC" => (TransactionType::Exc, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid transaction type '{}', must be NWR, REV, AGR, ACK, ISW, or EXC", trimmed) }];
                (TransactionType::Nwr, warnings)
            }
        }
    }
}

/// Group ID for sequentially numbered groups
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupId(pub u32);

impl GroupId {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldParse for GroupId {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(id) if (1..=99999).contains(&id) => (GroupId(id), vec![]),
            Ok(id) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Group ID {} outside valid range 1-99999", id) }];
                (GroupId(id.clamp(1, 99999)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid group ID format: {}", trimmed) }];
                (GroupId(1), warnings)
            }
        }
    }
}

/// CWR Version Number (e.g., "02.20" for v2.2)
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
        match trimmed {
            "02.20" => (CwrVersionNumber(trimmed.to_string()), vec![]),
            "02.10" => (CwrVersionNumber(trimmed.to_string()), vec![]),
            "02.00" => (CwrVersionNumber(trimmed.to_string()), vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid CWR version number '{}', expected format like '02.20'", trimmed) }];
                (CwrVersionNumber("02.20".to_string()), warnings)
            }
        }
    }
}

/// Publisher sequence number (must be sequential starting from 1)
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
            Ok(num) if (1..=99).contains(&num) => (PublisherSequenceNumber(num), vec![]),
            Ok(num) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Publisher sequence number {} outside valid range 1-99", num) }];
                (PublisherSequenceNumber(num.clamp(1, 99)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid publisher sequence number format: {}", trimmed) }];
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

/// Ownership share percentage (0-100.00% stored as 0-10000)
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

impl CwrFieldParse for Option<OwnershipShare> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00000" {
            (None, vec![])
        } else {
            match trimmed.parse::<u16>() {
                Ok(share) if share <= 10000 => (Some(OwnershipShare(share)), vec![]),
                Ok(share) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Ownership share {} exceeds maximum 100.00% (10000)", share) }];
                    (Some(OwnershipShare(share.min(10000))), warnings)
                }
                Err(_) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid ownership share format: {}", trimmed) }];
                    (None, warnings)
                }
            }
        }
    }
}

/// Transaction count for group/file trailers
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TransactionCount(pub u32);

impl TransactionCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldParse for TransactionCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(count) if count <= 99999999 => (TransactionCount(count), vec![]),
            Ok(count) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Transaction count {} exceeds maximum 99999999", count) }];
                (TransactionCount(count.min(99999999)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid transaction count format: {}", trimmed) }];
                (TransactionCount(0), warnings)
            }
        }
    }
}

/// Record count for group/file trailers
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordCount(pub u32);

impl RecordCount {
    pub fn as_str(&self) -> String {
        format!("{:08}", self.0)
    }
}

impl CwrFieldParse for RecordCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(count) if count <= 99999999 => (RecordCount(count), vec![]),
            Ok(count) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Record count {} exceeds maximum 99999999", count) }];
                (RecordCount(count.min(99999999)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid record count format: {}", trimmed) }];
                (RecordCount(0), warnings)
            }
        }
    }
}

/// ISO 4217 currency code
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CurrencyCode(pub Option<String>);

impl CurrencyCode {
    pub fn as_str(&self) -> String {
        self.0.as_deref().unwrap_or("").to_string()
    }
}

impl CwrFieldWrite for CurrencyCode {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}
impl CwrFieldParse for CurrencyCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::currency_codes::is_valid_currency_code;

        let trimmed = source.trim();
        if trimmed.is_empty() {
            (CurrencyCode(None), vec![])
        } else if trimmed.len() == 3 && trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
            let uppercase_code = trimmed.to_uppercase();
            if is_valid_currency_code(&uppercase_code) {
                (CurrencyCode(Some(uppercase_code)), vec![])
            } else {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Currency code '{}' not found in ISO 4217 table", trimmed) }];
                (CurrencyCode(Some(uppercase_code)), warnings)
            }
        } else {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid currency code '{}', should be 3-letter ISO 4217 code", trimmed) }];
            (CurrencyCode(Some(trimmed.to_uppercase())), warnings)
        }
    }
}

/// Group count for file trailers (5 digits, so we use u32)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupCount(pub u32);

impl GroupCount {
    pub fn as_str(&self) -> String {
        format!("{:05}", self.0)
    }
}

impl CwrFieldParse for GroupCount {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u32>() {
            Ok(count) if count <= 99999 => (GroupCount(count), vec![]),
            Ok(count) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Group count {} exceeds maximum 99999", count) }];
                (GroupCount(count.min(99999)), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid group count format: {}", trimmed) }];
                (GroupCount(0), warnings)
            }
        }
    }
}

/// Prior royalty status for AGR records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PriorRoyaltyStatus {
    #[default]
    None, // N - No entitlement to prior royalties
    Acquired,   // A - Entitlement to all prior royalties
    Designated, // D - Entitlement to prior royalties from designated date
}

impl PriorRoyaltyStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PriorRoyaltyStatus::None => "N",
            PriorRoyaltyStatus::Acquired => "A",
            PriorRoyaltyStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PriorRoyaltyStatus {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for PriorRoyaltyStatus {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PriorRoyaltyStatus::None, vec![]),
            "A" => (PriorRoyaltyStatus::Acquired, vec![]),
            "D" => (PriorRoyaltyStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid prior royalty status '{}', must be N, A, or D", trimmed) }];
                (PriorRoyaltyStatus::None, warnings)
            }
        }
    }
}

/// Post-term collection status for AGR records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PostTermCollectionStatus {
    #[default]
    None, // N - No post-term collection
    Original,   // O - Original agreement terms
    Designated, // D - Designated end date
}

impl PostTermCollectionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PostTermCollectionStatus::None => "N",
            PostTermCollectionStatus::Original => "O",
            PostTermCollectionStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PostTermCollectionStatus {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for PostTermCollectionStatus {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PostTermCollectionStatus::None, vec![]),
            "O" => (PostTermCollectionStatus::Original, vec![]),
            "D" => (PostTermCollectionStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid post-term collection status '{}', must be N, O, or D", trimmed) }];
                (PostTermCollectionStatus::None, warnings)
            }
        }
    }
}

/// Duration in HHMMSS format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Duration(pub Option<chrono::NaiveTime>);

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
        } else if trimmed.len() != 6 {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Duration should be 6 characters HHMMSS, got {}", trimmed.len()) }];
            (None, warnings)
        } else {
            match chrono::NaiveTime::parse_from_str(trimmed, "%H%M%S") {
                Ok(time) => (Some(Duration(Some(time))), vec![]),
                Err(_) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid duration format: {}", trimmed) }];
                    (None, warnings)
                }
            }
        }
    }
}

/// Flag type for Y/N/U (Yes/No/Unknown) values
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum FlagYNU {
    #[default]
    Unknown,
    Yes,
    No,
}

impl FlagYNU {
    pub fn as_str(&self) -> &str {
        match self {
            FlagYNU::Yes => "Y",
            FlagYNU::No => "N",
            FlagYNU::Unknown => "U",
        }
    }
}

impl CwrFieldWrite for FlagYNU {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for FlagYNU {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (FlagYNU::Yes, vec![]),
            "N" => (FlagYNU::No, vec![]),
            "U" => (FlagYNU::Unknown, vec![]),
            "" => (FlagYNU::Unknown, vec![]), // Default for empty
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid flag value '{}', must be Y, N, or U", trimmed) }];
                (FlagYNU::Unknown, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<FlagYNU> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (flag, warnings) = FlagYNU::parse_cwr_field(source, field_name, field_title);
            (Some(flag), warnings)
        }
    }
}

/// Composite component count (for ASCAP composite works)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct CompositeComponentCount(pub Option<u16>);

impl CompositeComponentCount {
    pub fn as_str(&self) -> String {
        match &self.0 {
            Some(count) => format!("{:03}", count),
            None => "000".to_string(),
        }
    }
}

impl CwrFieldParse for Option<CompositeComponentCount> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "000" {
            (None, vec![])
        } else {
            match trimmed.parse::<u16>() {
                Ok(count) if count > 0 && count <= 999 => (Some(CompositeComponentCount(Some(count))), vec![]),
                Ok(count) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Composite component count {} outside valid range 1-999", count) }];
                    (Some(CompositeComponentCount(Some(count.clamp(1, 999)))), warnings)
                }
                Err(_) => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid composite component count format: {}", trimmed) }];
                    (None, warnings)
                }
            }
        }
    }
}

/// Inclusion/Exclusion indicator for territory records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum InclusionExclusionIndicator {
    #[default]
    Included,
    Excluded,
}

impl InclusionExclusionIndicator {
    pub fn as_str(&self) -> &str {
        match self {
            InclusionExclusionIndicator::Included => "I",
            InclusionExclusionIndicator::Excluded => "E",
        }
    }
}

impl CwrFieldWrite for InclusionExclusionIndicator {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for InclusionExclusionIndicator {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "I" => (InclusionExclusionIndicator::Included, vec![]),
            "E" => (InclusionExclusionIndicator::Excluded, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid inclusion/exclusion indicator '{}', must be I (Included) or E (Excluded)", trimmed) }];
                (InclusionExclusionIndicator::Included, warnings)
            }
        }
    }
}

/// TIS (Territory Information System) numeric code
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TisNumericCode(pub u16);

impl TisNumericCode {
    pub fn as_str(&self) -> String {
        format!("{:04}", self.0)
    }
}

impl CwrFieldParse for TisNumericCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::tis_codes::is_valid_tis_code;

        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(code) if code > 0 => {
                let mut warnings = vec![];
                if !is_valid_tis_code(code) {
                    warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("TIS Numeric Code {} not found in official CISAC TIS territory codes table", code) });
                }
                (TisNumericCode(code), warnings)
            }
            Ok(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: "TIS Numeric Code must be greater than 0".to_string() }];
                (TisNumericCode(1), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid TIS Numeric Code format: {}", trimmed) }];
                (TisNumericCode(1), warnings)
            }
        }
    }
}

/// Agreement role code for IPA records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AgreementRoleCode {
    #[default]
    Assignor,
    Acquirer,
}

impl AgreementRoleCode {
    pub fn as_str(&self) -> &str {
        match self {
            AgreementRoleCode::Assignor => "AS",
            AgreementRoleCode::Acquirer => "AC",
        }
    }
}

impl CwrFieldWrite for AgreementRoleCode {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for AgreementRoleCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::agreement_role_codes::is_valid_agreement_role_code;

        let trimmed = source.trim();
        match trimmed {
            "AS" => (AgreementRoleCode::Assignor, vec![]),
            "AC" => (AgreementRoleCode::Acquirer, vec![]),
            _ => {
                let mut warnings = vec![];
                if !is_valid_agreement_role_code(trimmed) {
                    warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid agreement role code '{}', must be AS (Assignor) or AC (Acquirer)", trimmed) });
                }
                (AgreementRoleCode::Assignor, warnings)
            }
        }
    }
}

/// Title type for alternate title records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TitleType {
    #[default]
    AlternateTitle, // AT - Alternate Title
    TransliterationAlt,   // AL - Transliterated Alternate Title
    TranslatedTitle,      // TT - Translated Title
    TransliterationTrans, // TL - Transliterated Translated Title
    FictionalTitle,       // FT - Fictional Title (v2.1+)
    OriginalTitle,        // OT - Original Title (v2.1+)
}

impl TitleType {
    pub fn as_str(&self) -> &str {
        match self {
            TitleType::AlternateTitle => "AT",
            TitleType::TransliterationAlt => "AL",
            TitleType::TranslatedTitle => "TT",
            TitleType::TransliterationTrans => "TL",
            TitleType::FictionalTitle => "FT",
            TitleType::OriginalTitle => "OT",
        }
    }
}

impl CwrFieldWrite for TitleType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for TitleType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "AT" => (TitleType::AlternateTitle, vec![]),
            "AL" => (TitleType::TransliterationAlt, vec![]),
            "TT" => (TitleType::TranslatedTitle, vec![]),
            "TL" => (TitleType::TransliterationTrans, vec![]),
            "FT" => (TitleType::FictionalTitle, vec![]),
            "OT" => (TitleType::OriginalTitle, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid title type '{}', must be AT, AL, TT, TL, FT, or OT", trimmed) }];
                (TitleType::AlternateTitle, warnings)
            }
        }
    }
}

/// Recording format for REC records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingFormat {
    #[default]
    Unknown,
    Stereo,       // S - Stereo
    Mono,         // M - Mono
    Quadrophonic, // Q - Quadrophonic
}

impl RecordingFormat {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingFormat::Unknown => " ",
            RecordingFormat::Stereo => "S",
            RecordingFormat::Mono => "M",
            RecordingFormat::Quadrophonic => "Q",
        }
    }
}

impl CwrFieldWrite for RecordingFormat {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for Option<RecordingFormat> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            match trimmed {
                "S" => (Some(RecordingFormat::Stereo), vec![]),
                "M" => (Some(RecordingFormat::Mono), vec![]),
                "Q" => (Some(RecordingFormat::Quadrophonic), vec![]),
                _ => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid recording format '{}', must be S (Stereo), M (Mono), or Q (Quadrophonic)", trimmed) }];
                    (Some(RecordingFormat::Unknown), warnings)
                }
            }
        }
    }
}

/// Recording technique for REC records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingTechnique {
    #[default]
    Unknown,
    Analog,  // A - Analog
    Digital, // D - Digital
}

impl RecordingTechnique {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingTechnique::Unknown => " ",
            RecordingTechnique::Analog => "A",
            RecordingTechnique::Digital => "D",
        }
    }
}

impl CwrFieldWrite for RecordingTechnique {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for Option<RecordingTechnique> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            match trimmed {
                "A" => (Some(RecordingTechnique::Analog), vec![]),
                "D" => (Some(RecordingTechnique::Digital), vec![]),
                _ => {
                    let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid recording technique '{}', must be A (Analog) or D (Digital)", trimmed) }];
                    (Some(RecordingTechnique::Unknown), warnings)
                }
            }
        }
    }
}

/// Publisher type for SPU records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PublisherType {
    #[default]
    Unknown,
    AdministeringPublisher, // AS - Administering Publisher
    OriginalPublisher,      // PA - Original Publisher (Assignor)
    SubPublisher,           // SE - Sub-Publisher
    IncomeParticipant,      // PA - Income Participant
    Unclassified,           // E  - Unclassified (catch-all)
}

impl PublisherType {
    pub fn as_str(&self) -> &str {
        match self {
            PublisherType::Unknown => "",
            PublisherType::AdministeringPublisher => "AS",
            PublisherType::OriginalPublisher => "PA",
            PublisherType::SubPublisher => "SE",
            PublisherType::IncomeParticipant => "PA",
            PublisherType::Unclassified => "E",
        }
    }
}

impl CwrFieldWrite for PublisherType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for Option<PublisherType> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::publisher_types::is_valid_publisher_type;

        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            match trimmed {
                "AS" => (Some(PublisherType::AdministeringPublisher), vec![]),
                "PA" => (Some(PublisherType::OriginalPublisher), vec![]),
                "SE" => (Some(PublisherType::SubPublisher), vec![]),
                "E" => (Some(PublisherType::Unclassified), vec![]),
                _ => {
                    let mut warnings = vec![];
                    if !is_valid_publisher_type(trimmed) {
                        warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid publisher type '{}', must be AS, PA, SE, or E", trimmed) });
                    }
                    (Some(PublisherType::Unknown), warnings)
                }
            }
        }
    }
}

/// Sender name for HDR record with context-aware validation
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
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: "Sender Name is required".to_string() }];
            return (SenderName(String::new()), warnings);
        }

        // Basic validation - comprehensive validation requires SenderType context in post_process
        // This validates general format and checks against known society names
        let mut warnings = vec![];

        // Check if it might be a society name (contains letters and possibly spaces)
        if trimmed.chars().any(|c| c.is_ascii_alphabetic()) {
            use crate::lookups::society_codes::is_valid_society_code;

            // Try exact match against society codes first
            if !is_valid_society_code(trimmed) {
                // Try with common variations (remove spaces, convert to uppercase)
                let normalized_variants = vec![trimmed.replace(" ", ""), trimmed.replace(" ", "").to_uppercase(), trimmed.to_uppercase(), trimmed.replace("-", "").replace(" ", "")];

                let mut found_match = false;
                for variant in &normalized_variants {
                    if is_valid_society_code(variant) {
                        found_match = true;
                        break;
                    }
                }

                // For now, skip the expensive transmitter code check
                // Full validation will be done in post_process with SenderType context

                if !found_match {
                    warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Sender Name '{}' not found in society lookup tables - validation will be performed in post-process with SenderType context", trimmed) });
                }
            }
        }

        (SenderName(trimmed.to_string()), warnings)
    }
}

// TODO: Implement complex Sender validation requiring post_process step:
//    - Cross-validate SenderType + SenderId + SenderName combination
//    - Handle IPNN > 9 digits case (SenderType numeric prefix + SenderId)
//    - Validate against appropriate lookup tables based on sender type

impl CwrFieldWrite for GroupId {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for TisNumericCode {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for GroupCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for TransactionCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for RecordCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for OwnershipShare {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for CompositeComponentCount {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}

impl CwrFieldWrite for CwrVersion {
    fn to_cwr_str(&self) -> String {
        self.as_str()
    }
}
