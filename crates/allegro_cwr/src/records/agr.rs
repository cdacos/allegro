//! AGR - Agreement Transaction Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// AGR - Agreement Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "AGR00000001000000011234567890123               AA20231201                Y        Y                00001                 ")]
pub struct AgrRecord {
    #[cwr(title = "Always 'AGR'", start = 0, len = 3)]
    pub record_type: &'static str,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Submitter agreement number", start = 19, len = 14)]
    pub submitter_agreement_number: String,

    #[cwr(title = "International standard agreement code (optional)", start = 33, len = 14)]
    pub international_standard_agreement_code: Option<String>,

    #[cwr(title = "Agreement type", start = 47, len = 2)]
    pub agreement_type: String,

    #[cwr(title = "Agreement start date YYYYMMDD", start = 49, len = 8)]
    pub agreement_start_date: Date,

    #[cwr(title = "Agreement end date YYYYMMDD (optional)", start = 57, len = 8)]
    pub agreement_end_date: Option<Date>,

    #[cwr(title = "Retention end date YYYYMMDD (optional)", start = 65, len = 8)]
    pub retention_end_date: Option<Date>,

    #[cwr(title = "Prior royalty status (1 char)", start = 73, len = 1)]
    pub prior_royalty_status: YesNo,

    #[cwr(title = "Prior royalty start date YYYYMMDD (conditional)", start = 74, len = 8)]
    pub prior_royalty_start_date: Option<Date>,

    #[cwr(title = "Post-term collection status (1 char)", start = 82, len = 1)]
    pub post_term_collection_status: YesNo,

    #[cwr(title = "Post-term collection end date YYYYMMDD (conditional)", start = 83, len = 8)]
    pub post_term_collection_end_date: Option<Date>,

    #[cwr(title = "Date of signature of agreement YYYYMMDD (optional)", start = 91, len = 8)]
    pub date_of_signature_of_agreement: Option<Date>,

    #[cwr(title = "Number of works", start = 99, len = 5)]
    pub number_of_works: WorksCount,

    #[cwr(title = "Sales/manufacture clause (1 char, conditional)", start = 104, len = 1)]
    pub sales_manufacture_clause: Option<String>,

    #[cwr(title = "Shares change (1 char, optional)", start = 105, len = 1)]
    pub shares_change: Option<String>,

    #[cwr(title = "Advance given (1 char, optional)", start = 106, len = 1)]
    pub advance_given: Option<String>,

    #[cwr(title = "Society assigned agreement number (optional, v2.1+)", start = 107, len = 14)]
    pub society_assigned_agreement_number: Option<String>,
}

