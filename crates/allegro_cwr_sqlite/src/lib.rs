//! SQLite database operations for CWR (Common Works Registration) files
//! 
//! This crate provides database setup, schema management, and record operations
//! for storing and querying CWR file data in SQLite databases.

pub mod connection;
pub mod error;
pub mod operations;
pub mod statements;

// Re-export main types and functions
pub use connection::{CwrDatabase, determine_db_filename, setup_database, setup_database_with_overwrite};
pub use error::CwrDbError;
pub use operations::{CwrRecordInserter, log_error, insert_file_record, insert_file_line_record};
pub use statements::PreparedStatements;

/// Result type for database operations
pub type Result<T> = std::result::Result<T, CwrDbError>;