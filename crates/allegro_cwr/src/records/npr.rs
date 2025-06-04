//! NPR - Non-Roman Alphabet Performing Artist Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// NPR - Non-Roman Alphabet Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NprRecord {
    /// Always "NPR"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Performing artist name (160 chars, conditional)
    pub performing_artist_name: Option<String>,
    
    /// Performing artist first name (160 chars, optional)
    pub performing_artist_first_name: Option<String>,
    
    /// Performing artist IPI name number (11 chars, optional)
    pub performing_artist_ipi_name_num: Option<String>,
    
    /// Performing artist IPI base number (13 chars, optional)
    pub performing_artist_ipi_base_number: Option<String>,
    
    /// Language code (2 chars, optional)
    pub language_code: Option<String>,
    
    /// Performance language (2 chars, conditional, v2.1+)
    pub performance_language: Option<String>,
    
    /// Performance dialect (3 chars, conditional, v2.1+)
    pub performance_dialect: Option<String>,
}

impl NprRecord {
    /// Create a new NPR record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
    ) -> Self {
        Self {
            record_type: "NPR".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            performing_artist_name: None,
            performing_artist_first_name: None,
            performing_artist_ipi_name_num: None,
            performing_artist_ipi_base_number: None,
            language_code: None,
            performance_language: None,
            performance_dialect: None,
        }
    }
    
    /// Parse a CWR line into a NPR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 19 {
            return Err(CwrParseError::BadFormat("NPR line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "NPR" {
            return Err(CwrParseError::BadFormat(format!("Expected NPR, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        
        let performing_artist_name = if line.len() > 19 {
            line.get(19..179)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let performing_artist_first_name = if line.len() > 179 {
            line.get(179..339)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let performing_artist_ipi_name_num = if line.len() > 339 {
            line.get(339..350)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let performing_artist_ipi_base_number = if line.len() > 350 {
            line.get(350..363)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let language_code = if line.len() > 363 {
            line.get(363..365)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let performance_language = if line.len() > 365 {
            line.get(365..367)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let performance_dialect = if line.len() > 367 {
            line.get(367..370)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(NprRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            performing_artist_name,
            performing_artist_first_name,
            performing_artist_ipi_name_num,
            performing_artist_ipi_base_number,
            language_code,
            performance_language,
            performance_dialect,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num)
        ];
        
        if self.performing_artist_name.is_some() || self.performing_artist_first_name.is_some() ||
           self.performing_artist_ipi_name_num.is_some() || self.performing_artist_ipi_base_number.is_some() ||
           self.language_code.is_some() || self.performance_language.is_some() ||
           self.performance_dialect.is_some() {
            
            fields.push(format!("{:160}", self.performing_artist_name.as_deref().unwrap_or("")));
            fields.push(format!("{:160}", self.performing_artist_first_name.as_deref().unwrap_or("")));
            fields.push(format!("{:11}", self.performing_artist_ipi_name_num.as_deref().unwrap_or("")));
            fields.push(format!("{:13}", self.performing_artist_ipi_base_number.as_deref().unwrap_or("")));
            fields.push(format!("{:2}", self.language_code.as_deref().unwrap_or("")));
            
            // v2.1+ fields
            if self.performance_language.is_some() || self.performance_dialect.is_some() {
                fields.push(format!("{:2}", self.performance_language.as_deref().unwrap_or("")));
                fields.push(format!("{:3}", self.performance_dialect.as_deref().unwrap_or("")));
            }
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npr_creation() {
        let npr = NprRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
        );
        
        assert_eq!(npr.record_type, "NPR");
    }
    
    #[test]
    fn test_npr_round_trip() {
        let original = NprRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = NprRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 19);
    }
}