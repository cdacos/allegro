//! NWR - New Work Registration Record
//!
//! Also handles REV (Revised Registration), ISW (ISWC Notification), and EXC (Existing Work in Conflict).

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// NWR - New Work Registration Record
///
/// Used for NWR, REV, ISW, and EXC record types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NwrRecord {
    /// "NWR", "REV", "ISW", or "EXC"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Work title (60 chars)
    pub work_title: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Submitter work number (14 chars)
    pub submitter_work_num: String,

    /// ISWC (11 chars, optional)
    pub iswc: Option<String>,

    /// Copyright date (8 chars, optional)
    pub copyright_date: Option<String>,

    /// Copyright number (12 chars, optional)
    pub copyright_number: Option<String>,

    /// Musical work distribution category (3 chars)
    pub musical_work_distribution_category: String,

    /// Duration HHMMSS (6 chars, conditional)
    pub duration: Option<String>,

    /// Recorded indicator (1 char)
    pub recorded_indicator: String,

    /// Text music relationship (3 chars, optional)
    pub text_music_relationship: Option<String>,

    /// Composite type (3 chars, optional)
    pub composite_type: Option<String>,

    /// Version type (3 chars)
    pub version_type: String,

    /// Excerpt type (3 chars, optional)
    pub excerpt_type: Option<String>,

    /// Music arrangement (3 chars, conditional)
    pub music_arrangement: Option<String>,

    /// Lyric adaptation (3 chars, conditional)
    pub lyric_adaptation: Option<String>,

    /// Contact name (30 chars, optional)
    pub contact_name: Option<String>,

    /// Contact ID (10 chars, optional)
    pub contact_id: Option<String>,

    /// CWR work type (2 chars, optional)
    pub cwr_work_type: Option<String>,

    /// Grand rights indicator (1 char, conditional)
    pub grand_rights_ind: Option<String>,

    /// Composite component count (3 chars, conditional)
    pub composite_component_count: Option<String>,

    /// Date of publication of printed edition (8 chars, optional)
    pub date_of_publication_of_printed_edition: Option<String>,

    /// Exceptional clause (1 char, optional)
    pub exceptional_clause: Option<String>,

    /// Opus number (25 chars, optional)
    pub opus_number: Option<String>,

    /// Catalogue number (25 chars, optional)
    pub catalogue_number: Option<String>,

    /// Priority flag (1 char, optional, v2.1+)
    pub priority_flag: Option<String>,
}

impl NwrRecord {
    /// Create a new NWR record with required fields
    pub fn new(record_type: String, transaction_sequence_num: String, record_sequence_num: String, work_title: String, submitter_work_num: String, musical_work_distribution_category: String, recorded_indicator: String, version_type: String) -> Self {
        Self {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            work_title,
            language_code: None,
            submitter_work_num,
            iswc: None,
            copyright_date: None,
            copyright_number: None,
            musical_work_distribution_category,
            duration: None,
            recorded_indicator,
            text_music_relationship: None,
            composite_type: None,
            version_type,
            excerpt_type: None,
            music_arrangement: None,
            lyric_adaptation: None,
            contact_name: None,
            contact_id: None,
            cwr_work_type: None,
            grand_rights_ind: None,
            composite_component_count: None,
            date_of_publication_of_printed_edition: None,
            exceptional_clause: None,
            opus_number: None,
            catalogue_number: None,
            priority_flag: None,
        }
    }

    /// Parse a CWR line into an NWR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 259 {
            return Err(CwrParseError::BadFormat("NWR line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if !["NWR", "REV", "ISW", "EXC"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected NWR/REV/ISW/EXC, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let work_title = line.get(19..79).unwrap().trim().to_string();

        let language_code = line.get(79..81).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let submitter_work_num = line.get(81..95).unwrap().trim().to_string();

        let iswc = line.get(95..106).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let copyright_date = line.get(106..114).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let copyright_number = line.get(114..126).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let musical_work_distribution_category = line.get(126..129).unwrap().trim().to_string();

        let duration = line.get(129..135).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let recorded_indicator = line.get(135..136).unwrap().trim().to_string();

        let text_music_relationship = line.get(136..139).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let composite_type = line.get(139..142).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let version_type = line.get(142..145).unwrap().trim().to_string();

        let excerpt_type = line.get(145..148).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let music_arrangement = line.get(148..151).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let lyric_adaptation = line.get(151..154).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let contact_name = line.get(154..184).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let contact_id = line.get(184..194).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let cwr_work_type = line.get(194..196).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let grand_rights_ind = line.get(196..197).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let composite_component_count = line.get(197..200).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let date_of_publication_of_printed_edition = line.get(200..208).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let exceptional_clause = line.get(208..209).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let opus_number = line.get(209..234).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let catalogue_number = line.get(234..259).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let priority_flag = if line.len() > 259 { line.get(259..260).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(NwrRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            work_title,
            language_code,
            submitter_work_num,
            iswc,
            copyright_date,
            copyright_number,
            musical_work_distribution_category,
            duration,
            recorded_indicator,
            text_music_relationship,
            composite_type,
            version_type,
            excerpt_type,
            music_arrangement,
            lyric_adaptation,
            contact_name,
            contact_id,
            cwr_work_type,
            grand_rights_ind,
            composite_component_count,
            date_of_publication_of_printed_edition,
            exceptional_clause,
            opus_number,
            catalogue_number,
            priority_flag,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:60}", self.work_title),
            format!("{:2}", self.language_code.as_deref().unwrap_or("")),
            format!("{:14}", self.submitter_work_num),
            format!("{:11}", self.iswc.as_deref().unwrap_or("")),
            format!("{:8}", self.copyright_date.as_deref().unwrap_or("")),
            format!("{:12}", self.copyright_number.as_deref().unwrap_or("")),
            format!("{:3}", self.musical_work_distribution_category),
            format!("{:6}", self.duration.as_deref().unwrap_or("")),
            format!("{:1}", self.recorded_indicator),
            format!("{:3}", self.text_music_relationship.as_deref().unwrap_or("")),
            format!("{:3}", self.composite_type.as_deref().unwrap_or("")),
            format!("{:3}", self.version_type),
            format!("{:3}", self.excerpt_type.as_deref().unwrap_or("")),
            format!("{:3}", self.music_arrangement.as_deref().unwrap_or("")),
            format!("{:3}", self.lyric_adaptation.as_deref().unwrap_or("")),
            format!("{:30}", self.contact_name.as_deref().unwrap_or("")),
            format!("{:10}", self.contact_id.as_deref().unwrap_or("")),
            format!("{:2}", self.cwr_work_type.as_deref().unwrap_or("")),
            format!("{:1}", self.grand_rights_ind.as_deref().unwrap_or("")),
            format!("{:3}", self.composite_component_count.as_deref().unwrap_or("")),
            format!("{:8}", self.date_of_publication_of_printed_edition.as_deref().unwrap_or("")),
            format!("{:1}", self.exceptional_clause.as_deref().unwrap_or("")),
            format!("{:25}", self.opus_number.as_deref().unwrap_or("")),
            format!("{:25}", self.catalogue_number.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ field
        if let Some(ref priority_flag) = self.priority_flag {
            fields.push(format!("{:1}", priority_flag));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nwr_creation() {
        let nwr = NwrRecord::new("NWR".to_string(), "00000001".to_string(), "00000001".to_string(), "Test Song".to_string(), "SW0000000001".to_string(), "SER".to_string(), "Y".to_string(), "ORI".to_string());

        assert_eq!(nwr.record_type, "NWR");
        assert_eq!(nwr.work_title, "Test Song");
        assert_eq!(nwr.submitter_work_num, "SW0000000001");
    }

    #[test]
    fn test_nwr_round_trip() {
        let original = NwrRecord::new("NWR".to_string(), "00000001".to_string(), "00000001".to_string(), "Test Song".to_string(), "SW0000000001".to_string(), "SER".to_string(), "Y".to_string(), "ORI".to_string());

        let line = original.to_cwr_line();
        let parsed = NwrRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 259); // Without optional v2.1 field
    }
}
