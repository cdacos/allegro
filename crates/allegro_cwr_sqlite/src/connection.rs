use crate::error::CwrDbError;
use rusqlite::Connection;
use std::path::Path;

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
pub fn determine_db_filename(input_filename: &str, output_path: Option<&str>) -> String {
    match output_path {
        Some(path) => path.to_string(),
        None => {
            // Find next available filename with increment
            let mut n = 0;
            let mut db_filename = format!("{}.db", input_filename);
            while Path::new(&db_filename).exists() {
                n += 1;
                db_filename = format!("{}.{}.db", input_filename, n);
            }
            db_filename
        }
    }
}

/// Sets up the CWR database schema
pub fn setup_database(db_filename: &str) -> Result<(), CwrDbError> {
    setup_database_with_overwrite(db_filename, false)
}

/// Sets up the CWR database schema with optional overwrite
pub fn setup_database_with_overwrite(db_filename: &str, overwrite: bool) -> Result<(), CwrDbError> {
    // Schema is embedded directly into the binary at compile time
    const SCHEMA_SQL: &str = include_str!("schema.sql");
    
    if overwrite && Path::new(db_filename).exists() {
        println!("Removing existing database file...");
        std::fs::remove_file(db_filename)?;
    }
    
    let conn = Connection::open(db_filename)?;

    // Check if tables already exist to avoid erroring on re-runs
    let table_count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'", 
        [], 
        |row| row.get(0)
    )?;

    if table_count == 0 {
        println!("Applying embedded schema...");
        conn.execute_batch(SCHEMA_SQL)?;
    } else if overwrite {
        println!("Clearing existing data and reapplying schema...");
        // Drop and recreate all tables to ensure clean state
        conn.execute_batch("DROP TABLE IF EXISTS error")?;
        conn.execute_batch("DROP TABLE IF EXISTS file")?;
        // Drop all CWR tables
        let table_names: Vec<String> = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'")?
            .query_map([], |row| Ok(row.get::<_, String>(0)?))?
            .collect::<Result<Vec<_>, _>>()?;
        
        for table_name in table_names {
            conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), [])?;
        }
        
        conn.execute_batch(SCHEMA_SQL)?;
    } else {
        println!("Database schema already exists.");
    }

    Ok(())
}