//! Recording technique for CWR recording (REC) records
//!
//! Indicates the recording technique used for an audio recording.

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Recording technique for REC record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingTechnique {
    #[default]
    Unknown,
    Analog,
    Digital,
}

impl RecordingTechnique {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingTechnique::Unknown => "U",
            RecordingTechnique::Analog => "A",
            RecordingTechnique::Digital => "D",
        }
    }
}

impl CwrFieldWrite for RecordingTechnique {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(&self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for RecordingTechnique {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "U" => (RecordingTechnique::Unknown, vec![]),
            "A" => (RecordingTechnique::Analog, vec![]),
            "D" => (RecordingTechnique::Digital, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid recording technique '{}', defaulting to Unknown", trimmed),
                }];
                (RecordingTechnique::Unknown, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<RecordingTechnique> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (technique, warnings) = RecordingTechnique::parse_cwr_field(source, field_name, field_title);
            (Some(technique), warnings)
        }
    }
}
