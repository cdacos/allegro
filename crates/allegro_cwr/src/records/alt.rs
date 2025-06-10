use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ALT - Alternate Title Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = alt_custom_validate, test_data = "ALT0000000200000326BABY CAN T YOU SEE                                          AT  ")]
pub struct AltRecord {
    #[cwr(title = "Always 'ALT'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Alternate title", start = 19, len = 60)]
    pub alternate_title: String,

    #[cwr(title = "Title type", start = 79, len = 2)]
    pub title_type: TitleType,

    #[cwr(title = "Language code (conditional)", start = 81, len = 2)]
    pub language_code: Option<LanguageCode>,
}

// Custom validation function for ALT record
fn alt_custom_validate(record: &mut AltRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Alternate title cannot be empty
    if record.alternate_title.trim().is_empty() {
        warnings.push(CwrWarning { field_name: "alternate_title", field_title: "Alternate title", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Alternate title cannot be empty".to_string() });
    }

    // Business rule: Language code required for translated and transliterated titles
    if matches!(record.title_type, TitleType::TranslatedTitle | TitleType::TransliterationTrans | TitleType::TransliterationAlt) && (record.language_code.is_none() || record.language_code.as_ref().is_none_or(|s| s.trim().is_empty())) {
        warnings.push(CwrWarning { field_name: "language_code", field_title: "Language code (conditional)", source_str: std::borrow::Cow::Borrowed(""), level: WarningLevel::Critical, description: "Language code is required for translated and transliterated titles".to_string() });
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)
    // - Language codes must be valid ISO 639-1 or ISO 639-2 codes (requires lookup table)
    // - Alternate titles should not duplicate the original work title (requires cross-record validation)
    // - Fictional titles (FT) and Original titles (OT) are v2.1+ features (requires version validation)

    warnings
}
