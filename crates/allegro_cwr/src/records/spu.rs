use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SPU - Publisher Controlled by Submitter Record (also OPU - Other Publisher)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["SPU", "OPU"], validator = spu_custom_validate, test_data = "SPU0000000100000001011234567890PUBLISHER NAME                             N AS1234567890123456789    BMI  50.00000000000000000000000000000  N N                                                            ")]
pub struct SpuRecord {
    #[cwr(title = "'SPU' or 'OPU'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    // This enables a rights organization to link sub-publishers and administrators to the
    // proper original publisher. Each original publisher will start a new chain. An income participant may start a chain,
    // or be included in a chain begun by the original publisher which has allocated rights to the income participant.
    #[cwr(title = "Publisher sequence number", start = 19, len = 2)]
    pub publisher_sequence_num: PublisherSequenceNumber,

    #[cwr(title = "Interested party number (conditional)", start = 21, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Publisher name (conditional)", start = 30, len = 45)]
    pub publisher_name: Option<String>,

    #[cwr(title = "Publisher unknown indicator (1 char, conditional)", start = 75, len = 1)]
    pub publisher_unknown_indicator: Option<Flag>,

    #[cwr(title = "Publisher type (conditional)", start = 76, len = 2)]
    pub publisher_type: Option<PublisherType>,

    #[cwr(title = "Tax ID number (optional)", start = 78, len = 9)]
    pub tax_id_num: Option<String>,

    #[cwr(title = "Publisher IPI name number (conditional)", start = 87, len = 11)]
    pub publisher_ipi_name_num: Option<IpiNameNumber>,

    #[cwr(title = "Submitter agreement number (optional)", start = 98, len = 14)]
    pub submitter_agreement_number: Option<String>,

    #[cwr(title = "PR affiliation society number (conditional)", start = 112, len = 3)]
    pub pr_affiliation_society_num: Option<SocietyCode>,

    #[cwr(title = "PR ownership share (conditional)", start = 115, len = 5)]
    pub pr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "MR society (conditional)", start = 120, len = 3)]
    pub mr_society: Option<SocietyCode>,

    #[cwr(title = "MR ownership share (conditional)", start = 123, len = 5)]
    pub mr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "SR society (conditional)", start = 128, len = 3)]
    pub sr_society: Option<SocietyCode>,

    #[cwr(title = "SR ownership share (conditional)", start = 131, len = 5)]
    pub sr_ownership_share: Option<OwnershipShare>,

    #[cwr(title = "Special agreements indicator (1 char, optional)", start = 136, len = 1)]
    pub special_agreements_indicator: Option<Flag>,

    #[cwr(title = "First recording refusal indicator (1 char, optional)", start = 137, len = 1)]
    pub first_recording_refusal_ind: Option<Flag>,

    #[cwr(title = "Filler (1 char, optional)", start = 138, len = 1)]
    pub filler: Option<String>,

    #[cwr(title = "Publisher IPI base number (optional)", start = 139, len = 13)]
    pub publisher_ipi_base_number: Option<IpiBaseNumber>,

    #[cwr(title = "International standard agreement code (optional)", start = 152, len = 14)]
    pub international_standard_agreement_code: Option<String>,

    #[cwr(title = "Society-assigned agreement number (optional)", start = 166, len = 14)]
    pub society_assigned_agreement_number: Option<String>,

    #[cwr(title = "Agreement type (optional, v2.1+)", start = 180, len = 2, min_version = 2.1)]
    pub agreement_type: Option<AgreementType>,

    #[cwr(title = "USA license indicator (1 char, optional, v2.1+)", start = 182, len = 1, min_version = 2.1)]
    pub usa_license_ind: Option<UsaLicenseIndicator>,
}

// Custom validation function for SPU record
fn spu_custom_validate(record: &mut SpuRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // TODO: Add record length validation
    // For v2.0: should end at position 179 (society_assigned_agreement_number) = 180 chars
    // For v2.1+: should end at position 182 (usa_license_ind) = 183 chars
    // If input line is longer, warn about extra characters

    // SPU-specific validations (vs OPU)
    if record.record_type == "SPU" {
        // For SPU records: Interested Party #, Publisher Name, and Publisher Type are required
        if record.interested_party_num.is_none() || record.interested_party_num.as_ref().is_none_or(|s| s.trim().is_empty()) {
            warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Interested Party Number is required for SPU records".to_string() });
        }

        if record.publisher_name.is_none() || record.publisher_name.as_ref().is_none_or(|s| s.trim().is_empty()) {
            warnings.push(CwrWarning { field_name: "publisher_name", field_title: "Publisher name (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Publisher Name is required for SPU records".to_string() });
        }

        // Publisher Unknown Indicator must be None or Unknown for SPU records
        if let Some(ref indicator) = record.publisher_unknown_indicator {
            if !matches!(indicator, Flag::Unknown) {
                warnings.push(CwrWarning {
                    field_name: "publisher_unknown_indicator",
                    field_title: "Publisher unknown indicator (1 char, conditional)",
                    source_str: std::borrow::Cow::Owned(indicator.as_str().to_string()),
                    level: WarningLevel::Critical,
                    description: "Publisher Unknown Indicator must be blank/unknown for SPU records".to_string(),
                });
            }
        }
    }

    // Ownership share validation for PR (Performance Rights) - max 50%
    if let Some(ref pr_share) = record.pr_ownership_share {
        if pr_share.0 > 5000 {
            // 50.00% = 5000
            warnings.push(CwrWarning { field_name: "pr_ownership_share", field_title: "PR ownership share (conditional)", source_str: std::borrow::Cow::Owned(pr_share.as_str()), level: WarningLevel::Critical, description: format!("PR ownership share {}% exceeds maximum 50.00%", pr_share.as_percentage()) });
        }
    }

    // TODO: Version-specific validations
    // - Version 2.1+: Agreement Type and USA License Ind validation
    // - Version 2.2+: Enhanced IPI Name # validation for collecting publishers
    // These would be implemented when we have access to CWR version context

    warnings
}
