//! HDR - Transmission Header Record
//!
//! The Transmission Header record contains information about the sender and the transmission itself.

use crate::validators::{date_yyyymmdd, one_of};
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

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


impl_cwr_parsing! {
    HdrRecord {
        record_type: (0, 3, required, one_of(&["HDR"])),
        sender_type: (3, 5, required),
        sender_id: (5, 14, required),
        sender_name: (14, 59, required),
        edi_standard_version_number: (59, 64, required),
        creation_date: (64, 72, required, date_yyyymmdd),
        creation_time: (72, 78, required),
        transmission_date: (78, 86, required, date_yyyymmdd),
        character_set: (86, 101, optional),
        version: (101, 104, optional),
        revision: (104, 107, optional),
        software_package: (107, 137, optional),
        software_package_version: (137, 167, optional),
    }
    with_tests ["HDR01BMI      BMI MUSIC                                    01.1020050101120000200501010123456789012345  2. 1DEV MUSIC SOFTWARE VERSION 1.0  MUSIC PACKAGE VERSION 2.0  "]
}

