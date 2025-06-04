//! SPU - Publisher Controlled by Submitter Record / OPU - Other Publisher Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// SPU - Publisher Controlled by Submitter Record (also OPU - Other Publisher)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpuRecord {
    /// "SPU" or "OPU"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Publisher sequence number (2 chars)
    pub publisher_sequence_num: String,

    /// Interested party number (9 chars, conditional)
    pub interested_party_num: Option<String>,

    /// Publisher name (45 chars, conditional)
    pub publisher_name: Option<String>,

    /// Publisher unknown indicator (1 char, conditional)
    pub publisher_unknown_indicator: Option<String>,

    /// Publisher type (2 chars, conditional)
    pub publisher_type: Option<String>,

    /// Tax ID number (9 chars, optional)
    pub tax_id_num: Option<String>,

    /// Publisher IPI name number (11 chars, conditional)
    pub publisher_ipi_name_num: Option<String>,

    /// Submitter agreement number (14 chars, optional)
    pub submitter_agreement_number: Option<String>,

    /// PR affiliation society number (3 chars, conditional)
    pub pr_affiliation_society_num: Option<String>,

    /// PR ownership share (5 chars, conditional)
    pub pr_ownership_share: Option<String>,

    /// MR society (3 chars, conditional)
    pub mr_society: Option<String>,

    /// MR ownership share (5 chars, conditional)
    pub mr_ownership_share: Option<String>,

    /// SR society (3 chars, conditional)
    pub sr_society: Option<String>,

    /// SR ownership share (5 chars, conditional)
    pub sr_ownership_share: Option<String>,

    /// Special agreements indicator (1 char, optional)
    pub special_agreements_indicator: Option<String>,

    /// First recording refusal indicator (1 char, optional)
    pub first_recording_refusal_ind: Option<String>,

    /// Filler (1 char, optional)
    pub filler: Option<String>,

    /// Publisher IPI base number (13 chars, optional)
    pub publisher_ipi_base_number: Option<String>,

    /// International standard agreement code (14 chars, optional)
    pub international_standard_agreement_code: Option<String>,

    /// Society-assigned agreement number (14 chars, optional)
    pub society_assigned_agreement_number: Option<String>,

    /// Agreement type (2 chars, optional, v2.1+)
    pub agreement_type: Option<String>,

    /// USA license indicator (1 char, optional, v2.1+)
    pub usa_license_ind: Option<String>,
}

impl SpuRecord {
    /// Create a new SPU record
    pub fn new(record_type: String, transaction_sequence_num: String, record_sequence_num: String, publisher_sequence_num: String) -> Self {
        Self {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            publisher_sequence_num,
            interested_party_num: None,
            publisher_name: None,
            publisher_unknown_indicator: None,
            publisher_type: None,
            tax_id_num: None,
            publisher_ipi_name_num: None,
            submitter_agreement_number: None,
            pr_affiliation_society_num: None,
            pr_ownership_share: None,
            mr_society: None,
            mr_ownership_share: None,
            sr_society: None,
            sr_ownership_share: None,
            special_agreements_indicator: None,
            first_recording_refusal_ind: None,
            filler: None,
            publisher_ipi_base_number: None,
            international_standard_agreement_code: None,
            society_assigned_agreement_number: None,
            agreement_type: None,
            usa_license_ind: None,
        }
    }

    /// Parse a CWR line into a SPU record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 180 {
            return Err(CwrParseError::BadFormat("SPU line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if !["SPU", "OPU"].contains(&record_type.as_str()) {
            return Err(CwrParseError::BadFormat(format!("Expected SPU/OPU, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let publisher_sequence_num = line.get(19..21).unwrap().trim().to_string();

        let interested_party_num = line.get(21..30).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let publisher_name = line.get(30..75).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let publisher_unknown_indicator = line.get(75..76).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let publisher_type = line.get(76..78).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let tax_id_num = line.get(78..87).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let publisher_ipi_name_num = line.get(87..98).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let submitter_agreement_number = line.get(98..112).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let pr_affiliation_society_num = line.get(112..115).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let pr_ownership_share = line.get(115..120).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let mr_society = line.get(120..123).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let mr_ownership_share = line.get(123..128).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let sr_society = line.get(128..131).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let sr_ownership_share = line.get(131..136).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let special_agreements_indicator = line.get(136..137).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let first_recording_refusal_ind = line.get(137..138).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let filler = line.get(138..139).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let publisher_ipi_base_number = line.get(139..152).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let international_standard_agreement_code = line.get(152..166).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
        let society_assigned_agreement_number = line.get(166..180).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());

        let agreement_type = if line.len() > 180 { line.get(180..182).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let usa_license_ind = if line.len() > 182 { line.get(182..183).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(SpuRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            publisher_sequence_num,
            interested_party_num,
            publisher_name,
            publisher_unknown_indicator,
            publisher_type,
            tax_id_num,
            publisher_ipi_name_num,
            submitter_agreement_number,
            pr_affiliation_society_num,
            pr_ownership_share,
            mr_society,
            mr_ownership_share,
            sr_society,
            sr_ownership_share,
            special_agreements_indicator,
            first_recording_refusal_ind,
            filler,
            publisher_ipi_base_number,
            international_standard_agreement_code,
            society_assigned_agreement_number,
            agreement_type,
            usa_license_ind,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:2}", self.publisher_sequence_num),
            format!("{:9}", self.interested_party_num.as_deref().unwrap_or("")),
            format!("{:45}", self.publisher_name.as_deref().unwrap_or("")),
            format!("{:1}", self.publisher_unknown_indicator.as_deref().unwrap_or("")),
            format!("{:2}", self.publisher_type.as_deref().unwrap_or("")),
            format!("{:9}", self.tax_id_num.as_deref().unwrap_or("")),
            format!("{:11}", self.publisher_ipi_name_num.as_deref().unwrap_or("")),
            format!("{:14}", self.submitter_agreement_number.as_deref().unwrap_or("")),
            format!("{:3}", self.pr_affiliation_society_num.as_deref().unwrap_or("")),
            format!("{:5}", self.pr_ownership_share.as_deref().unwrap_or("")),
            format!("{:3}", self.mr_society.as_deref().unwrap_or("")),
            format!("{:5}", self.mr_ownership_share.as_deref().unwrap_or("")),
            format!("{:3}", self.sr_society.as_deref().unwrap_or("")),
            format!("{:5}", self.sr_ownership_share.as_deref().unwrap_or("")),
            format!("{:1}", self.special_agreements_indicator.as_deref().unwrap_or("")),
            format!("{:1}", self.first_recording_refusal_ind.as_deref().unwrap_or("")),
            format!("{:1}", self.filler.as_deref().unwrap_or("")),
            format!("{:13}", self.publisher_ipi_base_number.as_deref().unwrap_or("")),
            format!("{:14}", self.international_standard_agreement_code.as_deref().unwrap_or("")),
            format!("{:14}", self.society_assigned_agreement_number.as_deref().unwrap_or("")),
        ];

        // Add v2.1+ fields
        if let Some(ref agreement_type) = self.agreement_type {
            fields.push(format!("{:2}", agreement_type));
        }

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
    fn test_spu_creation() {
        let spu = SpuRecord::new("SPU".to_string(), "00000001".to_string(), "00000001".to_string(), "01".to_string());

        assert_eq!(spu.record_type, "SPU");
        assert_eq!(spu.publisher_sequence_num, "01");
    }

    #[test]
    fn test_spu_round_trip() {
        let original = SpuRecord::new("SPU".to_string(), "00000001".to_string(), "00000001".to_string(), "01".to_string());

        let line = original.to_cwr_line();
        let parsed = SpuRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
    }
}
