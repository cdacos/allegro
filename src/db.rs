use crate::error::CwrParseError;
use rusqlite::{params, Connection, Statement};
use std::path::Path;

// Structure to hold all prepared statements
pub struct PreparedStatements<'conn> {
    pub error_stmt: Statement<'conn>,
    pub file_stmt: Statement<'conn>,
    pub hdr_stmt: Statement<'conn>,
    pub grh_stmt: Statement<'conn>,
    pub grt_stmt: Statement<'conn>,
    pub trl_stmt: Statement<'conn>,
    pub agr_stmt: Statement<'conn>,
    pub nwr_stmt: Statement<'conn>, // Covers NWR, REV, ISW, EXC
    pub ack_stmt: Statement<'conn>,
    pub ter_stmt: Statement<'conn>,
    pub ipa_stmt: Statement<'conn>,
    pub npa_stmt: Statement<'conn>,
    pub spu_stmt: Statement<'conn>, // Covers SPU, OPU
    pub npn_stmt: Statement<'conn>,
    pub spt_stmt: Statement<'conn>, // Covers SPT, OPT
    pub swr_stmt: Statement<'conn>, // Covers SWR, OWR
    pub nwn_stmt: Statement<'conn>,
    pub swt_stmt: Statement<'conn>, // Covers SWT, OWT
    pub pwr_stmt: Statement<'conn>,
    pub alt_stmt: Statement<'conn>,
    pub nat_stmt: Statement<'conn>,
    pub ewt_stmt: Statement<'conn>,
    pub ver_stmt: Statement<'conn>,
    pub per_stmt: Statement<'conn>,
    pub npr_stmt: Statement<'conn>,
    pub rec_stmt: Statement<'conn>,
    pub orn_stmt: Statement<'conn>,
    pub ins_stmt: Statement<'conn>,
    pub ind_stmt: Statement<'conn>,
    pub com_stmt: Statement<'conn>,
    pub msg_stmt: Statement<'conn>,
    pub net_stmt: Statement<'conn>, // Covers NET, NCT, NVT
    pub now_stmt: Statement<'conn>,
    pub ari_stmt: Statement<'conn>,
    pub xrf_stmt: Statement<'conn>,
}


pub fn determine_db_filename(input_filename: &str) -> String {
    let mut n = 0;
    let mut db_filename = format!("{}.db", input_filename);
    while Path::new(&db_filename).exists() {
        n += 1;
        db_filename = format!("{}.{}.db", input_filename, n);
    }
    db_filename
}

pub fn setup_database(db_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Schema is embedded directly into the binary at compile time
    const SCHEMA_SQL: &str = include_str!("../docs/cwr_2.2_schema_sqlite.sql");
    let conn = Connection::open(db_filename)?;

    // Check if tables already exist to avoid erroring on re-runs
    let table_count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'",
        [],
        |row| row.get(0),
    )?;

    if table_count == 0 {
        println!("Applying embedded schema...");
        conn.execute_batch(SCHEMA_SQL)?;
    } else {
        println!("Database schema already exists.");
    }

    Ok(())
}

/// Inserts a record into the 'error' table using a prepared statement.
pub fn log_error(
    error_stmt: &mut Statement, // Changed to take the whole struct
    line_number: usize,
    description: String,
) -> Result<(), rusqlite::Error> {
    error_stmt.execute(params![line_number as i64, description])?;
    Ok(())
}


/// Inserts a record into the 'file' table using a prepared statement.
pub fn insert_file_record(
    file_stmt: &mut Statement, // Changed to take the whole struct
    line_number: usize,
    record_type: &str,
    record_id: i64,
) -> Result<(), CwrParseError> {
    file_stmt.execute(params![line_number as i64, record_type, record_id])?;
    Ok(())
}
