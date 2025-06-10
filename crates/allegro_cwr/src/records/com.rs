use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// COM - Composite Component Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = com_custom_validate,
    test_data = "COM0000000100000002PLACEHOLDER TITLE                                    12345678901234567890PLACEHOLDER WRITER                      FIRSTNAME           12345678901PLACEHOLDER WRITER 2                     FIRSTNAME 2         123456789011234567890123456789012345                                                                                        "
)]
pub struct ComRecord {
    #[cwr(title = "Always 'COM'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Title", start = 19, len = 60)]
    pub title: String,

    #[cwr(title = "ISWC of component (optional)", start = 79, len = 11)]
    pub iswc_of_component: Option<String>,

    #[cwr(title = "Submitter work number (optional)", start = 90, len = 14)]
    pub submitter_work_num: Option<String>,

    #[cwr(title = "Duration HHMMSS (optional)", start = 104, len = 6)]
    pub duration: Option<Time>,

    #[cwr(title = "Writer 1 last name", start = 110, len = 45)]
    pub writer_1_last_name: String,

    #[cwr(title = "Writer 1 first name (optional)", start = 155, len = 30)]
    pub writer_1_first_name: Option<String>,

    #[cwr(title = "Writer 1 IPI name number (optional)", start = 185, len = 11)]
    pub writer_1_ipi_name_num: Option<IpiNameNumber>,

    #[cwr(title = "Writer 2 last name (optional)", start = 196, len = 45)]
    pub writer_2_last_name: Option<String>,

    #[cwr(title = "Writer 2 first name (optional)", start = 241, len = 30)]
    pub writer_2_first_name: Option<String>,

    #[cwr(title = "Writer 2 IPI name number (optional)", start = 271, len = 11)]
    pub writer_2_ipi_name_num: Option<IpiNameNumber>,

    #[cwr(title = "Writer 1 IPI base number (optional)", start = 282, len = 13)]
    pub writer_1_ipi_base_number: Option<IpiBaseNumber>,

    #[cwr(title = "Writer 2 IPI base number (optional)", start = 295, len = 13)]
    pub writer_2_ipi_base_number: Option<IpiBaseNumber>,
}

// Custom validation function for COM record
fn com_custom_validate(record: &mut ComRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Title cannot be empty
    if record.title.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "title", field_title: "Title", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Component title cannot be empty".to_string() });
    }

    // Business rule: Writer 1 last name cannot be empty (required field)
    if record.writer_1_last_name.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "writer_1_last_name", field_title: "Writer 1 last name", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Writer 1 last name cannot be empty".to_string() });
    }

    // Business rule: ISWC format validation (if provided)
    if let Some(ref iswc) = record.iswc_of_component {
        let iswc_trimmed = iswc.trim();
        if !iswc_trimmed.is_empty() && iswc_trimmed.len() != 11 {
            warnings.push(CwrWarning { field_name: "iswc_of_component", field_title: "ISWC of component (optional)", source_str: std::borrow::Cow::Owned(iswc.clone()), level: WarningLevel::Warning, description: "ISWC should be exactly 11 characters (T-NNNNNNNN-C format)".to_string() });
        }
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR record with composite type (requires parsing context)
    // - Component count must match the composite component count in NWR record (requires cross-record validation)
    // - IPI Name Number must match IPI system entry if provided (requires IPI lookup)
    // - IPI Base Number must match IPI system entry if provided (requires IPI lookup)

    warnings
}
