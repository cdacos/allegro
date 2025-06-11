//! Agreement Type

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Agreement Type (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct AgreementType(pub String);

impl AgreementType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl CwrFieldWrite for AgreementType {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for AgreementType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::agreement_types::is_valid_agreement_type;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_agreement_type(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Agreement Type '{}' not found in lookup table", trimmed),
            });
        }

        (AgreementType(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<AgreementType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (agreement_type, warnings) = AgreementType::parse_cwr_field(source, field_name, field_title);
            (Some(agreement_type), warnings)
        }
    }
}
