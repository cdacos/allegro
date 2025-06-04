//! PER - Performing Artist Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// PER - Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerRecord {
    /// Always "PER"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Performing artist last name (45 chars)
    pub performing_artist_last_name: String,

    /// Performing artist first name (30 chars, optional)
    pub performing_artist_first_name: Option<String>,

    /// Performing artist IPI name number (11 chars, optional)
    pub performing_artist_ipi_name_num: Option<String>,

    /// Performing artist IPI base number (13 chars, optional)
    pub performing_artist_ipi_base_number: Option<String>,
}

impl PerRecord {
    /// Create a new PER record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, performing_artist_last_name: String) -> Self {
        Self { record_type: "PER".to_string(), transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name: None, performing_artist_ipi_name_num: None, performing_artist_ipi_base_number: None }
    }

    /// Parse a CWR line into a PER record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 64 {
            return Err(CwrParseError::BadFormat("PER line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "PER" {
            return Err(CwrParseError::BadFormat(format!("Expected PER, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let performing_artist_last_name = line.get(19..64).unwrap().trim().to_string();

        let performing_artist_first_name = if line.len() > 64 { line.get(64..94).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let performing_artist_ipi_name_num = if line.len() > 94 { line.get(94..105).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let performing_artist_ipi_base_number = if line.len() > 105 { line.get(105..118).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(PerRecord { record_type, transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:45}", self.performing_artist_last_name)];

        if self.performing_artist_first_name.is_some() || self.performing_artist_ipi_name_num.is_some() || self.performing_artist_ipi_base_number.is_some() {
            fields.push(format!("{:30}", self.performing_artist_first_name.as_deref().unwrap_or("")));
        }

        if self.performing_artist_ipi_name_num.is_some() || self.performing_artist_ipi_base_number.is_some() {
            fields.push(format!("{:11}", self.performing_artist_ipi_name_num.as_deref().unwrap_or("")));
        }

        if self.performing_artist_ipi_base_number.is_some() {
            fields.push(format!("{:13}", self.performing_artist_ipi_base_number.as_deref().unwrap_or("")));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_creation() {
        let per = PerRecord::new("00000001".to_string(), "00000001".to_string(), "SMITH".to_string());

        assert_eq!(per.record_type, "PER");
        assert_eq!(per.performing_artist_last_name, "SMITH");
    }

    #[test]
    fn test_per_round_trip() {
        let original = PerRecord::new("00000001".to_string(), "00000001".to_string(), "SMITH".to_string());

        let line = original.to_cwr_line();
        let parsed = PerRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 64);
    }
}
