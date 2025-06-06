//! IPA - Interested Party of Agreement Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
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


impl_cwr_parsing! {
    IpaRecord {
        record_type: (0, 3, required, one_of(&["IPA"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        agreement_role_code: (19, 21, required),
        interested_party_ipi_name_num: (21, 32, optional),
        ipi_base_number: (32, 45, optional),
        interested_party_num: (45, 54, required),
        interested_party_last_name: (54, 99, required),
        interested_party_writer_first_name: (99, 129, optional),
        pr_affiliation_society: (129, 132, optional),
        pr_share: (132, 137, optional),
        mr_affiliation_society: (137, 140, optional),
        mr_share: (140, 145, optional),
        sr_affiliation_society: (145, 148, optional),
        sr_share: (148, 153, optional),
    }
    with_test_data ["IPA0000000100000001AS           123456789JONES                                                                                                           "]
}

