//! INS - Instrumentation Summary Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// INS - Instrumentation Summary Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsRecord {
    /// Always "INS"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Number of voices (3 chars, optional)
    pub number_of_voices: Option<String>,

    /// Standard instrumentation type (3 chars, conditional)
    pub standard_instrumentation_type: Option<String>,

    /// Instrumentation description (50 chars, conditional)
    pub instrumentation_description: Option<String>,
}

impl InsRecord {
    /// Create a new INS record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String) -> Self {
        Self { record_type: "INS".to_string(), transaction_sequence_num, record_sequence_num, number_of_voices: None, standard_instrumentation_type: None, instrumentation_description: None }
    }

    /// Parse a CWR line into an INS record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 19 {
            return Err(CwrParseError::BadFormat("INS line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "INS" {
            return Err(CwrParseError::BadFormat(format!("Expected INS, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();

        let number_of_voices = if line.len() > 19 { line.get(19..22).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let standard_instrumentation_type = if line.len() > 22 { line.get(22..25).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let instrumentation_description = if line.len() > 25 { line.get(25..75).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(InsRecord { record_type, transaction_sequence_num, record_sequence_num, number_of_voices, standard_instrumentation_type, instrumentation_description })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num)];

        if self.number_of_voices.is_some() || self.standard_instrumentation_type.is_some() || self.instrumentation_description.is_some() {
            fields.push(format!("{:3}", self.number_of_voices.as_deref().unwrap_or("")));
            fields.push(format!("{:3}", self.standard_instrumentation_type.as_deref().unwrap_or("")));
            fields.push(format!("{:50}", self.instrumentation_description.as_deref().unwrap_or("")));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ins_creation() {
        let ins = InsRecord::new("00000001".to_string(), "00000001".to_string());

        assert_eq!(ins.record_type, "INS");
    }

    #[test]
    fn test_ins_round_trip() {
        let original = InsRecord::new("00000001".to_string(), "00000001".to_string());

        let line = original.to_cwr_line();
        let parsed = InsRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 19);
    }
}
