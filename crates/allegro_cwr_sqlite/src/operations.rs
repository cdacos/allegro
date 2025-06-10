use crate::{error::CwrDbError, statements::PreparedStatements};
use rusqlite::{Connection, Statement, Transaction, params};
use std::collections::HashMap;

/// High-level interface for inserting CWR records
pub struct CwrRecordInserter<'conn> {
    pub statements: PreparedStatements<'conn>,
}

impl<'conn> CwrRecordInserter<'conn> {
    /// Create a new record inserter from a transaction
    pub fn new(tx: &'conn Transaction) -> Result<Self, CwrDbError> {
        let statements = crate::statements::get_prepared_statements(tx)?;
        Ok(CwrRecordInserter { statements })
    }

    /// Get mutable access to the prepared statements
    pub fn statements_mut(&mut self) -> &mut PreparedStatements<'conn> {
        &mut self.statements
    }
}

/// Inserts a record into the 'error' table using a prepared statement
pub fn log_error(
    error_stmt: &mut Statement, file_id: i64, line_number: usize, description: String,
) -> Result<(), CwrDbError> {
    error_stmt.execute(params![file_id, line_number as i64, description])?;
    Ok(())
}

/// Inserts a record into the 'file' table and returns the file_id
pub fn insert_file_record(
    tx: &Transaction, file_insert_stmt: &mut Statement, file_path: &str,
) -> Result<i64, CwrDbError> {
    file_insert_stmt.execute(params![file_path])?;
    Ok(tx.last_insert_rowid())
}

/// Inserts a record into the 'file_line' table using a prepared statement
pub fn insert_file_line_record(
    file_stmt: &mut Statement, file_id: i64, line_number: usize, record_type: &str, record_id: i64,
) -> Result<(), CwrDbError> {
    file_stmt.execute(params![file_id, line_number as i64, record_type, record_id])?;
    Ok(())
}

/// Count records by type in all CWR tables
pub fn count_records_by_type(db_path: &str) -> Result<HashMap<String, i32>, CwrDbError> {
    let conn = Connection::open(db_path)?;
    let mut counts = HashMap::new();

    // Query SQLite system tables to find all tables starting with "cwr_"
    let table_query = "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'";
    let mut stmt = conn.prepare(table_query)?;
    let table_rows = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table_result in table_rows {
        let table_name = table_result?;

        // Get all distinct record_type values from this table and count them
        let distinct_query = format!("SELECT record_type, COUNT(*) FROM {} GROUP BY record_type", table_name);
        let mut distinct_stmt = conn.prepare(&distinct_query)?;
        let type_rows = distinct_stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?)))?;

        for type_result in type_rows {
            let (record_type, count) = type_result?;
            if count > 0 {
                counts.insert(record_type, count);
            }
        }
    }

    Ok(counts)
}

/// Count validation errors by record type from error descriptions
pub fn count_errors_by_record_type(db_path: &str) -> Result<HashMap<String, i32>, CwrDbError> {
    let conn = Connection::open(db_path)?;
    let mut error_counts = HashMap::new();

    // Query all error descriptions and extract record types
    let error_query = "SELECT description, COUNT(*) FROM error GROUP BY description";
    let mut stmt = conn.prepare(error_query)?;
    let error_rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?)))?;

    for error_result in error_rows {
        let (description, count) = error_result?;

        // Extract record type from error description
        // Examples: "SWR Interested Party # is mandatory", "PWR missing or empty..."
        if let Some(record_type) = description.split_whitespace().next() {
            if record_type.len() == 3 && record_type.chars().all(|c| c.is_ascii_uppercase()) {
                *error_counts.entry(record_type.to_string()).or_insert(0) += count;
            }
        }
    }

    Ok(error_counts)
}
