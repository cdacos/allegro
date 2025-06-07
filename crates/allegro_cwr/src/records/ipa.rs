use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// IPA - Interested Party of Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "IPA0000000100000001AS           123456789JONES                                                                                                           ")]
pub struct IpaRecord {
    #[cwr(title = "Always 'IPA'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Agreement role code", start = 19, len = 2)]
    pub agreement_role_code: String,

    #[cwr(title = "Interested party IPI name number (optional)", start = 21, len = 11)]
    pub interested_party_ipi_name_num: Option<String>,

    #[cwr(title = "IPI base number (optional)", start = 32, len = 13)]
    pub ipi_base_number: Option<String>,

    #[cwr(title = "Interested party number", start = 45, len = 9)]
    pub interested_party_num: String,

    #[cwr(title = "Interested party last name", start = 54, len = 45)]
    pub interested_party_last_name: String,

    #[cwr(title = "Interested party writer first name (optional)", start = 99, len = 30)]
    pub interested_party_writer_first_name: Option<String>,

    #[cwr(title = "PR affiliation society (conditional)", start = 129, len = 3)]
    pub pr_affiliation_society: Option<String>,

    #[cwr(title = "PR share (conditional)", start = 132, len = 5)]
    pub pr_share: Option<String>,

    #[cwr(title = "MR affiliation society (conditional)", start = 137, len = 3)]
    pub mr_affiliation_society: Option<String>,

    #[cwr(title = "MR share (conditional)", start = 140, len = 5)]
    pub mr_share: Option<String>,

    #[cwr(title = "SR affiliation society (conditional)", start = 145, len = 3)]
    pub sr_affiliation_society: Option<String>,

    #[cwr(title = "SR share (conditional)", start = 148, len = 5)]
    pub sr_share: Option<String>,
}
