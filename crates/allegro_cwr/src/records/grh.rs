//! GRH - Group Header Record
//!
//! The Group Header record starts a new group of transactions within a CWR transmission.

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// GRH - Group Header Record
///
/// Starts a new group of transactions within a CWR transmission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrhRecord {
    /// Always "GRH"
    pub record_type: String,

    /// Transaction type code (3 chars)
    pub transaction_type: String,

    /// Group identifier within the transmission (5 chars)
    pub group_id: String,

    /// Version number for this transaction type (5 chars)
    pub version_number: String,

    /// Optional batch request identifier (10 chars)
    pub batch_request: Option<String>,

    /// Optional submission/distribution type (2 chars, blank for CWR)
    pub submission_distribution_type: Option<String>,
}

impl GrhRecord {
    /// Create a new GRH record with required fields
    pub fn new(transaction_type: String, group_id: String, version_number: String) -> Self {
        Self { record_type: "GRH".to_string(), transaction_type, group_id, version_number, batch_request: None, submission_distribution_type: None }
    }

    /// Parse a CWR line into a GRH record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 28 {
            return Err(CwrParseError::BadFormat("GRH line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "GRH" {
            return Err(CwrParseError::BadFormat(format!("Expected GRH, found {}", record_type)));
        }

        let transaction_type = line.get(3..6).unwrap().trim().to_string();
        let group_id = line.get(6..11).unwrap().trim().to_string();
        let version_number = line.get(11..16).unwrap().trim().to_string();

        let batch_request = line.get(16..26).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let submission_distribution_type = line.get(26..28).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        Ok(GrhRecord { record_type, transaction_type, group_id, version_number, batch_request, submission_distribution_type })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        [format!("{:3}", self.record_type), format!("{:3}", self.transaction_type), format!("{:5}", self.group_id), format!("{:5}", self.version_number), format!("{:10}", self.batch_request.as_deref().unwrap_or("")), format!("{:2}", self.submission_distribution_type.as_deref().unwrap_or(""))].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grh_creation() {
        let grh = GrhRecord::new("AGR".to_string(), "00001".to_string(), "02.10".to_string());

        assert_eq!(grh.record_type, "GRH");
        assert_eq!(grh.transaction_type, "AGR");
        assert_eq!(grh.group_id, "00001");
        assert_eq!(grh.version_number, "02.10");
    }

    #[test]
    fn test_grh_to_cwr_line() {
        let grh = GrhRecord::new("AGR".to_string(), "00001".to_string(), "02.10".to_string());

        let line = grh.to_cwr_line();
        // Total: 3 + 3 + 5 + 5 + 10 + 2 = 28 characters
        assert_eq!(line.len(), 28);
        assert_eq!(line, "GRHAGR0000102.10            ");
    }

    #[test]
    fn test_grh_from_cwr_line() {
        let line = "GRHAGR0000102.10            ";
        let grh = GrhRecord::from_cwr_line(line).unwrap();

        assert_eq!(grh.record_type, "GRH");
        assert_eq!(grh.transaction_type, "AGR");
        assert_eq!(grh.group_id, "00001");
        assert_eq!(grh.version_number, "02.10");
        assert_eq!(grh.batch_request, None);
        assert_eq!(grh.submission_distribution_type, None);
    }

    #[test]
    fn test_grh_round_trip() {
        let original = GrhRecord::new("NWR".to_string(), "12345".to_string(), "02.20".to_string());

        let line = original.to_cwr_line();
        let parsed = GrhRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
    }
}
