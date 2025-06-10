use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPR - Non-Roman Alphabet Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = npr_custom_validate,
    test_data = "NPR0000000100000002PLACEHOLDER PERFORMING ARTIST                                                                                                                                   PLACEHOLDER FIRST NAME                                                                                                                                          12345678901123456789012ENENABC "
)]
pub struct NprRecord {
    #[cwr(title = "Always 'NPR'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Performing artist name (conditional)", start = 19, len = 160)]
    pub performing_artist_name: Option<String>,

    #[cwr(title = "Performing artist first name (optional)", start = 179, len = 160)]
    pub performing_artist_first_name: Option<String>,

    #[cwr(title = "Performing artist IPI name number (optional)", start = 339, len = 11)]
    pub performing_artist_ipi_name_num: Option<IpiNameNumber>,

    #[cwr(title = "Performing artist IPI base number (optional)", start = 350, len = 13)]
    pub performing_artist_ipi_base_number: Option<IpiBaseNumber>,

    #[cwr(title = "Language code (optional)", start = 363, len = 2)]
    pub language_code: Option<LanguageCode>,

    #[cwr(title = "Performance language (conditional, v2.1+)", start = 365, len = 2, min_version = 2.1)]
    pub performance_language: Option<LanguageCode>,

    #[cwr(title = "Performance dialect (conditional, v2.1+)", start = 367, len = 3, min_version = 2.1)]
    pub performance_dialect: Option<LanguageDialect>,
}

// Custom validation function for NPR record
fn npr_custom_validate(record: &mut NprRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Validate record type
    if record.record_type != "NPR" {
        warnings.push(CwrWarning {
            field_name: "record_type",
            field_title: "Always 'NPR'",
            source_str: std::borrow::Cow::Owned(record.record_type.clone()),
            level: WarningLevel::Critical,
            description: "Record type must be 'NPR'".to_string(),
        });
    }

    // Validate transaction sequence number is numeric
    // Validate record sequence number is numeric
    // Validate performing artist name (conditional but required if present)
    if let Some(ref name) = record.performing_artist_name {
        if name.trim().is_empty() {
            warnings.push(CwrWarning {
                field_name: "performing_artist_name",
                field_title: "Performing artist name (conditional)",
                source_str: std::borrow::Cow::Owned(name.clone()),
                level: WarningLevel::Warning,
                description: "Performing artist name should not be empty if specified".to_string(),
            });
        }
    }

    // Validate IPI name number format if present
    if let Some(ref ipi_name) = record.performing_artist_ipi_name_num {
        if !ipi_name.as_str().trim().is_empty() {
            if ipi_name.as_str().len() != 11 {
                warnings.push(CwrWarning {
                    field_name: "performing_artist_ipi_name_num",
                    field_title: "Performing artist IPI name number (optional)",
                    source_str: std::borrow::Cow::Owned(ipi_name.as_str().to_string()),
                    level: WarningLevel::Warning,
                    description: "IPI name number should be 11 characters if specified".to_string(),
                });
            }
            if !ipi_name.as_str().chars().all(|c| c.is_ascii_digit()) {
                warnings.push(CwrWarning {
                    field_name: "performing_artist_ipi_name_num",
                    field_title: "Performing artist IPI name number (optional)",
                    source_str: std::borrow::Cow::Owned(ipi_name.as_str().to_string()),
                    level: WarningLevel::Warning,
                    description: "IPI name number should be numeric".to_string(),
                });
            }
        }
    }

    // Validate IPI base number format if present
    if let Some(ref ipi_base) = record.performing_artist_ipi_base_number {
        if !ipi_base.as_str().trim().is_empty() {
            if ipi_base.as_str().len() != 13 {
                warnings.push(CwrWarning {
                    field_name: "performing_artist_ipi_base_number",
                    field_title: "Performing artist IPI base number (optional)",
                    source_str: std::borrow::Cow::Owned(ipi_base.as_str().to_string()),
                    level: WarningLevel::Warning,
                    description: "IPI base number should be 13 characters if specified".to_string(),
                });
            }
            // IPI base numbers are typically alphanumeric
            if !ipi_base.as_str().chars().all(|c| c.is_ascii_alphanumeric()) {
                warnings.push(CwrWarning {
                    field_name: "performing_artist_ipi_base_number",
                    field_title: "Performing artist IPI base number (optional)",
                    source_str: std::borrow::Cow::Owned(ipi_base.as_str().to_string()),
                    level: WarningLevel::Warning,
                    description: "IPI base number should be alphanumeric".to_string(),
                });
            }
        }
    }

    // Language code and performance language validation is now handled by the LanguageCode domain type

    // Performance dialect validation is now handled by the LanguageDialect domain type

    warnings
}
