use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SWR - Writer Controlled by Submitter Record (also OWR - Other Writer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["SWR", "OWR"], validator = swr_custom_validate, test_data = "SWR0000000000000226WOMA     WOMACK                                       BOBBY                          CA00000000000033188001021050000990000009900000 N                           B")]
pub struct SwrRecord {
    #[cwr(title = "'SWR' or 'OWR'", start = 0, len = 3)]
    pub record_type: String,

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
    pub pr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "MR society (optional)", start = 134, len = 3)]
    pub mr_society: Option<String>,

    #[cwr(title = "MR ownership share (optional)", start = 137, len = 5)]
    pub mr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "SR society (optional)", start = 142, len = 3)]
    pub sr_society: Option<String>,

    #[cwr(title = "SR ownership share (optional)", start = 145, len = 5)]
    pub sr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "Reversionary indicator (1 char, optional)", start = 150, len = 1)]
    pub reversionary_indicator: Option<FlagYNU>,

    #[cwr(title = "First recording refusal indicator (1 char, optional)", start = 151, len = 1)]
    pub first_recording_refusal_ind: Option<FlagYNU>,

    #[cwr(title = "Work for hire indicator (1 char, optional)", start = 152, len = 1)]
    pub work_for_hire_indicator: Option<FlagYNU>,

    #[cwr(title = "Filler (1 char, optional)", start = 153, len = 1)]
    pub filler: Option<String>,

    #[cwr(title = "Writer IPI base number (optional)", start = 154, len = 13)]
    pub writer_ipi_base_number: Option<String>,

    #[cwr(title = "Personal number (optional)", start = 167, len = 12)]
    pub personal_number: Option<String>,

    #[cwr(title = "USA license indicator (1 char, optional, v2.1+)", start = 179, len = 1)]
    pub usa_license_ind: Option<String>,
}

// Custom validation function for SWR record
fn swr_custom_validate(record: &mut SwrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Writer identification - either Interested Party Number or Writer Last Name required
    if (record.interested_party_num.is_none() || record.interested_party_num.as_ref().is_none_or(|s| s.trim().is_empty())) && (record.writer_last_name.is_none() || record.writer_last_name.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Either Interested Party Number or Writer Last Name must be provided".to_string() });
    }

    // Business rule: At least one ownership share must be provided if writer is being credited
    let pr_share = record.pr_ownership_share.as_ref().map_or(0, |s| s.0);
    let mr_share = record.mr_ownership_share.as_ref().map_or(0, |s| s.0);
    let sr_share = record.sr_ownership_share.as_ref().map_or(0, |s| s.0);

    if pr_share == 0 && mr_share == 0 && sr_share == 0 {
        warnings.push(CwrWarning { field_name: "pr_ownership_share", field_title: "PR ownership share (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "At least one ownership share (PR, MR, or SR) should be provided".to_string() });
    }

    // Business rule: If ownership share > 0, corresponding society should be provided
    if pr_share > 0 && (record.pr_affiliation_society_num.is_none() || record.pr_affiliation_society_num.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "pr_affiliation_society_num", field_title: "PR affiliation society number (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "PR affiliation society number should be provided when PR ownership share > 0".to_string() });
    }

    if mr_share > 0 && (record.mr_society.is_none() || record.mr_society.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "mr_society", field_title: "MR society (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "MR society should be provided when MR ownership share > 0".to_string() });
    }

    if sr_share > 0 && (record.sr_society.is_none() || record.sr_society.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "sr_society", field_title: "SR society (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "SR society should be provided when SR ownership share > 0".to_string() });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)
    // - Writer designation codes must be valid (requires lookup table)
    // - IPI Name Number must match IPI system entry if provided (requires IPI lookup)
    // - Society codes must exist in Society Code Table (requires lookup table)
    // - First name only allowed for certain writer types (business rule refinement)
    // - Tax ID format validation (country-specific rules)
    // - Personal Number format validation (society-specific rules)

    warnings
}
