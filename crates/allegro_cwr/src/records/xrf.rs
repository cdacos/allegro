use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// XRF - Work ID Cross Reference Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = xrf_custom_validate, test_data = "XRF0000000100000001ISWT1234567890123WY")]
pub struct XrfRecord {
    #[cwr(title = "Always 'XRF'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Organisation code", start = 19, len = 3)]
    pub organisation_code: SocietyCode,

    #[cwr(title = "Identifier", start = 22, len = 14)]
    pub identifier: String,

    #[cwr(title = "Identifier type (1 char)", start = 36, len = 1)]
    pub identifier_type: IdentifierType,

    #[cwr(title = "Validity (1 char)", start = 37, len = 1)]
    pub validity: Flag,
}

// Custom validation function for XRF record
fn xrf_custom_validate(record: &mut XrfRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Organisation code validation is now handled by the SocietyCode domain type

    // Business rule: Identifier cannot be empty
    if record.identifier.trim().is_empty() {
        warnings.push(CwrWarning {
            field_name: "identifier",
            field_title: "Identifier",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Identifier cannot be empty".to_string(),
        });
    }

    // Identifier type validation is now handled by the IdentifierType domain type

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)
    // - Organisation codes must be valid (requires lookup table)
    // - Identifier format validation based on type (requires type-specific validation)
    // - Cross-reference should point to valid external work (requires external lookup)

    warnings
}
