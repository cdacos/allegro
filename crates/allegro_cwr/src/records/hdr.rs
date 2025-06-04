//! HDR - Transmission Header Record
//! 
//! The Transmission Header record contains information about the sender and the transmission itself.

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// HDR - Transmission Header Record
/// 
/// Contains information about the sender and the transmission itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HdrRecord {
    /// Always "HDR"
    pub record_type: String,
    
    /// Sender type (2 chars)
    pub sender_type: String,
    
    /// Sender ID (9 chars) 
    pub sender_id: String,
    
    /// Sender name (45 chars)
    pub sender_name: String,
    
    /// EDI standard version number (5 chars)
    pub edi_standard_version_number: String,
    
    /// Creation date YYYYMMDD (8 chars)
    pub creation_date: String,
    
    /// Creation time HHMMSS (6 chars)
    pub creation_time: String,
    
    /// Transmission date YYYYMMDD (8 chars)
    pub transmission_date: String,
    
    /// Character set (15 chars, v2.1+)
    pub character_set: Option<String>,
    
    /// Version (3 chars, v2.2+)
    pub version: Option<String>,
    
    /// Revision (3 chars, v2.2+)
    pub revision: Option<String>,
    
    /// Software package (30 chars, v2.2+)
    pub software_package: Option<String>,
    
    /// Software package version (30 chars, v2.2+)
    pub software_package_version: Option<String>,
}

impl HdrRecord {
    /// Create a new HDR record with required fields
    pub fn new(
        sender_type: String,
        sender_id: String,
        sender_name: String,
        edi_standard_version_number: String,
        creation_date: String,
        creation_time: String,
        transmission_date: String,
    ) -> Self {
        Self {
            record_type: "HDR".to_string(),
            sender_type,
            sender_id,
            sender_name,
            edi_standard_version_number,
            creation_date,
            creation_time,
            transmission_date,
            character_set: None,
            version: None,
            revision: None,
            software_package: None,
            software_package_version: None,
        }
    }
    
    /// Parse a CWR line into an HDR record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 86 {
            return Err(CwrParseError::BadFormat("HDR line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "HDR" {
            return Err(CwrParseError::BadFormat(format!("Expected HDR, found {}", record_type)));
        }
        
        let sender_type = line.get(3..5).unwrap().trim().to_string();
        let sender_id = line.get(5..14).unwrap().trim().to_string();
        let sender_name = line.get(14..59).unwrap().trim().to_string();
        let edi_standard_version_number = line.get(59..64).unwrap().trim().to_string();
        let creation_date = line.get(64..72).unwrap().trim().to_string();
        let creation_time = line.get(72..78).unwrap().trim().to_string();
        let transmission_date = line.get(78..86).unwrap().trim().to_string();
        
        let character_set = if line.len() > 86 {
            line.get(86..101)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let version = if line.len() > 101 {
            line.get(101..104)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let revision = if line.len() > 104 {
            line.get(104..107)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let software_package = if line.len() > 107 {
            line.get(107..137)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let software_package_version = if line.len() > 137 {
            line.get(137..167)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(HdrRecord {
            record_type,
            sender_type,
            sender_id,
            sender_name,
            edi_standard_version_number,
            creation_date,
            creation_time,
            transmission_date,
            character_set,
            version,
            revision,
            software_package,
            software_package_version,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:2}", self.sender_type),
            format!("{:9}", self.sender_id),
            format!("{:45}", self.sender_name),
            format!("{:5}", self.edi_standard_version_number),
            format!("{:8}", self.creation_date),
            format!("{:6}", self.creation_time),
            format!("{:8}", self.transmission_date),
        ];
        
        // Add optional v2.1+ fields
        if self.character_set.is_some() || self.version.is_some() || 
           self.revision.is_some() || self.software_package.is_some() || 
           self.software_package_version.is_some() {
            fields.push(format!("{:15}", self.character_set.as_deref().unwrap_or("")));
        }
        
        // Add optional v2.2+ fields
        if self.version.is_some() || self.revision.is_some() || 
           self.software_package.is_some() || self.software_package_version.is_some() {
            fields.push(format!("{:3}", self.version.as_deref().unwrap_or("")));
        }
        
        if self.revision.is_some() || self.software_package.is_some() || 
           self.software_package_version.is_some() {
            fields.push(format!("{:3}", self.revision.as_deref().unwrap_or("")));
        }
        
        if self.software_package.is_some() || self.software_package_version.is_some() {
            fields.push(format!("{:30}", self.software_package.as_deref().unwrap_or("")));
        }
        
        if self.software_package_version.is_some() {
            fields.push(format!("{:30}", self.software_package_version.as_deref().unwrap_or("")));
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdr_creation() {
        let hdr = HdrRecord::new(
            "01".to_string(),
            "BMI".to_string(),
            "BMI".to_string(),
            "01.10".to_string(),
            "20050101".to_string(),
            "120000".to_string(),
            "20050101".to_string(),
        );
        
        assert_eq!(hdr.record_type, "HDR");
        assert_eq!(hdr.sender_type, "01");
        assert_eq!(hdr.sender_id, "BMI");
        assert_eq!(hdr.creation_date, "20050101");
    }
    
    #[test]
    fn test_hdr_to_cwr_line() {
        let hdr = HdrRecord::new(
            "01".to_string(),
            "BMI".to_string(),
            "BMI".to_string(),
            "01.10".to_string(),
            "20050101".to_string(),
            "120000".to_string(),
            "20050101".to_string(),
        );
        
        let line = hdr.to_cwr_line();
        assert_eq!(line.len(), 86);  // Basic HDR length
        assert!(line.starts_with("HDR01BMI"));
    }
    
    #[test]
    fn test_hdr_round_trip() {
        let original = HdrRecord::new(
            "01".to_string(),
            "BMI".to_string(),
            "BMI MUSIC".to_string(),
            "01.10".to_string(),
            "20050101".to_string(),
            "120000".to_string(),
            "20050101".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = HdrRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
    }
}