//! EWT - Entire Work Title for Excerpts Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// EWT - Entire Work Title for Excerpts Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EwtRecord {
    /// Always "EWT"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Entire work title (60 chars)
    pub entire_work_title: String,

    /// ISWC of entire work (11 chars, optional)
    pub iswc_of_entire_work: Option<String>,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Writer 1 last name (45 chars, optional)
    pub writer_1_last_name: Option<String>,

    /// Writer 1 first name (30 chars, optional)
    pub writer_1_first_name: Option<String>,

    /// Source (60 chars, optional)
    pub source: Option<String>,

    /// Writer 1 IPI name number (11 chars, optional)
    pub writer_1_ipi_name_num: Option<String>,

    /// Writer 1 IPI base number (13 chars, optional)
    pub writer_1_ipi_base_number: Option<String>,

    /// Writer 2 last name (45 chars, optional)
    pub writer_2_last_name: Option<String>,

    /// Writer 2 first name (30 chars, optional)
    pub writer_2_first_name: Option<String>,

    /// Writer 2 IPI name number (11 chars, optional)
    pub writer_2_ipi_name_num: Option<String>,

    /// Writer 2 IPI base number (13 chars, optional)
    pub writer_2_ipi_base_number: Option<String>,

    /// Submitter work number (14 chars, optional)
    pub submitter_work_num: Option<String>,
}

impl EwtRecord {
    /// Create a new EWT record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, entire_work_title: String) -> Self {
        Self {
            record_type: "EWT".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            entire_work_title,
            iswc_of_entire_work: None,
            language_code: None,
            writer_1_last_name: None,
            writer_1_first_name: None,
            source: None,
            writer_1_ipi_name_num: None,
            writer_1_ipi_base_number: None,
            writer_2_last_name: None,
            writer_2_first_name: None,
            writer_2_ipi_name_num: None,
            writer_2_ipi_base_number: None,
            submitter_work_num: None,
        }
    }

    /// Parse a CWR line into an EWT record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 79 {
            return Err(CwrParseError::BadFormat("EWT line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "EWT" {
            return Err(CwrParseError::BadFormat(format!("Expected EWT, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let entire_work_title = line.get(19..79).unwrap().trim().to_string();

        let iswc_of_entire_work = if line.len() > 79 { line.get(79..90).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let language_code = if line.len() > 90 { line.get(90..92).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_1_last_name = if line.len() > 92 { line.get(92..137).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_1_first_name = if line.len() > 137 { line.get(137..167).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let source = if line.len() > 167 { line.get(167..227).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_1_ipi_name_num = if line.len() > 227 { line.get(227..238).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_1_ipi_base_number = if line.len() > 238 { line.get(238..251).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_2_last_name = if line.len() > 251 { line.get(251..296).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_2_first_name = if line.len() > 296 { line.get(296..326).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_2_ipi_name_num = if line.len() > 326 { line.get(326..337).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let writer_2_ipi_base_number = if line.len() > 337 { line.get(337..350).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let submitter_work_num = if line.len() > 350 { line.get(350..364).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(EwtRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            entire_work_title,
            iswc_of_entire_work,
            language_code,
            writer_1_last_name,
            writer_1_first_name,
            source,
            writer_1_ipi_name_num,
            writer_1_ipi_base_number,
            writer_2_last_name,
            writer_2_first_name,
            writer_2_ipi_name_num,
            writer_2_ipi_base_number,
            submitter_work_num,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:60}", self.entire_work_title)];

        if self.iswc_of_entire_work.is_some()
            || self.language_code.is_some()
            || self.writer_1_last_name.is_some()
            || self.writer_1_first_name.is_some()
            || self.source.is_some()
            || self.writer_1_ipi_name_num.is_some()
            || self.writer_1_ipi_base_number.is_some()
            || self.writer_2_last_name.is_some()
            || self.writer_2_first_name.is_some()
            || self.writer_2_ipi_name_num.is_some()
            || self.writer_2_ipi_base_number.is_some()
            || self.submitter_work_num.is_some()
        {
            fields.push(format!("{:11}", self.iswc_of_entire_work.as_deref().unwrap_or("")));
            fields.push(format!("{:2}", self.language_code.as_deref().unwrap_or("")));
            fields.push(format!("{:45}", self.writer_1_last_name.as_deref().unwrap_or("")));
            fields.push(format!("{:30}", self.writer_1_first_name.as_deref().unwrap_or("")));
            fields.push(format!("{:60}", self.source.as_deref().unwrap_or("")));
            fields.push(format!("{:11}", self.writer_1_ipi_name_num.as_deref().unwrap_or("")));
            fields.push(format!("{:13}", self.writer_1_ipi_base_number.as_deref().unwrap_or("")));
            fields.push(format!("{:45}", self.writer_2_last_name.as_deref().unwrap_or("")));
            fields.push(format!("{:30}", self.writer_2_first_name.as_deref().unwrap_or("")));
            fields.push(format!("{:11}", self.writer_2_ipi_name_num.as_deref().unwrap_or("")));
            fields.push(format!("{:13}", self.writer_2_ipi_base_number.as_deref().unwrap_or("")));
            fields.push(format!("{:14}", self.submitter_work_num.as_deref().unwrap_or("")));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ewt_creation() {
        let ewt = EwtRecord::new("00000001".to_string(), "00000001".to_string(), "Entire Work Title".to_string());

        assert_eq!(ewt.record_type, "EWT");
        assert_eq!(ewt.entire_work_title, "Entire Work Title");
    }

    #[test]
    fn test_ewt_round_trip() {
        let original = EwtRecord::new("00000001".to_string(), "00000001".to_string(), "Entire Work Title".to_string());

        let line = original.to_cwr_line();
        let parsed = EwtRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 79);
    }
}
