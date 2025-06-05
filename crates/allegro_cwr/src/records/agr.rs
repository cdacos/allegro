//! AGR - Agreement Transaction Record

use crate::validators::{date_yyyymmdd, one_of, yes_no, works_count};
use crate::impl_cwr_parsing;
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
}

// Generate the from_cwr_line_v2 method using the macro
impl_cwr_parsing! {
    AgrRecord {
        record_type: (0, 3, required, one_of(&["AGR"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        submitter_agreement_number: (19, 33, required),
        international_standard_agreement_code: (33, 47, optional),
        agreement_type: (47, 49, required),
        agreement_start_date: (49, 57, required, date_yyyymmdd),
        agreement_end_date: (57, 65, optional, date_yyyymmdd),
        retention_end_date: (65, 73, optional, date_yyyymmdd),
        prior_royalty_status: (73, 74, required, yes_no),
        prior_royalty_start_date: (74, 82, optional, date_yyyymmdd),
        post_term_collection_status: (82, 83, required, yes_no),
        post_term_collection_end_date: (83, 91, optional, date_yyyymmdd),
        date_of_signature_of_agreement: (91, 99, optional, date_yyyymmdd),
        number_of_works: (99, 104, required, works_count),
        sales_manufacture_clause: (104, 105, optional),
        shares_change: (105, 106, optional),
        advance_given: (106, 107, optional),
        society_assigned_agreement_number: (107, 121, optional),
    }
}

impl AgrRecord {
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
