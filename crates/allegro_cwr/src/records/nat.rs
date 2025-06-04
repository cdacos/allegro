//! NAT - Non-Roman Alphabet Title Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// NAT - Non-Roman Alphabet Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NatRecord {
    /// Always "NAT"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Title (640 chars)
    pub title: String,
    
    /// Title type (2 chars)
    pub title_type: String,
    
    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NatRecord {
    /// Create a new NAT record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        title: String,
        title_type: String,
    ) -> Self {
        Self {
            record_type: "NAT".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            title,
            title_type,
            language_code: None,
        }
    }
    
    /// Parse a CWR line into a NAT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 661 {
            return Err(CwrParseError::BadFormat("NAT line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NAT" {
            return Err(CwrParseError::BadFormat(format!("Expected NAT, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let title = line.get(19..659).unwrap().trim().to_string();
        let title_type = line.get(659..661).unwrap().trim().to_string();
        
        let language_code = if line.len() > 661 {
            line.get(661..663)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(NatRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            title,
            title_type,
            language_code,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:640}", self.title),
            format!("{:2}", self.title_type)
        ];
        
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
    fn test_nat_creation() {
        let nat = NatRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Title".to_string(),
            "OT".to_string(),
        );
        
        assert_eq!(nat.record_type, "NAT");
        assert_eq!(nat.title, "Non-Roman Title");
        assert_eq!(nat.title_type, "OT");
    }
    
    #[test]
    fn test_nat_round_trip() {
        let original = NatRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Title".to_string(),
            "OT".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = NatRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 661);
    }
}