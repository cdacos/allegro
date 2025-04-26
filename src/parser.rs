use crate::db::log_error;
use crate::error::CwrParseError;
use crate::{db, error, record_handlers};
use rusqlite::Connection;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek};

// Context struct to hold file-level metadata like CWR version
#[derive(Debug, Clone)]
pub struct ParsingContext {
    pub cwr_version: f32, // e.g., 2.0, 2.1, 2.2
    // Add other metadata like charset later if needed
}

fn get_cwr_version(hdr_line: &str) -> Result<f32, CwrParseError> {
    // Define valid CWR versions
    let valid_cwr_versions = vec![2.0, 2.1, 2.2];

    // Determine version based on header line length
    let cwr_version = if hdr_line.len() < 87 {
        2.0
    } else if hdr_line.len() > 104 {
        // Try to parse version from specific position in header
        if let Some(version_str) = hdr_line.get(101..104) {
            match version_str.trim().parse::<f32>() {
                Ok(version) => version,
                Err(_) => return Err(CwrParseError::BadFormat(
                    format!("Invalid CWR version value: {}", version_str)
                ))
            }
        } else {
            return Err(CwrParseError::BadFormat(
                "Unable to extract CWR version from header".to_string()
            ));
        }
    } else {
        2.1
    };

    // Validate the version
    if valid_cwr_versions.contains(&cwr_version) {
        Ok(cwr_version)
    } else {
        Err(CwrParseError::BadFormat(
            format!("Invalid CWR version: {}", cwr_version)
        ))
    }
}

pub fn process_and_load_file(input_filename: &str, db_filename: &str) -> Result<usize, CwrParseError> {
    let file = File::open(input_filename)?;
    // Use BufReader::new directly, we need to consume the first line separately
    let mut reader = BufReader::new(file);

    // --- Read the first line to determine CWR version ---
    let mut first_line = String::new();
    let bytes_read = reader.read_line(&mut first_line)?;
    if bytes_read == 0 {
        return Err(CwrParseError::BadFormat("File is empty".to_string()));
    }
    let hdr_line = first_line.trim_end(); // Remove trailing newline chars

    if !hdr_line.starts_with("HDR") {
        return Err(CwrParseError::BadFormat(format!(
            "File does not start with HDR record. Found: '{}'",
            hdr_line.get(0..std::cmp::min(hdr_line.len(), 50)).unwrap_or("") // Show first 50 chars
        )));
    }

    let context = ParsingContext { cwr_version: get_cwr_version(hdr_line)? };
    println!("Determined CWR Version: {}", context.cwr_version); // Log detected version

    // --- Setup Database and Transaction ---
    let mut conn = Connection::open(db_filename)?;
    // Set PRAGMAs before transaction
    conn.pragma_update(None, "journal_mode", "OFF")?;
    conn.pragma_update(None, "synchronous", "OFF")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;

    // Start transaction *before* preparing statements
    let tx = conn.transaction()?;
    let mut line_number: usize = 0; // Start at 0, HDR is line 1

    { // Scope for the main processing loop and prepared statements
        // Prepare all statements *using the transaction*
        // Keep the struct mutable as inserts happen within the handlers
        let mut prepared_statements = db::get_prepared_statements(&tx)?;

        // Reset the position to the start of the file
        reader.seek(io::SeekFrom::Start(0))?;

        for line_result in reader.lines() {
            line_number += 1; // Increment for subsequent lines
            let line = match line_result {
                Ok(l) => l,
                Err(io_err) => {
                    // Log IO error reading the line
                    let parse_err = CwrParseError::Io(io_err);
                     if let Err(log_err) = error::log_cwr_parse_error(&mut prepared_statements, line_number, &parse_err) {
                         eprintln!("CRITICAL Error: Failed to log IO error to database on line {}: {} (Original error was: {})", line_number, log_err, parse_err);
                         return Err(CwrParseError::Db(log_err));
                     }
                     eprintln!("Aborting transaction due to IO error reading line {}: {}", line_number, parse_err);
                     return Err(parse_err); // Abort on IO Error
                }
            };

            if line.is_empty() || line.trim().is_empty() {
                continue;
            }

            if line.len() < 3 {
                log_error(&mut prepared_statements.error_stmt, line_number, format!("Line {} is too short (less than 3 chars), skipping.", line_number))?;
                continue;
            }

            let record_type = &line[0..3];

            // --- Define the Safe Slice Helper Closure for the current line ---
            let safe_slice = |start: usize, end: usize| -> Result<Option<String>, CwrParseError> {
                let slice_opt = if end > line.len() {
                    if start >= line.len() { None } else { line.get(start..line.len()) }
                } else {
                    line.get(start..end)
                };
                match slice_opt {
                    Some(slice) => {
                        let trimmed = slice.trim();
                        if trimmed.is_empty() { Ok(None) } else { Ok(Some(trimmed.to_string())) }
                    },
                    None => Ok(None)
                }
            };
            // --- End of Safe Slice Definition ---

            // Use a separate scope to handle the result processing cleanly
            let process_result: Result<(), CwrParseError> = {
                match record_type {
                    // Apply the pattern: call handler, set flag on success
                    "HDR" => { record_handlers::parse_and_insert_hdr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "GRH" => { record_handlers::parse_and_insert_grh(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "GRT" => { record_handlers::parse_and_insert_grt(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "TRL" => { record_handlers::parse_and_insert_trl(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "AGR" => { record_handlers::parse_and_insert_agr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NWR" | "REV" | "ISW" | "EXC" => { record_handlers::parse_and_insert_nwr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "ACK" => { record_handlers::parse_and_insert_ack(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "TER" => { record_handlers::parse_and_insert_ter(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "IPA" => { record_handlers::parse_and_insert_ipa(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NPA" => { record_handlers::parse_and_insert_npa(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "SPU" | "OPU" => { record_handlers::parse_and_insert_spu(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NPN" => { record_handlers::parse_and_insert_npn(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "SPT" | "OPT" => { record_handlers::parse_and_insert_spt(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "SWR" | "OWR" => { record_handlers::parse_and_insert_swr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NWN" => { record_handlers::parse_and_insert_nwn(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "SWT" | "OWT" => { record_handlers::parse_and_insert_swt(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "PWR" => { record_handlers::parse_and_insert_pwr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "ALT" => { record_handlers::parse_and_insert_alt(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NAT" => { record_handlers::parse_and_insert_nat(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "EWT" => { record_handlers::parse_and_insert_ewt(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "VER" => { record_handlers::parse_and_insert_ver(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "PER" => { record_handlers::parse_and_insert_per(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NPR" => { record_handlers::parse_and_insert_npr(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "REC" => { record_handlers::parse_and_insert_rec(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "ORN" => { record_handlers::parse_and_insert_orn(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "INS" => { record_handlers::parse_and_insert_ins(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "IND" => { record_handlers::parse_and_insert_ind(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "COM" => { record_handlers::parse_and_insert_com(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "MSG" => { record_handlers::parse_and_insert_msg(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NET" | "NCT" | "NVT" => { record_handlers::parse_and_insert_net(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "NOW" => { record_handlers::parse_and_insert_now(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "ARI" => { record_handlers::parse_and_insert_ari(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    "XRF" => { record_handlers::parse_and_insert_xrf(line_number, &tx, &mut prepared_statements, &context, &safe_slice) },
                    _ => { // Unrecognized record type
                        log_error(&mut prepared_statements.error_stmt, line_number, format!("Unrecognized record type '{}', skipping.", record_type))?;
                        // known_record_processed remains false
                        Ok(()) // Still Ok overall, just skipped this line
                    }
                }
            }; // End of inner scope for process_result

            // Check the result of processing this line
            match process_result {
                Ok(_) => { }
                Err(e) => {
                    // An error occurred processing this line (e.g., BadFormat from validation, or DB error from macro)

                    if let Err(log_err) = error::log_cwr_parse_error(&mut prepared_statements, line_number, &e) {
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
    }

    // Commit the transaction *only if* all lines were processed without error
    tx.commit()?;

    Ok(line_number)
}

// Helper macro for mandatory fields. Logs error to DB (using prepared statement) and returns "" if missing/empty.
// Propagates DB errors or fundamental slice errors.
#[macro_export]
macro_rules! get_mandatory_field {
    ($stmts:expr, $slice_fn:expr, $start:expr, $end:expr, $line_num:expr, $rec_type:expr, $field_name:expr) => {
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

                match $stmts.error_stmt.execute(params![$line_num as i64, error_description]) {
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
