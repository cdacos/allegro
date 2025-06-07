use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// SPT - Publisher Territory of Control Record (also OPT - Other Publisher Territory)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["SPT", "OPT"], validator = spt_custom_validate, test_data = "SPT0000000000000002ABKC           025000750000000I0840N001")]
pub struct SptRecord {
    #[cwr(title = "'SPT' or 'OPT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number", start = 19, len = 9)]
    pub interested_party_num: String,

    #[cwr(title = "Constant - spaces", start = 28, len = 6)]
    pub constant: String,

    #[cwr(title = "PR collection share (conditional)", start = 34, len = 5)]
    pub pr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "MR collection share (conditional)", start = 39, len = 5)]
    pub mr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "SR collection share (conditional)", start = 44, len = 5)]
    pub sr_collection_share: Option<OwnershipShare>,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 49, len = 1)]
    pub inclusion_exclusion_indicator: InclusionExclusionIndicator,

    #[cwr(title = "TIS numeric code", start = 50, len = 4)]
    pub tis_numeric_code: TisNumericCode,

    #[cwr(title = "Shares change (1 char, optional)", start = 54, len = 1)]
    pub shares_change: Option<FlagYNU>,

    #[cwr(title = "Sequence number (v2.1+)", start = 55, len = 3)]
    pub sequence_num: Option<String>,
}

// Custom validation function for SPT record
fn spt_custom_validate(record: &mut SptRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Interested party number cannot be empty
    if record.interested_party_num.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "interested_party_num", field_title: "Interested party number", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Interested party number cannot be empty".to_string() });
    }

    // Business rule: At least one collection share must be provided
    let pr_share = record.pr_collection_share.as_ref().map_or(0, |s| s.0);
    let mr_share = record.mr_collection_share.as_ref().map_or(0, |s| s.0);
    let sr_share = record.sr_collection_share.as_ref().map_or(0, |s| s.0);

    if pr_share == 0 && mr_share == 0 && sr_share == 0 {
        warnings.push(CwrWarning { field_name: "pr_collection_share", field_title: "PR collection share (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "At least one collection share (PR, MR, or SR) should be provided".to_string() });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a SPU/OPU record (requires parsing context)
    // - Interested party number must match preceding SPU/OPU record (requires cross-record validation)
    // - TIS numeric code must match TIS lookup table (requires lookup table data)
    // - Collection shares must not exceed ownership shares in SPU record (requires cross-record validation)
    // - Sequence number format validation for v2.1+ (requires version context)

    warnings
}
