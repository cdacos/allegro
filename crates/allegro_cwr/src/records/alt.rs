//! ALT - Alternate Title Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// ALT - Alternate Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AltRecord {
    /// Always "ALT"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Alternate title (60 chars)
    pub alternate_title: String,
    
    /// Title type (2 chars)
    pub title_type: String,
    
    /// Language code (2 chars, conditional)
    pub language_code: Option<String>,
}

impl AltRecord {
    /// Create a new ALT record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        alternate_title: String,
        title_type: String,
    ) -> Self {
        Self {
            record_type: "ALT".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            alternate_title,
            title_type,
            language_code: None,
        }
    }
    
    /// Parse a CWR line into an ALT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 81 {
            return Err(CwrParseError::BadFormat("ALT line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "ALT" {
            return Err(CwrParseError::BadFormat(format!("Expected ALT, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let alternate_title = line.get(19..79).unwrap().trim().to_string();
        let title_type = line.get(79..81).unwrap().trim().to_string();
        
        let language_code = if line.len() > 81 {
            line.get(81..83)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(AltRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            alternate_title,
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
            format!("{:60}", self.alternate_title),
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
    fn test_alt_creation() {
        let alt = AltRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "My Alternate Title".to_string(),
            "AT".to_string(),
        );
        
        assert_eq!(alt.record_type, "ALT");
        assert_eq!(alt.alternate_title, "My Alternate Title");
        assert_eq!(alt.title_type, "AT");
    }
    
    #[test]
    fn test_alt_round_trip() {
        let original = AltRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "My Alternate Title".to_string(),
            "AT".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = AltRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 81);
    }
}