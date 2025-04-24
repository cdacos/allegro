use std::env;
use std::fs; // Use fs for reading the schema file
use std::path::Path;
use std::process;

// Import necessary items from rusqlite
use rusqlite::Connection;

// --- Configuration ---
// Define the path to the schema file relative to the project root
const SCHEMA_FILE_PATH: &str = "docs/cwr_2.2_schema_sqlite.sql";
// ---------------------

fn main() {
    // 1. Get input filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        eprintln!("This will create a database named <input_filename>.db");
        process::exit(1);
    }
    let input_filename = &args[1];

    // 2. Construct the database filename
    let db_filename = format!("{}.db", input_filename);

    // 3. Set up the database
    match setup_database(&db_filename, SCHEMA_FILE_PATH) {
        Ok(_) => {
            println!(
                "Successfully created database '{}' and applied schema from '{}'.",
                db_filename, SCHEMA_FILE_PATH
            );
        }
        Err(e) => {
            eprintln!(
                "Error setting up database '{}' with schema '{}': {}",
                db_filename, SCHEMA_FILE_PATH, e
            );
            process::exit(1);
        }
    }
}

/// Creates an SQLite database file and executes the schema definition script.
///
/// # Arguments
///
/// * `db_filename` - The path where the SQLite database file should be created.
/// * `schema_path` - The path to the SQL file containing the schema definition.
///
/// # Errors
///
/// Returns an error if:
/// - The schema file cannot be read.
/// - The database connection cannot be established.
/// - The schema SQL cannot be executed successfully.
fn setup_database(db_filename: &str, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if schema file exists before proceeding
    if !Path::new(schema_path).exists() {
        return Err(format!("Schema file not found: {}", schema_path).into());
    }

    // Read the schema SQL from the file
    let schema_sql = fs::read_to_string(schema_path)?;
    println!("Read schema from '{}'", schema_path);

    // Open (or create) the SQLite database connection
    // `Connection::open` creates the file if it doesn't exist.
    let conn = Connection::open(db_filename)?;
    println!("Opened/Created database file '{}'", db_filename);

    // Execute the schema SQL script
    // `execute_batch` is suitable for running multiple SQL statements from a string
    conn.execute_batch(&schema_sql)?;
    println!("Successfully executed schema SQL.");

    // Connection will be closed automatically when `conn` goes out of scope.
    Ok(())
}
