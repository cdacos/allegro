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
pub fn log_error(error_stmt: &mut Statement, file_id: i64, line_number: usize, description: String) -> Result<(), CwrDbError> {
    error_stmt.execute(params![file_id, line_number as i64, description])?;
    Ok(())
}

/// Inserts a record into the 'file' table and returns the file_id
pub fn insert_file_record(tx: &Transaction, file_insert_stmt: &mut Statement, file_path: &str) -> Result<i64, CwrDbError> {
    file_insert_stmt.execute(params![file_path])?;
    Ok(tx.last_insert_rowid())
}

/// Inserts a record into the 'file_line' table using a prepared statement
pub fn insert_file_line_record(file_stmt: &mut Statement, file_id: i64, line_number: usize, record_type: &str, record_id: i64) -> Result<(), CwrDbError> {
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
        let record_type = table_name.strip_prefix("cwr_").unwrap().to_uppercase();
        let count_query = format!("SELECT COUNT(*) FROM {}", table_name);

        if let Ok(count) = conn.query_row(&count_query, [], |row| row.get::<_, i32>(0)) {
            if count > 0 {
                counts.insert(record_type, count);
            }
        }
    }

    Ok(counts)
}
