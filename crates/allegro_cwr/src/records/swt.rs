use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SWT - Writer Territory of Control Record (also OWT - Other Writer Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["SWT", "OWT"], validator = swt_custom_validate, test_data = "SWT0000000000000227WOMA     050000000000000I2100N001")]
pub struct SwtRecord {
    #[cwr(title = "'SWT' or 'OWT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number (conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "PR collection share (optional)", start = 28, len = 5)]
    pub pr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "MR collection share (optional)", start = 33, len = 5)]
    pub mr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "SR collection share (optional)", start = 38, len = 5)]
    pub sr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 43, len = 1)]
    pub inclusion_exclusion_indicator: InclusionExclusionIndicator,

    #[cwr(title = "TIS numeric code", start = 44, len = 4)]
    pub tis_numeric_code: TisNumericCode,

    #[cwr(title = "Shares change (1 char, optional)", start = 48, len = 1)]
    pub shares_change: Option<FlagYNU>,

    #[cwr(title = "Sequence number (v2.1+)", start = 49, len = 3, min_version = 2.1)]
    pub sequence_num: Option<String>,
}

// Custom validation function for SWT record
fn swt_custom_validate(record: &mut SwtRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Interested party number should be provided for most cases
    if record.interested_party_num.is_none() || record.interested_party_num.as_ref().is_none_or(|s| s.trim().is_empty()) {
        warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "Interested party number is typically required for writer territory records".to_string() });
    }

    // Business rule: At least one collection share should be provided
    let pr_share = record.pr_collection_share.as_ref().map_or(0, |s| s.0);
    let mr_share = record.mr_collection_share.as_ref().map_or(0, |s| s.0);
    let sr_share = record.sr_collection_share.as_ref().map_or(0, |s| s.0);

    if pr_share == 0 && mr_share == 0 && sr_share == 0 {
        warnings.push(CwrWarning { field_name: "pr_collection_share", field_title: "PR collection share (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "At least one collection share (PR, MR, or SR) should be provided".to_string() });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a SWR/OWR record (requires parsing context)
    // - Interested party number must match preceding SWR/OWR record (requires cross-record validation)
    // - TIS numeric code must match TIS lookup table (requires lookup table data)
    // - Collection shares must not exceed ownership shares in SWR record (requires cross-record validation)
    // - Sequence number format validation for v2.1+ (requires version context)

    warnings
}
