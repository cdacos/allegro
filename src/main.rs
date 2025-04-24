// --- START OF FILE main.rs.txt ---

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;

// Import rusqlite features
// Added Transaction explicitly for clarity in function signatures
use rusqlite::{params, Connection, Transaction};

// --- Configuration ---
const SCHEMA_FILE_PATH: &str = "docs/cwr_2.2_schema_sqlite.sql";
// ---------------------

// Define a custom error type (same as before)
#[derive(Debug)]
enum CwrParseError {
    Io(io::Error),
    Db(rusqlite::Error),
    BadFormat(String),
}

impl From<io::Error> for CwrParseError {
    fn from(err: io::Error) -> CwrParseError {
        CwrParseError::Io(err)
    }
}

impl From<rusqlite::Error> for CwrParseError {
    fn from(err: rusqlite::Error) -> CwrParseError {
        CwrParseError::Db(err)
    }
}

impl std::fmt::Display for CwrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CwrParseError::Io(err) => write!(f, "IO Error: {}", err),
            CwrParseError::Db(err) => write!(f, "Database Error: {}", err),
            CwrParseError::BadFormat(msg) => write!(f, "Bad Format: {}", msg),
        }
    }
}

impl std::error::Error for CwrParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CwrParseError::Io(err) => Some(err),
            CwrParseError::Db(err) => Some(err),
            CwrParseError::BadFormat(_) => None,
        }
    }
}


fn main() {
    // 1. Get input filename
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        eprintln!("This will create or use a database named <input_filename>.db (or .N.db if needed)");
        process::exit(1);
    }
    let input_filename = &args[1];

    // 2. Determine database filename
    let db_filename = determine_db_filename(input_filename);
    println!("Using database filename: '{}'", db_filename);

    // 3. Set up the database
    match setup_database(&db_filename, SCHEMA_FILE_PATH) {
        Ok(_) => println!(
            "Database '{}' ready, schema applied if needed.",
            db_filename
        ),
        Err(e) => {
            eprintln!(
                "Error setting up database '{}' with schema '{}': {}",
                db_filename, SCHEMA_FILE_PATH, e
            );
            process::exit(1);
        }
    }

    // 4. Process the file and load into DB
    match process_and_load_file(input_filename, &db_filename) {
        Ok(count) => {
            println!(
                "Successfully processed {} lines from '{}' into '{}'.",
                count, input_filename, db_filename
            );
        }
        Err(e) => {
            eprintln!(
                "Error processing file '{}' into '{}': {}",
                input_filename, db_filename, e
            );
            // Attempt to delete the potentially partially filled DB on error
            // This might fail, but it's often good practice
            match std::fs::remove_file(&db_filename) {
                Ok(_) => eprintln!("Removed incomplete database file '{}'.", db_filename),
                Err(remove_err) => eprintln!("Failed to remove incomplete database file '{}': {}", db_filename, remove_err),
            }
            process::exit(1);
        }
    }
}

fn determine_db_filename(input_filename: &str) -> String {
    let mut n = 0;
    let mut db_filename = format!("{}.db", input_filename);

    while Path::new(&db_filename).exists() {
        n += 1;
        db_filename = format!("{}.{}.db", input_filename, n);
    }
    db_filename
}

/// Reads the CWR file line by line and inserts data into the database using a single transaction.
fn process_and_load_file(input_filename: &str, db_filename: &str) -> Result<usize, CwrParseError> {
    let file = File::open(input_filename)?;
    let reader = BufReader::new(file);
    // Open connection mutable because we need to start a transaction
    let mut conn = Connection::open(db_filename)?;

    // Start a single transaction for the entire file load
    // `tx` needs to be mutable because execute methods on it take `&mut self` implicitly
    let mut tx = conn.transaction()?;

    let mut line_number = 0;
    let mut processed_lines = 0; // Count only lines that were successfully parsed and inserted

    for line_result in reader.lines() {
        line_number += 1;
        let line = line_result?; // Propagate I/O errors

        if line.is_empty() {
            continue;
        }

        if line.len() < 3 {
            eprintln!("Warning: Line {} is too short (less than 3 chars), skipping.", line_number);
            continue;
        }

        let record_type = &line[0..3];

        // Pass a mutable reference to the transaction (&mut tx) to the parsing functions
        match record_type {
            "HDR" => {
                // Use block to limit scope of borrow if needed, although not strictly necessary here
                parse_and_insert_hdr(&line, line_number, &mut tx)?;
                processed_lines += 1;
            }
            // --- Add cases for other record types later ---
            // "GRH" => {
            //     parse_and_insert_grh(&line, line_number, &mut tx)?;
            //     processed_lines += 1;
            // }
            // ... etc.
            _ => {
                // eprintln!("Warning: Line {}: Unrecognized record type '{}', skipping.", line_number, record_type);
            }
        }
    }

    // Commit the transaction *only if* all lines were processed without error
    // If any `?` propagated an error, `tx.commit()` will not be reached,
    // and the transaction will be automatically rolled back when `tx` goes out of scope.
    tx.commit()?;

    // Return the count of successfully processed lines
    Ok(processed_lines)
}

/// Parses a HDR line and inserts it into the cwr_hdr table within the provided transaction.
// Takes a mutable reference to the Transaction
fn parse_and_insert_hdr(line: &str, line_number: usize, tx: &mut Transaction) -> Result<(), CwrParseError> {
    // Helper closure for safe slicing (same as before)
    let safe_slice = |start: usize, end: usize| -> Option<&str> {
        if end > line.len() {
            None
        } else {
            let slice = line.get(start..end)?;
            if slice.is_empty() { None } else { Some(slice) }
        }
    };

    // Define field positions (0-based start, end=start+size)
    let record_type = safe_slice(0, 3).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing record_type", line_number)))?;
    let sender_type = safe_slice(3, 5).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing sender_type", line_number)))?;
    let sender_id = safe_slice(5, 14).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing sender_id", line_number)))?;
    let sender_name = safe_slice(14, 59).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing sender_name", line_number)))?;
    let edi_version = safe_slice(59, 64).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing edi_standard_version_number", line_number)))?;
    let creation_date = safe_slice(64, 72).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing creation_date", line_number)))?;
    let creation_time = safe_slice(72, 78).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing creation_time", line_number)))?;
    let transmission_date = safe_slice(78, 86).ok_or_else(|| CwrParseError::BadFormat(format!("Line {}: HDR missing transmission_date", line_number)))?;
    let character_set = safe_slice(86, 101);
    let version = safe_slice(101, 104);
    let revision = safe_slice(104, 107);
    let software_package = safe_slice(107, 137);
    let software_package_version = safe_slice(137, 167);

    // Basic Validation
    if record_type != "HDR" {
        return Err(CwrParseError::BadFormat(format!(
            "Line {}: Expected record type HDR but found '{}'", line_number, record_type
        )));
    }

    // --- More Validation Examples (Optional but Recommended) ---
    // Date format check (YYYYMMDD)
    if creation_date.len() != 8 || !creation_date.chars().all(|c| c.is_ascii_digit()) {
        // Note: This is a basic check. A regex or date parsing library offers more robust validation.
        // Consider adding validation for transmission_date as well.
        return Err(CwrParseError::BadFormat(format!("Line {}: Invalid Creation Date format '{}'", line_number, creation_date)));
    }
    // Time format check (HHMMSS)
    if creation_time.len() != 6 || !creation_time.chars().all(|c| c.is_ascii_digit()) {
        return Err(CwrParseError::BadFormat(format!("Line {}: Invalid Creation Time format '{}'", line_number, creation_time)));
    }
    // Could add checks for Sender Type against allowed values ('PB', 'SO', etc.) if needed

    // Insert into the database *using the transaction*
    tx.execute( // Changed from conn.execute to tx.execute
                "INSERT INTO cwr_hdr (
            file_line_number, record_type, sender_type, sender_id, sender_name,
            edi_standard_version_number, creation_date, creation_time, transmission_date,
            character_set, version, revision, software_package, software_package_version
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                params![
            line_number as i64, // Store line number as integer
            record_type,
            sender_type,
            sender_id,
            sender_name,
            edi_version,
            creation_date,
            creation_time,
            transmission_date,
            character_set,
            version,
            revision,
            software_package,
            software_package_version
        ],
    )?; // Propagate DB errors

    Ok(())
}

fn setup_database(db_filename: &str, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(schema_path).exists() {
        return Err(format!("Schema file not found: {}", schema_path).into());
    }
    let schema_sql = fs::read_to_string(schema_path)?;
    let conn = Connection::open(db_filename)?;
    conn.execute_batch(&schema_sql)?;
    Ok(())
}
// --- END OF FILE main.rs.txt ---