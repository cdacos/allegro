use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

// --- Configuration ---
const KEY_WIDTH: usize = 9; // Width for the 3-char key column ("ABC      ")
const VALUE_WIDTH: usize = 15; // Width for the count column ("      1,234,567")
// ---------------------

fn main() {
    // 1. Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    // 2. Process the file
    match process_file(filename) {
        Ok(stats) => {
            // 3. Print the results
            print_stats(&stats);
        }
        Err(e) => {
            eprintln!("Error processing file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

fn process_file(filename: &str) -> io::Result<HashMap<String, usize>> {
    // Open the file
    let file = File::open(filename)?;
    // Use a BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);

    let mut stats: HashMap<String, usize> = HashMap::new();

    // Iterate through lines, handling potential I/O errors
    for line_result in reader.lines() {
        let line = line_result?; // Propagate I/O errors

        // Ignore '\r' (implicitly handled by .lines())

        // Check if line has at least 3 characters
        if line.len() >= 3 {
            // Extract the first three characters
            // Using .get() is safer than slicing directly as it returns Option<&str>
            // but we already checked length, so slicing is fine here.
            let prefix = &line[0..3];

            // Update the count in the HashMap
            // The entry API is efficient: finds or inserts, then allows modification
            *stats.entry(prefix.to_string()).or_insert(0) += 1;
        }
        // Lines shorter than 3 chars are implicitly skipped
    }

    Ok(stats)
}

fn print_stats(stats: &HashMap<String, usize>) {
    println!("RECORD STATISTICS");
    println!("--------------------------"); // 26 dashes

    // For potentially nicer output, sort by key before printing
    let mut sorted_stats: Vec<_> = stats.iter().collect();
    sorted_stats.sort_by(|a, b| a.0.cmp(b.0)); // Sort alphabetically by key (prefix)

    for (key, count) in sorted_stats {
        let formatted_count = format_count(*count);
        // Print formatted line: Key left-aligned, Count right-aligned
        println!(
            "{:<key_width$} | {:>value_width$}",
            key,
            formatted_count,
            key_width = KEY_WIDTH,
            value_width = VALUE_WIDTH
        );
    }
}

// Helper function to format numbers with commas
fn format_count(n: usize) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut result = Vec::with_capacity(len + (len - 1) / 3);

    let first_digit_index = len % 3;
    if first_digit_index != 0 {
        result.extend_from_slice(&bytes[0..first_digit_index]);
    }

    for (i, chunk) in bytes[first_digit_index..].chunks(3).enumerate() {
        if i > 0 || first_digit_index != 0 {
            result.push(b',');
        }
        result.extend_from_slice(chunk);
    }

    // Convert the byte vector back to a String
    // Since we started with digits and added commas, this is safe.
    String::from_utf8(result).unwrap()
}
