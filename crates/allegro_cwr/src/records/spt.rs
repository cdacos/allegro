//! SPT - Publisher Territory of Control Record / OPT - Other Publisher Territory Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// SPT - Publisher Territory of Control Record (also OPT - Other Publisher Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SptRecord {
    /// "SPT" or "OPT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars)
    pub interested_party_num: String,

    /// Constant - spaces (6 chars)
    pub constant: String,

    /// PR collection share (5 chars, conditional)
    pub pr_collection_share: Option<String>,

    /// MR collection share (5 chars, conditional)
    pub mr_collection_share: Option<String>,

    /// SR collection share (5 chars, conditional)
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

impl SptRecord {
    /// Create a new SPT record
    pub fn new(record_type: String, transaction_sequence_num: String, record_sequence_num: String, interested_party_num: String, inclusion_exclusion_indicator: String, tis_numeric_code: String) -> Self {
        Self {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            interested_party_num,
            constant: "      ".to_string(), // 6 spaces
            pr_collection_share: None,
            mr_collection_share: None,
            sr_collection_share: None,
            inclusion_exclusion_indicator,
            tis_numeric_code,
            shares_change: None,
            sequence_num: None,
        }
    }

    /// Parse a CWR line into a SPT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 55 {
            return Err(CwrParseError::BadFormat("SPT line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if !["SPT", "OPT"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected SPT/OPT, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let interested_party_num = line.get(19..28).unwrap().trim().to_string();
        let constant = line.get(28..34).unwrap().to_string();

        let pr_collection_share = line.get(34..39).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let mr_collection_share = line.get(39..44).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sr_collection_share = line.get(44..49).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let inclusion_exclusion_indicator = line.get(49..50).unwrap().trim().to_string();
        let tis_numeric_code = line.get(50..54).unwrap().trim().to_string();

        let shares_change = line.get(54..55).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sequence_num = if line.len() > 55 { line.get(55..58).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(SptRecord { record_type, transaction_sequence_num, record_sequence_num, interested_party_num, constant, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:9}", self.interested_party_num),
            format!("{:6}", self.constant),
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
    fn test_spt_creation() {
        let spt = SptRecord::new("SPT".to_string(), "00000001".to_string(), "00000001".to_string(), "123456789".to_string(), "I".to_string(), "2840".to_string());

        assert_eq!(spt.record_type, "SPT");
        assert_eq!(spt.inclusion_exclusion_indicator, "I");
        assert_eq!(spt.tis_numeric_code, "2840");
    }

    #[test]
    fn test_spt_round_trip() {
        let original = SptRecord::new("SPT".to_string(), "00000001".to_string(), "00000001".to_string(), "123456789".to_string(), "I".to_string(), "2840".to_string());

        let line = original.to_cwr_line();
        let parsed = SptRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 55);
    }
}
