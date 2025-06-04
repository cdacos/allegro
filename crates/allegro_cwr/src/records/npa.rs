//! NPA - Non-Roman Alphabet Publisher Name Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// NPA - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpaRecord {
    /// Always "NPA"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Interested party name (160 chars)
    pub interested_party_name: String,

    /// Interested party writer first name (160 chars)
    pub interested_party_writer_first_name: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NpaRecord {
    /// Create a new NPA record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, interested_party_name: String, interested_party_writer_first_name: String) -> Self {
        Self { record_type: "NPA".to_string(), transaction_sequence_num, record_sequence_num, interested_party_num: None, interested_party_name, interested_party_writer_first_name, language_code: None }
    }

    /// Parse a CWR line into a NPA record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 348 {
            return Err(CwrParseError::BadFormat("NPA line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NPA" {
            return Err(CwrParseError::BadFormat(format!("Expected NPA, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();

        let interested_party_num = line.get(19..28).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let interested_party_name = line.get(28..188).unwrap().trim().to_string();
        let interested_party_writer_first_name = line.get(188..348).unwrap().trim().to_string();

        let language_code = if line.len() > 348 { line.get(348..350).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(NpaRecord { record_type, transaction_sequence_num, record_sequence_num, interested_party_num, interested_party_name, interested_party_writer_first_name, language_code })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields =
            vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:9}", self.interested_party_num.as_deref().unwrap_or("")), format!("{:160}", self.interested_party_name), format!("{:160}", self.interested_party_writer_first_name)];

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
    fn test_npa_creation() {
        let npa = NpaRecord::new("00000001".to_string(), "00000001".to_string(), "Non-Roman Publisher Name".to_string(), "Non-Roman First Name".to_string());

        assert_eq!(npa.record_type, "NPA");
        assert_eq!(npa.interested_party_name, "Non-Roman Publisher Name");
    }

    #[test]
    fn test_npa_round_trip() {
        let original = NpaRecord::new("00000001".to_string(), "00000001".to_string(), "Non-Roman Publisher Name".to_string(), "Non-Roman First Name".to_string());

        let line = original.to_cwr_line();
        let parsed = NpaRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 348);
    }
}
