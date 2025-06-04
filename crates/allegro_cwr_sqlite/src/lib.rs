//! SQLite database operations for CWR (Common Works Registration) files
//!
//! This crate provides database setup, schema management, and record operations
//! for storing and querying CWR file data in SQLite databases.

pub mod connection;
pub mod error;
pub mod operations;
pub mod statements;

// Re-export main types and functions
pub use connection::{CwrDatabase, determine_db_filename, setup_database};
pub use error::CwrDbError;
pub use operations::{CwrRecordInserter, count_errors_by_record_type, count_records_by_type, insert_file_line_record, insert_file_record, log_error};
pub use statements::PreparedStatements;

/// Result type for database operations
pub type Result<T> = std::result::Result<T, CwrDbError>;
