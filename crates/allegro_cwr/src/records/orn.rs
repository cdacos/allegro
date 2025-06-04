//! ORN - Work Origin Record

use crate::error::CwrParseError;
use serde::{Deserialize, Serialize};

/// ORN - Work Origin Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrnRecord {
    /// Always "ORN"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Intended purpose (3 chars)
    pub intended_purpose: String,

    /// Production title (60 chars, conditional)
    pub production_title: Option<String>,

    /// CD identifier (15 chars, conditional)
    pub cd_identifier: Option<String>,

    /// Cut number (4 chars, optional)
    pub cut_number: Option<String>,

    /// Library (60 chars, conditional, v2.1+)
    pub library: Option<String>,

    /// BLTVR (1 char, optional, v2.1+)
    pub bltvr: Option<String>,

    /// Filler (25 chars, optional, v2.1+)
    pub filler: Option<String>,

    /// Production number (12 chars, optional, v2.1+)
    pub production_num: Option<String>,

    /// Episode title (60 chars, optional, v2.1+)
    pub episode_title: Option<String>,

    /// Episode number (20 chars, optional, v2.1+)
    pub episode_num: Option<String>,

    /// Year of production (4 chars, optional, v2.1+)
    pub year_of_production: Option<String>,

    /// AVI society code (3 chars, optional, v2.1+)
    pub avi_society_code: Option<String>,

    /// Audio-visual number (15 chars, optional, v2.1+)
    pub audio_visual_number: Option<String>,

    /// V-ISAN/ISAN (12 chars, optional, v2.2+)
    pub v_isan_isan: Option<String>,

    /// V-ISAN/Episode (4 chars, optional, v2.2+)
    pub v_isan_episode: Option<String>,

    /// V-ISAN/Check Digit 1 (1 char, optional, v2.2+)
    pub v_isan_check_digit_1: Option<String>,

    /// V-ISAN/Version (8 chars, optional, v2.2+)
    pub v_isan_version: Option<String>,

    /// V-ISAN/Check Digit 2 (1 char, optional, v2.2+)
    pub v_isan_check_digit_2: Option<String>,

    /// EIDR (20 chars, optional, v2.2+)
    pub eidr: Option<String>,

    /// EIDR/Check Digit (1 char, optional, v2.2+)
    pub eidr_check_digit: Option<String>,
}

impl OrnRecord {
    /// Create a new ORN record
    pub fn new(transaction_sequence_num: String, record_sequence_num: String, intended_purpose: String) -> Self {
        Self {
            record_type: "ORN".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            intended_purpose,
            production_title: None,
            cd_identifier: None,
            cut_number: None,
            library: None,
            bltvr: None,
            filler: None,
            production_num: None,
            episode_title: None,
            episode_num: None,
            year_of_production: None,
            avi_society_code: None,
            audio_visual_number: None,
            v_isan_isan: None,
            v_isan_episode: None,
            v_isan_check_digit_1: None,
            v_isan_version: None,
            v_isan_check_digit_2: None,
            eidr: None,
            eidr_check_digit: None,
        }
    }

    /// Parse a CWR line into an ORN record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 22 {
            return Err(CwrParseError::BadFormat("ORN line too short".to_string()));
        }

        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "ORN" {
            return Err(CwrParseError::BadFormat(format!("Expected ORN, found {}", record_type)));
        }

        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let intended_purpose = line.get(19..22).unwrap().trim().to_string();

        let production_title = if line.len() > 22 { line.get(22..82).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let cd_identifier = if line.len() > 82 { line.get(82..97).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let cut_number = if line.len() > 97 { line.get(97..101).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        // v2.1+ fields
        let library = if line.len() > 101 { line.get(101..161).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let bltvr = if line.len() > 161 { line.get(161..162).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let filler = if line.len() > 162 { line.get(162..187).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let production_num = if line.len() > 187 { line.get(187..199).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let episode_title = if line.len() > 199 { line.get(199..259).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let episode_num = if line.len() > 259 { line.get(259..279).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let year_of_production = if line.len() > 279 { line.get(279..283).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let avi_society_code = if line.len() > 283 { line.get(283..286).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let audio_visual_number = if line.len() > 286 { line.get(286..301).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        // v2.2+ fields
        let v_isan_isan = if line.len() > 301 { line.get(301..313).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let v_isan_episode = if line.len() > 313 { line.get(313..317).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let v_isan_check_digit_1 = if line.len() > 317 { line.get(317..318).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let v_isan_version = if line.len() > 318 { line.get(318..326).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let v_isan_check_digit_2 = if line.len() > 326 { line.get(326..327).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let eidr = if line.len() > 327 { line.get(327..347).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        let eidr_check_digit = if line.len() > 347 { line.get(347..348).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string()) } else { None };

        Ok(OrnRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            intended_purpose,
            production_title,
            cd_identifier,
            cut_number,
            library,
            bltvr,
            filler,
            production_num,
            episode_title,
            episode_num,
            year_of_production,
            avi_society_code,
            audio_visual_number,
            v_isan_isan,
            v_isan_episode,
            v_isan_check_digit_1,
            v_isan_version,
            v_isan_check_digit_2,
            eidr,
            eidr_check_digit,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![format!("{:3}", self.record_type), format!("{:8}", self.transaction_sequence_num), format!("{:8}", self.record_sequence_num), format!("{:3}", self.intended_purpose)];

        if self.production_title.is_some()
            || self.cd_identifier.is_some()
            || self.cut_number.is_some()
            || self.library.is_some()
            || self.bltvr.is_some()
            || self.filler.is_some()
            || self.production_num.is_some()
            || self.episode_title.is_some()
            || self.episode_num.is_some()
            || self.year_of_production.is_some()
            || self.avi_society_code.is_some()
            || self.audio_visual_number.is_some()
            || self.v_isan_isan.is_some()
            || self.v_isan_episode.is_some()
            || self.v_isan_check_digit_1.is_some()
            || self.v_isan_version.is_some()
            || self.v_isan_check_digit_2.is_some()
            || self.eidr.is_some()
            || self.eidr_check_digit.is_some()
        {
            fields.push(format!("{:60}", self.production_title.as_deref().unwrap_or("")));
            fields.push(format!("{:15}", self.cd_identifier.as_deref().unwrap_or("")));
            fields.push(format!("{:4}", self.cut_number.as_deref().unwrap_or("")));

            // v2.1+ fields
            if self.library.is_some()
                || self.bltvr.is_some()
                || self.filler.is_some()
                || self.production_num.is_some()
                || self.episode_title.is_some()
                || self.episode_num.is_some()
                || self.year_of_production.is_some()
                || self.avi_society_code.is_some()
                || self.audio_visual_number.is_some()
                || self.v_isan_isan.is_some()
                || self.v_isan_episode.is_some()
                || self.v_isan_check_digit_1.is_some()
                || self.v_isan_version.is_some()
                || self.v_isan_check_digit_2.is_some()
                || self.eidr.is_some()
                || self.eidr_check_digit.is_some()
            {
                fields.push(format!("{:60}", self.library.as_deref().unwrap_or("")));
                fields.push(format!("{:1}", self.bltvr.as_deref().unwrap_or("")));
                fields.push(format!("{:25}", self.filler.as_deref().unwrap_or("")));
                fields.push(format!("{:12}", self.production_num.as_deref().unwrap_or("")));
                fields.push(format!("{:60}", self.episode_title.as_deref().unwrap_or("")));
                fields.push(format!("{:20}", self.episode_num.as_deref().unwrap_or("")));
                fields.push(format!("{:4}", self.year_of_production.as_deref().unwrap_or("")));
                fields.push(format!("{:3}", self.avi_society_code.as_deref().unwrap_or("")));
                fields.push(format!("{:15}", self.audio_visual_number.as_deref().unwrap_or("")));

                // v2.2+ fields
                if self.v_isan_isan.is_some() || self.v_isan_episode.is_some() || self.v_isan_check_digit_1.is_some() || self.v_isan_version.is_some() || self.v_isan_check_digit_2.is_some() || self.eidr.is_some() || self.eidr_check_digit.is_some() {
                    fields.push(format!("{:12}", self.v_isan_isan.as_deref().unwrap_or("")));
                    fields.push(format!("{:4}", self.v_isan_episode.as_deref().unwrap_or("")));
                    fields.push(format!("{:1}", self.v_isan_check_digit_1.as_deref().unwrap_or("")));
                    fields.push(format!("{:8}", self.v_isan_version.as_deref().unwrap_or("")));
                    fields.push(format!("{:1}", self.v_isan_check_digit_2.as_deref().unwrap_or("")));
                    fields.push(format!("{:20}", self.eidr.as_deref().unwrap_or("")));
                    fields.push(format!("{:1}", self.eidr_check_digit.as_deref().unwrap_or("")));
                }
            }
        }

        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orn_creation() {
        let orn = OrnRecord::new("00000001".to_string(), "00000001".to_string(), "LIB".to_string());

        assert_eq!(orn.record_type, "ORN");
        assert_eq!(orn.intended_purpose, "LIB");
    }

    #[test]
    fn test_orn_round_trip() {
        let original = OrnRecord::new("00000001".to_string(), "00000001".to_string(), "LIB".to_string());

        let line = original.to_cwr_line();
        let parsed = OrnRecord::from_cwr_line(&line).unwrap();

        assert_eq!(original, parsed);
        assert_eq!(line.len(), 22);
    }
}
