use std::env;
use std::process;
use std::time::Instant;

mod db;
mod error;
mod report;
mod util;
mod record_handlers;
mod parser;

// Use specific items from modules
use crate::db::{determine_db_filename, setup_database};
use crate::report::report_summary;
use crate::util::format_int_with_commas;

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

    match setup_database(&db_filename) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error setting up database '{}': {}", db_filename, e);
            process::exit(1);
        }
    }

    println!("Processing input file: {}...", input_filename);

    let start_time = Instant::now();

    let result = parser::process_and_load_file(input_filename, &db_filename);

    let elapsed_time = start_time.elapsed();

    println!("Done!");

    // Handle the result of file processing
    let count = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Error processing file '{}' into '{}' after {:.2?}: {}",
                input_filename, db_filename, elapsed_time, e
            );
            process::exit(1);
        }
    };

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