//! ACK - Acknowledgement of Transaction Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// ACK - Acknowledgement of Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AckRecord {
    /// Always "ACK"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Creation date of original file YYYYMMDD (8 chars)
    pub creation_date: String,

    /// Creation time of original file HHMMSS (6 chars)
    pub creation_time: String,

    /// Original group ID (5 chars)
    pub original_group_id: String,

    /// Original transaction sequence number (8 chars)
    pub original_transaction_sequence_num: String,

    /// Original transaction type (3 chars)
    pub original_transaction_type: String,

    /// Creation title (60 chars, conditional)
    pub creation_title: Option<String>,

    /// Submitter creation number (20 chars, conditional)
    pub submitter_creation_num: Option<String>,

    /// Recipient creation number (20 chars, conditional)
    pub recipient_creation_num: Option<String>,

    /// Processing date YYYYMMDD (8 chars)
    pub processing_date: String,

    /// Transaction status (2 chars)
    pub transaction_status: String,
}

impl AckRecord {
    /// Create a new ACK record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, creation_date: String, creation_time: String, original_group_id: String, original_transaction_sequence_num: String, original_transaction_type: String, processing_date: String, transaction_status: String) -> Self {
        Self {
            record_type: "ACK".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            creation_date,
            creation_time,
            original_group_id,
            original_transaction_sequence_num,
            original_transaction_type,
            creation_title: None,
            submitter_creation_num: None,
            recipient_creation_num: None,
            processing_date,
            transaction_status,
        }
    }

    /// Parse a CWR line into an ACK record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 159 {
            return Err(CwrParseError::BadFormat("ACK line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "ACK" {
            return Err(CwrParseError::BadFormat(format!("Expected ACK, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let creation_date = line.get(19..27).unwrap().trim().to_string();
        let creation_time = line.get(27..33).unwrap().trim().to_string();
        let original_group_id = line.get(33..38).unwrap().trim().to_string();
        let original_transaction_sequence_num = line.get(38..46).unwrap().trim().to_string();
        let original_transaction_type = line.get(46..49).unwrap().trim().to_string();

        let creation_title = line.get(49..109).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let submitter_creation_num = line.get(109..129).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let recipient_creation_num = line.get(129..149).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let processing_date = line.get(149..157).unwrap().trim().to_string();
        let transaction_status = line.get(157..159).unwrap().trim().to_string();

        Ok(AckRecord { record_type, transaction_sequence_num, record_sequence_num, creation_date, creation_time, original_group_id, original_transaction_sequence_num, original_transaction_type, creation_title, submitter_creation_num, recipient_creation_num, processing_date, transaction_status })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:8}", self.creation_date),
            format!("{:6}", self.creation_time),
            format!("{:5}", self.original_group_id),
            format!("{:8}", self.original_transaction_sequence_num),
            format!("{:3}", self.original_transaction_type),
            format!("{:60}", self.creation_title.as_deref().unwrap_or("")),
            format!("{:20}", self.submitter_creation_num.as_deref().unwrap_or("")),
            format!("{:20}", self.recipient_creation_num.as_deref().unwrap_or("")),
            format!("{:8}", self.processing_date),
            format!("{:2}", self.transaction_status),
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ack_creation() {
        let ack = AckRecord::new("00000001".to_string(), "00000001".to_string(), "20050101".to_string(), "120000".to_string(), "00001".to_string(), "00000001".to_string(), "NWR".to_string(), "20050102".to_string(), "AS".to_string());

        assert_eq!(ack.record_type, "ACK");
        assert_eq!(ack.creation_date, "20050101");
        assert_eq!(ack.transaction_status, "AS");
    }

    #[test]
    fn test_ack_round_trip() {
        let original = AckRecord::new("00000001".to_string(), "00000001".to_string(), "20050101".to_string(), "120000".to_string(), "00001".to_string(), "00000001".to_string(), "NWR".to_string(), "20050102".to_string(), "AS".to_string());

        let line = original.to_cwr_line();
        let parsed = AckRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 159);
    }
}
