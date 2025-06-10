//! Publisher sequence number

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Publisher sequence number
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct PublisherSequenceNumber(pub u8);

impl PublisherSequenceNumber {
    pub fn as_str(&self) -> String {
        format!("{:02}", self.0)
    }
}

impl CwrFieldWrite for PublisherSequenceNumber {
    fn to_cwr_str(&self, _width: usize) -> String {
        self.as_str()
    }
}

impl CwrFieldParse for PublisherSequenceNumber {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed.parse::<u8>() {
            Ok(num) if num > 0 && num <= 99 => (PublisherSequenceNumber(num), vec![]),
            Ok(num) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Publisher sequence number {} out of valid range 1-99", num),
                }];
                (PublisherSequenceNumber(1), warnings)
            }
            Err(_) => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid publisher sequence number format: {}", trimmed),
                }];
                (PublisherSequenceNumber(1), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<PublisherSequenceNumber> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() || trimmed == "00" {
            (None, vec![])
        } else {
            let (seq_num, warnings) = PublisherSequenceNumber::parse_cwr_field(source, field_name, field_title);
            (Some(seq_num), warnings)
        }
    }
}
