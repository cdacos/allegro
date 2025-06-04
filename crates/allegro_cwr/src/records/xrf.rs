//! XRF - Work ID Cross Reference Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// XRF - Work ID Cross Reference Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XrfRecord {
    /// Always "XRF"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Organisation code (3 chars)
    pub organisation_code: String,

    /// Identifier (14 chars)
    pub identifier: String,

    /// Identifier type (1 char)
    pub identifier_type: String,

    /// Validity (1 char)
    pub validity: String,
}

impl XrfRecord {
    /// Create a new XRF record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, organisation_code: String, identifier: String, identifier_type: String, validity: String) -> Self {
        Self { record_type: "XRF".to_string(), transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity }
    }

    /// Parse a CWR line into an XRF record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 38 {
            return Err(CwrParseError::BadFormat("XRF line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "XRF" {
            return Err(CwrParseError::BadFormat(format!("Expected XRF, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let organisation_code = line.get(19..22).unwrap().trim().to_string();
        let identifier = line.get(22..36).unwrap().trim().to_string();
        let identifier_type = line.get(36..37).unwrap().trim().to_string();
        let validity = line.get(37..38).unwrap().trim().to_string();

        Ok(XrfRecord { record_type, transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        [format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:3}", self.organisation_code), format!("{:14}", self.identifier), format!("{:1}", self.identifier_type), format!("{:1}", self.validity)].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xrf_creation() {
        let xrf = XrfRecord::new("00000001".to_string(), "00000001".to_string(), "ISW".to_string(), "T1234567890123".to_string(), "W".to_string(), "Y".to_string());

        assert_eq!(xrf.record_type, "XRF");
        assert_eq!(xrf.organisation_code, "ISW");
        assert_eq!(xrf.identifier_type, "W");
        assert_eq!(xrf.validity, "Y");
    }

    #[test]
    fn test_xrf_round_trip() {
        let original = XrfRecord::new("00000001".to_string(), "00000001".to_string(), "ISW".to_string(), "T1234567890123".to_string(), "W".to_string(), "Y".to_string());

        let line = original.to_cwr_line();
        let parsed = XrfRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 38);
    }
}
