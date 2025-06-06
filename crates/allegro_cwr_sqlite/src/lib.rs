//! SQLite database operations for CWR (Common Works Registration) files
//!
//! This crate provides database setup, schema management, and record operations
//! for storing and querying CWR file data in SQLite databases.

pub mod connection;
pub mod domain_conversions;
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
    batch_size: usize,
    statements: Option<statements::PreparedStatements<'static>>,
}

impl SqliteHandler {
    pub fn new(input_filename: &str, db_filename: &str) -> Result<Self> {
        Self::new_with_batch_size(input_filename, db_filename, 1000)
    }

    pub fn new_with_batch_size(input_filename: &str, db_filename: &str, batch_size: usize) -> Result<Self> {
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

        Ok(SqliteHandler { conn, tx: None, file_id, processed_count: 0, error_count: 0, db_filename: db_filename.to_string(), batch_size, statements: None })
    }

    fn start_batch(&mut self) -> Result<()> {
        if self.tx.is_none() {
            // Start transaction
            let tx = self.conn.transaction()?;
            // We need to use unsafe to extend the lifetime
            let tx: rusqlite::Transaction<'static> = unsafe { std::mem::transmute(tx) };
            let statements = statements::get_prepared_statements(&tx)?;
            let statements: statements::PreparedStatements<'static> = unsafe { std::mem::transmute(statements) };

            self.tx = Some(tx);
            self.statements = Some(statements);
        }
        Ok(())
    }

    fn commit_batch(&mut self) -> Result<()> {
        if let Some(tx) = self.tx.take() {
            self.statements = None;
            tx.commit()?;
        }
        Ok(())
    }

    fn should_commit_batch(&self) -> bool {
        self.processed_count % self.batch_size == 0
    }
}

impl allegro_cwr::CwrHandler for SqliteHandler {
    type Error = CwrDbError;

    fn process_record(&mut self, parsed_record: allegro_cwr::ParsedRecord) -> std::result::Result<(), Self::Error> {
        self.start_batch()?;

        if let Some(ref mut statements) = self.statements {
            // For now, just log to file_line table - TODO: implement full record insertion
            let record_id = 1; // Placeholder
            insert_file_line_record(&mut statements.file_stmt, self.file_id, parsed_record.line_number, parsed_record.record.record_type(), record_id)?;
        }

        self.processed_count += 1;

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn handle_parse_error(&mut self, line_number: usize, error: &allegro_cwr::CwrParseError) -> std::result::Result<(), Self::Error> {
        self.start_batch()?;

        if let Some(ref mut statements) = self.statements {
            log_error(&mut statements.error_stmt, self.file_id, line_number, error.to_string())?;
        }

        self.error_count += 1;

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn finalize(&mut self) -> std::result::Result<(), Self::Error> {
        // Commit any remaining batch
        self.commit_batch()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_records_are_actually_inserted_into_database() {
        // Create a temporary CWR file with a few records
        let temp_dir = tempdir().unwrap();
        let cwr_file_path = temp_dir.path().join("test.cwr");
        let db_file_path = temp_dir.path().join("test.db");
        
        let mut file = File::create(&cwr_file_path).unwrap();
        writeln!(file, "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221").unwrap();
        writeln!(file, "GRHTRK0000102.10                                                          ").unwrap();
        writeln!(file, "NWR0000000100000001SAMPLE SONG                                    EN13579246801234500000000000000                                                                                                                                                                                                    00000000            UNC000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "GRT000010000000100000003").unwrap();
        writeln!(file, "TRL00001000000010000003").unwrap();
        
        // Process the file
        let (file_id, processed_count, _report) = process_cwr_to_sqlite(
            cwr_file_path.to_str().unwrap(),
            db_file_path.to_str().unwrap()
        ).unwrap();
        
        // Verify processing happened
        assert_eq!(processed_count, 5, "Should have processed 5 records");
        
        // Connect to database and verify records were actually inserted
        let conn = rusqlite::Connection::open(&db_file_path).unwrap();
        
        // Check file_line table - should have entries for each record type
        let mut stmt = conn.prepare("SELECT record_type, COUNT(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type").unwrap();
        let rows: std::result::Result<Vec<(String, i64)>, rusqlite::Error> = stmt.query_map([file_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        }).unwrap().collect();
        
        let record_counts = rows.unwrap();
        println!("Record counts from file_line table: {:?}", record_counts);
        
        // Should have one of each record type
        let expected_records = vec![
            ("GRH".to_string(), 1i64),
            ("GRT".to_string(), 1i64), 
            ("HDR".to_string(), 1i64),
            ("NWR".to_string(), 1i64),
            ("TRL".to_string(), 1i64),
        ];
        assert_eq!(record_counts, expected_records, "file_line table should track all record types");
        
        // MORE IMPORTANTLY: Check that actual record data was inserted into specific tables
        
        // Check HDR table
        let hdr_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM hdr WHERE file_id = ?1", 
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(hdr_count, 1, "HDR table should have 1 record");
        
        // Check GRH table  
        let grh_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM grh WHERE file_id = ?1",
            [file_id],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(grh_count, 1, "GRH table should have 1 record");
        
        // Check NWR table
        let nwr_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM nwr WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(nwr_count, 1, "NWR table should have 1 record");
        
        // Verify the HDR record actually contains the parsed data
        let (sender_name, creation_date): (String, String) = conn.query_row(
            "SELECT sender_name, creation_date FROM hdr WHERE file_id = ?1",
            [file_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        ).unwrap();
        
        assert_eq!(sender_name, "WARNER CHAPPELL MUSIC PUBLISHING LTD", "HDR should contain parsed sender name");
        assert_eq!(creation_date, "20221221", "HDR should contain parsed creation date");
        
        println!("✅ Test should pass once record insertion is implemented");
    }
}
