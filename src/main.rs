use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;
use std::time::Instant;

use rusqlite::{params, Connection, Transaction};

// --- Configuration ---
const SCHEMA_FILE_PATH: &str = "docs/cwr_2.2_schema_sqlite.sql";
// ---------------------

#[derive(Debug)]
enum CwrParseError {
    Io(io::Error),
    Db(rusqlite::Error),
    BadFormat(String),
}

impl From<io::Error> for CwrParseError {
    fn from(err: io::Error) -> CwrParseError { CwrParseError::Io(err) }
}

impl From<rusqlite::Error> for CwrParseError {
    fn from(err: rusqlite::Error) -> CwrParseError { CwrParseError::Db(err) }
}

impl std::fmt::Display for CwrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CwrParseError::Io(err) => write!(f, "IO Error: {}", err),
            CwrParseError::Db(err) => write!(f, "Database Error: {}", err),
            CwrParseError::BadFormat(msg) => write!(f, "{}", msg),
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        eprintln!("This will create or use a database named <input_filename>.db (or .N.db if needed)");
        process::exit(1);
    }
    let input_filename = &args[1];
    let db_filename = determine_db_filename(input_filename);
    println!("Using database filename: '{}'", db_filename);

    match setup_database(&db_filename, SCHEMA_FILE_PATH) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error setting up database '{}': {}", db_filename, e);
            process::exit(1);
        }
    }

    println!("Processing input file: {} ...", input_filename);

    let start_time = Instant::now(); // Record start time

    let result = process_and_load_file(input_filename, &db_filename);

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time

    // Handle the result of file processing
    let count = match result {
        Ok(c) => c, // Store the count on success
        Err(e) => {
            // Print error message including elapsed time before exiting
            eprintln!(
                "Error processing file '{}' into '{}' after {:.2?}: {}",
                input_filename, db_filename, elapsed_time, e
            );
            // Removed extra ); here
            process::exit(1);
        }
    };

    println!("Done!");

    // --- Reporting ---
    match report_summary(&db_filename) {
        Ok(_) => {},
        Err(e) => eprintln!("Error generating report from database '{}': {}", db_filename, e),
    }

    println!(
        "Successfully processed {} CWR records from '{}' into '{}' in {:.2?}.",
        format_int_with_commas(count as i64), input_filename, db_filename, elapsed_time // Use i64 for format func
    );
}

/// Generates and prints summary reports from the database.
fn report_summary(db_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_filename)?;

    // Record Type Report
    println!();
    println!("{:<5} | {:>10}", "Type", "Count"); // Header (Right-align Count)
    println!("{:-<5}-+-{:-<10}", "", "");   // Separator (No change needed here)
    let mut stmt_rec = conn.prepare("SELECT record_type, count(*) FROM file GROUP BY record_type ORDER BY record_type")?;
    let mut rows_rec = stmt_rec.query([])?;
    let mut record_found = false;
    while let Some(row) = rows_rec.next()? {
        record_found = true;
        let record_type: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        println!("{:<5} | {:>10}", record_type, format_int_with_commas(count)); // Right-align count
    }
     if !record_found {
        println!("  No records loaded into 'file' table.");
    }

    // Error Report
    println!();
    println!("{:<60} | {:>10}", "Error", "Count"); // Header (Right-align Count)
    println!("{:-<60}-+-{:-<10}", "", "");      // Separator (No change needed here)
    let mut stmt_err = conn.prepare("SELECT description, count(*) FROM error GROUP BY description ORDER BY count(*) DESC")?;
    let mut rows_err = stmt_err.query([])?;
    let mut error_found = false;
    while let Some(row) = rows_err.next()? {
        error_found = true;
        let description: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        // Truncate description if too long for alignment
        let desc_display = if description.len() > 65 {
            description[..62].to_string().to_owned() + "..."
        } else {
            description
        };
        println!("{:<60} | {:>10}", desc_display, format_int_with_commas(count)); // Right-align count
    }
    if !error_found {
        println!("  No errors recorded.");
    }

    println!();

    Ok(())
}

/// Formats an integer with commas as thousands separators.
fn format_int_with_commas(num: i64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        result.push(c);
        let pos = len - 1 - i;
        if pos > 0 && pos % 3 == 0 {
            result.push(',');
        }
    }
    result
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

fn setup_database(db_filename: &str, schema_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(schema_path).exists() {
        return Err(format!("Schema file not found: {}", schema_path).into());
    }
    let schema_sql = fs::read_to_string(schema_path)?;
    let conn = Connection::open(db_filename)?;

    // Check if tables already exist to avoid erroring on re-runs
    let table_count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%'",
        [],
        |row| row.get(0),
    )?;

    if table_count == 0 {
        println!("Applying schema from '{}'", schema_path);
        conn.execute_batch(&schema_sql)?;
    } else {
        println!("Database schema already exists.");
    }

    Ok(())
}

/// Inserts a record into the 'error' table to log errors.
fn log_error(tx: &mut Transaction, line_number: usize, description: String) -> Result<(), rusqlite::Error> {
    // Insert into the database table
    tx.execute(
        "INSERT INTO error (line_number, description) VALUES (?1, ?2)",
        params![line_number as i64, description],
    )?;
    Ok(())
}

/// Logs a CwrParseError to stderr and the error table.                                                                                                                                                         
fn log_cwr_parse_error(tx: &mut Transaction, line_number: usize, error: &CwrParseError) -> Result<(), rusqlite::Error> {
    let description = error.to_string();
    log_error(tx, line_number, description)
}

/// Inserts a record into the 'file' table to link a CWR record to its source line.
fn insert_file_record(
    tx: &Transaction,
    line_number: usize,
    record_type: &str,
    record_id: i64,
) -> Result<(), CwrParseError> {
    tx.execute(
        "INSERT INTO file (line_number, record_type, record_id) VALUES (?1, ?2, ?3)",
        params![line_number as i64, record_type, record_id],
    )?;
    Ok(())
}

fn process_and_load_file(input_filename: &str, db_filename: &str) -> Result<usize, CwrParseError> {
    let file = File::open(input_filename)?;
    let reader = BufReader::new(file);
    let mut conn = Connection::open(db_filename)?;

    conn.pragma_update(None, "journal_mode", "OFF")?;
    conn.pragma_update(None, "synchronous", "OFF")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;

    let mut tx = conn.transaction()?;
    let mut line_number = 0;
    let mut processed_records = 0; // Count successfully processed CWR records

    for line_result in reader.lines() {
        line_number += 1;
        let line = line_result?;

        if line.is_empty() || line.trim().is_empty() {
            continue;
        }

        if line.len() < 3 {
            log_error(&mut tx, line_number, format!("Line {} is too short (less than 3 chars), skipping.", line_number))?;
            continue;
        }

        let record_type = &line[0..3];

        // --- Define the Safe Slice Helper Closure ---
        // It's defined here to capture `line` and `line_number` for error messages if needed
        let safe_slice = |start: usize, end: usize| -> Result<Option<String>, CwrParseError> {
            let slice_opt = if end > line.len() {
                if start >= line.len() {
                    None // Start is already out of bounds
                } else {
                    // Slice what's available up to the end
                    line.get(start..line.len())
                }
            } else {
                // Slice normally
                line.get(start..end)
            };

            match slice_opt {
                Some(slice) => {
                    let trimmed = slice.trim();
                    if trimmed.is_empty() {
                        Ok(None) // Treat empty or all-whitespace as None (NULL)
                    } else {
                        Ok(Some(trimmed.to_string())) // Return trimmed, owned string
                    }
                },
                None => Ok(None) // Slice failed (e.g., start >= line.len())
            }
        };
        // --- End of Safe Slice Definition ---

        // Wrap the record processing in a result to handle errors cleanly
        let process_result: Result<(), CwrParseError> = match record_type {
            "HDR" => parse_and_insert_hdr(line_number, &mut tx, &safe_slice),
            "GRH" => parse_and_insert_grh(line_number, &mut tx, &safe_slice),
            "GRT" => parse_and_insert_grt(line_number, &mut tx, &safe_slice),
            "TRL" => parse_and_insert_trl(line_number, &mut tx, &safe_slice),
            "AGR" => parse_and_insert_agr(line_number, &mut tx, &safe_slice),
            "NWR" | "REV" | "ISW" | "EXC" => parse_and_insert_nwr(line_number, &mut tx, &safe_slice),
            "ACK" => parse_and_insert_ack(line_number, &mut tx, &safe_slice),
            "TER" => parse_and_insert_ter(line_number, &mut tx, &safe_slice),
            "IPA" => parse_and_insert_ipa(line_number, &mut tx, &safe_slice),
            "NPA" => parse_and_insert_npa(line_number, &mut tx, &safe_slice),
            "SPU" | "OPU" => parse_and_insert_spu(line_number, &mut tx, &safe_slice),
            "NPN" => parse_and_insert_npn(line_number, &mut tx, &safe_slice),
            "SPT" | "OPT" => parse_and_insert_spt(line_number, &mut tx, &safe_slice),
            "SWR" | "OWR" => parse_and_insert_swr(line_number, &mut tx, &safe_slice),
            "NWN" => parse_and_insert_nwn(line_number, &mut tx, &safe_slice),
            "SWT" | "OWT" => parse_and_insert_swt(line_number, &mut tx, &safe_slice),
            "PWR" => parse_and_insert_pwr(line_number, &mut tx, &safe_slice),
            "ALT" => parse_and_insert_alt(line_number, &mut tx, &safe_slice),
            "NAT" => parse_and_insert_nat(line_number, &mut tx, &safe_slice),
            "EWT" => parse_and_insert_ewt(line_number, &mut tx, &safe_slice),
            "VER" => parse_and_insert_ver(line_number, &mut tx, &safe_slice),
            "PER" => parse_and_insert_per(line_number, &mut tx, &safe_slice),
            "NPR" => parse_and_insert_npr(line_number, &mut tx, &safe_slice),
            "REC" => parse_and_insert_rec(line_number, &mut tx, &safe_slice),
            "ORN" => parse_and_insert_orn(line_number, &mut tx, &safe_slice),
            "INS" => parse_and_insert_ins(line_number, &mut tx, &safe_slice),
            "IND" => parse_and_insert_ind(line_number, &mut tx, &safe_slice),
            "COM" => parse_and_insert_com(line_number, &mut tx, &safe_slice),
            "MSG" => parse_and_insert_msg(line_number, &mut tx, &safe_slice),
            "NET" | "NCT" | "NVT" => parse_and_insert_net(line_number, &mut tx, &safe_slice),
            "NOW" => parse_and_insert_now(line_number, &mut tx, &safe_slice),
            "ARI" => parse_and_insert_ari(line_number, &mut tx, &safe_slice),
            "XRF" => parse_and_insert_xrf(line_number, &mut tx, &safe_slice),
            _ => {
                log_error(&mut tx, line_number, format!("Unrecognized record type '{}', skipping.", record_type))?;
                Ok(()) // Don't treat unknown as an error for the whole file
            }
        };

        // Check the result of processing this line
        match process_result {
            Ok(_) => {
                // Only increment if it wasn't an unknown/skipped type
                if record_type != "_" { // Use a placeholder or check against known types
                    processed_records += 1;
                }
            }
            Err(e) => {
                // An error occurred processing this line (e.g., BadFormat from validation, or DB error from macro)                                                                                                 

                // Attempt to log the error first.                                                                                                                                                                  
                if let Err(log_err) = log_cwr_parse_error(&mut tx, line_number, &e) {
                    // If logging *itself* fails, we have a serious problem (likely DB issue). Abort immediately.                                                                                                   
                    eprintln!(
                        "CRITICAL Error: Failed to log error to database on line {}: {} (Original error was: {})",
                        line_number, log_err, e
                    );
                    // Return the database error that occurred during logging.                                                                                                                                      
                    return Err(CwrParseError::Db(log_err));
                }

                // Logging succeeded. Now decide if the *original* error warrants stopping.                                                                                                                         
                match e {
                    // BadFormat errors are logged per record, but we continue processing the file.                                                                                                                 
                    CwrParseError::BadFormat(_) => {
                        // Logged above. Continue to the next line.                                                                                                                                                 
                        // No action needed here, the loop will just continue.                                                                                                                                      
                    }
                    // IO errors (reading the file) or DB errors (during parsing/insertion, *not* during logging)                                                                                                   
                    // are usually fatal for the whole process.                                                                                                                                                     
                    CwrParseError::Io(_) | CwrParseError::Db(_) => {
                        eprintln!("Aborting transaction due to unrecoverable error: {}", e);
                        // Propagate the original error to trigger transaction rollback.                                                                                                                            
                        return Err(e);
                    }
                }
            }
        }
    }

    // Commit the transaction *only if* all lines were processed without error
    tx.commit()?;

    Ok(processed_records)
}

// --- Record Parsing and Insertion Functions ---

// Helper macro for mandatory fields. Logs error to DB and returns "" if missing/empty.
// Propagates DB errors or fundamental slice errors.
macro_rules! get_mandatory_field {
    // Add $tx parameter for database access
    ($tx:expr, $slice_fn:expr, $start:expr, $end:expr, $line_num:expr, $rec_type:expr, $field_name:expr) => {
        // Match on the result of the slice function
        match $slice_fn($start, $end) {
            // Case 1: Slice function itself returned an error (rare with current safe_slice, but good practice)
            Err(slice_err) => Err(slice_err), // Propagate the underlying error

            // Case 2: Slice succeeded and found a non-empty value
            Ok(Some(value)) => Ok(value), // Return the found value

            // Case 3: Slice succeeded but returned None (missing or empty/whitespace field)
            Ok(None) => {
                // Construct the error description
                let error_description = format!(
                    "{} missing or empty mandatory field '{}' (Expected at {}-{}). Using fallback ''.",
                    $rec_type, $field_name, $start + 1, $end // Use 1-based indexing for user message
                );

                // Attempt to log the error to the database
                match $tx.execute(
                    "INSERT INTO error (line_number, description) VALUES (?1, ?2)",
                    // Use rusqlite::params! macro for parameters
                    params![$line_num as i64, error_description], // Ensure line_number is i64 for DB
                ) {
                    // Subcase 3a: Database insertion failed
                    Err(db_err) => Err(CwrParseError::Db(db_err)), // Propagate the DB error
                    // Subcase 3b: Database insertion succeeded
                    Ok(_) => Ok(String::new()), // Return the fallback empty string
                }
            }
        }? // Use '?' *after* the match block to propagate any Err returned from the match arms
          // This ensures the macro returns Result<String, CwrParseError>
    };
}

// Helper for parsing the standard transaction prefix (Type 1-3, TransSeq 4-11, RecSeq 12-19)
fn parse_transaction_prefix(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(String, String, String), CwrParseError> {
    let record_type = get_mandatory_field!(&tx, safe_slice, 0, 3, line_number, "Transaction", "Record Type");
    let transaction_sequence_num = get_mandatory_field!(&tx, safe_slice, 3, 11, line_number, &record_type, "Transaction Sequence #");
    let record_sequence_num = get_mandatory_field!(&tx, safe_slice, 11, 19, line_number, &record_type, "Record Sequence #");
    Ok((record_type, transaction_sequence_num, record_sequence_num))
}

// HDR - Transmission Header
fn parse_and_insert_hdr(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let record_type = get_mandatory_field!(&tx, safe_slice, 0, 3, line_number, "HDR", "Record Type");
    if record_type != "HDR" { return Err(CwrParseError::BadFormat(format!("Expected HDR, found {}", record_type))); }

    let sender_type = get_mandatory_field!(&tx, safe_slice, 3, 5, line_number, "HDR", "Sender Type");
    let sender_id = get_mandatory_field!(&tx, safe_slice, 5, 14, line_number, "HDR", "Sender ID");
    let sender_name = get_mandatory_field!(&tx, safe_slice, 14, 59, line_number, "HDR", "Sender Name");
    let edi_version = get_mandatory_field!(&tx, safe_slice, 59, 64, line_number, "HDR", "EDI Standard Version Number");
    let creation_date = get_mandatory_field!(&tx, safe_slice, 64, 72, line_number, "HDR", "Creation Date");
    let creation_time = get_mandatory_field!(&tx, safe_slice, 72, 78, line_number, "HDR", "Creation Time");
    let transmission_date = get_mandatory_field!(&tx, safe_slice, 78, 86, line_number, "HDR", "Transmission Date");
    let character_set = safe_slice(86, 101)?; // Opt v2.1
    let version = safe_slice(101, 104)?; // Opt v2.2
    let revision = safe_slice(104, 107)?; // Opt v2.2
    let software_package = safe_slice(107, 137)?; // Opt v2.2
    let software_package_version = safe_slice(137, 167)?; // Opt v2.2

    // Basic Validation (Date/Time format - consider a helper function)
    if creation_date.len() != 8 || !creation_date.chars().all(|c| c.is_ascii_digit()) {
        return Err(CwrParseError::BadFormat(format!("Invalid HDR Creation Date format '{}'", creation_date)));
    }
    if creation_time.len() != 6 || !creation_time.chars().all(|c| c.is_ascii_digit()) {
        return Err(CwrParseError::BadFormat(format!("Invalid HDR Creation Time format '{}'", creation_time)));
    }
    if transmission_date.len() != 8 || !transmission_date.chars().all(|c| c.is_ascii_digit()) {
        return Err(CwrParseError::BadFormat(format!("Invalid HDR Transmission Date format '{}'", transmission_date)));
    }

    tx.execute(
        "INSERT INTO cwr_hdr (record_type, sender_type, sender_id, sender_name, edi_standard_version_number, creation_date, creation_time, transmission_date, character_set, version, revision, software_package, software_package_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![&record_type, sender_type, sender_id, sender_name, edi_version, creation_date, creation_time, transmission_date, character_set, version, revision, software_package, software_package_version],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// GRH - Group Header
fn parse_and_insert_grh(line_number: usize, tx: &mut Transaction, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,) -> Result<(), CwrParseError> {
    let record_type = get_mandatory_field!(&tx, safe_slice, 0, 3, line_number, "GRH", "Record Type");
    if record_type != "GRH" { return Err(CwrParseError::BadFormat(format!("Expected GRH, found {}", record_type))); }
    let transaction_type = get_mandatory_field!(&tx, safe_slice, 3, 6, line_number, "GRH", "Transaction Type");
    let group_id = get_mandatory_field!(&tx, safe_slice, 6, 11, line_number, "GRH", "Group ID");
    let version_number = get_mandatory_field!(&tx, safe_slice, 11, 16, line_number, "GRH", "Version Number for this transaction type");
    let batch_request = safe_slice(16, 26)?; // Opt
    let submission_distribution_type = safe_slice(26, 28)?; // Cond (blank for CWR)

    tx.execute(
        "INSERT INTO cwr_grh (record_type, transaction_type, group_id, version_number_for_this_transaction_type, batch_request, submission_distribution_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&record_type, transaction_type, group_id, version_number, batch_request, submission_distribution_type],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// GRT - Group Trailer
fn parse_and_insert_grt(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let record_type = get_mandatory_field!(&tx, safe_slice, 0, 3, line_number, "GRT", "Record Type");
    if record_type != "GRT" { return Err(CwrParseError::BadFormat(format!("Expected GRT, found {}", record_type))); }
    let group_id = get_mandatory_field!(&tx, safe_slice, 3, 8, line_number, "GRT", "Group ID");
    let transaction_count = get_mandatory_field!(&tx, safe_slice, 8, 16, line_number, "GRT", "Transaction Count");
    let record_count = get_mandatory_field!(&tx, safe_slice, 16, 24, line_number, "GRT", "Record Count");
    let currency_indicator = safe_slice(24, 27)?; // Cond
    let total_monetary_value = safe_slice(27, 37)?; // Opt

    tx.execute(
        "INSERT INTO cwr_grt (record_type, group_id, transaction_count, record_count, currency_indicator, total_monetary_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&record_type, group_id, transaction_count, record_count, currency_indicator, total_monetary_value],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// TRL - Transmission Trailer
fn parse_and_insert_trl(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let record_type = get_mandatory_field!(&tx, safe_slice, 0, 3, line_number, "TRL", "Record Type");
    if record_type != "TRL" { return Err(CwrParseError::BadFormat(format!("Expected TRL, found {}", record_type))); }
    let group_count = get_mandatory_field!(&tx, safe_slice, 3, 8, line_number, "TRL", "Group Count");
    let transaction_count = get_mandatory_field!(&tx, safe_slice, 8, 16, line_number, "TRL", "Transaction Count");
    let record_count = get_mandatory_field!(&tx, safe_slice, 16, 24, line_number, "TRL", "Record Count");

    tx.execute(
        "INSERT INTO cwr_trl (record_type, group_count, transaction_count, record_count) VALUES (?1, ?2, ?3, ?4)",
        params![&record_type, group_count, transaction_count, record_count],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// AGR - Agreement Transaction
fn parse_and_insert_agr(line_number: usize, tx: &mut Transaction, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "AGR" { return Err(CwrParseError::BadFormat(format!("Expected AGR, found {}", record_type))); }

    let submitter_agreement_number = get_mandatory_field!(&tx, safe_slice, 19, 33, line_number, "AGR", "Submitter Agreement Number");
    let international_standard_agreement_code = safe_slice(33, 47)?; // Opt
    let agreement_type = get_mandatory_field!(&tx, safe_slice, 47, 49, line_number, "AGR", "Agreement Type");
    let agreement_start_date = get_mandatory_field!(&tx, safe_slice, 49, 57, line_number, "AGR", "Agreement Start Date");
    let agreement_end_date = safe_slice(57, 65)?; // Opt
    let retention_end_date = safe_slice(65, 73)?; // Opt
    let prior_royalty_status = get_mandatory_field!(&tx, safe_slice, 73, 74, line_number, "AGR", "Prior Royalty Status");
    let prior_royalty_start_date = safe_slice(74, 82)?; // Cond
    let post_term_collection_status = get_mandatory_field!(&tx, safe_slice, 82, 83, line_number, "AGR", "Post-term Collection Status");
    let post_term_collection_end_date = safe_slice(83, 91)?; // Cond
    let date_of_signature_of_agreement = safe_slice(91, 99)?; // Opt
    let number_of_works = get_mandatory_field!(&tx, safe_slice, 99, 104, line_number, "AGR", "Number of Works");
    let sales_manufacture_clause = safe_slice(104, 105)?; // Cond
    let shares_change = safe_slice(105, 106)?; // Opt
    let advance_given = safe_slice(106, 107)?; // Opt
    let society_assigned_agreement_number = safe_slice(107, 121)?; // Opt v2.1

    // Conditional Validation Example
    if prior_royalty_status == "D" && prior_royalty_start_date.is_none() {
        return Err(CwrParseError::BadFormat("AGR Prior Royalty Start Date is mandatory when Prior Royalty Status is 'D'".to_string()));
    }
    if post_term_collection_status == "D" && post_term_collection_end_date.is_none() {
        return Err(CwrParseError::BadFormat("AGR Post-term Collection End Date is mandatory when Post-term Collection Status is 'D'".to_string()));
    }
    if (agreement_type == "OS" || agreement_type == "PS") && sales_manufacture_clause.is_none() {
        return Err(CwrParseError::BadFormat("AGR Sales/Manufacture Clause is mandatory when Agreement Type is 'OS' or 'PS'".to_string()));
    }
    // Date validation
    // ... add checks for start_date, end_date etc. format ...

    tx.execute(
        "INSERT INTO cwr_agr (record_type, transaction_sequence_num, record_sequence_num, submitter_agreement_number, international_standard_agreement_code, agreement_type, agreement_start_date, agreement_end_date, retention_end_date, prior_royalty_status, prior_royalty_start_date, post_term_collection_status, post_term_collection_end_date, date_of_signature_of_agreement, number_of_works, sales_manufacture_clause, shares_change, advance_given, society_assigned_agreement_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![&record_type, transaction_sequence_num, record_sequence_num, submitter_agreement_number, international_standard_agreement_code, agreement_type, agreement_start_date, agreement_end_date, retention_end_date, prior_royalty_status, prior_royalty_start_date, post_term_collection_status, post_term_collection_end_date, date_of_signature_of_agreement, number_of_works, sales_manufacture_clause, shares_change, advance_given, society_assigned_agreement_number],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

//noinspection ALL
// NWR - New Work Registration / REV / ISW / EXC
fn parse_and_insert_nwr(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // No need to check exact type, handled by caller match

    let work_title = get_mandatory_field!(&tx, safe_slice, 19, 79, line_number, &record_type, "Work Title");
    let language_code = safe_slice(79, 81)?; // Opt
    let submitter_work_num = get_mandatory_field!(&tx, safe_slice, 81, 95, line_number, &record_type, "Submitter Work #");
    let iswc = safe_slice(95, 106)?; // Opt
    let copyright_date = safe_slice(106, 114)?; // Opt
    let copyright_number = safe_slice(114, 126)?; // Opt
    let musical_work_distribution_category = get_mandatory_field!(&tx, safe_slice, 126, 129, line_number, &record_type, "Musical Work Distribution Category");
    let duration = safe_slice(129, 135)?; // Cond
    let recorded_indicator = get_mandatory_field!(&tx, safe_slice, 135, 136, line_number, &record_type, "Recorded Indicator");
    let text_music_relationship = safe_slice(136, 139)?; // Opt
    let composite_type = safe_slice(139, 142)?; // Opt
    let version_type = get_mandatory_field!(&tx, safe_slice, 142, 145, line_number, &record_type, "Version Type");
    let excerpt_type = safe_slice(145, 148)?; // Opt
    let music_arrangement = safe_slice(148, 151)?; // Cond
    let lyric_adaptation = safe_slice(151, 154)?; // Cond
    let contact_name = safe_slice(154, 184)?; // Opt
    let contact_id = safe_slice(184, 194)?; // Opt
    let cwr_work_type = safe_slice(194, 196)?; // Opt
    let grand_rights_ind = safe_slice(196, 197)?; // Cond
    let composite_component_count = safe_slice(197, 200)?; // Cond
    let date_of_publication_of_printed_edition = safe_slice(200, 208)?; // Opt
    let exceptional_clause = safe_slice(208, 209)?; // Opt
    let opus_number = safe_slice(209, 234)?; // Opt
    let catalogue_number = safe_slice(234, 259)?; // Opt
    let priority_flag = safe_slice(259, 260)?; // Opt v2.1

    // Conditional Validation
    if musical_work_distribution_category == "SER" && (duration.is_none() || duration == Some("000000".to_string())) {
        return Err(CwrParseError::BadFormat(format!("{} Duration must be > 000000 when Musical Work Distribution Category is 'SER'", record_type)));
    }
    // Add check for JAZ requiring duration for some societies if needed
    if version_type == "MOD" && music_arrangement.is_none() {
        // Note: spec says "indicates the type", not strictly mandatory if MOD? Re-check exact wording if needed. Assuming mandatory if MOD for now.
        return Err(CwrParseError::BadFormat(format!("{} Music Arrangement is expected when Version Type is 'MOD'", record_type)));
    }
    if version_type == "MOD" && lyric_adaptation.is_none() {
        return Err(CwrParseError::BadFormat(format!("{} Lyric Adaptation is expected when Version Type is 'MOD'", record_type)));
    }
    // UK societies mandatory grand rights ind check - requires context not available here (sender/recipient)
    // ASCAP composite count check - requires context
    // GEMA date/exceptional clause info noted in spec

    tx.execute(
        "INSERT INTO cwr_nwr (record_type, transaction_sequence_num, record_sequence_num, work_title, language_code, submitter_work_num, iswc, copyright_date, copyright_number, musical_work_distribution_category, duration, recorded_indicator, text_music_relationship, composite_type, version_type, excerpt_type, music_arrangement, lyric_adaptation, contact_name, contact_id, cwr_work_type, grand_rights_ind, composite_component_count, date_of_publication_of_printed_edition, exceptional_clause, opus_number, catalogue_number, priority_flag) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28)",
        params![&record_type, transaction_sequence_num, record_sequence_num, work_title, language_code, submitter_work_num, iswc, copyright_date, copyright_number, musical_work_distribution_category, duration, recorded_indicator, text_music_relationship, composite_type, version_type, excerpt_type, music_arrangement, lyric_adaptation, contact_name, contact_id, cwr_work_type, grand_rights_ind, composite_component_count, date_of_publication_of_printed_edition, exceptional_clause, opus_number, catalogue_number, priority_flag],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// ACK - Acknowledgement of Transaction
fn parse_and_insert_ack(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "ACK" { return Err(CwrParseError::BadFormat(format!("Expected ACK, found {}", record_type))); }

    let creation_date = get_mandatory_field!(&tx, safe_slice, 19, 27, line_number, "ACK", "Creation Date");
    let creation_time = get_mandatory_field!(&tx, safe_slice, 27, 33, line_number, "ACK", "Creation Time");
    let original_group_id = get_mandatory_field!(&tx, safe_slice, 33, 38, line_number, "ACK", "Original Group ID");
    let original_transaction_sequence_num = get_mandatory_field!(&tx, safe_slice, 38, 46, line_number, "ACK", "Original Transaction Sequence #");
    let original_transaction_type = get_mandatory_field!(&tx, safe_slice, 46, 49, line_number, "ACK", "Original Transaction Type");
    let creation_title = safe_slice(49, 109)?; // Cond
    let submitter_creation_num = safe_slice(109, 129)?; // Cond
    let recipient_creation_num = safe_slice(129, 149)?; // Cond
    let processing_date = get_mandatory_field!(&tx, safe_slice, 149, 157, line_number, "ACK", "Processing Date");
    let transaction_status = get_mandatory_field!(&tx, safe_slice, 157, 159, line_number, "ACK", "Transaction Status");

    // Conditional Validation
    let is_nwr_rev = original_transaction_type == "NWR" || original_transaction_type == "REV";
    let is_ack_for_transaction = original_transaction_type != "HDR" && original_transaction_type != "TRL"; // Assuming any other type is a transaction

    if is_nwr_rev && creation_title.is_none() {
        return Err(CwrParseError::BadFormat("ACK Creation Title is required for NWR/REV".to_string()));
    }
    if is_ack_for_transaction && submitter_creation_num.is_none() {
        return Err(CwrParseError::BadFormat("ACK Submitter Creation # is required for transaction responses".to_string()));
    }
    // Recipient Creation # depends on transaction status (e.g., 'AC' - Accepted)
    // This check might be complex depending on exact status values. Example:
    // if is_ack_for_transaction && transaction_status == "AC" && recipient_creation_num.is_none() {
    //     return Err(CwrParseError::BadFormat(format!("ACK Recipient Creation # is required when Transaction Status indicates acceptance")));
    // }

    tx.execute(
        "INSERT INTO cwr_ack (record_type, transaction_sequence_num, record_sequence_num, creation_date, creation_time, original_group_id, original_transaction_sequence_num, original_transaction_type, creation_title, submitter_creation_num, recipient_creation_num, processing_date, transaction_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![&record_type, transaction_sequence_num, record_sequence_num, creation_date, creation_time, original_group_id, original_transaction_sequence_num, original_transaction_type, creation_title, submitter_creation_num, recipient_creation_num, processing_date, transaction_status],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// TER - Territory in Agreement
fn parse_and_insert_ter(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "TER" { return Err(CwrParseError::BadFormat(format!("Expected TER, found {}", record_type))); }

    let inclusion_exclusion_indicator = get_mandatory_field!(&tx, safe_slice, 19, 20, line_number, "TER", "Inclusion/Exclusion Indicator");
    let tis_numeric_code = get_mandatory_field!(&tx, safe_slice, 20, 24, line_number, "TER", "TIS Numeric Code");

    tx.execute(
        "INSERT INTO cwr_ter (record_type, transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&record_type, transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// IPA - Interested Party of Agreement
fn parse_and_insert_ipa(line_number: usize, tx: &mut Transaction, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "IPA" { return Err(CwrParseError::BadFormat(format!("Expected IPA, found {}", record_type))); }

    let agreement_role_code = get_mandatory_field!(&tx, safe_slice, 19, 21, line_number, "IPA", "Agreement Role Code");
    let interested_party_ipi_name_num = safe_slice(21, 32)?; // Opt
    let ipi_base_number = safe_slice(32, 45)?; // Opt
    let interested_party_num = get_mandatory_field!(&tx, safe_slice, 45, 54, line_number, "IPA", "Interested Party #");
    let interested_party_last_name = get_mandatory_field!(&tx, safe_slice, 54, 99, line_number, "IPA", "Interested Party Last Name");
    let interested_party_writer_first_name = safe_slice(99, 129)?; // Opt (Cond based on Agreement Type in AGR - requires context)
    let pr_affiliation_society = safe_slice(129, 132)?; // Cond
    let pr_share = safe_slice(132, 137)?; // Cond
    let mr_affiliation_society = safe_slice(137, 140)?; // Cond
    let mr_share = safe_slice(140, 145)?; // Cond
    let sr_affiliation_society = safe_slice(145, 148)?; // Cond
    let sr_share = safe_slice(148, 153)?; // Cond

    // Conditional Validation: At least one share must be > 0 (requires parsing share values)
    // Conditional Validation: Society required if corresponding share > 0
    let pr_share_val = pr_share.as_deref().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
    let mr_share_val = mr_share.as_deref().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
    let sr_share_val = sr_share.as_deref().unwrap_or("0").parse::<f32>().unwrap_or(0.0);

    if pr_share_val <= 0.0 && mr_share_val <= 0.0 && sr_share_val <= 0.0 {
        return Err(CwrParseError::BadFormat("IPA At least one of PR, MR, or SR share must be greater than 0".to_string()));
    }
    if pr_share_val > 0.0 && pr_affiliation_society.is_none() {
        return Err(CwrParseError::BadFormat("IPA PR Affiliation Society is required when PR Share > 0".to_string()));
    }
    if mr_share_val > 0.0 && mr_affiliation_society.is_none() {
        return Err(CwrParseError::BadFormat("IPA MR Affiliation Society is required when MR Share > 0".to_string()));
    }
    if sr_share_val > 0.0 && sr_affiliation_society.is_none() {
        return Err(CwrParseError::BadFormat("IPA SR Affiliation Society is required when SR Share > 0".to_string()));
    }
    // Add check for writer first name allowed only for specific agreement types/roles (needs AGR context)

    tx.execute(
        "INSERT INTO cwr_ipa (record_type, transaction_sequence_num, record_sequence_num, agreement_role_code, interested_party_ipi_name_num, ipi_base_number, interested_party_num, interested_party_last_name, interested_party_writer_first_name, pr_affiliation_society, pr_share, mr_affiliation_society, mr_share, sr_affiliation_society, sr_share) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![&record_type, transaction_sequence_num, record_sequence_num, agreement_role_code, interested_party_ipi_name_num, ipi_base_number, interested_party_num, interested_party_last_name, interested_party_writer_first_name, pr_affiliation_society, pr_share, mr_affiliation_society, mr_share, sr_affiliation_society, sr_share],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NPA - Non-Roman Alphabet Interested Party Name (associated with IPA)
fn parse_and_insert_npa(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NPA" { return Err(CwrParseError::BadFormat(format!("Expected NPA, found {}", record_type))); }

    // Schema shows Interested Party # as Optional (VARCHAR(9)) but spec shows (A,C) - Conditional?
    // Let's assume optional based on schema. If Mandatory, change to get_mandatory_field!
    let interested_party_num = safe_slice(19, 28)?; // Cond? (Schema says nullable)

    let interested_party_name = get_mandatory_field!(&tx, safe_slice, 28, 188, line_number, "NPA", "Interested Party Name");
    let interested_party_writer_first_name = get_mandatory_field!(&tx, safe_slice, 188, 348, line_number, "NPA", "Interested Party Writer First Name");
    let language_code = safe_slice(348, 350)?; // Opt

    tx.execute(
        "INSERT INTO cwr_npa (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, interested_party_name, interested_party_writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, interested_party_num, interested_party_name, interested_party_writer_first_name, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// SPU - Publisher Controlled by Submitter / OPU - Other Publisher
fn parse_and_insert_spu(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // record_type will be "SPU" or "OPU"

    let publisher_sequence_num = get_mandatory_field!(&tx, safe_slice, 19, 21, line_number, &record_type, "Publisher Sequence #");
    let interested_party_num = safe_slice(21, 30)?; // Cond (Mandatory for SPU, Optional for OPU)
    let publisher_name = safe_slice(30, 75)?; // Cond (Mandatory for SPU, Optional for OPU)
    let publisher_unknown_indicator = safe_slice(75, 76)?; // Cond (Must be blank for SPU, 'Y' if OPU name is blank)
    let publisher_type = safe_slice(76, 78)?; // Cond (Mandatory for SPU, Optional for OPU)
    let tax_id_num = safe_slice(78, 87)?; // Opt
    let publisher_ipi_name_num = safe_slice(87, 98)?; // Cond (Mandatory for SPU if followed by SPT representing submitter)
    let submitter_agreement_number = safe_slice(98, 112)?; // Opt
    let pr_affiliation_society_num = safe_slice(112, 115)?; // Cond
    let pr_ownership_share = safe_slice(115, 120)?; // Cond
    let mr_society = safe_slice(120, 123)?; // Cond
    let mr_ownership_share = safe_slice(123, 128)?; // Cond
    let sr_society = safe_slice(128, 131)?; // Cond
    let sr_ownership_share = safe_slice(131, 136)?; // Cond
    let special_agreements_indicator = safe_slice(136, 137)?; // Opt
    let first_recording_refusal_ind = safe_slice(137, 138)?; // Opt (Mandatory for UK socs - context)
    let filler = safe_slice(138, 139)?; // Opt
    let publisher_ipi_base_number = safe_slice(139, 152)?; // Opt
    let international_standard_agreement_code = safe_slice(152, 166)?; // Opt
    let society_assigned_agreement_number = safe_slice(166, 180)?; // Opt
    let agreement_type = safe_slice(180, 182)?; // Opt v2.1
    let usa_license_ind = safe_slice(182, 183)?; // Opt v2.1

    // Conditional Validation
    if record_type == "SPU" {
        if interested_party_num.is_none() { return Err(CwrParseError::BadFormat("SPU Interested Party # is mandatory".to_string())); }
        if publisher_name.is_none() { return Err(CwrParseError::BadFormat("SPU Publisher Name is mandatory".to_string())); }
        if publisher_type.is_none() { return Err(CwrParseError::BadFormat("SPU Publisher Type is mandatory".to_string())); }
        if publisher_unknown_indicator.is_some() && publisher_unknown_indicator != Some("".to_string()) {
            return Err(CwrParseError::BadFormat("SPU Publisher Unknown Indicator must be blank".to_string()));
        }
        // Mandatory IPI Name # if followed by SPT representing submitter requires lookahead or state - complex, skip for now
    } else { // OPU
        if publisher_name.is_none() && publisher_unknown_indicator != Some("Y".to_string()) {
            return Err(CwrParseError::BadFormat("OPU Publisher Unknown Indicator must be 'Y' if Publisher Name is blank".to_string()));
        }
        if publisher_name.is_some() && publisher_unknown_indicator == Some("Y".to_string()) {
            return Err(CwrParseError::BadFormat("OPU Publisher Unknown Indicator must be blank if Publisher Name is present".to_string()));
        }
    }
    // Share/Society validation similar to IPA (if share > 0, society required)
    // ... add checks for PR/MR/SR shares vs societies ...

    tx.execute(
        "INSERT INTO cwr_spu (record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, publisher_unknown_indicator, publisher_type, tax_id_num, publisher_ipi_name_num, submitter_agreement_number, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, special_agreements_indicator, first_recording_refusal_ind, filler, publisher_ipi_base_number, international_standard_agreement_code, society_assigned_agreement_number, agreement_type, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)",
        params![&record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, publisher_unknown_indicator, publisher_type, tax_id_num, publisher_ipi_name_num, submitter_agreement_number, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, special_agreements_indicator, first_recording_refusal_ind, filler, publisher_ipi_base_number, international_standard_agreement_code, society_assigned_agreement_number, agreement_type, usa_license_ind],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NPN - Non-Roman Alphabet Publisher Name
fn parse_and_insert_npn(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NPN" { return Err(CwrParseError::BadFormat(format!("Expected NPN, found {}", record_type))); }

    let publisher_sequence_num = get_mandatory_field!(&tx, safe_slice, 19, 21, line_number, "NPN", "Publisher Sequence #");
    let interested_party_num = get_mandatory_field!(&tx, safe_slice, 21, 30, line_number, "NPN", "Interested Party #");
    let publisher_name = get_mandatory_field!(&tx, safe_slice, 30, 510, line_number, "NPN", "Publisher Name");
    let language_code = safe_slice(510, 512)?; // Opt

    tx.execute(
        "INSERT INTO cwr_npn (record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// SPT - Publisher Territory of Control / OPT - Other Publisher Territory
fn parse_and_insert_spt(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // record_type will be "SPT" or "OPT"

    let interested_party_num = get_mandatory_field!(&tx, safe_slice, 19, 28, line_number, &record_type, "Interested Party #");
    let constant_spaces = safe_slice(28, 34)?; // Should be spaces
    let pr_collection_share = safe_slice(34, 39)?; // Cond
    let mr_collection_share = safe_slice(39, 44)?; // Cond
    let sr_collection_share = safe_slice(44, 49)?; // Cond
    let inclusion_exclusion_indicator = get_mandatory_field!(&tx, safe_slice, 49, 50, line_number, &record_type, "Inclusion/Exclusion Indicator");
    let tis_numeric_code = get_mandatory_field!(&tx, safe_slice, 50, 54, line_number, &record_type, "TIS Numeric Code");
    let shares_change = safe_slice(54, 55)?; // Opt
    // V2.1 Sequence # (Mandatory)
    let sequence_num = get_mandatory_field!(&tx, safe_slice, 55, 58, line_number, &record_type, "Sequence # (v2.1)");

    // Validation: Ensure at least one collection share is present? Spec doesn't explicitly state, but implied.
    // if pr_collection_share.is_none() && mr_collection_share.is_none() && sr_collection_share.is_none() {
    //     return Err(CwrParseError::BadFormat(format!("{} At least one collection share expected", record_type)));
    // }

    tx.execute(
        "INSERT INTO cwr_spt (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, constant_spaces, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![&record_type, transaction_sequence_num, record_sequence_num, interested_party_num, constant_spaces, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// SWR - Writer Controlled by Submitter / OWR - Other Writer
fn parse_and_insert_swr(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // record_type is SWR or OWR

    let interested_party_num = safe_slice(19, 28)?; // Cond (Mandatory for SWR, Opt for OWR)
    let writer_last_name = safe_slice(28, 73)?; // Cond (Mandatory for SWR, Opt for OWR)
    let writer_first_name = safe_slice(73, 103)?; // Opt
    let writer_unknown_indicator = safe_slice(103, 104)?; // Cond (Blank for SWR, 'Y' if OWR name blank)
    let writer_designation_code = safe_slice(104, 106)?; // Cond (Mandatory for SWR, Opt for OWR)
    let tax_id_num = safe_slice(106, 115)?; // Opt
    let writer_ipi_name_num = safe_slice(115, 126)?; // Opt
    let pr_affiliation_society_num = safe_slice(126, 129)?; // Opt (Cond based on share)
    let pr_ownership_share = safe_slice(129, 134)?; // Opt
    let mr_society = safe_slice(134, 137)?; // Opt (Cond based on share)
    let mr_ownership_share = safe_slice(137, 142)?; // Opt
    let sr_society = safe_slice(142, 145)?; // Opt (Cond based on share)
    let sr_ownership_share = safe_slice(145, 150)?; // Opt
    let reversionary_indicator = safe_slice(150, 151)?; // Opt
    let first_recording_refusal_ind = safe_slice(151, 152)?; // Opt (Mandatory UK Socs)
    let work_for_hire_indicator = safe_slice(152, 153)?; // Opt
    let filler = safe_slice(153, 154)?; // Opt
    let writer_ipi_base_number = safe_slice(154, 167)?; // Opt
    let personal_number = safe_slice(167, 179)?; // Opt
    let usa_license_ind = safe_slice(179, 180)?; // Opt v2.1

    // Conditional Validation
    if record_type == "SWR" {
        if interested_party_num.is_none() { return Err(CwrParseError::BadFormat("SWR Interested Party # is mandatory".to_string())); }
        if writer_last_name.is_none() { return Err(CwrParseError::BadFormat("SWR Writer Last Name is mandatory".to_string())); }
        if writer_designation_code.is_none() { return Err(CwrParseError::BadFormat("SWR Writer Designation Code is mandatory".to_string())); }
        if writer_unknown_indicator.is_some() && writer_unknown_indicator != Some("".to_string()) {
            return Err(CwrParseError::BadFormat("SWR Writer Unknown Indicator must be blank".to_string()));
        }
    } else { // OWR
        if writer_last_name.is_none() && writer_unknown_indicator != Some("Y".to_string()) {
            return Err(CwrParseError::BadFormat("OWR Writer Unknown Indicator must be 'Y' if Writer Last Name is blank".to_string()));
        }
        if writer_last_name.is_some() && writer_unknown_indicator == Some("Y".to_string()) {
            return Err(CwrParseError::BadFormat("OWR Writer Unknown Indicator must be blank if Writer Last Name is present".to_string()));
        }
    }
    // Share/Society validation
    // ... add checks for PR/MR/SR shares vs societies ...

    tx.execute(
        "INSERT INTO cwr_swr (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, writer_unknown_indicator, writer_designation_code, tax_id_num, writer_ipi_name_num, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, reversionary_indicator, first_recording_refusal_ind, work_for_hire_indicator, filler, writer_ipi_base_number, personal_number, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)",
        params![&record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, writer_unknown_indicator, writer_designation_code, tax_id_num, writer_ipi_name_num, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, reversionary_indicator, first_recording_refusal_ind, work_for_hire_indicator, filler, writer_ipi_base_number, personal_number, usa_license_ind],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NWN - Non-Roman Alphabet Writer Name
fn parse_and_insert_nwn(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NWN" { return Err(CwrParseError::BadFormat(format!("Expected NWN, found {}", record_type))); }

    let interested_party_num = safe_slice(19, 28)?; // Cond? Schema allows NULL
    let writer_last_name = get_mandatory_field!(&tx, safe_slice, 28, 188, line_number, "NWN", "Writer Last Name");
    let writer_first_name = safe_slice(188, 348)?; // Opt? Schema allows NULL, Spec says O,O
    let language_code = safe_slice(348, 350)?; // Opt

    tx.execute(
        "INSERT INTO cwr_nwn (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// SWT - Writer Territory of Control / OWT - Other Writer Territory
fn parse_and_insert_swt(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // record_type is SWT or OWT

    let interested_party_num = safe_slice(19, 28)?; // Cond? Schema allows NULL, Spec says C
    let pr_collection_share = safe_slice(28, 33)?; // Opt
    let mr_collection_share = safe_slice(33, 38)?; // Opt
    let sr_collection_share = safe_slice(38, 43)?; // Opt
    let inclusion_exclusion_indicator = get_mandatory_field!(&tx, safe_slice, 43, 44, line_number, &record_type, "Inclusion/Exclusion Indicator");
    let tis_numeric_code = get_mandatory_field!(&tx, safe_slice, 44, 48, line_number, &record_type, "TIS Numeric Code");
    let shares_change = safe_slice(48, 49)?; // Opt
    // V2.1 Sequence # (Mandatory)
    let sequence_num = get_mandatory_field!(&tx, safe_slice, 49, 52, line_number, &record_type, "Sequence # (v2.1)");

    tx.execute(
        "INSERT INTO cwr_swt (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![&record_type, transaction_sequence_num, record_sequence_num, interested_party_num, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// PWR - Publisher for Writer relationship
fn parse_and_insert_pwr(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "PWR" { return Err(CwrParseError::BadFormat(format!("Expected PWR, found {}", record_type))); }

    let publisher_ip_num = safe_slice(19, 28)?; // Cond? Schema allows NULL, Spec says C
    let publisher_name = safe_slice(28, 73)?; // Cond? Schema allows NULL, Spec says C
    let submitter_agreement_number = safe_slice(73, 87)?; // Opt
    let society_assigned_agreement_number = safe_slice(87, 101)?; // Opt
    let writer_ip_num = safe_slice(101, 110)?; // Cond? v2.1 Schema allows NULL, Spec says C
    // V2.2 Publisher Sequence # (Mandatory)
    let publisher_sequence_num = get_mandatory_field!(&tx, safe_slice, 110, 112, line_number, "PWR", "Publisher Sequence # (v2.2)");

    tx.execute(
        "INSERT INTO cwr_pwr (record_type, transaction_sequence_num, record_sequence_num, publisher_ip_num, publisher_name, submitter_agreement_number, society_assigned_agreement_number, writer_ip_num, publisher_sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![&record_type, transaction_sequence_num, record_sequence_num, publisher_ip_num, publisher_name, submitter_agreement_number, society_assigned_agreement_number, writer_ip_num, publisher_sequence_num],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// ALT - Alternate Title
fn parse_and_insert_alt(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "ALT" { return Err(CwrParseError::BadFormat(format!("Expected ALT, found {}", record_type))); }

    let alternate_title = get_mandatory_field!(&tx, safe_slice, 19, 79, line_number, "ALT", "Alternate Title");
    let title_type = get_mandatory_field!(&tx, safe_slice, 79, 81, line_number, "ALT", "Title Type");
    let language_code = safe_slice(81, 83)?; // Cond

    // Conditional Validation
    if (title_type == "OL" || title_type == "AL") && language_code.is_none() {
        return Err(CwrParseError::BadFormat("ALT Language Code is mandatory when Title Type is 'OL' or 'AL'".to_string()));
    }

    tx.execute(
        "INSERT INTO cwr_alt (record_type, transaction_sequence_num, record_sequence_num, alternate_title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&record_type, transaction_sequence_num, record_sequence_num, alternate_title, title_type, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NAT - Non-Roman Alphabet Title
fn parse_and_insert_nat(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NAT" { return Err(CwrParseError::BadFormat(format!("Expected NAT, found {}", record_type))); }

    let title = get_mandatory_field!(&tx, safe_slice, 19, 659, line_number, "NAT", "Title");
    let title_type = get_mandatory_field!(&tx, safe_slice, 659, 661, line_number, "NAT", "Title Type");
    let language_code = safe_slice(661, 663)?; // Opt

    tx.execute(
        "INSERT INTO cwr_nat (record_type, transaction_sequence_num, record_sequence_num, title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&record_type, transaction_sequence_num, record_sequence_num, title, title_type, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// EWT - Entire Work Title for Excerpts
fn parse_and_insert_ewt(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "EWT" { return Err(CwrParseError::BadFormat(format!("Expected EWT, found {}", record_type))); }

    let entire_work_title = get_mandatory_field!(&tx, safe_slice, 19, 79, line_number, "EWT", "Entire Work Title");
    let iswc_of_entire_work = safe_slice(79, 90)?; // Opt
    let language_code = safe_slice(90, 92)?; // Opt
    let writer_1_last_name = safe_slice(92, 137)?; // Opt
    let writer_1_first_name = safe_slice(137, 167)?; // Opt
    let source = safe_slice(167, 227)?; // Opt
    let writer_1_ipi_name_num = safe_slice(227, 238)?; // Opt
    let writer_1_ipi_base_number = safe_slice(238, 251)?; // Opt
    let writer_2_last_name = safe_slice(251, 296)?; // Opt
    let writer_2_first_name = safe_slice(296, 326)?; // Opt
    let writer_2_ipi_name_num = safe_slice(326, 337)?; // Opt
    let writer_2_ipi_base_number = safe_slice(337, 350)?; // Opt
    let submitter_work_num = safe_slice(350, 364)?; // Opt

    tx.execute(
        "INSERT INTO cwr_ewt (record_type, transaction_sequence_num, record_sequence_num, entire_work_title, iswc_of_entire_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![&record_type, transaction_sequence_num, record_sequence_num, entire_work_title, iswc_of_entire_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// VER - Original Work Title for Versions
fn parse_and_insert_ver(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "VER" { return Err(CwrParseError::BadFormat(format!("Expected VER, found {}", record_type))); }

    let original_work_title = get_mandatory_field!(&tx, safe_slice, 19, 79, line_number, "VER", "Original Work Title");
    let iswc_of_original_work = safe_slice(79, 90)?; // Opt
    let language_code = safe_slice(90, 92)?; // Opt
    let writer_1_last_name = safe_slice(92, 137)?; // Opt
    let writer_1_first_name = safe_slice(137, 167)?; // Opt
    let source = safe_slice(167, 227)?; // Opt
    let writer_1_ipi_name_num = safe_slice(227, 238)?; // Opt
    let writer_1_ipi_base_number = safe_slice(238, 251)?; // Opt
    let writer_2_last_name = safe_slice(251, 296)?; // Opt
    let writer_2_first_name = safe_slice(296, 326)?; // Opt
    let writer_2_ipi_name_num = safe_slice(326, 337)?; // Opt
    let writer_2_ipi_base_number = safe_slice(337, 350)?; // Opt
    let submitter_work_num = safe_slice(350, 364)?; // Opt

    tx.execute(
        "INSERT INTO cwr_ver (record_type, transaction_sequence_num, record_sequence_num, original_work_title, iswc_of_original_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![&record_type, transaction_sequence_num, record_sequence_num, original_work_title, iswc_of_original_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// PER - Performing Artist
fn parse_and_insert_per(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "PER" { return Err(CwrParseError::BadFormat(format!("Expected PER, found {}", record_type))); }

    let performing_artist_last_name = get_mandatory_field!(&tx, safe_slice, 19, 64, line_number, "PER", "Performing Artist Last Name");
    let performing_artist_first_name = safe_slice(64, 94)?; // Opt
    let performing_artist_ipi_name_num = safe_slice(94, 105)?; // Opt
    let performing_artist_ipi_base_number = safe_slice(105, 118)?; // Opt

    tx.execute(
        "INSERT INTO cwr_per (record_type, transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NPR - Non-Roman Alphabet Performing Artist Name
fn parse_and_insert_npr(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NPR" { return Err(CwrParseError::BadFormat(format!("Expected NPR, found {}", record_type))); }

    // Schema allows NULL, Spec says C for Name
    let performing_artist_name = safe_slice(19, 179)?; // Cond? Schema allows NULL
    let performing_artist_first_name = safe_slice(179, 339)?; // Opt
    let performing_artist_ipi_name_num = safe_slice(339, 350)?; // Opt
    let performing_artist_ipi_base_number = safe_slice(350, 363)?; // Opt
    let language_code = safe_slice(363, 365)?; // Opt
    let performance_language = safe_slice(365, 367)?; // Cond v2.1? Schema allows NULL
    let performance_dialect = safe_slice(367, 370)?; // Cond v2.1? Schema allows NULL

    // Minimal validation: if name is None, perhaps first name should also be None?
    // Spec doesn't explicitly state.

    tx.execute(
        "INSERT INTO cwr_npr (record_type, transaction_sequence_num, record_sequence_num, performing_artist_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number, language_code, performance_language, performance_dialect) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![&record_type, transaction_sequence_num, record_sequence_num, performing_artist_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number, language_code, performance_language, performance_dialect],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// REC - Recording Detail
fn parse_and_insert_rec(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "REC" { return Err(CwrParseError::BadFormat(format!("Expected REC, found {}", record_type))); }

    let release_date = safe_slice(19, 27)?; // Opt
    let constant_blanks_1 = safe_slice(27, 87)?; // Opt
    let release_duration = safe_slice(87, 93)?; // Opt
    let constant_blanks_2 = safe_slice(93, 98)?; // Opt
    let album_title = safe_slice(98, 158)?; // Opt
    let album_label = safe_slice(158, 218)?; // Opt
    let release_catalog_num = safe_slice(218, 236)?; // Opt
    let ean = safe_slice(236, 249)?; // Opt
    let isrc = safe_slice(249, 261)?; // Opt
    let recording_format = safe_slice(261, 262)?; // Opt
    let recording_technique = safe_slice(262, 263)?; // Opt
    let media_type = safe_slice(263, 266)?; // Opt v2.1
    // V2.2 fields
    let recording_title = safe_slice(266, 326)?; // Opt v2.2
    let version_title = safe_slice(326, 386)?; // Opt v2.2
    let display_artist = safe_slice(386, 446)?; // Opt v2.2
    let record_label = safe_slice(446, 506)?; // Opt v2.2
    let isrc_validity = safe_slice(506, 526)?; // Cond v2.2
    let submitter_recording_identifier = safe_slice(526, 540)?; // Opt v2.2

    // Conditional Validation
    if isrc.is_some() && isrc_validity.is_none() {
        // Spec implies validity is conditional on ISRC presence
        // It's VARCHAR(20) in schema, spec says L (lookup) Y/U/N.
        // Let's warn rather than fail hard if missing/empty.
        eprintln!("Warning: REC ISRC Validity is expected when ISRC is present.");
        // Or make it an error:
        // return Err(CwrParseError::BadFormat(format!("REC ISRC Validity is required when ISRC is present", line_number)));
    }

    tx.execute(
        "INSERT INTO cwr_rec (record_type, transaction_sequence_num, record_sequence_num, release_date, constant_blanks_1, release_duration, constant_blanks_2, album_title, album_label, release_catalog_num, ean, isrc, recording_format, recording_technique, media_type, recording_title, version_title, display_artist, record_label, isrc_validity, submitter_recording_identifier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
        params![&record_type, transaction_sequence_num, record_sequence_num, release_date, constant_blanks_1, release_duration, constant_blanks_2, album_title, album_label, release_catalog_num, ean, isrc, recording_format, recording_technique, media_type, recording_title, version_title, display_artist, record_label, isrc_validity, submitter_recording_identifier],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// ORN - Work Origin
fn parse_and_insert_orn(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "ORN" { return Err(CwrParseError::BadFormat(format!("Expected ORN, found {}", record_type))); }

    let intended_purpose = get_mandatory_field!(&tx, safe_slice, 19, 22, line_number, "ORN", "Intended Purpose");
    let production_title = safe_slice(22, 82)?; // Cond
    let cd_identifier = safe_slice(82, 97)?; // Cond
    let cut_number = safe_slice(97, 101)?; // Opt (Cond based on Intended Purpose)
    let library = safe_slice(101, 161)?; // Cond v2.1
    let bltvr = safe_slice(161, 162)?; // Opt v2.1
    let filler_reserved = safe_slice(162, 187)?; // Opt v2.1
    let production_num = safe_slice(187, 199)?; // Opt v2.1
    let episode_title = safe_slice(199, 259)?; // Opt v2.1
    let episode_num = safe_slice(259, 279)?; // Opt v2.1
    let year_of_production = safe_slice(279, 283)?; // Opt v2.1
    let avi_society_code = safe_slice(283, 286)?; // Opt v2.1 (N?)
    let audio_visual_number = safe_slice(286, 301)?; // Opt v2.1
    // V2.2 fields
    let v_isan_isan = safe_slice(301, 313)?; // Opt v2.2
    let v_isan_episode = safe_slice(313, 317)?; // Opt v2.2
    let v_isan_check_digit_1 = safe_slice(317, 318)?; // Opt v2.2
    let v_isan_version = safe_slice(318, 326)?; // Opt v2.2
    let v_isan_check_digit_2 = safe_slice(326, 327)?; // Opt v2.2
    let eidr = safe_slice(327, 347)?; // Opt v2.2
    let eidr_check_digit = safe_slice(347, 348)?; // Opt v2.2

    // Conditional Validation
    // Spec: Production Title required when CWR Work Type on NWR is 'FM' - requires NWR context.
    if intended_purpose == "LIB" && cd_identifier.is_none() {
        return Err(CwrParseError::BadFormat("ORN CD Identifier required when Intended Purpose is 'LIB'".to_string()));
    }
    if intended_purpose == "LIB" && library.is_none() { // Assuming library name also required for LIB
        return Err(CwrParseError::BadFormat("ORN Library required when Intended Purpose is 'LIB'".to_string()));
    }

    tx.execute(
        "INSERT INTO cwr_orn (record_type, transaction_sequence_num, record_sequence_num, intended_purpose, production_title, cd_identifier, cut_number, library, bltvr, filler_reserved, production_num, episode_title, episode_num, year_of_production, avi_society_code, audio_visual_number, v_isan_isan, v_isan_episode, v_isan_check_digit_1, v_isan_version, v_isan_check_digit_2, eidr, eidr_check_digit) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)",
        params![&record_type, transaction_sequence_num, record_sequence_num, intended_purpose, production_title, cd_identifier, cut_number, library, bltvr, filler_reserved, production_num, episode_title, episode_num, year_of_production, avi_society_code, audio_visual_number, v_isan_isan, v_isan_episode, v_isan_check_digit_1, v_isan_version, v_isan_check_digit_2, eidr, eidr_check_digit],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// INS - Instrumentation Summary
fn parse_and_insert_ins(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "INS" { return Err(CwrParseError::BadFormat(format!("Expected INS, found {}", record_type))); }

    let number_of_voices = safe_slice(19, 22)?; // Opt
    let standard_instrumentation_type = safe_slice(22, 25)?; // Cond
    let instrumentation_description = safe_slice(25, 75)?; // Cond

    // Conditional Validation: Required if IND not entered and other field blank
    // This requires context (knowing if IND records follow). Cannot validate fully here.
    // Basic check: at least one should be present if IND is not used.
    // if standard_instrumentation_type.is_none() && instrumentation_description.is_none() {
    // This might be valid if IND records *are* present following this INS.
    // eprintln!("Warning: INS both standard type and description are blank. Assumes IND records follow.", line_number);
    // }

    tx.execute(
        "INSERT INTO cwr_ins (record_type, transaction_sequence_num, record_sequence_num, number_of_voices, standard_instrumentation_type, instrumentation_description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![&record_type, transaction_sequence_num, record_sequence_num, number_of_voices, standard_instrumentation_type, instrumentation_description],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// IND - Instrumentation Detail
fn parse_and_insert_ind(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "IND" { return Err(CwrParseError::BadFormat(format!("Expected IND, found {}", record_type))); }

    let instrument_code = get_mandatory_field!(&tx, safe_slice, 19, 22, line_number, "IND", "Instrument Code");
    let number_of_players = safe_slice(22, 25)?; // Opt

    tx.execute(
        "INSERT INTO cwr_ind (record_type, transaction_sequence_num, record_sequence_num, instrument_code, number_of_players) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&record_type, transaction_sequence_num, record_sequence_num, instrument_code, number_of_players],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// COM - Composite Component
fn parse_and_insert_com(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "COM" { return Err(CwrParseError::BadFormat(format!("Expected COM, found {}", record_type))); }

    let title = get_mandatory_field!(&tx, safe_slice, 19, 79, line_number, "COM", "Title");
    let iswc_of_component = safe_slice(79, 90)?; // Opt
    let submitter_work_num = safe_slice(90, 104)?; // Opt
    let duration = safe_slice(104, 110)?; // Opt
    let writer_1_last_name = get_mandatory_field!(&tx, safe_slice, 110, 155, line_number, "COM", "Writer 1 Last Name");
    let writer_1_first_name = safe_slice(155, 185)?; // Opt
    let writer_1_ipi_name_num = safe_slice(185, 196)?; // Opt
    let writer_2_last_name = safe_slice(196, 241)?; // Opt
    let writer_2_first_name = safe_slice(241, 271)?; // Opt
    let writer_2_ipi_name_num = safe_slice(271, 282)?; // Opt
    let writer_1_ipi_base_number = safe_slice(282, 295)?; // Opt
    let writer_2_ipi_base_number = safe_slice(295, 308)?; // Opt

    tx.execute(
        "INSERT INTO cwr_com (record_type, transaction_sequence_num, record_sequence_num, title, iswc_of_component, submitter_work_num, duration, writer_1_last_name, writer_1_first_name, writer_1_ipi_name_num, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_1_ipi_base_number, writer_2_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![&record_type, transaction_sequence_num, record_sequence_num, title, iswc_of_component, submitter_work_num, duration, writer_1_last_name, writer_1_first_name, writer_1_ipi_name_num, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_1_ipi_base_number, writer_2_ipi_base_number],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// MSG - Message (Part of ACK Transaction usually)
fn parse_and_insert_msg(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "MSG" { return Err(CwrParseError::BadFormat(format!("Expected MSG, found {}", record_type))); }

    let message_type = get_mandatory_field!(&tx, safe_slice, 19, 20, line_number, "MSG", "Message Type");
    let original_record_sequence_num = get_mandatory_field!(&tx, safe_slice, 20, 28, line_number, "MSG", "Original Record Sequence #");
    let msg_record_type = get_mandatory_field!(&tx, safe_slice, 28, 31, line_number, "MSG", "Record Type (in original transaction)"); // Renamed column to avoid clash
    let message_level = get_mandatory_field!(&tx, safe_slice, 31, 32, line_number, "MSG", "Message Level");
    let validation_number = get_mandatory_field!(&tx, safe_slice, 32, 35, line_number, "MSG", "Validation Number");
    let message_text = get_mandatory_field!(&tx, safe_slice, 35, 185, line_number, "MSG", "Message Text");

    tx.execute(
        "INSERT INTO cwr_msg (record_type, transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, msg_record_type, message_level, validation_number, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![&record_type, transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, msg_record_type, message_level, validation_number, message_text],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NET - Non-Roman Alphabet Title (for EWT/COM/VER) / NCT / NVT
fn parse_and_insert_net(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    // record_type is NET, NCT, or NVT

    let title = get_mandatory_field!(&tx, safe_slice, 19, 659, line_number, &record_type, "Title");
    let language_code = safe_slice(659, 661)?; // Opt

    tx.execute(
        "INSERT INTO cwr_net (record_type, transaction_sequence_num, record_sequence_num, title, language_code) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&record_type, transaction_sequence_num, record_sequence_num, title, language_code],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// NOW - Non-Roman Alphabet Writer Name (for EWT/VER/COM)
fn parse_and_insert_now(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "NOW" { return Err(CwrParseError::BadFormat(format!("Expected NOW, found {}", record_type))); }

    let writer_name = get_mandatory_field!(&tx, safe_slice, 19, 179, line_number, "NOW", "Writer Name");
    // Spec says O,M for First Name - assume Mandatory based on schema
    let writer_first_name = get_mandatory_field!(&tx, safe_slice, 179, 339, line_number, "NOW", "Writer First Name");
    let language_code = safe_slice(339, 341)?; // Opt
    let writer_position = safe_slice(341, 342)?; // Opt

    tx.execute(
        "INSERT INTO cwr_now (record_type, transaction_sequence_num, record_sequence_num, writer_name, writer_first_name, language_code, writer_position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, writer_name, writer_first_name, language_code, writer_position],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// ARI - Additional Related Information
fn parse_and_insert_ari(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "ARI" { return Err(CwrParseError::BadFormat(format!("Expected ARI, found {}", record_type))); }

    let society_num = get_mandatory_field!(&tx, safe_slice, 19, 22, line_number, "ARI", "Society #");
    let work_num = safe_slice(22, 36)?; // Cond? Schema allows NULL
    let type_of_right = get_mandatory_field!(&tx, safe_slice, 36, 39, line_number, "ARI", "Type of Right");
    let subject_code = safe_slice(39, 41)?; // Cond? Schema allows NULL
    let note = safe_slice(41, 201)?; // Cond? Schema allows NULL

    // Conditional Validation: Subject Code and Note relationship? Spec is vague ('C').
    // Assume they are optional unless specific conditions require them.

    tx.execute(
        "INSERT INTO cwr_ari (record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![&record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}

// XRF - Work ID Cross Reference
fn parse_and_insert_xrf(
    line_number: usize,
    tx: &mut Transaction,
    safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>,
) -> Result<(), CwrParseError> {
    let (record_type, transaction_sequence_num, record_sequence_num) = parse_transaction_prefix(line_number, tx, safe_slice)?;
    if record_type != "XRF" { return Err(CwrParseError::BadFormat(format!("Expected XRF, found {}", record_type))); }

    let organisation_code = get_mandatory_field!(&tx, safe_slice, 19, 22, line_number, "XRF", "Organisation Code");
    let identifier = get_mandatory_field!(&tx, safe_slice, 22, 36, line_number, "XRF", "Identifier");
    let identifier_type = get_mandatory_field!(&tx, safe_slice, 36, 37, line_number, "XRF", "Identifier Type");
    let validity = get_mandatory_field!(&tx, safe_slice, 37, 38, line_number, "XRF", "Validity");

    tx.execute(
        "INSERT INTO cwr_xrf (record_type, transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![&record_type, transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity],
    )?;
    let record_id = tx.last_insert_rowid();
    insert_file_record(tx, line_number, &record_type, record_id)?;
    Ok(())
}