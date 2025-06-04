//! SWT - Writer Territory of Control Record / OWT - Other Writer Territory Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// SWT - Writer Territory of Control Record (also OWT - Other Writer Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwtRecord {
    /// "SWT" or "OWT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// PR collection share (5 chars, optional)
    pub pr_collection_share: Option<String>,

    /// MR collection share (5 chars, optional)
    pub mr_collection_share: Option<String>,

    /// SR collection share (5 chars, optional)
    pub sr_collection_share: Option<String>,

    /// Inclusion/Exclusion indicator (1 char)
    pub inclusion_exclusion_indicator: String,

    /// TIS numeric code (4 chars)
    pub tis_numeric_code: String,

    /// Shares change (1 char, optional)
    pub shares_change: Option<String>,

    /// Sequence number (3 chars, v2.1+)
    pub sequence_num: Option<String>,
}

impl SwtRecord {
    /// Create a new SWT record
    pub fn new(record_type: String, transaction_sequence_num: String, record_sequence_num: String, inclusion_exclusion_indicator: String, tis_numeric_code: String) -> Self {
        Self { record_type, transaction_sequence_num, record_sequence_num, interested_party_num: None, pr_collection_share: None, mr_collection_share: None, sr_collection_share: None, inclusion_exclusion_indicator, tis_numeric_code, shares_change: None, sequence_num: None }
    }

    /// Parse a CWR line into a SWT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 49 {
            return Err(CwrParseError::BadFormat("SWT line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if !["SWT", "OWT"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected SWT/OWT, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();

        let interested_party_num = line.get(19..28).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let pr_collection_share = line.get(28..33).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let mr_collection_share = line.get(33..38).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sr_collection_share = line.get(38..43).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let inclusion_exclusion_indicator = line.get(43..44).unwrap().trim().to_string();
        let tis_numeric_code = line.get(44..48).unwrap().trim().to_string();

        let shares_change = line.get(48..49).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sequence_num = if line.len() > 49 { line.get(49..52).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(SwtRecord { record_type, transaction_sequence_num, record_sequence_num, interested_party_num, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:9}", self.interested_party_num.as_deref().unwrap_or("")),
            format!("{:5}", self.pr_collection_share.as_deref().unwrap_or("")),
            format!("{:5}", self.mr_collection_share.as_deref().unwrap_or("")),
            format!("{:5}", self.sr_collection_share.as_deref().unwrap_or("")),
            format!("{:1}", self.inclusion_exclusion_indicator),
            format!("{:4}", self.tis_numeric_code),
            format!("{:1}", self.shares_change.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ field
        if let Some(ref sequence) = self.sequence_num {
            fields.push(format!("{:3}", sequence));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swt_creation() {
        let swt = SwtRecord::new("SWT".to_string(), "00000001".to_string(), "00000001".to_string(), "I".to_string(), "2840".to_string());

        assert_eq!(swt.record_type, "SWT");
        assert_eq!(swt.inclusion_exclusion_indicator, "I");
        assert_eq!(swt.tis_numeric_code, "2840");
    }

    #[test]
    fn test_swt_round_trip() {
        let original = SwtRecord::new("SWT".to_string(), "00000001".to_string(), "00000001".to_string(), "I".to_string(), "2840".to_string());

        let line = original.to_cwr_line();
        let parsed = SwtRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 49);
    }
}
