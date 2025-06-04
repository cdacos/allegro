//! PWR - Publisher for Writer Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// PWR - Publisher for Writer Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PwrRecord {
    /// Always "PWR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher IP number (9 chars, conditional)
    pub publisher_ip_num: Option<String>,

    /// Publisher name (45 chars, conditional)
    pub publisher_name: Option<String>,

    /// Submitter agreement number (14 chars, optional)
    pub submitter_agreement_number: Option<String>,

    /// Society-assigned agreement number (14 chars, optional)
    pub society_assigned_agreement_number: Option<String>,

    /// Writer IP number (9 chars, conditional, v2.1+)
    pub writer_ip_num: Option<String>,

    /// Publisher sequence number (2 chars, v2.2+)
    pub publisher_sequence_num: Option<String>,
}

impl PwrRecord {
    /// Create a new PWR record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String) -> Self {
        Self { record_type: "PWR".to_string(), transaction_sequence_num, record_sequence_num, publisher_ip_num: None, publisher_name: None, submitter_agreement_number: None, society_assigned_agreement_number: None, writer_ip_num: None, publisher_sequence_num: None }
    }

    /// Parse a CWR line into a PWR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 101 {
            return Err(CwrParseError::BadFormat("PWR line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "PWR" {
            return Err(CwrParseError::BadFormat(format!("Expected PWR, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();

        let publisher_ip_num = line.get(19..28).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let publisher_name = line.get(28..73).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let submitter_agreement_number = line.get(73..87).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let society_assigned_agreement_number = line.get(87..101).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let writer_ip_num = if line.len() > 101 { line.get(101..110).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let publisher_sequence_num = if line.len() > 110 { line.get(110..112).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(PwrRecord { record_type, transaction_sequence_num, record_sequence_num, publisher_ip_num, publisher_name, submitter_agreement_number, society_assigned_agreement_number, writer_ip_num, publisher_sequence_num })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:9}", self.publisher_ip_num.as_deref().unwrap_or("")),
            format!("{:45}", self.publisher_name.as_deref().unwrap_or("")),
            format!("{:14}", self.submitter_agreement_number.as_deref().unwrap_or("")),
            format!("{:14}", self.society_assigned_agreement_number.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ field
        if let Some(ref writer_ip) = self.writer_ip_num {
            fields.push(format!("{:9}", writer_ip));
        }

        // Add v2.2+ field
        if let Some(ref publisher_seq) = self.publisher_sequence_num {
            fields.push(format!("{:2}", publisher_seq));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwr_creation() {
        let pwr = PwrRecord::new("00000001".to_string(), "00000001".to_string());

        assert_eq!(pwr.record_type, "PWR");
    }

    #[test]
    fn test_pwr_round_trip() {
        let original = PwrRecord::new("00000001".to_string(), "00000001".to_string());

        let line = original.to_cwr_line();
        let parsed = PwrRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 101);
    }
}
