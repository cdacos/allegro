//! IPA - Interested Party of Agreement Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// IPA - Interested Party of Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpaRecord {
    /// Always "IPA"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Agreement role code (2 chars)
    pub agreement_role_code: String,

    /// Interested party IPI name number (11 chars, optional)
    pub interested_party_ipi_name_num: Option<String>,

    /// IPI base number (13 chars, optional)
    pub ipi_base_number: Option<String>,

    /// Interested party number (9 chars)
    pub interested_party_num: String,

    /// Interested party last name (45 chars)
    pub interested_party_last_name: String,

    /// Interested party writer first name (30 chars, optional)
    pub interested_party_writer_first_name: Option<String>,

    /// PR affiliation society (3 chars, conditional)
    pub pr_affiliation_society: Option<String>,

    /// PR share (5 chars, conditional)
    pub pr_share: Option<String>,

    /// MR affiliation society (3 chars, conditional)
    pub mr_affiliation_society: Option<String>,

    /// MR share (5 chars, conditional)
    pub mr_share: Option<String>,

    /// SR affiliation society (3 chars, conditional)
    pub sr_affiliation_society: Option<String>,

    /// SR share (5 chars, conditional)
    pub sr_share: Option<String>,
}

impl IpaRecord {
    /// Create a new IPA record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, agreement_role_code: String, interested_party_num: String, interested_party_last_name: String) -> Self {
        Self {
            record_type: "IPA".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            agreement_role_code,
            interested_party_ipi_name_num: None,
            ipi_base_number: None,
            interested_party_num,
            interested_party_last_name,
            interested_party_writer_first_name: None,
            pr_affiliation_society: None,
            pr_share: None,
            mr_affiliation_society: None,
            mr_share: None,
            sr_affiliation_society: None,
            sr_share: None,
        }
    }

    /// Parse a CWR line into an IPA record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 153 {
            return Err(CwrParseError::BadFormat("IPA line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "IPA" {
            return Err(CwrParseError::BadFormat(format!("Expected IPA, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let agreement_role_code = line.get(19..21).unwrap().trim().to_string();

        let interested_party_ipi_name_num = line.get(21..32).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let ipi_base_number = line.get(32..45).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let interested_party_num = line.get(45..54).unwrap().trim().to_string();
        let interested_party_last_name = line.get(54..99).unwrap().trim().to_string();

        let interested_party_writer_first_name = line.get(99..129).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let pr_affiliation_society = line.get(129..132).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let pr_share = line.get(132..137).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let mr_affiliation_society = line.get(137..140).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let mr_share = line.get(140..145).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sr_affiliation_society = line.get(145..148).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let sr_share = line.get(148..153).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        Ok(IpaRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            agreement_role_code,
            interested_party_ipi_name_num,
            ipi_base_number,
            interested_party_num,
            interested_party_last_name,
            interested_party_writer_first_name,
            pr_affiliation_society,
            pr_share,
            mr_affiliation_society,
            mr_share,
            sr_affiliation_society,
            sr_share,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:2}", self.agreement_role_code),
            format!("{:11}", self.interested_party_ipi_name_num.as_deref().unwrap_or("")),
            format!("{:13}", self.ipi_base_number.as_deref().unwrap_or("")),
            format!("{:9}", self.interested_party_num),
            format!("{:45}", self.interested_party_last_name),
            format!("{:30}", self.interested_party_writer_first_name.as_deref().unwrap_or("")),
            format!("{:3}", self.pr_affiliation_society.as_deref().unwrap_or("")),
            format!("{:5}", self.pr_share.as_deref().unwrap_or("")),
            format!("{:3}", self.mr_affiliation_society.as_deref().unwrap_or("")),
            format!("{:5}", self.mr_share.as_deref().unwrap_or("")),
            format!("{:3}", self.sr_affiliation_society.as_deref().unwrap_or("")),
            format!("{:5}", self.sr_share.as_deref().unwrap_or("")),
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipa_creation() {
        let ipa = IpaRecord::new("00000001".to_string(), "00000001".to_string(), "AS".to_string(), "123456789".to_string(), "JONES".to_string());

        assert_eq!(ipa.record_type, "IPA");
        assert_eq!(ipa.agreement_role_code, "AS");
        assert_eq!(ipa.interested_party_last_name, "JONES");
    }

    #[test]
    fn test_ipa_round_trip() {
        let original = IpaRecord::new("00000001".to_string(), "00000001".to_string(), "AS".to_string(), "123456789".to_string(), "JONES".to_string());

        let line = original.to_cwr_line();
        let parsed = IpaRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 153);
    }
}
