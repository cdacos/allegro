//! TER - Territory in Agreement Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// TER - Territory in Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerRecord {
    /// Always "TER"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Inclusion/Exclusion indicator (1 char)
    pub inclusion_exclusion_indicator: String,

    /// TIS Numeric Code (4 chars)
    pub tis_numeric_code: String,
}

impl TerRecord {
    /// Create a new TER record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, inclusion_exclusion_indicator: String, tis_numeric_code: String) -> Self {
        Self { record_type: "TER".to_string(), transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code }
    }

    /// Parse a CWR line into a TER record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 24 {
            return Err(CwrParseError::BadFormat("TER line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "TER" {
            return Err(CwrParseError::BadFormat(format!("Expected TER, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let inclusion_exclusion_indicator = line.get(19..20).unwrap().trim().to_string();
        let tis_numeric_code = line.get(20..24).unwrap().trim().to_string();

        Ok(TerRecord { record_type, transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:1}", self.inclusion_exclusion_indicator), format!("{:4}", self.tis_numeric_code)].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ter_creation() {
        let ter = TerRecord::new("00000001".to_string(), "00000001".to_string(), "I".to_string(), "2840".to_string());

        assert_eq!(ter.record_type, "TER");
        assert_eq!(ter.inclusion_exclusion_indicator, "I");
        assert_eq!(ter.tis_numeric_code, "2840");
    }

    #[test]
    fn test_ter_round_trip() {
        let original = TerRecord::new("00000001".to_string(), "00000001".to_string(), "I".to_string(), "2840".to_string());

        let line = original.to_cwr_line();
        let parsed = TerRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 24);
    }
}
