//! Musical Work Distribution Category

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Musical Work Distribution Category (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct MusicalWorkDistributionCategory(pub String);

impl MusicalWorkDistributionCategory {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for MusicalWorkDistributionCategory {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for MusicalWorkDistributionCategory {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::musical_work_distribution_categories::is_valid_musical_work_distribution_category;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_musical_work_distribution_category(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Musical Work Distribution Category '{}' not found in lookup table", trimmed),
            });
        }

        (MusicalWorkDistributionCategory(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<MusicalWorkDistributionCategory> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (category, warnings) =
                MusicalWorkDistributionCategory::parse_cwr_field(source, field_name, field_title);
            (Some(category), warnings)
        }
    }
}
