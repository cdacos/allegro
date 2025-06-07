use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// IPA - Interested Party of Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ipa_custom_validate, test_data = "IPA0000000100000001AS                        123456789JONES                                                                      BMI01000   00000   00000")]
pub struct IpaRecord {
    #[cwr(title = "Always 'IPA'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Agreement role code", start = 19, len = 2)]
    pub agreement_role_code: AgreementRoleCode,

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
    pub pr_share: Option<OwnershipShare>,

    #[cwr(title = "MR affiliation society (conditional)", start = 137, len = 3)]
    pub mr_affiliation_society: Option<String>,

    #[cwr(title = "MR share (conditional)", start = 140, len = 5)]
    pub mr_share: Option<OwnershipShare>,

    #[cwr(title = "SR affiliation society (conditional)", start = 145, len = 3)]
    pub sr_affiliation_society: Option<String>,

    #[cwr(title = "SR share (conditional)", start = 148, len = 5)]
    pub sr_share: Option<OwnershipShare>,
}

// Custom validation function for IPA record
fn ipa_custom_validate(record: &mut IpaRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: At least one of PR, MR, or SR share must be > 0
    let pr_share = record.pr_share.as_ref().map_or(0, |s| s.0);
    let mr_share = record.mr_share.as_ref().map_or(0, |s| s.0);
    let sr_share = record.sr_share.as_ref().map_or(0, |s| s.0);

    if pr_share == 0 && mr_share == 0 && sr_share == 0 {
        warnings.push(CwrWarning { field_name: "pr_share", field_title: "PR share (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "At least one of PR, MR, or SR share must be > 0".to_string() });
    }

    // Business rule: If share > 0, corresponding society must be provided
    if pr_share > 0 && (record.pr_affiliation_society.is_none() || record.pr_affiliation_society.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "pr_affiliation_society", field_title: "PR affiliation society (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "PR affiliation society is required when PR share > 0".to_string() });
    }

    if mr_share > 0 && (record.mr_affiliation_society.is_none() || record.mr_affiliation_society.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "mr_affiliation_society", field_title: "MR affiliation society (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "MR affiliation society is required when MR share > 0".to_string() });
    }

    if sr_share > 0 && (record.sr_affiliation_society.is_none() || record.sr_affiliation_society.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "sr_affiliation_society", field_title: "SR affiliation society (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "SR affiliation society is required when SR share > 0".to_string() });
    }

    // Business rule: Writer first name only allowed for OS/OG agreements with assignor role
    if matches!(record.agreement_role_code, AgreementRoleCode::Acquirer) && record.interested_party_writer_first_name.is_some() && !record.interested_party_writer_first_name.as_ref().is_none_or(|s| s.trim().is_empty()) {
        warnings.push(CwrWarning {
            field_name: "interested_party_writer_first_name",
            field_title: "Interested party writer first name (optional)",
            source_str: std::borrow::Cow::Owned(record.interested_party_writer_first_name.as_ref().unwrap().clone()),
            level: WarningLevel::Warning,
            description: "Writer first name typically only used for OS/OG agreements with assignor role".to_string(),
        });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a TER or IPA record (requires parsing context)
    // - Each AGR must have one assignor and at least one acquirer IPA record (requires AGR context)
    // - PR/MR/SR shares across all IPA records in AGR must sum to 100% or 0% (requires cross-record validation)
    // - Interested Party # cannot duplicate existing party numbers (requires context)
    // - IPI Name # must match IPI system entry if provided (requires IPI lookup)
    // - Society codes must exist in Society Code Table (requires lookup table)
    // - At least one of PR/MR affiliation society must be entered (business rule refinement)

    warnings
}
