use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// PER - Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = per_custom_validate, test_data = "PER0000050400000429DEVVON TERRELL                                                                                     ")]
pub struct PerRecord {
    #[cwr(title = "Always 'PER'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Performing artist last name", start = 19, len = 45)]
    pub performing_artist_last_name: String,

    #[cwr(title = "Performing artist first name (optional)", start = 64, len = 30)]
    pub performing_artist_first_name: Option<String>,

    #[cwr(title = "Performing artist IPI name number (optional)", start = 94, len = 11)]
    pub performing_artist_ipi_name_num: Option<IpiNameNumber>,

    #[cwr(title = "Performing artist IPI base number (optional)", start = 105, len = 13)]
    pub performing_artist_ipi_base_number: Option<IpiBaseNumber>,
}

// Custom validation function for PER record
fn per_custom_validate(record: &mut PerRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Performing artist last name cannot be empty
    if record.performing_artist_last_name.trim().is_empty() {
        warnings.push(CwrWarning {
            field_name: "performing_artist_last_name",
            field_title: "Performing artist last name",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Performing artist last name cannot be empty".to_string(),
        });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)
    // - IPI Name Number must match IPI system entry if provided (requires IPI lookup)
    // - IPI Base Number must match IPI system entry if provided (requires IPI lookup)
    // - Performing artist names should not duplicate writer names in same work (requires cross-record validation)

    warnings
}
