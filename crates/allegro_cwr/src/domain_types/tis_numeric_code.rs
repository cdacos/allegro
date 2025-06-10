//! TIS numeric code for territory records

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// TIS numeric code for territory records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct TisNumericCode(pub u16);

impl TisNumericCode {
    pub fn as_str(&self) -> String {
        format!("{:04}", self.0)
    }
}

impl CwrFieldWrite for TisNumericCode {
    fn to_cwr_str(&self, _width: usize) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for TisNumericCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::tis_codes::{is_valid_tis_code, territory_exists};

        let trimmed = source.trim();
        match trimmed.parse::<u16>() {
            Ok(num) => {
                let code_str = format!("{:04}", num);
                let mut warnings = vec![];

                if !territory_exists(num) {
                    warnings.push(CwrWarning {
                        field_name,
                        field_title,
                        source_str: Cow::Owned(source.to_string()),
                        level: WarningLevel::Warning,
                        description: format!("TIS code '{}' not found in territory table", code_str),
                    });
                } else if !is_valid_tis_code(num) {
                    warnings.push(CwrWarning {
                        field_name,
                        field_title,
                        source_str: Cow::Owned(source.to_string()),
                        level: WarningLevel::Warning,
                        description: format!("TIS code '{}' is marked as unusable in territory table", code_str),
                    });
                }

                (TisNumericCode(num), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid TIS numeric code format: {}", trimmed),
                }];
                (TisNumericCode(0), warnings)
            }
        }
    }
}
