//! SWR - Writer Controlled by Submitter Record
//!
//! Also handles OWR (Other Writer) records.

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// SWR - Writer Controlled by Submitter Record (also OWR - Other Writer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwrRecord {
    /// "SWR" or "OWR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Writer last name (45 chars, conditional)
    pub writer_last_name: Option<String>,

    /// Writer first name (30 chars, optional)
    pub writer_first_name: Option<String>,

    /// Writer unknown indicator (1 char, conditional)
    pub writer_unknown_indicator: Option<String>,

    /// Writer designation code (2 chars, conditional)
    pub writer_designation_code: Option<String>,

    /// Tax ID number (9 chars, optional)
    pub tax_id_num: Option<String>,

    /// Writer IPI name number (11 chars, optional)
    pub writer_ipi_name_num: Option<String>,

    /// PR affiliation society number (3 chars, optional)
    pub pr_affiliation_society_num: Option<String>,

    /// PR ownership share (5 chars, optional)
    pub pr_ownership_share: Option<String>,

    /// MR society (3 chars, optional)
    pub mr_society: Option<String>,

    /// MR ownership share (5 chars, optional)
    pub mr_ownership_share: Option<String>,

    /// SR society (3 chars, optional)
    pub sr_society: Option<String>,

    /// SR ownership share (5 chars, optional)
    pub sr_ownership_share: Option<String>,

    /// Reversionary indicator (1 char, optional)
    pub reversionary_indicator: Option<String>,

    /// First recording refusal indicator (1 char, optional)
    pub first_recording_refusal_ind: Option<String>,

    /// Work for hire indicator (1 char, optional)
    pub work_for_hire_indicator: Option<String>,

    /// Filler (1 char, optional)
    pub filler: Option<String>,

    /// Writer IPI base number (13 chars, optional)
    pub writer_ipi_base_number: Option<String>,

    /// Personal number (12 chars, optional)
    pub personal_number: Option<String>,

    /// USA license indicator (1 char, optional, v2.1+)
    pub usa_license_ind: Option<String>,
}

impl SwrRecord {
    fn post_process_fields(_record: &mut SwrRecord, _warnings: &mut Vec<String>) {
        // No specific post-processing needed for SWR
    }
}

impl_cwr_parsing! {
    SwrRecord {
        record_type: (0, 3, required, one_of(&["SWR", "OWR"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        interested_party_num: (19, 28, optional),
        writer_last_name: (28, 73, optional),
        writer_first_name: (73, 103, optional),
        writer_unknown_indicator: (103, 104, optional),
        writer_designation_code: (104, 106, optional),
        tax_id_num: (106, 115, optional),
        writer_ipi_name_num: (115, 126, optional),
        pr_affiliation_society_num: (126, 129, optional),
        pr_ownership_share: (129, 134, optional),
        mr_society: (134, 137, optional),
        mr_ownership_share: (137, 142, optional),
        sr_society: (142, 145, optional),
        sr_ownership_share: (145, 150, optional),
        reversionary_indicator: (150, 151, optional),
        first_recording_refusal_ind: (151, 152, optional),
        work_for_hire_indicator: (152, 153, optional),
        filler: (153, 154, optional),
        writer_ipi_base_number: (154, 167, optional),
        personal_number: (167, 179, optional),
        usa_license_ind: (179, 180, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(SwrRecord, ["SWR00000010000000201234567890WRITER LAST NAME                     WRITER FIRST NAME             N WR12345678901234567890   50.000   50.000   50.000N N N 0123456789012012345678901"]);
