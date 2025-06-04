use crate::util::format_int_with_commas;
use rusqlite::Connection;

/// Generates and prints summary reports from the database for a specific file import.
pub fn report_summary(db_filename: &str, file_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_filename)?;

    // Record Type Report
    println!();
    println!("{:<5} | {:>10}", "Type", "Count"); // Header (Right-align Count)
    println!("{:-<5}-+-{:-<10}", "", ""); // Separator (No change needed here)
    let mut stmt_rec = conn.prepare("SELECT record_type, count(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type")?;
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
    let mut stmt_err = conn.prepare("SELECT description, count(*) FROM error WHERE file_id = ?1 GROUP BY description ORDER BY count(*) DESC")?;
    let mut rows_err = stmt_err.query([file_id])?;
    let mut error_found = false;
    while let Some(row) = rows_err.next()? {
        error_found = true;
        let description: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        // Truncate description if too long for alignment
        let desc_display = if description.len() > 60 { description[..57].to_string().to_owned() + "..." } else { description };
        println!("{:<60} | {:>10}", desc_display, format_int_with_commas(count)); // Right-align count
    }
    if !error_found {
        println!("  No errors recorded.");
    }

    println!();

    Ok(())
}
