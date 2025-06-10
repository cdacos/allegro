//! SQLite conversions for CWR domain types

use allegro_cwr::domain_types::*;

/// Extension trait to convert CWR domain types to strings for SQLite storage
pub trait CwrToSqlString {
    fn to_sql_string(&self) -> String;
}

/// Extension trait to convert CWR domain types to integer values for SQLite storage  
pub trait CwrToSqlInt {
    fn to_sql_int(&self) -> i64;
}

// String conversions for domain types
impl CwrToSqlString for Date {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for String {
    fn to_sql_string(&self) -> String {
        self.clone()
    }
}

impl CwrToSqlString for RecordingFormat {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for RecordingTechnique {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for PublisherType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for Flag {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for InclusionExclusionIndicator {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for AgreementRoleCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CharacterSet {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for TitleType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for TransactionType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for PriorRoyaltyStatus {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for PostTermCollectionStatus {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CwrVersionNumber {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CurrencyCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for SenderId {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for SenderType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for EdiStandardVersion {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CwrVersion {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for CwrRevision {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for Time {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for Boolean {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CompositeComponentCount {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for Number {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

// Integer conversions for numeric domain types
impl CwrToSqlInt for OwnershipShare {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for TisNumericCode {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for CompositeComponentCount {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for PublisherSequenceNumber {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for GroupId {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for TransactionCount {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for RecordCount {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for GroupCount {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

impl CwrToSqlInt for Number {
    fn to_sql_int(&self) -> i64 {
        self.0 as i64
    }
}

// Utility functions for optional fields
pub fn opt_domain_to_string<T: CwrToSqlString>(opt: &Option<T>) -> Option<String> {
    opt.as_ref().map(|v| v.to_sql_string())
}

pub fn opt_domain_to_int<T: CwrToSqlInt>(opt: &Option<T>) -> Option<i64> {
    opt.as_ref().map(|v| v.to_sql_int())
}

/// Extension trait to parse CWR domain types from SQLite string values
pub trait CwrFromSqlString: Sized {
    fn from_sql_string(value: &str) -> Result<Self, String>;
}

/// Extension trait to parse CWR domain types from SQLite integer values
pub trait CwrFromSqlInt: Sized {
    fn from_sql_int(value: i64) -> Result<Self, String>;
}

// String parsing implementations using existing CwrFieldParse logic
impl CwrFromSqlString for SenderType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SenderType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing SenderType: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for SenderId {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SenderId::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing SenderId: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for SenderName {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SenderName::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing SenderName: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for EdiStandardVersion {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = EdiStandardVersion::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing EdiStandardVersion: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for Date {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Date::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing Date: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for Time {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Time::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing Time: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for CharacterSet {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<CharacterSet>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CharacterSet: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed.unwrap_or_default()) }
    }
}

impl CwrFromSqlString for CwrVersion {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrVersion::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CwrVersion: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for CwrRevision {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrRevision::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CwrRevision: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for TransactionType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TransactionType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing TransactionType: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for Boolean {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Boolean::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing Boolean: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for Flag {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Flag::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing FlagYNU: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for TitleType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TitleType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing TitleType: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for InclusionExclusionIndicator {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = InclusionExclusionIndicator::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing InclusionExclusionIndicator: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for AgreementRoleCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = AgreementRoleCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing AgreementRoleCode: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for RecordingFormat {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<RecordingFormat>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing RecordingFormat: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed.unwrap_or_default()) }
    }
}

impl CwrFromSqlString for RecordingTechnique {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<RecordingTechnique>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing RecordingTechnique: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed.unwrap_or_default()) }
    }
}

impl CwrFromSqlString for PublisherType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<PublisherType>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing PublisherType: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed.unwrap_or_default()) }
    }
}

impl CwrFromSqlString for PriorRoyaltyStatus {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = PriorRoyaltyStatus::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing PriorRoyaltyStatus: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for PostTermCollectionStatus {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = PostTermCollectionStatus::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing PostTermCollectionStatus: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for CwrVersionNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrVersionNumber::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CwrVersionNumber: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for CurrencyCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CurrencyCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CurrencyCode: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for CompositeComponentCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<CompositeComponentCount>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing CompositeComponentCount: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed.unwrap_or_default()) }
    }
}

// Integer parsing implementations
impl CwrFromSqlInt for OwnershipShare {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=10000).contains(&value) { Err(format!("OwnershipShare value {} is out of range 0-10000", value)) } else { Ok(OwnershipShare(value as u16)) }
    }
}

impl CwrFromSqlInt for TisNumericCode {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=u16::MAX as i64).contains(&value) { Err(format!("TisNumericCode value {} is out of range", value)) } else { Ok(TisNumericCode(value as u16)) }
    }
}

impl CwrFromSqlInt for PublisherSequenceNumber {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(1..=99).contains(&value) { Err(format!("PublisherSequenceNumber value {} is out of range 1-99", value)) } else { Ok(PublisherSequenceNumber(value as u8)) }
    }
}

impl CwrFromSqlInt for GroupId {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(1..=99999).contains(&value) { Err(format!("GroupId value {} is out of range 1-99999", value)) } else { Ok(GroupId(value as u32)) }
    }
}

impl CwrFromSqlInt for TransactionCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) { Err(format!("TransactionCount value {} is out of range 0-99999999", value)) } else { Ok(TransactionCount(value as u32)) }
    }
}

impl CwrFromSqlInt for RecordCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) { Err(format!("RecordCount value {} is out of range 0-99999999", value)) } else { Ok(RecordCount(value as u32)) }
    }
}

impl CwrFromSqlInt for GroupCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999).contains(&value) { Err(format!("GroupCount value {} is out of range 0-99999", value)) } else { Ok(GroupCount(value as u32)) }
    }
}

impl CwrFromSqlInt for Number {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) { Err(format!("Number value {} is out of range 0-99999999", value)) } else { Ok(Number(value as u32)) }
    }
}

// String parsing implementations for numeric types (since they're stored as VARCHAR in database)
impl CwrFromSqlString for GroupId {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = GroupId::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing GroupId: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for TransactionCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TransactionCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing TransactionCount: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for RecordCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = RecordCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing RecordCount: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for GroupCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = GroupCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing GroupCount: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

impl CwrFromSqlString for TisNumericCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        if let Ok(parsed_value) = value.parse::<u16>() { Ok(TisNumericCode(parsed_value)) } else { Err(format!("Failed to parse TisNumericCode from '{}'", value)) }
    }
}

impl CwrFromSqlString for OwnershipShare {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        // OwnershipShare is stored as a 5-digit padded integer string
        if let Ok(parsed_value) = value.parse::<u16>() { if parsed_value <= 10000 { Ok(OwnershipShare(parsed_value)) } else { Err(format!("OwnershipShare value {} is out of range 0-10000", parsed_value)) } } else { Err(format!("Failed to parse OwnershipShare from '{}'", value)) }
    }
}

impl CwrFromSqlString for PublisherSequenceNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        if let Ok(parsed_value) = value.parse::<u8>() { if (1..=99).contains(&parsed_value) { Ok(PublisherSequenceNumber(parsed_value)) } else { Err(format!("PublisherSequenceNumber value {} is out of range 1-99", parsed_value)) } } else { Err(format!("Failed to parse PublisherSequenceNumber from '{}'", value)) }
    }
}

impl CwrFromSqlString for Number {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Number::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) { Err(format!("Critical error parsing Number: {}", warnings.iter().find(|w| w.is_critical()).unwrap().description)) } else { Ok(parsed) }
    }
}

// Helper functions for optional parsing
pub fn opt_string_to_domain<T: CwrFromSqlString>(opt: Option<&str>) -> Result<Option<T>, String> {
    match opt {
        Some("") => Ok(None),
        Some(s) => T::from_sql_string(s).map(Some),
        None => Ok(None),
    }
}

pub fn opt_int_to_domain<T: CwrFromSqlInt>(opt: Option<i64>) -> Result<Option<T>, String> {
    match opt {
        Some(i) => T::from_sql_int(i).map(Some),
        None => Ok(None),
    }
}

// Helper for parsing optional numeric types from database strings
pub fn opt_string_to_numeric<T: CwrFromSqlString>(opt: Option<&str>) -> Result<Option<T>, String> {
    match opt {
        Some("") => Ok(None),
        Some(s) => T::from_sql_string(s).map(Some),
        None => Ok(None),
    }
}
