//! GRT - Group Trailer Record
//!
//! The Group Trailer record marks the end of a group and contains summary counts for that group.

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// GRT - Group Trailer Record
///
/// Marks the end of a group and contains summary counts for that group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrtRecord {
    /// Always "GRT"
    pub record_type: String,

    /// Group ID (5 chars)
    pub group_id: String,

    /// Transaction count (8 chars)
    pub transaction_count: String,

    /// Record count (8 chars)
    pub record_count: String,

    /// Currency indicator (3 chars, conditional)
    pub currency_indicator: Option<String>,

    /// Total monetary value (10 chars, optional)
    pub total_monetary_value: Option<String>,
}

impl GrtRecord {
    /// Create a new GRT record
    pub fn new(group_id: String, transaction_count: String, record_count: String) -> Self {
        Self { record_type: "GRT".to_string(), group_id, transaction_count, record_count, currency_indicator: None, total_monetary_value: None }
    }

    /// Parse a CWR line into a GRT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 24 {
            return Err(CwrParseError::BadFormat("GRT line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "GRT" {
            return Err(CwrParseError::BadFormat(format!("Expected GRT, found {}", record_type)));
        }

        let group_id = line.get(3..8).unwrap().trim().to_string();
        let transaction_count = line.get(8..16).unwrap().trim().to_string();
        let record_count = line.get(16..24).unwrap().trim().to_string();

        let currency_indicator = if line.len() > 24 { line.get(24..27).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let total_monetary_value = if line.len() > 27 { line.get(27..37).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(GrtRecord { record_type, group_id, transaction_count, record_count, currency_indicator, total_monetary_value })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![format!("{:3}", self.record_type), format!("{:5}", self.group_id), format!("{:8}", self.transaction_count), format!("{:8}", self.record_count), format!("{:3}", self.currency_indicator.as_deref().unwrap_or("")), format!("{:10}", self.total_monetary_value.as_deref().unwrap_or(""))].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grt_creation() {
        let grt = GrtRecord::new("00001".to_string(), "00000001".to_string(), "00000003".to_string());

        assert_eq!(grt.record_type, "GRT");
        assert_eq!(grt.group_id, "00001");
        assert_eq!(grt.transaction_count, "00000001");
        assert_eq!(grt.record_count, "00000003");
    }

    #[test]
    fn test_grt_round_trip() {
        let original = GrtRecord::new("00001".to_string(), "00000001".to_string(), "00000003".to_string());

        let line = original.to_cwr_line();
        let parsed = GrtRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 37); // Full length with optional fields
    }
}
