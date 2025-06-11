//! Agreement role code for CWR interested party agreement (IPA) records
//!
//! Indicates the role of an interested party in an agreement.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Agreement role code for IPA record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AgreementRoleCode {
    #[default]
    Assignor,
    Acquirer,
}

impl AgreementRoleCode {
    pub fn as_str(&self) -> &str {
        match self {
            AgreementRoleCode::Assignor => "AS",
            AgreementRoleCode::Acquirer => "AC",
        }
    }
}

impl CwrFieldWrite for AgreementRoleCode {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for AgreementRoleCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::agreement_role_codes::is_valid_agreement_role_code;

        let trimmed = source.trim();
        match trimmed {
            "AS" => (AgreementRoleCode::Assignor, vec![]),
            "AC" => (AgreementRoleCode::Acquirer, vec![]),
            _ => {
                let mut warnings = vec![];
                if !is_valid_agreement_role_code(trimmed) {
                    warnings.push(CwrWarning {
                        field_name,
                        field_title,
                        source_str: Cow::Owned(source.to_string()),
                        level: WarningLevel::Critical,
                        description: format!("Invalid agreement role code '{}', must be AS or AC", trimmed),
                    });
                }
                (AgreementRoleCode::Assignor, warnings)
            }
        }
    }
}
