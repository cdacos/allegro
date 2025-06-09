use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// TER - Territory in Agreement Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = ter_custom_validate, test_data = "TER0000000100000001I2840")]
pub struct TerRecord {
    #[cwr(title = "Always 'TER'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Inclusion/Exclusion indicator (1 char)", start = 19, len = 1)]
    pub inclusion_exclusion_indicator: InclusionExclusionIndicator,

    #[cwr(title = "TIS Numeric Code", start = 20, len = 4)]
    pub tis_numeric_code: TisNumericCode,
}

// Custom validation function for TER record
fn ter_custom_validate(record: &mut TerRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // TODO: Business rules requiring broader context:
    // - Must follow an AGR or TER record (requires parsing context)
    // - TIS Numeric Code must match an entry in the TIS lookup table (requires lookup table data)

    // Basic validation: TIS code should be reasonable
    if record.tis_numeric_code.0 > 9999 {
        warnings.push(CwrWarning { field_name: "tis_numeric_code", field_title: "TIS Numeric Code", source_str: std::borrow::Cow::Owned(record.tis_numeric_code.as_str()), level: WarningLevel::Warning, description: "TIS Numeric Code seems unusually high, please verify".to_string() });
    }

    warnings
}
