//! NET - Non-Roman Alphabet Entire Work Title for Excerpts/Components/Versions Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// NET - Non-Roman Alphabet Entire Work Title for Excerpts/Components/Versions Record
/// Also handles NCT (Non-Roman Alphabet Title for Components) and NVT (Non-Roman Alphabet Original Title for Versions)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetRecord {
    /// "NET", "NCT", or "NVT"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Title (640 chars)
    pub title: String,
    
    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NetRecord {
    /// Create a new NET record
    pub fn new(
        record_type: String,
        transaction_sequence_num: String,
        record_sequence_num: String,
        title: String,
    ) -> Self {
        Self {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            title,
            language_code: None,
        }
    }
    
    /// Parse a CWR line into a NET record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 659 {
            return Err(CwrParseError::BadFormat("NET line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if !["NET", "NCT", "NVT"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected NET/NCT/NVT, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let title = line.get(19..659).unwrap().trim().to_string();
        
        let language_code = if line.len() > 659 {
            line.get(659..661)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(NetRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            title,
            language_code,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:640}", self.title)
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
    fn test_net_creation() {
        let net = NetRecord::new(
            "NET".to_string(),
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Entire Work Title".to_string(),
        );
        
        assert_eq!(net.record_type, "NET");
        assert_eq!(net.title, "Non-Roman Entire Work Title");
    }
    
    #[test]
    fn test_net_round_trip() {
        let original = NetRecord::new(
            "NET".to_string(),
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Entire Work Title".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = NetRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 659);
    }
}