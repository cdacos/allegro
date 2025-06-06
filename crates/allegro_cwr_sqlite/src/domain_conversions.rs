//! SQLite conversions for CWR domain types

use allegro_cwr::domain_types::{Date, RecordType};

/// Extension trait to convert CWR domain types to strings for SQLite
pub trait CwrToSqlString {
    fn to_sql_string(&self) -> String;
}

impl CwrToSqlString for RecordType {
    fn to_sql_string(&self) -> String {
        self.to_string()
    }
}

impl CwrToSqlString for Date {
    fn to_sql_string(&self) -> String {
        self.as_str()
    }
}

impl CwrToSqlString for &'static str {
    fn to_sql_string(&self) -> String {
        (*self).to_string()
    }
}
