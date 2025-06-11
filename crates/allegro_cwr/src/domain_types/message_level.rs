//! Message Level

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// Message Level (1 character)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum MessageLevel {
    /// Record level message
    #[default]
    Record,
    /// Group level message
    Group,
    /// Transaction level message
    Transaction,
}

impl MessageLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageLevel::Record => "R",
            MessageLevel::Group => "G",
            MessageLevel::Transaction => "T",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "R" => Some(MessageLevel::Record),
            "G" => Some(MessageLevel::Group),
            "T" => Some(MessageLevel::Transaction),
            _ => None,
        }
    }
}

impl CwrFieldWrite for MessageLevel {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for MessageLevel {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match MessageLevel::from_str(trimmed) {
            Some(message_level) => (message_level, warnings),
            None => {
                warnings.push(CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!(
                        "Message Level '{}' not valid. Expected: R (Record), G (Group), T (Transaction)",
                        trimmed
                    ),
                });
                (MessageLevel::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<MessageLevel> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (message_level, warnings) = MessageLevel::parse_cwr_field(source, field_name, field_title);
            (Some(message_level), warnings)
        }
    }
}
