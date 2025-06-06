//! SWR - Writer Controlled by Submitter Record (also OWR - Other Writer)

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SWR - Writer Controlled by Submitter Record (also OWR - Other Writer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "SWR0000000000000226WOMA     WOMACK                                       BOBBY                          CA00000000000033188001021050000990000009900000 N                           B")]
pub struct SwrRecord {
    #[cwr(title = "'SWR' or 'OWR'", start = 0, len = 3)]
    pub record_type: &'static str,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number (conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Writer last name (conditional)", start = 28, len = 45)]
    pub writer_last_name: Option<String>,

    #[cwr(title = "Writer first name (optional)", start = 73, len = 30)]
    pub writer_first_name: Option<String>,

    #[cwr(title = "Writer unknown indicator (1 char, conditional)", start = 103, len = 1)]
    pub writer_unknown_indicator: Option<String>,

    #[cwr(title = "Writer designation code (conditional)", start = 104, len = 2)]
    pub writer_designation_code: Option<String>,

    #[cwr(title = "Tax ID number (optional)", start = 106, len = 9)]
    pub tax_id_num: Option<String>,

    #[cwr(title = "Writer IPI name number (optional)", start = 115, len = 11)]
    pub writer_ipi_name_num: Option<String>,

    #[cwr(title = "PR affiliation society number (optional)", start = 126, len = 3)]
    pub pr_affiliation_society_num: Option<String>,

    #[cwr(title = "PR ownership share (optional)", start = 129, len = 5)]
    pub pr_ownership_share: Option<String>,

    #[cwr(title = "MR society (optional)", start = 134, len = 3)]
    pub mr_society: Option<String>,

    #[cwr(title = "MR ownership share (optional)", start = 137, len = 5)]
    pub mr_ownership_share: Option<String>,

    #[cwr(title = "SR society (optional)", start = 142, len = 3)]
    pub sr_society: Option<String>,

    #[cwr(title = "SR ownership share (optional)", start = 145, len = 5)]
    pub sr_ownership_share: Option<String>,

    #[cwr(title = "Reversionary indicator (1 char, optional)", start = 150, len = 1)]
    pub reversionary_indicator: Option<String>,

    #[cwr(title = "First recording refusal indicator (1 char, optional)", start = 151, len = 1)]
    pub first_recording_refusal_ind: Option<String>,

    #[cwr(title = "Work for hire indicator (1 char, optional)", start = 152, len = 1)]
    pub work_for_hire_indicator: Option<String>,

    #[cwr(title = "Filler (1 char, optional)", start = 153, len = 1)]
    pub filler: Option<String>,

    #[cwr(title = "Writer IPI base number (optional)", start = 154, len = 13)]
    pub writer_ipi_base_number: Option<String>,

    #[cwr(title = "Personal number (optional)", start = 167, len = 12)]
    pub personal_number: Option<String>,

    #[cwr(title = "USA license indicator (1 char, optional, v2.1+)", start = 179, len = 1)]
    pub usa_license_ind: Option<String>,
}
