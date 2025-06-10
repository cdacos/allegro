use allegro_cwr::{OutputFormat, format_int_with_commas};
use rusqlite::Connection;

/// Generates and prints summary reports from the database for a specific file import.
pub fn report_summary(db_filename: &str, file_id: i64, format: OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_filename)?;

    match format {
        OutputFormat::Default | OutputFormat::Sql => generate_default_report(&conn, file_id),
        OutputFormat::Json => generate_json_report(&conn, file_id),
    }
}

fn generate_default_report(conn: &Connection, file_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    // Record Type Report
    println!();
    println!("{:<5} | {:>10}", "Type", "Count"); // Header (Right-align Count)
    println!("{:-<5}-+-{:-<10}", "", ""); // Separator (No change needed here)
    let mut stmt_rec = conn.prepare(
        "SELECT record_type, count(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type",
    )?;
    let mut rows_rec = stmt_rec.query([file_id])?;
    let mut record_found = false;
    while let Some(row) = rows_rec.next()? {
        record_found = true;
        let record_type: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        println!("{:<5} | {:>10}", record_type, format_int_with_commas(count)); // Right-align count
    }
    if !record_found {
        println!("  No records loaded into 'file_line' table.");
    }

    // Error Report
    println!();
    println!("{:<60} | {:>10}", "Error", "Count"); // Header (Right-align Count)
    println!("{:-<60}-+-{:-<10}", "", ""); // Separator (No change needed here)
    let mut stmt_err = conn.prepare(
        "SELECT description, count(*) FROM error WHERE file_id = ?1 GROUP BY description ORDER BY count(*) DESC",
    )?;
    let mut rows_err = stmt_err.query([file_id])?;
    let mut error_found = false;
    while let Some(row) = rows_err.next()? {
        error_found = true;
        let description: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        // Truncate description if too long for alignment
        let desc_display =
            if description.len() > 60 { description[..57].to_string().to_owned() + "..." } else { description };
        println!("{:<60} | {:>10}", desc_display, format_int_with_commas(count)); // Right-align count
    }
    if !error_found {
        println!("  No errors recorded.");
    }

    println!();
    Ok(())
}

fn generate_json_report(conn: &Connection, file_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;

    // Collect record type data
    let mut record_types = HashMap::new();
    let mut stmt_rec = conn.prepare(
        "SELECT record_type, count(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type",
    )?;
    let mut rows_rec = stmt_rec.query([file_id])?;
    while let Some(row) = rows_rec.next()? {
        let record_type: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        record_types.insert(record_type, count);
    }

    // Collect error data
    let mut errors = HashMap::new();
    let mut stmt_err = conn.prepare(
        "SELECT description, count(*) FROM error WHERE file_id = ?1 GROUP BY description ORDER BY count(*) DESC",
    )?;
    let mut rows_err = stmt_err.query([file_id])?;
    while let Some(row) = rows_err.next()? {
        let description: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        errors.insert(description, count);
    }

    // Generate JSON manually (simple format)
    println!("{{");
    println!("  \"file_id\": {},", file_id);
    println!("  \"record_types\": {{");

    let mut first = true;
    for (record_type, count) in &record_types {
        if !first {
            println!(",");
        }
        print!("    \"{}\": {}", record_type, count);
        first = false;
    }
    if !record_types.is_empty() {
        println!();
    }
    println!("  }},");

    println!("  \"errors\": {{");
    first = true;
    for (description, count) in &errors {
        if !first {
            println!(",");
        }
        // Escape quotes in description
        let escaped_description = description.replace("\"", "\\\"");
        print!("    \"{}\": {}", escaped_description, count);
        first = false;
    }
    if !errors.is_empty() {
        println!();
    }
    println!("  }}");
    println!("}}");

    Ok(())
}
