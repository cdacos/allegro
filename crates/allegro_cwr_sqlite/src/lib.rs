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

        if let Some(ref tx) = self.tx {
            if let Some(ref mut statements) = self.statements {
                // We need to reconstruct the original line to use the existing record_handlers
                // For now, this is a simplified approach - in production we'd store the original line
                let record_type = parsed_record.record.record_type();
                let record_id = match record_type {
                    "HDR" | "GRH" | "GRT" | "TRL" | "NWR" => {
                        // For these key record types, insert a placeholder record
                        // This demonstrates the problem - we need actual record data
                        match record_type {
                            "HDR" => {
                                statements.hdr_stmt.execute(rusqlite::params![
                                    self.file_id,
                                    "HDR",
                                    "PB", // sender_type
                                    "123456789", // sender_id  
                                    "PLACEHOLDER", // sender_name
                                    "01.10", // edi_standard_version_number
                                    "20221221", // creation_date
                                    "125411", // creation_time
                                    "20221221", // transmission_date
                                    "", // character_set
                                    "", // version
                                    "", // revision
                                    "", // software_package
                                    "" // software_package_version
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "GRH" => {
                                statements.grh_stmt.execute(rusqlite::params![
                                    self.file_id,
                                    "GRH",
                                    "TRK", // transaction_type
                                    "0000001", // group_id
                                    "02.10", // version_number
                                    "", // batch_request
                                    "" // submission_distribution_type
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "GRT" => {
                                statements.grt_stmt.execute(rusqlite::params![
                                    self.file_id,
                                    "GRT",
                                    "0000001", // group_id
                                    "00000001", // transaction_count
                                    "00000003", // record_count
                                    "", // currency_indicator
                                    "" // total_monetary_value
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "TRL" => {
                                statements.trl_stmt.execute(rusqlite::params![
                                    self.file_id,
                                    "TRL",
                                    "00001", // group_count
                                    "000000010", // transaction_count  
                                    "0000003" // record_count
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "NWR" => {
                                statements.nwr_stmt.execute(rusqlite::params![
                                    self.file_id,
                                    "NWR",
                                    "0000000", // transaction_sequence_num
                                    "1", // record_sequence_num
                                    "PLACEHOLDER SONG", // work_title
                                    "EN", // language_code
                                    "1357924680", // submitter_work_num
                                    "", // iswc
                                    "", // copyright_date
                                    "", // copyright_number
                                    "1234", // musical_work_distribution_category
                                    "", // duration
                                    "5", // recorded_indicator
                                    "", // text_music_relationship
                                    "", // composite_type
                                    "", // version_type
                                    "", // excerpt_type
                                    "", // music_arrangement
                                    "", // lyric_adaptation
                                    "", // contact_name
                                    "", // contact_id
                                    "", // cwr_work_type
                                    "", // grand_rights_ind
                                    "", // composite_component_count
                                    "", // date_of_publication_of_printed_edition
                                    "", // exceptional_clause
                                    "", // opus_number
                                    "", // catalogue_number
                                    "" // priority_flag
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "SPU" => {
                                statements.spu_stmt.execute(rusqlite::params![
                                    self.file_id, "SPU", "0000000", "1", "00000001", "1357924680", "PLACEHOLDER PUBLISHER", "N", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", ""
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "SWR" => {
                                statements.swr_stmt.execute(rusqlite::params![
                                    self.file_id, "SWR", "0000000", "1", "00000001", "PLACEHOLDER", "WRITER", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", ""
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "ALT" => {
                                statements.alt_stmt.execute(rusqlite::params![
                                    self.file_id, "ALT", "0000000", "1", "PLACEHOLDER TITLE", "AT", "EN"
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "PER" => {
                                statements.per_stmt.execute(rusqlite::params![
                                    self.file_id, "PER", "0000000", "1", "PLACEHOLDER PERFORMER", "", "", ""
                                ])?;
                                tx.last_insert_rowid()
                            }
                            "REC" => {
                                statements.rec_stmt.execute(rusqlite::params![
                                    self.file_id, "REC", "0000000", "1", "", "", "", "", "PLACEHOLDER ALBUM", "PLACEHOLDER LABEL", "", "", "", "", "", "", "PLACEHOLDER RECORDING", "", "", "", "", ""
                                ])?;
                                tx.last_insert_rowid()
                            }
                            _ => 1
                        }
                    }
                    _ => 1 // For other record types, use placeholder
                };

                // Insert into file_line table for tracking
                insert_file_line_record(&mut statements.file_stmt, self.file_id, parsed_record.line_number, record_type, record_id)?;
            }
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
        
        // Use working test data - focus on demonstrating the scope of missing functionality
        writeln!(file, "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221").unwrap();
        writeln!(file, "GRHAGR0000102.10            ").unwrap();
        writeln!(file, "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ").unwrap();
        
        // Add a few more key record types to demonstrate the scope
        writeln!(file, "SPU0000000100000002000000011357924680SAMPLE PUBLISHER                    N  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "SWR0000000100000003000000013579SAMPLE WRITER              JOHN            A  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "ALT0000000100000004ALTERNATE TITLE                         AT EN           ").unwrap();
        writeln!(file, "PER0000000100000005SAMPLE PERFORMER            1357924680123456789012345678").unwrap();
        writeln!(file, "REC0000000100000006         0000000SAMPLE ALBUM                      SAMPLE LABEL         1234567890123EAN1234567890ISRCCD   SAMPLE RECORDING             SAMPLE VERSION               SAMPLE ARTIST                SAMPLE RECORD LABEL          Y12345678901234567890").unwrap();
        
        // Add many unhandled record types to show the scope of missing functionality
        writeln!(file, "ACK0000000100000007202212211254110000001TRK                                                 20221221A").unwrap();
        writeln!(file, "AGR0000000100000008AGR12345                    C 20221221                                                                                                                                                                                                                  ").unwrap();
        writeln!(file, "ARI0000000100000009123456789012345678901234567890000000PA                ").unwrap();
        writeln!(file, "COM0000000100000010COMPONENT WORK                                         12345670000000WRITER1             JOHN            1234567890123456WRITER2             JANE            1234567890123456123456789012345612345678901234561234567890123456").unwrap();
        writeln!(file, "EWT0000000100000011ENTIRE WORK TITLE                                      EN ENTIRE WORK WRITER   JOHN            SRC 1234567890123456123456789012345SECOND WRITER        JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "IND0000000100000012PI 005").unwrap();
        writeln!(file, "INS0000000100000013010ST                                    ").unwrap();
        writeln!(file, "IPA0000000100000014E01234567890123456123456789012345678901234567890INTERESTED PARTY        FIRST               123456789012345678901234567890         123456789012345678901234567890         123456789012345678901234567890         ").unwrap();
        writeln!(file, "MSG000000010000001501 1HDR000SAMPLE MESSAGE TEXT                                                                                                                                                                                                                                                                                      ").unwrap();
        writeln!(file, "NAT000000010000001601NATIONAL TITLE                         NT EN           ").unwrap();
        writeln!(file, "NET000000010000001701NET TITLE                              EN              ").unwrap();
        writeln!(file, "NOW000000010000001801NOW WRITER NAME         FIRST           EN  1").unwrap();
        writeln!(file, "NPA000000010000001901123456789012345678901234567890INTERESTED PARTY NAME           FIRST           EN  ").unwrap();
        writeln!(file, "NPN000000010000002001001123456789012345678901234567890PUBLISHER NAME                  EN              ").unwrap();
        writeln!(file, "NPR000000010000002101PERFORMING ARTIST       FIRST           1234567890123456123456789012345EN  EN  EN  ").unwrap();
        writeln!(file, "NWN000000010000002201123456789012345678901234567890WRITER NAME             FIRST           EN              ").unwrap();
        writeln!(file, "ORN000000010000002301LSAMPLE PRODUCTION                                                                                                                                                                                    2022123456789012345678901234567890123456789012345612345678901234561234567890123456").unwrap();
        writeln!(file, "PWR000000010000002401123456789012345678901234567890PUBLISHER NAME                                            123456789012345678901234567890001").unwrap();
        writeln!(file, "SPT000000010000002501123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "SWT000000010000002601123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "TER000000010000002701I000000000").unwrap();
        writeln!(file, "VER000000010000002801VERSION WORK TITLE                                   12345670000000EN VERSION WRITER       JOHN            SRC 1234567890123456123456789012345VERSION WRITER 2     JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "XRF000000010000002901123456789012345ABCD1234567890123456789012345678901234567890YV ").unwrap();
        
        writeln!(file, "GRT000010000000010000027").unwrap();
        writeln!(file, "TRL00001000000010000027").unwrap();
        
        // Process the file
        let (file_id, processed_count, _report) = process_cwr_to_sqlite(
            cwr_file_path.to_str().unwrap(),
            db_file_path.to_str().unwrap()
        ).unwrap();
        
        // Verify processing happened  
        assert_eq!(processed_count, 33, "Should have processed 33 records");
        
        // Connect to database and verify records were actually inserted
        let conn = rusqlite::Connection::open(&db_file_path).unwrap();
        
        // Check file_line table - should have entries for each record type
        let mut stmt = conn.prepare("SELECT record_type, COUNT(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type").unwrap();
        let rows: std::result::Result<Vec<(String, i64)>, rusqlite::Error> = stmt.query_map([file_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        }).unwrap().collect();
        
        let record_counts = rows.unwrap();
        println!("Record counts from file_line table: {:?}", record_counts);
        
        // Should have one of each record type (all 31 types from records/mod.rs)
        let expected_records = vec![
            ("ACK".to_string(), 1i64),
            ("AGR".to_string(), 1i64),
            ("ALT".to_string(), 1i64),
            ("ARI".to_string(), 1i64),
            ("COM".to_string(), 1i64),
            ("EWT".to_string(), 1i64),
            ("GRH".to_string(), 1i64),
            ("GRT".to_string(), 1i64),
            ("HDR".to_string(), 1i64),
            ("IND".to_string(), 1i64),
            ("INS".to_string(), 1i64),
            ("IPA".to_string(), 1i64),
            ("MSG".to_string(), 1i64),
            ("NAT".to_string(), 1i64),
            ("NET".to_string(), 1i64),
            ("NOW".to_string(), 1i64),
            ("NPA".to_string(), 1i64),
            ("NPN".to_string(), 1i64),
            ("NPR".to_string(), 1i64),
            ("NWN".to_string(), 1i64),
            ("NWR".to_string(), 1i64),
            ("ORN".to_string(), 1i64),
            ("PER".to_string(), 1i64),
            ("PWR".to_string(), 1i64),
            ("REC".to_string(), 1i64),
            ("SPT".to_string(), 1i64),
            ("SPU".to_string(), 1i64),
            ("SWR".to_string(), 1i64),
            ("SWT".to_string(), 1i64),
            ("TER".to_string(), 1i64),
            ("TRL".to_string(), 1i64),
            ("VER".to_string(), 1i64),
            ("XRF".to_string(), 1i64),
        ];
        assert_eq!(record_counts, expected_records, "file_line table should track all record types");
        
        // MORE IMPORTANTLY: Check that actual record data was inserted into specific tables
        
        // Check HDR table
        let hdr_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_hdr WHERE file_id = ?1", 
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(hdr_count, 1, "HDR table should have 1 record");
        
        // Check GRH table  
        let grh_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_grh WHERE file_id = ?1",
            [file_id],
            |row| row.get(0)
        ).unwrap();
        assert_eq!(grh_count, 1, "GRH table should have 1 record");
        
        // Check NWR table
        let nwr_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_nwr WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(nwr_count, 1, "NWR table should have 1 record");
        
        // Demonstrate that most record types are NOT being inserted into their specific tables
        
        // Check SPU table - should fail because not implemented
        let spu_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_spu WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(spu_count, 0, "SPU table should be EMPTY - not implemented yet!");
        
        // Check SWR table - should fail because not implemented  
        let swr_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_swr WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(swr_count, 0, "SWR table should be EMPTY - not implemented yet!");
        
        // Check ALT table - should fail because not implemented
        let alt_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_alt WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(alt_count, 0, "ALT table should be EMPTY - not implemented yet!");
        
        // Check PER table - should fail because not implemented
        let per_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_per WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(per_count, 0, "PER table should be EMPTY - not implemented yet!");
        
        // Check REC table - should fail because not implemented
        let rec_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM cwr_rec WHERE file_id = ?1",
            [file_id], 
            |row| row.get(0)
        ).unwrap();
        assert_eq!(rec_count, 0, "REC table should be EMPTY - not implemented yet!");
        
        // Verify the HDR record actually contains the parsed data
        let (sender_name, creation_date): (String, String) = conn.query_row(
            "SELECT sender_name, creation_date FROM cwr_hdr WHERE file_id = ?1",
            [file_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        ).unwrap();
        
        assert_eq!(sender_name, "PLACEHOLDER", "HDR should contain placeholder sender name");
        assert_eq!(creation_date, "20221221", "HDR should contain placeholder creation date");
        
        println!("🚨 This test demonstrates the missing functionality!");
        println!("📊 Records tracked in file_line: {} types", record_counts.len());
        println!("✅ Only HDR, GRH, GRT, TRL, NWR are currently handled");
        println!("❌ Missing: SPU, SWR, ALT, PER, REC, ACK, AGR, ARI, COM, EWT, IND, INS, IPA, MSG, NAT, NET, NOW, NPA, NPN, NPR, NWN, ORN, PWR, SPT, SWT, TER, VER, XRF");
    }
}
