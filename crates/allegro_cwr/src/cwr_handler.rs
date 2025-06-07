use crate::{CwrParseError, ParsedRecord};

/// Trait for handling CWR records during processing
pub trait CwrHandler {
    type Error: std::error::Error;

    /// Process a single parsed CWR record
    #[must_use]
    fn process_record(&mut self, record: ParsedRecord) -> Result<(), Self::Error>;

    /// Handle a parsing error (e.g., log it, count it, etc.)
    #[must_use]
    fn handle_parse_error(&mut self, line_number: usize, error: &CwrParseError) -> Result<(), Self::Error>;

    /// Handle warnings from a successfully parsed record (optional override)
    #[must_use]
    fn handle_warnings(&mut self, line_number: usize, record_type: &str, warnings: &[String]) -> Result<(), Self::Error> {
        // Default implementation does nothing - handlers can override to store warnings
        let _ = (line_number, record_type, warnings);
        Ok(())
    }

    /// Finalize processing (e.g., commit transaction, close files, etc.)
    #[must_use]
    fn finalize(&mut self) -> Result<(), Self::Error>;

    /// Generate a report of the processing results
    fn get_report(&self) -> String;
}
