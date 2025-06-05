//! JSON output handler for CWR (Common Works Registration) files
//!
//! This crate provides JSON output functionality for CWR records.

/// JSON implementation of CwrHandler trait
pub struct JsonHandler {
    output_count: usize,
    error_count: usize,
    first_record: bool,
}

impl JsonHandler {
    pub fn new() -> Self {
        // Start JSON array
        println!("[");
        
        JsonHandler {
            output_count: 0,
            error_count: 0,
            first_record: true,
        }
    }
}

impl allegro_cwr::CwrHandler for JsonHandler {
    type Error = std::io::Error;

    fn process_record(&mut self, parsed_record: allegro_cwr::ParsedRecord) -> Result<(), Self::Error> {
        if !self.first_record {
            println!(",");
        }
        
        // Output JSON representation of the parsed record
        println!("  {{");
        println!("    \"line_number\": {},", parsed_record.line_number);
        println!("    \"record_type\": \"{}\",", parsed_record.record.record_type());
        println!("    \"cwr_version\": {},", parsed_record.context.cwr_version);
        println!("    \"status\": \"parsed\"");
        println!("  }}");
        
        self.first_record = false;
        self.output_count += 1;
        Ok(())
    }

    fn handle_parse_error(&mut self, line_number: usize, error: &allegro_cwr::CwrParseError) -> Result<(), Self::Error> {
        if !self.first_record {
            println!(",");
        }
        
        println!("  {{");
        println!("    \"line_number\": {},", line_number);
        println!("    \"status\": \"error\",");
        println!("    \"error_message\": \"{}\"", error.to_string().replace('"', "\\\""));
        println!("  }}");
        
        self.first_record = false;
        self.error_count += 1;
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), Self::Error> {
        // End JSON array
        println!();
        println!("]");
        Ok(())
    }

    fn get_report(&self) -> String {
        format!("JSON processing complete:\n  Records output: {}\n  Errors: {}", 
                self.output_count, self.error_count)
    }
}

/// Convenience function to process CWR file and output JSON
pub fn process_cwr_to_json(input_filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    process_cwr_to_json_with_version(input_filename, None)
}

/// Convenience function to process CWR file and output JSON with optional version hint
pub fn process_cwr_to_json_with_version(input_filename: &str, version_hint: Option<f32>) -> Result<usize, Box<dyn std::error::Error>> {
    let handler = JsonHandler::new();
    let report = allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?;
    
    // Extract count from report
    let output_count = report.lines()
        .find(|line| line.contains("Records output:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0);
    
    Ok(output_count)
}