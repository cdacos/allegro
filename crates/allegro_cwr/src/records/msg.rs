//! MSG - Message Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// MSG - Message Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgRecord {
    /// Always "MSG"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Message type (1 char)
    pub message_type: String,

    /// Original record sequence number (8 chars)
    pub original_record_sequence_num: String,

    /// Record type (3 chars)
    pub record_type_field: String,

    /// Message level (1 char)
    pub message_level: String,

    /// Validation number (3 chars)
    pub validation_number: String,

    /// Message text (150 chars)
    pub message_text: String,
}

impl MsgRecord {
    /// Create a new MSG record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, message_type: String, original_record_sequence_num: String, record_type_field: String, message_level: String, validation_number: String, message_text: String) -> Self {
        Self { record_type: "MSG".to_string(), transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, record_type_field, message_level, validation_number, message_text }
    }

    /// Parse a CWR line into a MSG record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 185 {
            return Err(CwrParseError::BadFormat("MSG line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "MSG" {
            return Err(CwrParseError::BadFormat(format!("Expected MSG, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let message_type = line.get(19..20).unwrap().trim().to_string();
        let original_record_sequence_num = line.get(20..28).unwrap().trim().to_string();
        let record_type_field = line.get(28..31).unwrap().trim().to_string();
        let message_level = line.get(31..32).unwrap().trim().to_string();
        let validation_number = line.get(32..35).unwrap().trim().to_string();
        let message_text = line.get(35..185).unwrap().trim().to_string();

        Ok(MsgRecord { record_type, transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, record_type_field, message_level, validation_number, message_text })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:1}", self.message_type),
            format!("{:8}", self.original_record_sequence_num),
            format!("{:3}", self.record_type_field),
            format!("{:1}", self.message_level),
            format!("{:3}", self.validation_number),
            format!("{:150}", self.message_text),
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_creation() {
        let msg = MsgRecord::new("00000001".to_string(), "00000001".to_string(), "E".to_string(), "00000002".to_string(), "NWR".to_string(), "R".to_string(), "001".to_string(), "Record rejected due to invalid format".to_string());

        assert_eq!(msg.record_type, "MSG");
        assert_eq!(msg.message_type, "E");
        assert_eq!(msg.message_text, "Record rejected due to invalid format");
    }

    #[test]
    fn test_msg_round_trip() {
        let original = MsgRecord::new("00000001".to_string(), "00000001".to_string(), "E".to_string(), "00000002".to_string(), "NWR".to_string(), "R".to_string(), "001".to_string(), "Record rejected due to invalid format".to_string());

        let line = original.to_cwr_line();
        let parsed = MsgRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 185);
    }
}
