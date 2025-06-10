//! Recording format for CWR recording (REC) records
//!
//! Indicates the format of an audio recording.

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_number, format_text};
use std::borrow::Cow;

/// Recording format for REC record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingFormat {
    #[default]
    Unknown,
    Stereo,
    Mono,
    Quadrophonic,
}

impl RecordingFormat {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingFormat::Unknown => "U",
            RecordingFormat::Stereo => "S",
            RecordingFormat::Mono => "M",
            RecordingFormat::Quadrophonic => "Q",
        }
    }
}

impl CwrFieldWrite for RecordingFormat {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for RecordingFormat {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "U" => (RecordingFormat::Unknown, vec![]),
            "S" => (RecordingFormat::Stereo, vec![]),
            "M" => (RecordingFormat::Mono, vec![]),
            "Q" => (RecordingFormat::Quadrophonic, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!("Invalid recording format '{}', defaulting to Unknown", trimmed),
                }];
                (RecordingFormat::Unknown, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<RecordingFormat> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (format, warnings) = RecordingFormat::parse_cwr_field(source, field_name, field_title);
            (Some(format), warnings)
        }
    }
}
