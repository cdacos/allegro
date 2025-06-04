//! ARI - Additional Related Information Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// ARI - Additional Related Information Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AriRecord {
    /// Always "ARI"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Society number (3 chars)
    pub society_num: String,

    /// Work number (14 chars, conditional)
    pub work_num: Option<String>,

    /// Type of right (3 chars)
    pub type_of_right: String,

    /// Subject code (2 chars, conditional)
    pub subject_code: Option<String>,

    /// Note (160 chars, conditional)
    pub note: Option<String>,
}

impl AriRecord {
    /// Create a new ARI record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, society_num: String, type_of_right: String) -> Self {
        Self { record_type: "ARI".to_string(), transaction_sequence_num, record_sequence_num, society_num, work_num: None, type_of_right, subject_code: None, note: None }
    }

    /// Parse a CWR line into an ARI record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 25 {
            return Err(CwrParseError::BadFormat("ARI line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "ARI" {
            return Err(CwrParseError::BadFormat(format!("Expected ARI, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let society_num = line.get(19..22).unwrap().trim().to_string();

        let work_num = if line.len() > 22 { line.get(22..36).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let type_of_right = if line.len() > 36 {
            line.get(36..39).unwrap().trim().to_string()
        } else {
            return Err(CwrParseError::BadFormat("ARI line missing type of right".to_string()));
        };

        let subject_code = if line.len() > 39 { line.get(39..41).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let note = if line.len() > 41 { line.get(41..201).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(AriRecord { record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:3}", self.society_num),
            format!("{:14}", self.work_num.as_deref().unwrap_or("")),
            format!("{:3}", self.type_of_right),
            format!("{:2}", self.subject_code.as_deref().unwrap_or("")),
            format!("{:160}", self.note.as_deref().unwrap_or("")),
        ];

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ari_creation() {
        let ari = AriRecord::new("00000001".to_string(), "00000001".to_string(), "021".to_string(), "ALL".to_string());

        assert_eq!(ari.record_type, "ARI");
        assert_eq!(ari.society_num, "021");
        assert_eq!(ari.type_of_right, "ALL");
    }

    #[test]
    fn test_ari_round_trip() {
        let original = AriRecord::new("00000001".to_string(), "00000001".to_string(), "021".to_string(), "ALL".to_string());

        let line = original.to_cwr_line();
        let parsed = AriRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 201);
    }
}
