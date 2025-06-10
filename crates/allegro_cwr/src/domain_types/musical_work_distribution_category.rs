//! Musical Work Distribution Category

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
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
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
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
