//! SPU - Publisher Controlled by Submitter Record / OPU - Other Publisher Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// SPU - Publisher Controlled by Submitter Record (also OPU - Other Publisher)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpuRecord {
    /// "SPU" or "OPU"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher sequence number (2 chars)
    pub publisher_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Publisher name (45 chars, conditional)
    pub publisher_name: Option<String>,

    /// Publisher unknown indicator (1 char, conditional)
    pub publisher_unknown_indicator: Option<String>,

    /// Publisher type (2 chars, conditional)
    pub publisher_type: Option<String>,

    /// Tax ID number (9 chars, optional)
    pub tax_id_num: Option<String>,

    /// Publisher IPI name number (11 chars, conditional)
    pub publisher_ipi_name_num: Option<String>,

    /// Submitter agreement number (14 chars, optional)
    pub submitter_agreement_number: Option<String>,

    /// PR affiliation society number (3 chars, conditional)
    pub pr_affiliation_society_num: Option<String>,

    /// PR ownership share (5 chars, conditional)
    pub pr_ownership_share: Option<String>,

    /// MR society (3 chars, conditional)
    pub mr_society: Option<String>,

    /// MR ownership share (5 chars, conditional)
    pub mr_ownership_share: Option<String>,

    /// SR society (3 chars, conditional)
    pub sr_society: Option<String>,

    /// SR ownership share (5 chars, conditional)
    pub sr_ownership_share: Option<String>,

    /// Special agreements indicator (1 char, optional)
    pub special_agreements_indicator: Option<String>,

    /// First recording refusal indicator (1 char, optional)
    pub first_recording_refusal_ind: Option<String>,

    /// Filler (1 char, optional)
    pub filler: Option<String>,

    /// Publisher IPI base number (13 chars, optional)
    pub publisher_ipi_base_number: Option<String>,

    /// International standard agreement code (14 chars, optional)
    pub international_standard_agreement_code: Option<String>,

    /// Society-assigned agreement number (14 chars, optional)
    pub society_assigned_agreement_number: Option<String>,

    /// Agreement type (2 chars, optional, v2.1+)
    pub agreement_type: Option<String>,

    /// USA license indicator (1 char, optional, v2.1+)
    pub usa_license_ind: Option<String>,
}


impl_cwr_parsing! {
    SpuRecord {
        record_type: (0, 3, required, one_of(&["SPU", "OPU"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        publisher_sequence_num: (19, 21, required),
        interested_party_num: (21, 30, optional),
        publisher_name: (30, 75, optional),
        publisher_unknown_indicator: (75, 76, optional),
        publisher_type: (76, 78, optional),
        tax_id_num: (78, 87, optional),
        publisher_ipi_name_num: (87, 98, optional),
        submitter_agreement_number: (98, 112, optional),
        pr_affiliation_society_num: (112, 115, optional),
        pr_ownership_share: (115, 120, optional),
        mr_society: (120, 123, optional),
        mr_ownership_share: (123, 128, optional),
        sr_society: (128, 131, optional),
        sr_ownership_share: (131, 136, optional),
        special_agreements_indicator: (136, 137, optional),
        first_recording_refusal_ind: (137, 138, optional),
        filler: (138, 139, optional),
        publisher_ipi_base_number: (139, 152, optional),
        international_standard_agreement_code: (152, 166, optional),
        society_assigned_agreement_number: (166, 180, optional),
        agreement_type: (180, 182, optional),
        usa_license_ind: (182, 183, optional),
    }
    with_test_data ["SPU0000000100000001011234567890PUBLISHER NAME                             N AS1234567890123456789    BMI  50.00000000000000000000000000000  N N                                                            "]
}

