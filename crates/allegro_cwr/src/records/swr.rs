//! SWR - Writer Controlled by Submitter Record
//!
//! Also handles OWR (Other Writer) records.

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// SWR - Writer Controlled by Submitter Record (also OWR - Other Writer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwrRecord {
    /// "SWR" or "OWR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Writer last name (45 chars, conditional)
    pub writer_last_name: Option<String>,

    /// Writer first name (30 chars, optional)
    pub writer_first_name: Option<String>,

    /// Writer unknown indicator (1 char, conditional)
    pub writer_unknown_indicator: Option<String>,

    /// Writer designation code (2 chars, conditional)
    pub writer_designation_code: Option<String>,

    /// Tax ID number (9 chars, optional)
    pub tax_id_num: Option<String>,

    /// Writer IPI name number (11 chars, optional)
    pub writer_ipi_name_num: Option<String>,

    /// PR affiliation society number (3 chars, optional)
    pub pr_affiliation_society_num: Option<String>,

    /// PR ownership share (5 chars, optional)
    pub pr_ownership_share: Option<String>,

    /// MR society (3 chars, optional)
    pub mr_society: Option<String>,

    /// MR ownership share (5 chars, optional)
    pub mr_ownership_share: Option<String>,

    /// SR society (3 chars, optional)
    pub sr_society: Option<String>,

    /// SR ownership share (5 chars, optional)
    pub sr_ownership_share: Option<String>,

    /// Reversionary indicator (1 char, optional)
    pub reversionary_indicator: Option<String>,

    /// First recording refusal indicator (1 char, optional)
    pub first_recording_refusal_ind: Option<String>,

    /// Work for hire indicator (1 char, optional)
    pub work_for_hire_indicator: Option<String>,

    /// Filler (1 char, optional)
    pub filler: Option<String>,

    /// Writer IPI base number (13 chars, optional)
    pub writer_ipi_base_number: Option<String>,

    /// Personal number (12 chars, optional)
    pub personal_number: Option<String>,

    /// USA license indicator (1 char, optional, v2.1+)
    pub usa_license_ind: Option<String>,
}

impl SwrRecord {
    /// Create a new SWR record
    pub fn new(record_type: String, transaction_sequence_num: String, record_sequence_num: String) -> Self {
        Self {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            interested_party_num: None,
            writer_last_name: None,
            writer_first_name: None,
            writer_unknown_indicator: None,
            writer_designation_code: None,
            tax_id_num: None,
            writer_ipi_name_num: None,
            pr_affiliation_society_num: None,
            pr_ownership_share: None,
            mr_society: None,
            mr_ownership_share: None,
            sr_society: None,
            sr_ownership_share: None,
            reversionary_indicator: None,
            first_recording_refusal_ind: None,
            work_for_hire_indicator: None,
            filler: None,
            writer_ipi_base_number: None,
            personal_number: None,
            usa_license_ind: None,
        }
    }

    /// Parse a CWR line into a SWR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 179 {
            return Err(CwrParseError::BadFormat("SWR line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if !["SWR", "OWR"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected SWR/OWR, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();

        // Extract all fields as optional
        let interested_party_num = line.get(19..28).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_last_name = line.get(28..73).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_first_name = line.get(73..103).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_unknown_indicator = line.get(103..104).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_designation_code = line.get(104..106).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let tax_id_num = line.get(106..115).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_ipi_name_num = line.get(115..126).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let pr_affiliation_society_num = line.get(126..129).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let pr_ownership_share = line.get(129..134).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let mr_society = line.get(134..137).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let mr_ownership_share = line.get(137..142).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let sr_society = line.get(142..145).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let sr_ownership_share = line.get(145..150).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let reversionary_indicator = line.get(150..151).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let first_recording_refusal_ind = line.get(151..152).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let work_for_hire_indicator = line.get(152..153).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let filler = line.get(153..154).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let writer_ipi_base_number = line.get(154..167).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let personal_number = line.get(167..179).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let usa_license_ind = if line.len() > 179 { line.get(179..180).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(SwrRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            interested_party_num,
            writer_last_name,
            writer_first_name,
            writer_unknown_indicator,
            writer_designation_code,
            tax_id_num,
            writer_ipi_name_num,
            pr_affiliation_society_num,
            pr_ownership_share,
            mr_society,
            mr_ownership_share,
            sr_society,
            sr_ownership_share,
            reversionary_indicator,
            first_recording_refusal_ind,
            work_for_hire_indicator,
            filler,
            writer_ipi_base_number,
            personal_number,
            usa_license_ind,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:9}", self.interested_party_num.as_deref().unwrap_or("")),
            format!("{:45}", self.writer_last_name.as_deref().unwrap_or("")),
            format!("{:30}", self.writer_first_name.as_deref().unwrap_or("")),
            format!("{:1}", self.writer_unknown_indicator.as_deref().unwrap_or("")),
            format!("{:2}", self.writer_designation_code.as_deref().unwrap_or("")),
            format!("{:9}", self.tax_id_num.as_deref().unwrap_or("")),
            format!("{:11}", self.writer_ipi_name_num.as_deref().unwrap_or("")),
            format!("{:3}", self.pr_affiliation_society_num.as_deref().unwrap_or("")),
            format!("{:5}", self.pr_ownership_share.as_deref().unwrap_or("")),
            format!("{:3}", self.mr_society.as_deref().unwrap_or("")),
            format!("{:5}", self.mr_ownership_share.as_deref().unwrap_or("")),
            format!("{:3}", self.sr_society.as_deref().unwrap_or("")),
            format!("{:5}", self.sr_ownership_share.as_deref().unwrap_or("")),
            format!("{:1}", self.reversionary_indicator.as_deref().unwrap_or("")),
            format!("{:1}", self.first_recording_refusal_ind.as_deref().unwrap_or("")),
            format!("{:1}", self.work_for_hire_indicator.as_deref().unwrap_or("")),
            format!("{:1}", self.filler.as_deref().unwrap_or("")),
            format!("{:13}", self.writer_ipi_base_number.as_deref().unwrap_or("")),
            format!("{:12}", self.personal_number.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ field
        if let Some(ref usa_license) = self.usa_license_ind {
            fields.push(format!("{:1}", usa_license));
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swr_creation() {
        let swr = SwrRecord::new("SWR".to_string(), "00000001".to_string(), "00000002".to_string());

        assert_eq!(swr.record_type, "SWR");
        assert_eq!(swr.transaction_sequence_num, "00000001");
    }

    #[test]
    fn test_swr_round_trip() {
        let original = SwrRecord::new("SWR".to_string(), "00000001".to_string(), "00000002".to_string());

        let line = original.to_cwr_line();
        let parsed = SwrRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
    }
}
