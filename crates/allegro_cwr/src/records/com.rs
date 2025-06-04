//! COM - Composite Component Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// COM - Composite Component Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComRecord {
    /// Always "COM"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Title (60 chars)
    pub title: String,
    
    /// ISWC of component (11 chars, optional)
    pub iswc_of_component: Option<String>,
    
    /// Submitter work number (14 chars, optional)
    pub submitter_work_num: Option<String>,
    
    /// Duration HHMMSS (6 chars, optional)
    pub duration: Option<String>,
    
    /// Writer 1 last name (45 chars)
    pub writer_1_last_name: String,
    
    /// Writer 1 first name (30 chars, optional)
    pub writer_1_first_name: Option<String>,
    
    /// Writer 1 IPI name number (11 chars, optional)
    pub writer_1_ipi_name_num: Option<String>,
    
    /// Writer 2 last name (45 chars, optional)
    pub writer_2_last_name: Option<String>,
    
    /// Writer 2 first name (30 chars, optional)
    pub writer_2_first_name: Option<String>,
    
    /// Writer 2 IPI name number (11 chars, optional)
    pub writer_2_ipi_name_num: Option<String>,
    
    /// Writer 1 IPI base number (13 chars, optional)
    pub writer_1_ipi_base_number: Option<String>,
    
    /// Writer 2 IPI base number (13 chars, optional)
    pub writer_2_ipi_base_number: Option<String>,
}

impl ComRecord {
    /// Create a new COM record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
        title: String,
        writer_1_last_name: String,
    ) -> Self {
        Self {
            record_type: "COM".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            title,
            iswc_of_component: None,
            submitter_work_num: None,
            duration: None,
            writer_1_last_name,
            writer_1_first_name: None,
            writer_1_ipi_name_num: None,
            writer_2_last_name: None,
            writer_2_first_name: None,
            writer_2_ipi_name_num: None,
            writer_1_ipi_base_number: None,
            writer_2_ipi_base_number: None,
        }
    }
    
    /// Parse a CWR line into a COM record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 155 {
            return Err(CwrParseError::BadFormat("COM line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "COM" {
            return Err(CwrParseError::BadFormat(format!("Expected COM, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        let title = line.get(19..79).unwrap().trim().to_string();
        
        let iswc_of_component = line.get(79..90)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let submitter_work_num = line.get(90..104)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let duration = line.get(104..110)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let writer_1_last_name = line.get(110..155).unwrap().trim().to_string();
        
        let writer_1_first_name = if line.len() > 155 {
            line.get(155..185)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_1_ipi_name_num = if line.len() > 185 {
            line.get(185..196)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_2_last_name = if line.len() > 196 {
            line.get(196..241)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_2_first_name = if line.len() > 241 {
            line.get(241..271)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_2_ipi_name_num = if line.len() > 271 {
            line.get(271..282)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_1_ipi_base_number = if line.len() > 282 {
            line.get(282..295)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let writer_2_ipi_base_number = if line.len() > 295 {
            line.get(295..308)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(ComRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            title,
            iswc_of_component,
            submitter_work_num,
            duration,
            writer_1_last_name,
            writer_1_first_name,
            writer_1_ipi_name_num,
            writer_2_last_name,
            writer_2_first_name,
            writer_2_ipi_name_num,
            writer_1_ipi_base_number,
            writer_2_ipi_base_number,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:60}", self.title),
            format!("{:11}", self.iswc_of_component.as_deref().unwrap_or("")),
            format!("{:14}", self.submitter_work_num.as_deref().unwrap_or("")),
            format!("{:6}", self.duration.as_deref().unwrap_or("")),
            format!("{:45}", self.writer_1_last_name)
        ];
        
        if self.writer_1_first_name.is_some() || self.writer_1_ipi_name_num.is_some() ||
           self.writer_2_last_name.is_some() || self.writer_2_first_name.is_some() ||
           self.writer_2_ipi_name_num.is_some() || self.writer_1_ipi_base_number.is_some() ||
           self.writer_2_ipi_base_number.is_some() {
            fields.push(format!("{:30}", self.writer_1_first_name.as_deref().unwrap_or("")));
            fields.push(format!("{:11}", self.writer_1_ipi_name_num.as_deref().unwrap_or("")));
            fields.push(format!("{:45}", self.writer_2_last_name.as_deref().unwrap_or("")));
            fields.push(format!("{:30}", self.writer_2_first_name.as_deref().unwrap_or("")));
            fields.push(format!("{:11}", self.writer_2_ipi_name_num.as_deref().unwrap_or("")));
            fields.push(format!("{:13}", self.writer_1_ipi_base_number.as_deref().unwrap_or("")));
            fields.push(format!("{:13}", self.writer_2_ipi_base_number.as_deref().unwrap_or("")));
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_creation() {
        let com = ComRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Component Title".to_string(),
            "COMPOSER".to_string(),
        );
        
        assert_eq!(com.record_type, "COM");
        assert_eq!(com.title, "Component Title");
        assert_eq!(com.writer_1_last_name, "COMPOSER");
    }
    
    #[test]
    fn test_com_round_trip() {
        let original = ComRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
            "Component Title".to_string(),
            "COMPOSER".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = ComRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 155);
    }
}