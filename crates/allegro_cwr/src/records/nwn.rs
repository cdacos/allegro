//! NWN - Non-Roman Alphabet Writer Name Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// NWN - Non-Roman Alphabet Writer Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NwnRecord {
    /// Always "NWN"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,
    
    /// Writer last name (160 chars)
    pub writer_last_name: String,
    
    /// Writer first name (160 chars, optional)
    pub writer_first_name: Option<String>,
    
    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
}

impl NwnRecord {
    /// Create a new NWN record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        writer_last_name: String,
    ) -> Self {
        Self {
            record_type: "NWN".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            interested_party_num: None,
            writer_last_name,
            writer_first_name: None,
            language_code: None,
        }
    }
    
    /// Parse a CWR line into a NWN record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 188 {
            return Err(CwrParseError::BadFormat("NWN line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NWN" {
            return Err(CwrParseError::BadFormat(format!("Expected NWN, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        
        let interested_party_num = line.get(19..28)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let writer_last_name = line.get(28..188).unwrap().trim().to_string();
        
        let writer_first_name = if line.len() > 188 {
            line.get(188..348)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let language_code = if line.len() > 348 {
            line.get(348..350)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(NwnRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            interested_party_num,
            writer_last_name,
            writer_first_name,
            language_code,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:9}", self.interested_party_num.as_deref().unwrap_or("")),
            format!("{:160}", self.writer_last_name)
        ];
        
        if self.writer_first_name.is_some() || self.language_code.is_some() {
            fields.push(format!("{:160}", self.writer_first_name.as_deref().unwrap_or("")));
        }
        
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
    fn test_nwn_creation() {
        let nwn = NwnRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Writer Name".to_string(),
        );
        
        assert_eq!(nwn.record_type, "NWN");
        assert_eq!(nwn.writer_last_name, "Non-Roman Writer Name");
    }
    
    #[test]
    fn test_nwn_round_trip() {
        let original = NwnRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Non-Roman Writer Name".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = NwnRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 188);
    }
}