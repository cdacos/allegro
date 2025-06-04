//! TRL - Transmission Trailer Record
//! 
//! The Transmission Trailer record marks the end of a CWR transmission and contains summary counts.

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// TRL - Transmission Trailer Record
/// 
/// Marks the end of a CWR transmission and contains summary counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrlRecord {
    /// Always "TRL"
    pub record_type: String,
    
    /// Group count (5 chars)
    pub group_count: String,
    
    /// Transaction count (8 chars)
    pub transaction_count: String,
    
    /// Record count (8 chars)
    pub record_count: String,
}

impl TrlRecord {
    /// Create a new TRL record
    pub fn new(
        group_count: String,
        transaction_count: String,
        record_count: String,
    ) -> Self {
        Self {
            record_type: "TRL".to_string(),
            group_count,
            transaction_count,
            record_count,
        }
    }
    
    /// Parse a CWR line into a TRL record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 24 {
            return Err(CwrParseError::BadFormat("TRL line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "TRL" {
            return Err(CwrParseError::BadFormat(format!("Expected TRL, found {}", record_type)));
        }
        
        let group_count = line.get(3..8).unwrap().trim().to_string();
        let transaction_count = line.get(8..16).unwrap().trim().to_string();
        let record_count = line.get(16..24).unwrap().trim().to_string();
        
        Ok(TrlRecord {
            record_type,
            group_count,
            transaction_count,
            record_count,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![
            format!("{:3}", self.record_type),
            format!("{:5}", self.group_count),
            format!("{:8}", self.transaction_count),
            format!("{:8}", self.record_count)
        ].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trl_creation() {
        let trl = TrlRecord::new(
            "00001".to_string(),
            "00000001".to_string(),
            "00000005".to_string(),
        );
        
        assert_eq!(trl.record_type, "TRL");
        assert_eq!(trl.group_count, "00001");
        assert_eq!(trl.transaction_count, "00000001");
        assert_eq!(trl.record_count, "00000005");
    }
    
    #[test]
    fn test_trl_round_trip() {
        let original = TrlRecord::new(
            "00001".to_string(),
            "00000001".to_string(),
            "00000005".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = TrlRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 24);
    }
}