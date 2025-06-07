use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SPU - Publisher Controlled by Submitter Record (also OPU - Other Publisher)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["SPU", "OPU"], test_data = "SPU0000000100000001011234567890PUBLISHER NAME                             N AS1234567890123456789    BMI  50.00000000000000000000000000000  N N                                                            ")]
pub struct SpuRecord {
    #[cwr(title = "'SPU' or 'OPU'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Publisher sequence number", start = 19, len = 2)]
    pub publisher_sequence_num: String,

    #[cwr(title = "Interested party number (conditional)", start = 21, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Publisher name (conditional)", start = 30, len = 45)]
    pub publisher_name: Option<String>,

    #[cwr(title = "Publisher unknown indicator (1 char, conditional)", start = 75, len = 1)]
    pub publisher_unknown_indicator: Option<String>,

    #[cwr(title = "Publisher type (conditional)", start = 76, len = 2)]
    pub publisher_type: Option<String>,

    #[cwr(title = "Tax ID number (optional)", start = 78, len = 9)]
    pub tax_id_num: Option<String>,

    #[cwr(title = "Publisher IPI name number (conditional)", start = 87, len = 11)]
    pub publisher_ipi_name_num: Option<String>,

    #[cwr(title = "Submitter agreement number (optional)", start = 98, len = 14)]
    pub submitter_agreement_number: Option<String>,

    #[cwr(title = "PR affiliation society number (conditional)", start = 112, len = 3)]
    pub pr_affiliation_society_num: Option<String>,

    #[cwr(title = "PR ownership share (conditional)", start = 115, len = 5)]
    pub pr_ownership_share: Option<String>,

    #[cwr(title = "MR society (conditional)", start = 120, len = 3)]
    pub mr_society: Option<String>,

    #[cwr(title = "MR ownership share (conditional)", start = 123, len = 5)]
    pub mr_ownership_share: Option<String>,

    #[cwr(title = "SR society (conditional)", start = 128, len = 3)]
    pub sr_society: Option<String>,

    #[cwr(title = "SR ownership share (conditional)", start = 131, len = 5)]
    pub sr_ownership_share: Option<String>,

    #[cwr(title = "Special agreements indicator (1 char, optional)", start = 136, len = 1)]
    pub special_agreements_indicator: Option<String>,

    #[cwr(title = "First recording refusal indicator (1 char, optional)", start = 137, len = 1)]
    pub first_recording_refusal_ind: Option<String>,

    #[cwr(title = "Filler (1 char, optional)", start = 138, len = 1)]
    pub filler: Option<String>,

    #[cwr(title = "Publisher IPI base number (optional)", start = 139, len = 13)]
    pub publisher_ipi_base_number: Option<String>,

    #[cwr(title = "International standard agreement code (optional)", start = 152, len = 14)]
    pub international_standard_agreement_code: Option<String>,

    #[cwr(title = "Society-assigned agreement number (optional)", start = 166, len = 14)]
    pub society_assigned_agreement_number: Option<String>,

    #[cwr(title = "Agreement type (optional, v2.1+)", start = 180, len = 2)]
    pub agreement_type: Option<String>,

    #[cwr(title = "USA license indicator (1 char, optional, v2.1+)", start = 182, len = 1)]
    pub usa_license_ind: Option<String>,
}
