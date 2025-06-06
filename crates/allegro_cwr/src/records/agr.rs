//! AGR - Agreement Transaction Record

use crate::validators::{date_yyyymmdd, one_of, works_count, yes_no};
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
    with_test_data ["AGR00000001000000011234567890123               AA20231201                Y        Y                00001                 "]
    with_post_process |record: &mut AgrRecord, warnings: &mut Vec<String>| {
        // Check that end date is after start date
        if let Some(end_date) = &record.agreement_end_date {
            if end_date <= &record.agreement_start_date {
                warnings.push("Agreement end date must be after start date".to_string());
            }
        }
        
        // Check that retention date is after agreement start date
        if let Some(retention_date) = &record.retention_end_date {
            if retention_date <= &record.agreement_start_date {
                warnings.push("Retention end date must be after agreement start date".to_string());
            }
        }
        
        // Check prior royalty logic
        if record.prior_royalty_status == "Y" && record.prior_royalty_start_date.is_none() {
            warnings.push("Prior royalty start date required when prior royalty status is Y".to_string());
        }
    }
}

