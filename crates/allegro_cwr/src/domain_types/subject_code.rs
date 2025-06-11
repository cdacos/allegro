//! Subject Code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text_to_cwr_bytes};
use std::borrow::Cow;

/// Subject Code (2 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum SubjectCode {
    /// Direct Licensing - Instructions for Direct Licensing
    #[default]
    DirectLicensing,
    /// Share Change - Writer added/deleted, share increase or decreased
    ShareChange,
    /// Different Work - This is not the same work as XXX
    DifferentWork,
    /// Inquiry List - Work contained in Inquiry List of society
    InquiryList,
    /// Requested Work - Work for which a notification was requested by society
    RequestedWork,
    /// GEMA - Structured note for shares agreed by authors for performing rights
    Gema,
    /// SACEM - The Sacem submitter is controlling the entire work
    Sacem,
}

impl SubjectCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubjectCode::DirectLicensing => "DL",
            SubjectCode::ShareChange => "SC",
            SubjectCode::DifferentWork => "DW",
            SubjectCode::InquiryList => "IQ",
            SubjectCode::RequestedWork => "RQ",
            SubjectCode::Gema => "GW",
            SubjectCode::Sacem => "EW",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "DL" => Some(SubjectCode::DirectLicensing),
            "SC" => Some(SubjectCode::ShareChange),
            "DW" => Some(SubjectCode::DifferentWork),
            "IQ" => Some(SubjectCode::InquiryList),
            "RQ" => Some(SubjectCode::RequestedWork),
            "GW" => Some(SubjectCode::Gema),
            "EW" => Some(SubjectCode::Sacem),
            _ => None,
        }
    }
}

impl CwrFieldWrite for SubjectCode {
    fn to_cwr_field_bytes(&self, width: usize, character_set: &CharacterSet) -> Vec<u8> {
        format_text_to_cwr_bytes(self.as_str(), width, character_set)
    }
}

impl CwrFieldParse for SubjectCode {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        let mut warnings = vec![];

        match SubjectCode::from_str(trimmed) {
            Some(subject_code) => (subject_code, warnings),
            None => {
                warnings.push(CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!(
                        "Subject Code '{}' not found in lookup table. Expected: DL, SC, DW, IQ, RQ, GW, EW",
                        trimmed
                    ),
                });
                (SubjectCode::default(), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<SubjectCode> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (subject_code, warnings) = SubjectCode::parse_cwr_field(source, field_name, field_title);
            (Some(subject_code), warnings)
        }
    }
}
