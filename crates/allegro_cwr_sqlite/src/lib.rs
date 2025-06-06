//! SQLite database operations for CWR (Common Works Registration) files
//!
//! This crate provides database setup, schema management, and record operations
//! for storing and querying CWR file data in SQLite databases.

pub mod connection;
pub mod error;
pub mod operations;
pub mod record_handlers;
pub mod report;
pub mod statements;

// Re-export main types and functions
pub use connection::{CwrDatabase, determine_db_filename, setup_database};
pub use error::CwrDbError;
pub use operations::{CwrRecordInserter, count_errors_by_record_type, count_records_by_type, insert_file_line_record, insert_file_record, log_error};
pub use statements::PreparedStatements;

/// Result type for database operations
pub type Result<T> = std::result::Result<T, CwrDbError>;

/// SQLite implementation of CwrHandler trait
pub struct SqliteHandler {
    conn: rusqlite::Connection,
    tx: Option<rusqlite::Transaction<'static>>,
    file_id: i64,
    processed_count: usize,
    error_count: usize,
    db_filename: String,
}

impl SqliteHandler {
    pub fn new(input_filename: &str, db_filename: &str) -> Result<Self> {
        use statements::get_prepared_statements;

        // Setup database
        setup_database(db_filename)?;

        // Open connection and setup transaction
        let mut conn = rusqlite::Connection::open(db_filename)?;
        conn.pragma_update(None, "journal_mode", "OFF")?;
        conn.pragma_update(None, "synchronous", "OFF")?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;

        // We need to work around the lifetime issue with transactions
        // For now, let's use a simpler approach without holding the transaction
        let file_id = {
            let tx = conn.transaction()?;
            let mut prepared_statements = get_prepared_statements(&tx)?;
            let file_id = insert_file_record(&tx, &mut prepared_statements.file_insert_stmt, input_filename)?;
            drop(prepared_statements); // Drop before commit to release borrow
            tx.commit()?;
            file_id
        };

        Ok(SqliteHandler { conn, tx: None, file_id, processed_count: 0, error_count: 0, db_filename: db_filename.to_string() })
    }
}

impl allegro_cwr::CwrHandler for SqliteHandler {
    type Error = CwrDbError;

    fn process_record(&mut self, parsed_record: allegro_cwr::ParsedRecord) -> std::result::Result<(), Self::Error> {
        // Start a transaction for this record
        let tx = self.conn.transaction()?;
        let mut prepared_statements = statements::get_prepared_statements(&tx)?;

        // For now, just log to file_line table - TODO: implement full record insertion
        let record_id = 1; // Placeholder
        insert_file_line_record(&mut prepared_statements.file_stmt, self.file_id, parsed_record.line_number, parsed_record.record.record_type(), record_id)?;

        drop(prepared_statements);
        tx.commit()?;

        self.processed_count += 1;
        Ok(())
    }

    fn handle_parse_error(&mut self, line_number: usize, error: &allegro_cwr::CwrParseError) -> std::result::Result<(), Self::Error> {
        // Log error to database
        let tx = self.conn.transaction()?;
        let mut prepared_statements = statements::get_prepared_statements(&tx)?;

        log_error(&mut prepared_statements.error_stmt, self.file_id, line_number, error.to_string())?;

        drop(prepared_statements);
        tx.commit()?;

        self.error_count += 1;
        Ok(())
    }

    fn finalize(&mut self) -> std::result::Result<(), Self::Error> {
        // Any final cleanup if needed
        Ok(())
    }

    fn get_report(&self) -> String {
        format!("SQLite processing complete:\n  Database: {}\n  Records processed: {}\n  Errors: {}", self.db_filename, self.processed_count, self.error_count)
    }
}

/// Convenience function to process CWR file with SQLite handler
pub fn process_cwr_to_sqlite(input_filename: &str, db_filename: &str) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    process_cwr_to_sqlite_with_version(input_filename, db_filename, None)
}

/// Convenience function to process CWR file with SQLite handler and optional version hint
pub fn process_cwr_to_sqlite_with_version(input_filename: &str, db_filename: &str, version_hint: Option<f32>) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    let handler = SqliteHandler::new(input_filename, db_filename)?;
    let file_id = handler.file_id;
    let report = allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?;

    // Extract count from report (simple parsing for now)
    let processed_count = report.lines().find(|line| line.contains("Records processed:")).and_then(|line| line.split(':').nth(1)).and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);

    Ok((file_id, processed_count, report))
}
