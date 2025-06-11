//! Message Type

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Message Type (1 character)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum MessageType {
    /// Error message
    #[default]
    Error,
    /// Warning message
    Warning,
    /// Fatal message
    Fatal,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Error => "E",
            MessageType::Warning => "W",
            MessageType::Fatal => "F",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "E" => Some(MessageType::Error),
            "W" => Some(MessageType::Warning),
            "F" => Some(MessageType::Fatal),
            _ => None,
        }
    }
}

impl CwrFieldWrite for MessageType {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for MessageType {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match MessageType::from_str(trimmed) {
            Some(message_type) => (message_type, warnings),
            None => {
                warnings.push(CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Critical,
                    description: format!(
                        "Message Type '{}' not valid. Expected: E (Error), W (Warning), F (Fatal)",
                        trimmed
                    ),
                });
                (MessageType::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<MessageType> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (message_type, warnings) = MessageType::parse_cwr_field(source, field_name, field_title);
            (Some(message_type), warnings)
        }
    }
}
