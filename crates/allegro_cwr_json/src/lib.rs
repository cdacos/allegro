//! JSON output handler for CWR (Common Works Registration) files
//!
//! This crate provides JSON output functionality for CWR records.

use allegro_cwr::CwrRegistry;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, Write};

/// JSON implementation of CwrHandler trait
pub struct JsonHandler<W: Write> {
    output_count: usize,
    error_count: usize,
    first_record: bool,
    context_written: bool,
    writer: W,
}

impl Default for JsonHandler<io::Stdout> {
    fn default() -> Self {
        Self::new(io::stdout())
    }
}

impl<W: Write> JsonHandler<W> {
    pub fn new(mut writer: W) -> Self {
        // Start JSON object
        writeln!(writer, "{{").expect("Failed to write to output");

        JsonHandler { output_count: 0, error_count: 0, first_record: true, context_written: false, writer }
    }
}

impl<W: Write> allegro_cwr::CwrHandler for JsonHandler<W> {
    type Error = std::io::Error;

    fn process_record(&mut self, parsed_record: allegro_cwr::ParsedRecord) -> Result<(), Self::Error> {
        // Write context once at the beginning
        if !self.context_written {
            writeln!(self.writer, "  \"context\": {{")?;
            writeln!(self.writer, "    \"cwr_version\": {},", parsed_record.context.cwr_version)?;
            writeln!(self.writer, "    \"file_id\": {}", parsed_record.context.file_id)?;
            writeln!(self.writer, "  }},")?;
            writeln!(self.writer, "  \"records\": [")?;
            self.context_written = true;
        }

        if !self.first_record {
            writeln!(self.writer, ",")?;
        }

        // Create a simplified record without context
        let record_without_context = serde_json::json!({
            "line_number": parsed_record.line_number,
            "record": parsed_record.record,
            "warnings": parsed_record.warnings
        });

        match serde_json::to_string_pretty(&record_without_context) {
            Ok(json_str) => {
                // Indent the JSON to match our array formatting
                let indented_json = json_str.lines().map(|line| format!("    {}", line)).collect::<Vec<_>>().join("\n");
                write!(self.writer, "{}", indented_json)?;
            }
            Err(e) => {
                // Fallback to basic metadata if serialization fails
                writeln!(self.writer, "    {{")?;
                writeln!(self.writer, "      \"line_number\": {},", parsed_record.line_number)?;
                writeln!(self.writer, "      \"record_type\": \"{}\",", parsed_record.record.record_type())?;
                writeln!(self.writer, "      \"status\": \"serialization_error\",")?;
                writeln!(self.writer, "      \"error_message\": \"{}\"", e.to_string().replace('"', "\\\""))?;
                write!(self.writer, "    }}")?;
            }
        }

        self.first_record = false;
        self.output_count += 1;
        Ok(())
    }

    fn handle_parse_error(
        &mut self, line_number: usize, error: &allegro_cwr::CwrParseError,
    ) -> Result<(), Self::Error> {
        // Initialize context if this is the first thing we encounter
        if !self.context_written {
            writeln!(self.writer, "  \"context\": {{")?;
            writeln!(self.writer, "    \"cwr_version\": null,")?;
            writeln!(self.writer, "    \"file_id\": 0")?;
            writeln!(self.writer, "  }},")?;
            writeln!(self.writer, "  \"records\": [")?;
            self.context_written = true;
        }

        if !self.first_record {
            writeln!(self.writer, ",")?;
        }

        writeln!(self.writer, "    {{")?;
        writeln!(self.writer, "      \"line_number\": {},", line_number)?;
        writeln!(self.writer, "      \"status\": \"error\",")?;
        writeln!(self.writer, "      \"error_message\": \"{}\"", error.to_string().replace('"', "\\\""))?;
        write!(self.writer, "    }}")?;

        self.first_record = false;
        self.error_count += 1;
        Ok(())
    }

    fn handle_warnings(
        &mut self, _line_number: usize, _record_type: &str, warnings: &[String],
    ) -> Result<(), Self::Error> {
        // Warnings are now included in each record's warnings array, so we don't need separate warning objects
        self.error_count += warnings.len();
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        // Close records array and main object
        if self.context_written {
            writeln!(self.writer)?;
            writeln!(self.writer, "  ]")?;
        }
        writeln!(self.writer, "}}")?;
        self.writer.flush()?;
        Ok(())
    }

    fn get_report(&self) -> String {
        format!("JSON processing complete:\n  Records output: {}\n  Errors: {}", self.output_count, self.error_count)
    }
}

/// Convenience function to process CWR file and output JSON
pub fn process_cwr_to_json(input_filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    process_cwr_to_json_with_version(input_filename, None)
}

/// Convenience function to process CWR file and output JSON with optional version hint
pub fn process_cwr_to_json_with_version(
    input_filename: &str, version_hint: Option<f32>,
) -> Result<usize, Box<dyn std::error::Error>> {
    process_cwr_to_json_with_version_and_output(input_filename, version_hint, None)
}

/// Convenience function to process CWR file and output JSON with optional version hint and output file
pub fn process_cwr_to_json_with_version_and_output(
    input_filename: &str, version_hint: Option<f32>, output_filename: Option<&str>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let report = match output_filename {
        Some(filename) => {
            let file = std::fs::File::create(filename)?;
            let handler = JsonHandler::new(file);
            allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?
        }
        None => {
            let handler = JsonHandler::new(io::stdout());
            allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?
        }
    };

    // Extract count from report
    let output_count = report
        .lines()
        .find(|line| line.contains("Records output:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0);

    Ok(output_count)
}

/// Structure representing the JSON format we expect to parse
#[derive(Deserialize)]
struct JsonCwrFile {
    context: JsonContext,
    records: Vec<JsonRecord>,
}

#[derive(Deserialize)]
struct JsonContext {
    cwr_version: f32,
    #[allow(dead_code)] // Included for JSON format compatibility
    file_id: Option<u32>,
}

#[derive(Deserialize)]
struct JsonRecord {
    #[allow(dead_code)] // Included for JSON format compatibility
    line_number: Option<usize>,
    record: CwrRegistry,
    #[allow(dead_code)] // Included for JSON format compatibility
    warnings: Option<Vec<String>>,
}

/// Convenience function to process JSON file and output CWR with optional version hint and output file
pub fn process_json_to_cwr_with_version_and_output(
    input_filename: &str, _version_hint: Option<f32>, output_filename: Option<&str>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input_filename)?;
    let reader = BufReader::new(file);

    // Parse the entire JSON file
    let json_data: JsonCwrFile = serde_json::from_reader(reader)?;

    // Use the CWR version from context or fallback to hint
    let cwr_version = allegro_cwr::domain_types::CwrVersion(json_data.context.cwr_version);

    // Create output writer with ASCII validation
    let output: Box<dyn Write> = match output_filename {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    };
    let mut ascii_writer = allegro_cwr::AsciiWriter::new(output);

    // Write each record as a CWR line
    let mut count = 0;
    for json_record in json_data.records {
        let cwr_line = json_record.record.to_cwr_line_without_newline(&cwr_version);
        ascii_writer.write_line(&cwr_line)?;
        count += 1;
    }

    // AsciiWriter doesn't need explicit flush - it writes directly
    Ok(count)
}
