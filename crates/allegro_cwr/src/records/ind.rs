//! IND - Instrumentation Detail Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// IND - Instrumentation Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndRecord {
    /// Always "IND"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Instrument code (3 chars)
    pub instrument_code: String,
    
    /// Number of players (3 chars, optional)
    pub number_of_players: Option<String>,
}

impl IndRecord {
    /// Create a new IND record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        instrument_code: String,
    ) -> Self {
        Self {
            record_type: "IND".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            instrument_code,
            number_of_players: None,
        }
    }
    
    /// Parse a CWR line into an IND record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 22 {
            return Err(CwrParseError::BadFormat("IND line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "IND" {
            return Err(CwrParseError::BadFormat(format!("Expected IND, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let instrument_code = line.get(19..22).unwrap().trim().to_string();
        
        let number_of_players = if line.len() > 22 {
            line.get(22..25)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(IndRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            instrument_code,
            number_of_players,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:3}", self.instrument_code)
        ];
        
        if let Some(ref players) = self.number_of_players {
            fields.push(format!("{:3}", players));
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ind_creation() {
        let ind = IndRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "PNO".to_string(),
        );
        
        assert_eq!(ind.record_type, "IND");
        assert_eq!(ind.instrument_code, "PNO");
    }
    
    #[test]
    fn test_ind_round_trip() {
        let original = IndRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "PNO".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = IndRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 22);
    }
}