//! ARI - Additional Related Information Record

use crate::error::CwrParseError;
use crate::util::{validate_record_type, extract_required_field, extract_optional_field};
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
        let mut warnings = Vec::new();

        let record_type = validate_record_type(line, "ARI")?;
        let transaction_sequence_num = extract_required_field(line, 3, 11, "transaction_sequence_num", &mut warnings)?;
        let record_sequence_num = extract_required_field(line, 11, 19, "record_sequence_num", &mut warnings)?;
        let society_num = extract_required_field(line, 19, 22, "society_num", &mut warnings)?;

        let work_num = extract_optional_field(line, 22, 36, "work_num", "ARI", &mut warnings);

        let type_of_right = if line.len() > 36 {
            extract_required_field(line, 36, 39, "type_of_right", &mut warnings)?
        } else {
            warnings.push("ARI record missing type of right".to_string());
            "".to_string()
        };

        let subject_code = extract_optional_field(line, 39, 41, "subject_code", "ARI", &mut warnings);
        let note = extract_optional_field(line, 41, 201, "note", "ARI", &mut warnings);

        let record = AriRecord { record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note };

        if !warnings.is_empty() {
            for warning in warnings {
                log::warn!("{}", warning);
            }
        }
        Ok(record)
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let fields = [
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
        let result = AriRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, result);
        assert_eq!(line.len(), 201);
    }
}
