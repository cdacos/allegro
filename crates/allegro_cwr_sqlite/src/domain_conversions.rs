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

impl CwrToSqlString for FlagYNU {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrToSqlString for Duration {
    fn to_sql_string(&self) -> String {
        self.as_str()
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
        self.as_str()
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
        self.0.unwrap_or(0) as i64
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

impl CwrToSqlInt for WorksCount {
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

// Utility functions for optional fields
pub fn opt_domain_to_string<T: CwrToSqlString>(opt: &Option<T>) -> Option<String> {
    opt.as_ref().map(|v| v.to_sql_string())
}

pub fn opt_domain_to_int<T: CwrToSqlInt>(opt: &Option<T>) -> Option<i64> {
    opt.as_ref().map(|v| v.to_sql_int())
}
