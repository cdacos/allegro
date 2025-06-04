use crate::{error::CwrDbError, statements::PreparedStatements};
use rusqlite::{params, Statement, Transaction};

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
    error_stmt: &mut Statement,
    line_number: usize,
    description: String,
) -> Result<(), CwrDbError> {
    error_stmt.execute(params![line_number as i64, description])?;
    Ok(())
}

/// Inserts a record into the 'file' table using a prepared statement
pub fn insert_file_record(
    file_stmt: &mut Statement,
    line_number: usize,
    record_type: &str,
    record_id: i64,
) -> Result<(), CwrDbError> {
    file_stmt.execute(params![line_number as i64, record_type, record_id])?;
    Ok(())
}