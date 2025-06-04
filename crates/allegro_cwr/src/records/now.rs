//! NOW - Non-Roman Alphabet Writer Name Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// NOW - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NowRecord {
    /// Always "NOW"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Writer name (160 chars)
    pub writer_name: String,
    
    /// Writer first name (160 chars)
    pub writer_first_name: String,
    
    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
    
    /// Writer position (1 char, optional)
    pub writer_position: Option<String>,
}

impl NowRecord {
    /// Create a new NOW record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        writer_name: String,
        writer_first_name: String,
    ) -> Self {
        Self {
            record_type: "NOW".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            writer_name,
            writer_first_name,
            language_code: None,
            writer_position: None,
        }
    }
    
    /// Parse a CWR line into a NOW record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 339 {
            return Err(CwrParseError::BadFormat("NOW line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NOW" {
            return Err(CwrParseError::BadFormat(format!("Expected NOW, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let writer_name = line.get(19..179).unwrap().trim().to_string();
        let writer_first_name = line.get(179..339).unwrap().trim().to_string();
        
        let language_code = if line.len() > 339 {
            line.get(339..341)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_position = if line.len() > 341 {
            line.get(341..342)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(NowRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            writer_name,
            writer_first_name,
            language_code,
            writer_position,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:160}", self.writer_name),
            format!("{:160}", self.writer_first_name)
        ];
        
        if self.language_code.is_some() || self.writer_position.is_some() {
            fields.push(format!("{:2}", self.language_code.as_deref().unwrap_or("")));
        }
        
        if let Some(ref position) = self.writer_position {
            fields.push(format!("{:1}", position));
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_creation() {
        let now = NowRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Writer Name".to_string(),
            "Non-Roman First Name".to_string(),
        );
        
        assert_eq!(now.record_type, "NOW");
        assert_eq!(now.writer_name, "Non-Roman Writer Name");
        assert_eq!(now.writer_first_name, "Non-Roman First Name");
    }
    
    #[test]
    fn test_now_round_trip() {
        let original = NowRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Writer Name".to_string(),
            "Non-Roman First Name".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = NowRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 339);
    }
}