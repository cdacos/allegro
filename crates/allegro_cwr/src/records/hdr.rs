//! /// Contains information about the sender and the transmission itself.

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// /// Contains information about the sender and the transmission itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "HDR01BMI      BMI MUSIC                                    01.1020050101120000200501010123456789012345  2. 1DEV MUSIC SOFTWARE VERSION 1.0  MUSIC PACKAGE VERSION 2.0  ")]
pub struct HdrRecord {
    #[cwr(title = "Always 'HDR'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Sender type", start = 3, len = 2)]
    pub sender_type: String,

    #[cwr(title = "Sender ID", start = 5, len = 9)]
    pub sender_id: String,

    #[cwr(title = "Sender name", start = 14, len = 45)]
    pub sender_name: String,

    #[cwr(title = "EDI standard version number", start = 59, len = 5)]
    pub edi_standard_version_number: String,

    #[cwr(title = "Creation date YYYYMMDD", start = 64, len = 8)]
    pub creation_date: Date,

    #[cwr(title = "Creation time HHMMSS", start = 72, len = 6)]
    pub creation_time: String,

    #[cwr(title = "Transmission date YYYYMMDD", start = 78, len = 8)]
    pub transmission_date: Date,

    #[cwr(title = "Character set (v2.1+)", start = 86, len = 15)]
    pub character_set: Option<String>,

    #[cwr(title = "Version (v2.2+)", start = 101, len = 3)]
    pub version: Option<String>,

    #[cwr(title = "Revision (v2.2+)", start = 104, len = 3)]
    pub revision: Option<String>,

    #[cwr(title = "Software package (v2.2+)", start = 107, len = 30)]
    pub software_package: Option<String>,

    #[cwr(title = "Software package version (v2.2+)", start = 137, len = 30)]
    pub software_package_version: Option<String>,

}