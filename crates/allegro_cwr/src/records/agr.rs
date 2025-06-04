//! AGR - Agreement Transaction Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// AGR - Agreement Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgrRecord {
    /// Always "AGR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Submitter agreement number (14 chars)
    pub submitter_agreement_number: String,

    /// International standard agreement code (14 chars, optional)
    pub international_standard_agreement_code: Option<String>,

    /// Agreement type (2 chars)
    pub agreement_type: String,

    /// Agreement start date YYYYMMDD (8 chars)
    pub agreement_start_date: String,

    /// Agreement end date YYYYMMDD (8 chars, optional)
    pub agreement_end_date: Option<String>,

    /// Retention end date YYYYMMDD (8 chars, optional)
    pub retention_end_date: Option<String>,

    /// Prior royalty status (1 char)
    pub prior_royalty_status: String,

    /// Prior royalty start date YYYYMMDD (8 chars, conditional)
    pub prior_royalty_start_date: Option<String>,

    /// Post-term collection status (1 char)
    pub post_term_collection_status: String,

    /// Post-term collection end date YYYYMMDD (8 chars, conditional)
    pub post_term_collection_end_date: Option<String>,

    /// Date of signature of agreement YYYYMMDD (8 chars, optional)
    pub date_of_signature_of_agreement: Option<String>,

    /// Number of works (5 chars)
    pub number_of_works: String,

    /// Sales/manufacture clause (1 char, conditional)
    pub sales_manufacture_clause: Option<String>,

    /// Shares change (1 char, optional)
    pub shares_change: Option<String>,

    /// Advance given (1 char, optional)
    pub advance_given: Option<String>,

    /// Society assigned agreement number (14 chars, optional, v2.1+)
    pub society_assigned_agreement_number: Option<String>,
}

impl AgrRecord {
    /// Create a new AGR record with required fields
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, submitter_agreement_number: String, agreement_type: String, agreement_start_date: String, prior_royalty_status: String, post_term_collection_status: String, number_of_works: String) -> Self {
        Self {
            record_type: "AGR".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            submitter_agreement_number,
            international_standard_agreement_code: None,
            agreement_type,
            agreement_start_date,
            agreement_end_date: None,
            retention_end_date: None,
            prior_royalty_status,
            prior_royalty_start_date: None,
            post_term_collection_status,
            post_term_collection_end_date: None,
            date_of_signature_of_agreement: None,
            number_of_works,
            sales_manufacture_clause: None,
            shares_change: None,
            advance_given: None,
            society_assigned_agreement_number: None,
        }
    }

    /// Parse a CWR line into an AGR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 107 {
            return Err(CwrParseError::BadFormat("AGR line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "AGR" {
            return Err(CwrParseError::BadFormat(format!("Expected AGR, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let submitter_agreement_number = line.get(19..33).unwrap().trim().to_string();

        let international_standard_agreement_code = line.get(33..47).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let agreement_type = line.get(47..49).unwrap().trim().to_string();
        let agreement_start_date = line.get(49..57).unwrap().trim().to_string();

        let agreement_end_date = line.get(57..65).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let retention_end_date = line.get(65..73).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let prior_royalty_status = line.get(73..74).unwrap().trim().to_string();

        let prior_royalty_start_date = line.get(74..82).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let post_term_collection_status = line.get(82..83).unwrap().trim().to_string();

        let post_term_collection_end_date = line.get(83..91).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let date_of_signature_of_agreement = line.get(91..99).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let number_of_works = line.get(99..104).unwrap().trim().to_string();

        let sales_manufacture_clause = line.get(104..105).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let shares_change = line.get(105..106).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let advance_given = line.get(106..107).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let society_assigned_agreement_number = if line.len() > 107 { line.get(107..121).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(AgrRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            submitter_agreement_number,
            international_standard_agreement_code,
            agreement_type,
            agreement_start_date,
            agreement_end_date,
            retention_end_date,
            prior_royalty_status,
            prior_royalty_start_date,
            post_term_collection_status,
            post_term_collection_end_date,
            date_of_signature_of_agreement,
            number_of_works,
            sales_manufacture_clause,
            shares_change,
            advance_given,
            society_assigned_agreement_number,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:14}", self.submitter_agreement_number),
            format!("{:14}", self.international_standard_agreement_code.as_deref().unwrap_or("")),
            format!("{:2}", self.agreement_type),
            format!("{:8}", self.agreement_start_date),
            format!("{:8}", self.agreement_end_date.as_deref().unwrap_or("")),
            format!("{:8}", self.retention_end_date.as_deref().unwrap_or("")),
            format!("{:1}", self.prior_royalty_status),
            format!("{:8}", self.prior_royalty_start_date.as_deref().unwrap_or("")),
            format!("{:1}", self.post_term_collection_status),
            format!("{:8}", self.post_term_collection_end_date.as_deref().unwrap_or("")),
            format!("{:8}", self.date_of_signature_of_agreement.as_deref().unwrap_or("")),
            format!("{:5}", self.number_of_works),
            format!("{:1}", self.sales_manufacture_clause.as_deref().unwrap_or("")),
            format!("{:1}", self.shares_change.as_deref().unwrap_or("")),
            format!("{:1}", self.advance_given.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ field
        if let Some(ref society_assigned) = self.society_assigned_agreement_number {
            fields.push(format!("{:14}", society_assigned));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agr_creation() {
        let agr = AgrRecord::new("00000001".to_string(), "00000001".to_string(), "AGREEMENT001".to_string(), "OS".to_string(), "20050101".to_string(), "Y".to_string(), "Y".to_string(), "00001".to_string());

        assert_eq!(agr.record_type, "AGR");
        assert_eq!(agr.agreement_type, "OS");
        assert_eq!(agr.submitter_agreement_number, "AGREEMENT001");
    }

    #[test]
    fn test_agr_round_trip() {
        let original = AgrRecord::new("00000001".to_string(), "00000001".to_string(), "AGREEMENT001".to_string(), "OS".to_string(), "20050101".to_string(), "Y".to_string(), "Y".to_string(), "00001".to_string());

        let line = original.to_cwr_line();
        let parsed = AgrRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
    }
}
