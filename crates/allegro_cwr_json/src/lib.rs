//! JSON output handler for CWR (Common Works Registration) files
//!
//! This crate provides JSON output functionality for CWR records.

use std::io::{self, Write};

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

    fn handle_parse_error(&mut self, line_number: usize, error: &allegro_cwr::CwrParseError) -> Result<(), Self::Error> {
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

    fn handle_warnings(&mut self, _line_number: usize, _record_type: &str, warnings: &[String]) -> Result<(), Self::Error> {
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
pub fn process_cwr_to_json_with_version(input_filename: &str, version_hint: Option<f32>) -> Result<usize, Box<dyn std::error::Error>> {
    process_cwr_to_json_with_version_and_output(input_filename, version_hint, None)
}

/// Convenience function to process CWR file and output JSON with optional version hint and output file
pub fn process_cwr_to_json_with_version_and_output(input_filename: &str, version_hint: Option<f32>, output_filename: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
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
    let output_count = report.lines().find(|line| line.contains("Records output:")).and_then(|line| line.split(':').nth(1)).and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);

    Ok(output_count)
}
