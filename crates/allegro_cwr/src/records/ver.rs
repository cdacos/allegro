use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// VER - Original Work Title for Versions Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = ver_custom_validate,
    test_data = "VER0000000100000002PLACEHOLDER ORIGINAL WORK TITLE                       1234567890 EN PLACEHOLDER WRITER 1                      FIRSTNAME 1         PLACEHOLDER SOURCE                                      12345678901123456789012PLACEHOLDER WRITER 2                     FIRSTNAME 2         123456789011234567890123456789012345                                        "
)]
pub struct VerRecord {
    #[cwr(title = "Always 'VER'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Original work title", start = 19, len = 60)]
    pub original_work_title: String,

    #[cwr(title = "ISWC of original work (optional)", start = 79, len = 11)]
    pub iswc_of_original_work: Option<String>,

    #[cwr(title = "Language code (optional)", start = 90, len = 2)]
    pub language_code: Option<String>,

    #[cwr(title = "Writer 1 last name (optional)", start = 92, len = 45)]
    pub writer_1_last_name: Option<String>,

    #[cwr(title = "Writer 1 first name (optional)", start = 137, len = 30)]
    pub writer_1_first_name: Option<String>,

    #[cwr(title = "Source (optional)", start = 167, len = 60)]
    pub source: Option<String>,

    #[cwr(title = "Writer 1 IPI name number (optional)", start = 227, len = 11)]
    pub writer_1_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 1 IPI base number (optional)", start = 238, len = 13)]
    pub writer_1_ipi_base_number: Option<String>,

    #[cwr(title = "Writer 2 last name (optional)", start = 251, len = 45)]
    pub writer_2_last_name: Option<String>,

    #[cwr(title = "Writer 2 first name (optional)", start = 296, len = 30)]
    pub writer_2_first_name: Option<String>,

    #[cwr(title = "Writer 2 IPI name number (optional)", start = 326, len = 11)]
    pub writer_2_ipi_name_num: Option<String>,

    #[cwr(title = "Writer 2 IPI base number (optional)", start = 337, len = 13)]
    pub writer_2_ipi_base_number: Option<String>,

    #[cwr(title = "Submitter work number (optional)", start = 350, len = 14)]
    pub submitter_work_num: Option<String>,
}

// Custom validation function for VER record
fn ver_custom_validate(record: &mut VerRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Original work title cannot be empty
    if record.original_work_title.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "original_work_title", field_title: "Original work title", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Original work title cannot be empty".to_string() });
    }

    // Business rule: ISWC format validation (if provided)
    if let Some(ref iswc) = record.iswc_of_original_work {
        let iswc_trimmed = iswc.trim();
        if !iswc_trimmed.is_empty() && iswc_trimmed.len() != 11 {
            warnings.push(CwrWarning { field_name: "iswc_of_original_work", field_title: "ISWC of original work (optional)", source_str: std::borrow::Cow::Owned(iswc.clone()), level: WarningLevel::Warning, description: "ISWC should be exactly 11 characters (T-NNNNNNNN-C format)".to_string() });
        }
    }

    // Business rule: Writer 1 last name should be provided if any writer 1 info is given
    if (record.writer_1_first_name.is_some() && record.writer_1_first_name.as_ref().map_or(false, |s| !s.trim().is_empty()))
        || (record.writer_1_ipi_name_num.is_some() && record.writer_1_ipi_name_num.as_ref().map_or(false, |s| !s.trim().is_empty()))
        || (record.writer_1_ipi_base_number.is_some() && record.writer_1_ipi_base_number.as_ref().map_or(false, |s| !s.trim().is_empty()))
    {
        if record.writer_1_last_name.is_none() || record.writer_1_last_name.as_ref().map_or(true, |s| s.trim().is_empty()) {
            warnings.push(CwrWarning { field_name: "writer_1_last_name", field_title: "Writer 1 last name (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "Writer 1 last name should be provided when other writer 1 information is given".to_string() });
        }
    }

    // Business rule: Writer 2 last name should be provided if any writer 2 info is given
    if (record.writer_2_first_name.is_some() && record.writer_2_first_name.as_ref().map_or(false, |s| !s.trim().is_empty()))
        || (record.writer_2_ipi_name_num.is_some() && record.writer_2_ipi_name_num.as_ref().map_or(false, |s| !s.trim().is_empty()))
        || (record.writer_2_ipi_base_number.is_some() && record.writer_2_ipi_base_number.as_ref().map_or(false, |s| !s.trim().is_empty()))
    {
        if record.writer_2_last_name.is_none() || record.writer_2_last_name.as_ref().map_or(true, |s| s.trim().is_empty()) {
            warnings.push(CwrWarning { field_name: "writer_2_last_name", field_title: "Writer 2 last name (optional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Warning, description: "Writer 2 last name should be provided when other writer 2 information is given".to_string() });
        }
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)
    // - Language codes must be valid ISO 639-1 codes (requires lookup table)
    // - IPI Name Number must match IPI system entry if provided (requires IPI lookup)
    // - IPI Base Number must match IPI system entry if provided (requires IPI lookup)
    // - Original work should not be the same as the current work (requires cross-record validation)

    warnings
}
