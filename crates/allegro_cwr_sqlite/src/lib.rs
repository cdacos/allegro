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

use domain_conversions::{CwrToSqlInt, CwrToSqlString, opt_domain_to_string};

/// Trait for inserting CWR records into SQLite
pub trait SqliteInsertable {
    /// Get the table name for this record type (e.g., "cwr_hdr")
    fn table_name(&self) -> &'static str;
    
    /// Convert record fields to SQL parameters
    fn to_sql_params(&self, file_id: i64) -> Vec<Box<dyn rusqlite::types::ToSql>>;
    
    /// Execute insertion using appropriate prepared statement
    fn execute_insert(&self, statements: &mut PreparedStatements, tx: &rusqlite::Transaction, file_id: i64) -> Result<i64>;
}

/// Trait for querying CWR records from SQLite
pub trait SqliteQueryable: Sized {
    /// Get the table name for this record type
    fn table_name() -> &'static str;
    
    /// Construct a record from a SQL row
    fn from_sql_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
}

// Implementation of SqliteInsertable for CwrRegistry - this centralizes the 33-case match logic
impl SqliteInsertable for allegro_cwr::CwrRegistry {
    fn table_name(&self) -> &'static str {
        match self {
            allegro_cwr::CwrRegistry::Hdr(_) => "cwr_hdr",
            allegro_cwr::CwrRegistry::Grh(_) => "cwr_grh",
            allegro_cwr::CwrRegistry::Grt(_) => "cwr_grt",
            allegro_cwr::CwrRegistry::Trl(_) => "cwr_trl",
            allegro_cwr::CwrRegistry::Agr(_) => "cwr_agr",
            allegro_cwr::CwrRegistry::Nwr(_) => "cwr_nwr",
            allegro_cwr::CwrRegistry::Ack(_) => "cwr_ack",
            allegro_cwr::CwrRegistry::Ter(_) => "cwr_ter",
            allegro_cwr::CwrRegistry::Ipa(_) => "cwr_ipa",
            allegro_cwr::CwrRegistry::Npa(_) => "cwr_npa",
            allegro_cwr::CwrRegistry::Spu(_) => "cwr_spu",
            allegro_cwr::CwrRegistry::Npn(_) => "cwr_npn",
            allegro_cwr::CwrRegistry::Spt(_) => "cwr_spt",
            allegro_cwr::CwrRegistry::Swr(_) => "cwr_swr",
            allegro_cwr::CwrRegistry::Nwn(_) => "cwr_nwn",
            allegro_cwr::CwrRegistry::Swt(_) => "cwr_swt",
            allegro_cwr::CwrRegistry::Pwr(_) => "cwr_pwr",
            allegro_cwr::CwrRegistry::Alt(_) => "cwr_alt",
            allegro_cwr::CwrRegistry::Nat(_) => "cwr_nat",
            allegro_cwr::CwrRegistry::Ewt(_) => "cwr_ewt",
            allegro_cwr::CwrRegistry::Ver(_) => "cwr_ver",
            allegro_cwr::CwrRegistry::Per(_) => "cwr_per",
            allegro_cwr::CwrRegistry::Npr(_) => "cwr_npr",
            allegro_cwr::CwrRegistry::Rec(_) => "cwr_rec",
            allegro_cwr::CwrRegistry::Orn(_) => "cwr_orn",
            allegro_cwr::CwrRegistry::Ins(_) => "cwr_ins",
            allegro_cwr::CwrRegistry::Ind(_) => "cwr_ind",
            allegro_cwr::CwrRegistry::Com(_) => "cwr_com",
            allegro_cwr::CwrRegistry::Msg(_) => "cwr_msg",
            allegro_cwr::CwrRegistry::Net(_) => "cwr_net",
            allegro_cwr::CwrRegistry::Now(_) => "cwr_now",
            allegro_cwr::CwrRegistry::Ari(_) => "cwr_ari",
            allegro_cwr::CwrRegistry::Xrf(_) => "cwr_xrf",
        }
    }
    
    fn to_sql_params(&self, _file_id: i64) -> Vec<Box<dyn rusqlite::types::ToSql>> {
        // This will be implemented via execute_insert for now
        // since the parameter structure varies significantly between record types
        vec![]
    }
    
    fn execute_insert(&self, statements: &mut PreparedStatements, tx: &rusqlite::Transaction, file_id: i64) -> Result<i64> {
        use rusqlite::params;
        
        match self {
            allegro_cwr::CwrRegistry::Hdr(hdr) => {
                statements.hdr_stmt.execute(params![
                    file_id,
                    "HDR",
                    hdr.sender_type.as_str(),
                    hdr.sender_id.as_str(),
                    hdr.sender_name.as_str(),
                    hdr.edi_standard_version_number.as_str(),
                    hdr.creation_date.as_str(),
                    hdr.creation_time.as_str(),
                    hdr.transmission_date.as_str(),
                    opt_domain_to_string(&hdr.character_set),
                    hdr.version.as_str(),
                    hdr.revision.as_str(),
                    hdr.software_package,
                    hdr.software_package_version
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Grh(grh) => {
                statements.grh_stmt.execute(params![
                    file_id, 
                    "GRH", 
                    grh.transaction_type.to_sql_string(), 
                    grh.group_id.to_sql_int(), 
                    grh.version_number.as_str(), 
                    grh.batch_request, 
                    grh.submission_distribution_type
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Grt(grt) => {
                statements.grt_stmt.execute(params![
                    file_id,
                    "GRT",
                    grt.group_id.to_sql_int(),
                    grt.transaction_count.to_sql_int(),
                    grt.record_count.to_sql_int(),
                    grt.currency_indicator.to_sql_string(),
                    grt.total_monetary_value.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Trl(trl) => {
                statements.trl_stmt.execute(params![
                    file_id,
                    "TRL",
                    trl.group_count.to_sql_int(),
                    trl.transaction_count.to_sql_int(),
                    trl.record_count.to_sql_int()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            // TODO: Add remaining 30 record types...
            // For now, let's implement just a few to demonstrate the pattern
            _ => {
                return Err(CwrDbError::Setup("Record type not yet implemented in trait".to_string()));
            }
        }
    }
}

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
                // Use the trait method to execute the insertion - replaces 434 lines of match statement!
                let record_id = parsed_record.record.execute_insert(statements, tx, self.file_id)?;

                // Insert into file_line table for tracking
                insert_file_line_record(&mut statements.file_stmt, self.file_id, parsed_record.line_number, parsed_record.record.record_type(), record_id)?;
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

    fn handle_warnings(&mut self, line_number: usize, record_type: &str, warnings: &[String]) -> std::result::Result<(), Self::Error> {
        if warnings.is_empty() {
            return Ok(());
        }

        self.start_batch()?;

        if let Some(ref mut statements) = self.statements {
            for warning in warnings {
                // Store warnings in the error table with "WARNING:" prefix to distinguish from errors
                let warning_description = format!("WARNING [{}]: {}", record_type, warning);
                log_error(&mut statements.error_stmt, self.file_id, line_number, warning_description)?;
                self.error_count += 1;
            }
        }

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
        writeln!(
            file,
            "ORN000000010000002301LSAMPLE PRODUCTION                                                                                                                                                                                    2022123456789012345678901234567890123456789012345612345678901234561234567890123456"
        )
        .unwrap();
        writeln!(file, "PWR000000010000002401123456789012345678901234567890PUBLISHER NAME                                            123456789012345678901234567890001").unwrap();
        writeln!(file, "SPT000000010000002501123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "SWT000000010000002601123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "TER000000010000002701I000000000").unwrap();
        writeln!(file, "VER000000010000002801VERSION WORK TITLE                                   12345670000000EN VERSION WRITER       JOHN            SRC 1234567890123456123456789012345VERSION WRITER 2     JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "XRF000000010000002901123456789012345ABCD1234567890123456789012345678901234567890YV ").unwrap();

        writeln!(file, "GRT000010000000010000027").unwrap();
        writeln!(file, "TRL00001000000010000027").unwrap();

        // Process the file
        let (file_id, processed_count, _report) = process_cwr_to_sqlite(cwr_file_path.to_str().unwrap(), db_file_path.to_str().unwrap()).unwrap();

        // Verify processing happened
        assert_eq!(processed_count, 33, "Should have processed 33 records");

        // Connect to database and verify records were actually inserted
        let conn = rusqlite::Connection::open(&db_file_path).unwrap();

        // Check file_line table - should have entries for each record type
        let mut stmt = conn.prepare("SELECT record_type, COUNT(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type").unwrap();
        let rows: std::result::Result<Vec<(String, i64)>, rusqlite::Error> = stmt.query_map([file_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))).unwrap().collect();

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

        // Get all cwr_ table names dynamically
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%' ORDER BY name").unwrap();
        let table_names: Vec<String> = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?)).unwrap().collect::<std::result::Result<Vec<_>, _>>().unwrap();

        println!("Found {} cwr_ tables: {:?}", table_names.len(), table_names);

        // Check count in each cwr_ table
        let mut total_records_in_tables = 0i64;
        let mut implemented_tables = Vec::new();
        let mut unimplemented_tables = Vec::new();

        for table_name in &table_names {
            let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM {} WHERE file_id = ?1", table_name), [file_id], |row| row.get(0)).unwrap();

            total_records_in_tables += count;

            if count == 1 {
                implemented_tables.push(table_name.clone());
                println!("✅ {}: {} records", table_name, count);
            } else {
                unimplemented_tables.push(table_name.clone());
                println!("❌ {}: {} records (not implemented)", table_name, count);
            }
        }

        // The key assertion: total should equal 33 when all are implemented
        println!("📊 Total records in cwr_ tables: {} / 33", total_records_in_tables);
        assert_eq!(total_records_in_tables, 33, "Should be 33 one for each record type");

        // Verify we have the expected number of tables (should be 33 corresponding to all record types)
        assert_eq!(table_names.len(), 33, "Should have 33 cwr_ tables for all record types");

        // Verify the HDR record actually contains the parsed data
        let (sender_name, creation_date): (String, String) = conn.query_row("SELECT sender_name, creation_date FROM cwr_hdr WHERE file_id = ?1", [file_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))).unwrap();

        assert_eq!(sender_name, "WARNER CHAPPELL MUSIC PUBLISHING LTD", "HDR should contain actual parsed sender name");
        assert_eq!(creation_date, "20221221", "HDR should contain parsed creation date");

        println!("🚨 This test demonstrates the missing functionality!");
        println!("📊 Records tracked in file_line: {} types", record_counts.len());
        println!("✅ Implemented tables: {:?}", implemented_tables);
        println!("❌ Unimplemented tables: {:?}", unimplemented_tables);
        println!("🎯 Goal: All 33 record types should sum to 33 total records in cwr_ tables");
    }
}
