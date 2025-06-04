//! NPN - Non-Roman Alphabet Publisher Name Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// NPN - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpnRecord {
    /// Always "NPN"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher sequence number (2 chars)
    pub publisher_sequence_num: String,

    /// Interested party number (9 chars)
    pub interested_party_num: String,

    /// Publisher name (480 chars)
    pub publisher_name: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NpnRecord {
    /// Create a new NPN record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, publisher_sequence_num: String, interested_party_num: String, publisher_name: String) -> Self {
        Self { record_type: "NPN".to_string(), transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code: None }
    }

    /// Parse a CWR line into a NPN record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 510 {
            return Err(CwrParseError::BadFormat("NPN line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NPN" {
            return Err(CwrParseError::BadFormat(format!("Expected NPN, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let publisher_sequence_num = line.get(19..21).unwrap().trim().to_string();
        let interested_party_num = line.get(21..30).unwrap().trim().to_string();
        let publisher_name = line.get(30..510).unwrap().trim().to_string();

        let language_code = if line.len() > 510 { line.get(510..512).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(NpnRecord { record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:2}", self.publisher_sequence_num), format!("{:9}", self.interested_party_num), format!("{:480}", self.publisher_name)];

        if let Some(ref language) = self.language_code {
            fields.push(format!("{:2}", language));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npn_creation() {
        let npn = NpnRecord::new("00000001".to_string(), "00000001".to_string(), "01".to_string(), "123456789".to_string(), "Non-Roman Publisher Name".to_string());

        assert_eq!(npn.record_type, "NPN");
        assert_eq!(npn.publisher_sequence_num, "01");
        assert_eq!(npn.publisher_name, "Non-Roman Publisher Name");
    }

    #[test]
    fn test_npn_round_trip() {
        let original = NpnRecord::new("00000001".to_string(), "00000001".to_string(), "01".to_string(), "123456789".to_string(), "Non-Roman Publisher Name".to_string());

        let line = original.to_cwr_line();
        let parsed = NpnRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 510);
    }
}
