use crate::error::CwrDbError;
use allegro_cwr_cli::find_next_available_filename;
use log::info;
use rusqlite::Connection;

/// Main database manager for CWR operations
pub struct CwrDatabase {
    connection: Connection,
}

impl CwrDatabase {
    /// Open or create a CWR database
    pub fn open(db_filename: &str) -> Result<Self, CwrDbError> {
        let conn = Connection::open(db_filename)?;
        Ok(CwrDatabase { connection: conn })
    }

    /// Get a reference to the underlying connection
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    /// Get a mutable reference to the underlying connection
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }
}

/// Determines database filename based on input and optional output path
/// This function is for single-file processing when no -o is specified
pub fn determine_db_filename(input_filename: &str, output_path: Option<&str>) -> String {
    match output_path {
        Some(path) => path.to_string(),
        None => {
            // For auto-generated database filenames, we want to avoid overwriting existing files
            let base_name = format!("{}.db", input_filename);
            if !std::path::Path::new(&base_name).exists() {
                base_name
            } else {
                find_next_available_filename(&base_name, 1)
            }
        }
    }
}

/// Sets up the CWR database schema
pub fn setup_database(db_filename: &str) -> Result<(), CwrDbError> {
    // Schema is embedded directly into the binary at compile time
    const SCHEMA_SQL: &str = include_str!("schema.sql");

    let conn = Connection::open(db_filename)?;

    // Check if tables already exist to avoid erroring on re-runs
    let table_count: i64 =
        conn.query_row("SELECT count(*) FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'", [], |row| {
            row.get(0)
        })?;

    if table_count == 0 {
        info!("Applying embedded schema");
        conn.execute_batch(SCHEMA_SQL)?;
    } else {
        info!("Database schema already exists, ready for import");
    }

    Ok(())
}
