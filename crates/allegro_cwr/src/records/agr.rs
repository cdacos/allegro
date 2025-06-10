use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// AGR - Agreement Transaction Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = agr_custom_validate, test_data = "AGR00000001000000011234567890123               AA20231201                N        N                00001                 ")]
pub struct AgrRecord {
    #[cwr(title = "Always 'AGR'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Submitter agreement number", start = 19, len = 14)]
    pub submitter_agreement_number: String,

    #[cwr(title = "International standard agreement code (optional)", start = 33, len = 14)]
    pub international_standard_agreement_code: Option<String>,

    #[cwr(title = "Agreement type", start = 47, len = 2)]
    pub agreement_type: AgreementType,

    #[cwr(title = "Agreement start date YYYYMMDD", start = 49, len = 8)]
    pub agreement_start_date: Date,

    #[cwr(title = "Agreement end date YYYYMMDD (optional)", start = 57, len = 8)]
    pub agreement_end_date: Option<Date>,

    #[cwr(title = "Retention end date YYYYMMDD (optional)", start = 65, len = 8)]
    pub retention_end_date: Option<Date>,

    #[cwr(title = "Prior royalty status (1 char)", start = 73, len = 1)]
    pub prior_royalty_status: PriorRoyaltyStatus,

    #[cwr(title = "Prior royalty start date YYYYMMDD (conditional)", start = 74, len = 8)]
    pub prior_royalty_start_date: Option<Date>,

    #[cwr(title = "Post-term collection status (1 char)", start = 82, len = 1)]
    pub post_term_collection_status: PostTermCollectionStatus,

    #[cwr(title = "Post-term collection end date YYYYMMDD (conditional)", start = 83, len = 8)]
    pub post_term_collection_end_date: Option<Date>,

    #[cwr(title = "Date of signature of agreement YYYYMMDD (optional)", start = 91, len = 8)]
    pub date_of_signature_of_agreement: Option<Date>,

    #[cwr(title = "Number of works", start = 99, len = 5)]
    pub number_of_works: Number,

    #[cwr(title = "Sales/manufacture clause (1 char, conditional)", start = 104, len = 1)]
    pub sales_manufacture_clause: Option<LookupPlaceholder>,

    #[cwr(title = "Shares change (1 char, optional)", start = 105, len = 1)]
    pub shares_change: Option<Boolean>,

    #[cwr(title = "Advance given (1 char, optional)", start = 106, len = 1)]
    pub advance_given: Option<Boolean>,

    #[cwr(title = "Society assigned agreement number (optional, v2.1+)", start = 107, len = 14, min_version = 2.1)]
    pub society_assigned_agreement_number: Option<String>,
}

// Custom validation function for AGR record
fn agr_custom_validate(record: &mut AgrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Prior Royalty Start Date required if Prior Royalty Status = "D"
    if matches!(record.prior_royalty_status, PriorRoyaltyStatus::Designated)
        && record.prior_royalty_start_date.is_none()
    {
        warnings.push(CwrWarning {
            field_name: "prior_royalty_start_date",
            field_title: "Prior royalty start date YYYYMMDD (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Prior Royalty Start Date is required when Prior Royalty Status is 'D' (Designated)"
                .to_string(),
        });
    }

    // Business rule: Post-term Collection End Date required if Post-term Collection Status = "D"
    if matches!(record.post_term_collection_status, PostTermCollectionStatus::Designated)
        && record.post_term_collection_end_date.is_none()
    {
        warnings.push(CwrWarning {
            field_name: "post_term_collection_end_date",
            field_title: "Post-term collection end date YYYYMMDD (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description:
                "Post-term Collection End Date is required when Post-term Collection Status is 'D' (Designated)"
                    .to_string(),
        });
    }

    // Business rule: Date validations
    if let (start_date, Some(end_date)) =
        (&record.agreement_start_date.0, &record.agreement_end_date.as_ref().map(|d| &d.0))
    {
        if *end_date < start_date {
            warnings.push(CwrWarning {
                field_name: "agreement_end_date",
                field_title: "Agreement end date YYYYMMDD (optional)",
                source_str: std::borrow::Cow::Owned(end_date.format("%Y%m%d").to_string()),
                level: WarningLevel::Critical,
                description: "Agreement End Date must be >= Agreement Start Date".to_string(),
            });
        }
    }

    // Business rule: Retention End Date must be >= Agreement End Date
    if let (Some(end_date), Some(retention_date)) =
        (&record.agreement_end_date.as_ref().map(|d| &d.0), &record.retention_end_date.as_ref().map(|d| &d.0))
    {
        if retention_date < end_date {
            warnings.push(CwrWarning {
                field_name: "retention_end_date",
                field_title: "Retention end date YYYYMMDD (optional)",
                source_str: std::borrow::Cow::Owned(retention_date.format("%Y%m%d").to_string()),
                level: WarningLevel::Critical,
                description: "Retention End Date must be >= Agreement End Date".to_string(),
            });
        }
    }

    // Business rule: Prior Royalty Start Date must be < Agreement Start Date
    if let (Some(prior_date), start_date) =
        (&record.prior_royalty_start_date.as_ref().map(|d| &d.0), &record.agreement_start_date.0)
    {
        if *prior_date >= start_date {
            warnings.push(CwrWarning {
                field_name: "prior_royalty_start_date",
                field_title: "Prior royalty start date YYYYMMDD (conditional)",
                source_str: std::borrow::Cow::Owned(prior_date.format("%Y%m%d").to_string()),
                level: WarningLevel::Critical,
                description: "Prior Royalty Start Date must be < Agreement Start Date".to_string(),
            });
        }
    }

    // TODO: Additional business rules requiring broader context:
    // - Sales/Manufacture Clause mandatory for Agreement Type "OS" or "PS"
    // - AGR must be followed by at least one TER record and at least two IPA records
    // - Assignor and acquirer shares must total 100% for each right type (requires IPA records)
    // - Version-specific validations for v2.1+ fields

    warnings
}
