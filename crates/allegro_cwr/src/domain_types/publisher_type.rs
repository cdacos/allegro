//! Publisher type for CWR sub-publisher (SPU) records
//!
//! Indicates the type of publisher in a CWR submission.

use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

/// Publisher type for SPU record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PublisherType {
    #[default]
    Acquirer,
    Administrator,
    IncomeParticipant,
    OriginalPublisher,
    SubstitutedPublisher,
    SubPublisher,
}

impl PublisherType {
    pub fn as_str(&self) -> &str {
        match self {
            PublisherType::Acquirer => "AQ",
            PublisherType::Administrator => "AM",
            PublisherType::IncomeParticipant => "PA",
            PublisherType::OriginalPublisher => "E",
            PublisherType::SubstitutedPublisher => "ES",
            PublisherType::SubPublisher => "SE",
        }
    }
}

impl CwrFieldWrite for PublisherType {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for PublisherType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::publisher_types::is_valid_publisher_type;

        let trimmed = source.trim();
        let default_type = PublisherType::Acquirer;

        if !is_valid_publisher_type(trimmed) {
            let warnings = vec![CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Invalid publisher type '{}', defaulting to AQ", trimmed),
            }];
            return (default_type, warnings);
        }

        match trimmed {
            "AQ" => (PublisherType::Acquirer, vec![]),
            "AM" => (PublisherType::Administrator, vec![]),
            "PA" => (PublisherType::IncomeParticipant, vec![]),
            "E" => (PublisherType::OriginalPublisher, vec![]),
            "ES" => (PublisherType::SubstitutedPublisher, vec![]),
            "SE" => (PublisherType::SubPublisher, vec![]),
            _ => (default_type, vec![]),
        }
    }
}

impl CwrFieldParse for Option<PublisherType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (publisher_type, warnings) = PublisherType::parse_cwr_field(source, field_name, field_title);
            (Some(publisher_type), warnings)
        }
    }
}
