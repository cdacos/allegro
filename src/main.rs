use std::env;
use std::process;
use std::time::Instant;

use allegro_cwr::{process_cwr_file, format_int_with_commas};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_filename>", args[0]);
        eprintln!("This will create or use a database named <input_filename>.db (or .N.db if needed)");
        process::exit(1);
    }
    let input_filename = &args[1];

    println!("Processing input file: {}...", input_filename);

    let start_time = Instant::now();

    let result = process_cwr_file(input_filename);

    let elapsed_time = start_time.elapsed();

    println!("Done!");

    // Handle the result of file processing
    let (db_filename, count) = match result {
        Ok((db, c)) => (db, c),
        Err(e) => {
            eprintln!("Error processing file '{}' after {:.2?}: {}", input_filename, elapsed_time, e);
            process::exit(1);
        }
    };

    println!("Successfully processed {} CWR records from '{}' into '{}' in {:.2?}.", format_int_with_commas(count as i64), input_filename, db_filename, elapsed_time);
}
