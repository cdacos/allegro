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

impl CwrToSqlString for TextMusicRelationship {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for ExcerptType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for MusicArrangement {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for LyricAdaptation {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for LookupPlaceholder {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for NonRomanAlphabet {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for LanguageCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for MusicalWorkDistributionCategory {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for VersionType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for CompositeType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for WorkType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for SocietyCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for AgreementType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for UsaLicenseIndicator {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for WriterDesignation {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for TransactionStatus {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for TypeOfRight {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for SubjectCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for IsrcValidityIndicator {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for InstrumentCode {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for MessageType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for MessageLevel {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for IdentifierType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for IntendedPurpose {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for MediaType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for StandardInstrumentationType {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for IpiNameNumber {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for IpiBaseNumber {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for Isrc {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for Ean {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
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
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SenderType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for SenderId {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SenderId::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SenderId: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for SenderName {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SenderName::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SenderName: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for EdiStandardVersion {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = EdiStandardVersion::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing EdiStandardVersion: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Date {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Date::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Date: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Time {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Time::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Time: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CharacterSet {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<CharacterSet>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CharacterSet: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed.unwrap_or_default())
        }
    }
}

impl CwrFromSqlString for CwrVersion {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrVersion::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CwrVersion: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CwrRevision {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrRevision::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CwrRevision: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TransactionType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TransactionType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TransactionType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Boolean {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Boolean::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Boolean: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Flag {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Flag::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing FlagYNU: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TitleType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TitleType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TitleType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for InclusionExclusionIndicator {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = InclusionExclusionIndicator::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing InclusionExclusionIndicator: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for AgreementRoleCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = AgreementRoleCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing AgreementRoleCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for RecordingFormat {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<RecordingFormat>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing RecordingFormat: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed.unwrap_or_default())
        }
    }
}

impl CwrFromSqlString for RecordingTechnique {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<RecordingTechnique>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing RecordingTechnique: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed.unwrap_or_default())
        }
    }
}

impl CwrFromSqlString for PublisherType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<PublisherType>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing PublisherType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed.unwrap_or_default())
        }
    }
}

impl CwrFromSqlString for PriorRoyaltyStatus {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = PriorRoyaltyStatus::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing PriorRoyaltyStatus: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for PostTermCollectionStatus {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = PostTermCollectionStatus::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing PostTermCollectionStatus: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CwrVersionNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CwrVersionNumber::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CwrVersionNumber: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CurrencyCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CurrencyCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CurrencyCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CompositeComponentCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Option::<CompositeComponentCount>::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CompositeComponentCount: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed.unwrap_or_default())
        }
    }
}

// Integer parsing implementations
impl CwrFromSqlInt for OwnershipShare {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=10000).contains(&value) {
            Err(format!("OwnershipShare value {} is out of range 0-10000", value))
        } else {
            Ok(OwnershipShare(value as u16))
        }
    }
}

impl CwrFromSqlInt for TisNumericCode {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=u16::MAX as i64).contains(&value) {
            Err(format!("TisNumericCode value {} is out of range", value))
        } else {
            Ok(TisNumericCode(value as u16))
        }
    }
}

impl CwrFromSqlInt for PublisherSequenceNumber {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(1..=99).contains(&value) {
            Err(format!("PublisherSequenceNumber value {} is out of range 1-99", value))
        } else {
            Ok(PublisherSequenceNumber(value as u8))
        }
    }
}

impl CwrFromSqlInt for GroupId {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(1..=99999).contains(&value) {
            Err(format!("GroupId value {} is out of range 1-99999", value))
        } else {
            Ok(GroupId(value as u32))
        }
    }
}

impl CwrFromSqlInt for TransactionCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) {
            Err(format!("TransactionCount value {} is out of range 0-99999999", value))
        } else {
            Ok(TransactionCount(value as u32))
        }
    }
}

impl CwrFromSqlInt for RecordCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) {
            Err(format!("RecordCount value {} is out of range 0-99999999", value))
        } else {
            Ok(RecordCount(value as u32))
        }
    }
}

impl CwrFromSqlInt for GroupCount {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999).contains(&value) {
            Err(format!("GroupCount value {} is out of range 0-99999", value))
        } else {
            Ok(GroupCount(value as u32))
        }
    }
}

impl CwrFromSqlInt for Number {
    fn from_sql_int(value: i64) -> Result<Self, String> {
        if !(0..=99999999).contains(&value) {
            Err(format!("Number value {} is out of range 0-99999999", value))
        } else {
            Ok(Number(value as u32))
        }
    }
}

// String parsing implementations for numeric types (since they're stored as VARCHAR in database)
impl CwrFromSqlString for GroupId {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = GroupId::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing GroupId: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TransactionCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TransactionCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TransactionCount: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for RecordCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = RecordCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing RecordCount: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for GroupCount {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = GroupCount::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing GroupCount: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TisNumericCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        if let Ok(parsed_value) = value.parse::<u16>() {
            Ok(TisNumericCode(parsed_value))
        } else {
            Err(format!("Failed to parse TisNumericCode from '{}'", value))
        }
    }
}

impl CwrFromSqlString for OwnershipShare {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        // OwnershipShare is stored as a 5-digit padded integer string
        if let Ok(parsed_value) = value.parse::<u16>() {
            if parsed_value <= 10000 {
                Ok(OwnershipShare(parsed_value))
            } else {
                Err(format!("OwnershipShare value {} is out of range 0-10000", parsed_value))
            }
        } else {
            Err(format!("Failed to parse OwnershipShare from '{}'", value))
        }
    }
}

impl CwrFromSqlString for PublisherSequenceNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        if let Ok(parsed_value) = value.parse::<u8>() {
            if (1..=99).contains(&parsed_value) {
                Ok(PublisherSequenceNumber(parsed_value))
            } else {
                Err(format!("PublisherSequenceNumber value {} is out of range 1-99", parsed_value))
            }
        } else {
            Err(format!("Failed to parse PublisherSequenceNumber from '{}'", value))
        }
    }
}

impl CwrFromSqlString for Number {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Number::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Number: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TextMusicRelationship {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TextMusicRelationship::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TextMusicRelationship: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for ExcerptType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = ExcerptType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing ExcerptType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for MusicArrangement {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = MusicArrangement::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing MusicArrangement: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for LyricAdaptation {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = LyricAdaptation::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing LyricAdaptation: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for LookupPlaceholder {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = LookupPlaceholder::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing LookupPlaceholder: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for NonRomanAlphabet {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = NonRomanAlphabet::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing NonRomanAlphabet: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for LanguageCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = LanguageCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing LanguageCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for MusicalWorkDistributionCategory {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = MusicalWorkDistributionCategory::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing MusicalWorkDistributionCategory: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for VersionType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = VersionType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing VersionType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for CompositeType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = CompositeType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing CompositeType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for WorkType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = WorkType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing WorkType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for SocietyCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SocietyCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SocietyCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for AgreementType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = AgreementType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing AgreementType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for UsaLicenseIndicator {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = UsaLicenseIndicator::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing UsaLicenseIndicator: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for WriterDesignation {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = WriterDesignation::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing WriterDesignation: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TransactionStatus {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TransactionStatus::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TransactionStatus: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for TypeOfRight {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = TypeOfRight::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing TypeOfRight: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for SubjectCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SubjectCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SubjectCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for IsrcValidityIndicator {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = IsrcValidityIndicator::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing IsrcValidityIndicator: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for InstrumentCode {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = InstrumentCode::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing InstrumentCode: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for MessageType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = MessageType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing MessageType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for MessageLevel {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = MessageLevel::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing MessageLevel: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for IdentifierType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = IdentifierType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing IdentifierType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for IntendedPurpose {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = IntendedPurpose::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing IntendedPurpose: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for MediaType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = MediaType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing MediaType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for StandardInstrumentationType {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = StandardInstrumentationType::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing StandardInstrumentationType: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for IpiNameNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = IpiNameNumber::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing IpiNameNumber: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for IpiBaseNumber {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = IpiBaseNumber::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing IpiBaseNumber: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Isrc {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Isrc::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Isrc: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for Ean {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = Ean::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing Ean: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for WriterPosition {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = WriterPosition::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing WriterPosition: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for SalesManufactureClause {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = SalesManufactureClause::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing SalesManufactureClause: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
    }
}

impl CwrFromSqlString for LanguageDialect {
    fn from_sql_string(value: &str) -> Result<Self, String> {
        let (parsed, warnings) = LanguageDialect::parse_cwr_field(value, "sql_field", "SQL Field");
        if warnings.iter().any(|w| w.is_critical()) {
            Err(format!(
                "Critical error parsing LanguageDialect: {}",
                warnings.iter().find(|w| w.is_critical()).unwrap().description
            ))
        } else {
            Ok(parsed)
        }
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
