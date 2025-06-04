//! REC - Recording Detail Record

use serde::{Deserialize, Serialize};
use crate::error::CwrParseError;

/// REC - Recording Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecRecord {
    /// Always "REC"
    pub record_type: String,
    
    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,
    
    /// Record sequence number (8 chars)
    pub record_sequence_num: String,
    
    /// Release date YYYYMMDD (8 chars, optional)
    pub release_date: Option<String>,
    
    /// Constant - spaces (60 chars)
    pub constant: String,
    
    /// Release duration HHMMSS (6 chars, optional)
    pub release_duration: Option<String>,
    
    /// Constant - spaces (5 chars)
    pub constant2: String,
    
    /// Album title (60 chars, optional)
    pub album_title: Option<String>,
    
    /// Album label (60 chars, optional)
    pub album_label: Option<String>,
    
    /// Release catalog number (18 chars, optional)
    pub release_catalog_num: Option<String>,
    
    /// EAN (13 chars, optional)
    pub ean: Option<String>,
    
    /// ISRC (12 chars, optional)
    pub isrc: Option<String>,
    
    /// Recording format (1 char, optional)
    pub recording_format: Option<String>,
    
    /// Recording technique (1 char, optional)
    pub recording_technique: Option<String>,
    
    /// Media type (3 chars, optional, v2.1+)
    pub media_type: Option<String>,
    
    /// Recording title (60 chars, optional, v2.2+)
    pub recording_title: Option<String>,
    
    /// Version title (60 chars, optional, v2.2+)
    pub version_title: Option<String>,
    
    /// Display artist (60 chars, optional, v2.2+)
    pub display_artist: Option<String>,
    
    /// Record label (60 chars, optional, v2.2+)
    pub record_label: Option<String>,
    
    /// ISRC validity (20 chars, conditional, v2.2+)
    pub isrc_validity: Option<String>,
    
    /// Submitter recording identifier (14 chars, optional, v2.2+)
    pub submitter_recording_identifier: Option<String>,
}

impl RecRecord {
    /// Create a new REC record
    pub fn new(
        transaction_sequence_num: String,
        record_sequence_num: String,
    ) -> Self {
        Self {
            record_type: "REC".to_string(),
            transaction_sequence_num,
            record_sequence_num,
            release_date: None,
            constant: " ".repeat(60),
            release_duration: None,
            constant2: " ".repeat(5),
            album_title: None,
            album_label: None,
            release_catalog_num: None,
            ean: None,
            isrc: None,
            recording_format: None,
            recording_technique: None,
            media_type: None,
            recording_title: None,
            version_title: None,
            display_artist: None,
            record_label: None,
            isrc_validity: None,
            submitter_recording_identifier: None,
        }
    }
    
    /// Parse a CWR line into a REC record
    pub fn from_cwr_line(line: &str) -> Result<Self, CwrParseError> {
        if line.len() < 263 {
            return Err(CwrParseError::BadFormat("REC line too short".to_string()));
        }
        
        let record_type = line.get(0..3).unwrap().to_string();
        if record_type != "REC" {
            return Err(CwrParseError::BadFormat(format!("Expected REC, found {}", record_type)));
        }
        
        let transaction_sequence_num = line.get(3..11).unwrap().trim().to_string();
        let record_sequence_num = line.get(11..19).unwrap().trim().to_string();
        
        let release_date = line.get(19..27)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let constant = line.get(27..87).unwrap().to_string();
        
        let release_duration = line.get(87..93)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let constant2 = line.get(93..98).unwrap().to_string();
        
        let album_title = line.get(98..158)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let album_label = line.get(158..218)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let release_catalog_num = line.get(218..236)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let ean = line.get(236..249)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let isrc = line.get(249..261)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let recording_format = line.get(261..262)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        let recording_technique = line.get(262..263)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
            
        // v2.1+ fields
        let media_type = if line.len() > 263 {
            line.get(263..266)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        // v2.2+ fields
        let recording_title = if line.len() > 266 {
            line.get(266..326)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let version_title = if line.len() > 326 {
            line.get(326..386)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let display_artist = if line.len() > 386 {
            line.get(386..446)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let record_label = if line.len() > 446 {
            line.get(446..506)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let isrc_validity = if line.len() > 506 {
            line.get(506..526)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        let submitter_recording_identifier = if line.len() > 526 {
            line.get(526..540)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        } else {
            None
        };
        
        Ok(RecRecord {
            record_type,
            transaction_sequence_num,
            record_sequence_num,
            release_date,
            constant,
            release_duration,
            constant2,
            album_title,
            album_label,
            release_catalog_num,
            ean,
            isrc,
            recording_format,
            recording_technique,
            media_type,
            recording_title,
            version_title,
            display_artist,
            record_label,
            isrc_validity,
            submitter_recording_identifier,
        })
    }

    /// Convert this record to a CWR format line
    pub fn to_cwr_line(&self) -> String {
        let mut fields = vec![
            format!("{:3}", self.record_type),
            format!("{:8}", self.transaction_sequence_num),
            format!("{:8}", self.record_sequence_num),
            format!("{:8}", self.release_date.as_deref().unwrap_or("")),
            format!("{:60}", self.constant),
            format!("{:6}", self.release_duration.as_deref().unwrap_or("")),
            format!("{:5}", self.constant2),
            format!("{:60}", self.album_title.as_deref().unwrap_or("")),
            format!("{:60}", self.album_label.as_deref().unwrap_or("")),
            format!("{:18}", self.release_catalog_num.as_deref().unwrap_or("")),
            format!("{:13}", self.ean.as_deref().unwrap_or("")),
            format!("{:12}", self.isrc.as_deref().unwrap_or("")),
            format!("{:1}", self.recording_format.as_deref().unwrap_or("")),
            format!("{:1}", self.recording_technique.as_deref().unwrap_or(""))
        ];
        
        // Add v2.1+ field
        if let Some(ref media) = self.media_type {
            fields.push(format!("{:3}", media));
        }
        
        // Add v2.2+ fields
        if let Some(ref recording) = self.recording_title {
            fields.push(format!("{:60}", recording));
        }
        
        if let Some(ref version) = self.version_title {
            fields.push(format!("{:60}", version));
        }
        
        if let Some(ref display) = self.display_artist {
            fields.push(format!("{:60}", display));
        }
        
        if let Some(ref label) = self.record_label {
            fields.push(format!("{:60}", label));
        }
        
        if let Some(ref validity) = self.isrc_validity {
            fields.push(format!("{:20}", validity));
        }
        
        if let Some(ref identifier) = self.submitter_recording_identifier {
            fields.push(format!("{:14}", identifier));
        }
        
        fields.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rec_creation() {
        let rec = RecRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
        );
        
        assert_eq!(rec.record_type, "REC");
    }
    
    #[test]
    fn test_rec_round_trip() {
        let original = RecRecord::new(
            "00000001".to_string(),
            "00000001".to_string(),
        );
        
        let line = original.to_cwr_line();
        let parsed = RecRecord::from_cwr_line(&line).unwrap();
        
        assert_eq!(original, parsed);
        assert_eq!(line.len(), 263);
    }
}