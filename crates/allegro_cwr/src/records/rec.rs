//! REC - Recording Detail Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

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


impl_cwr_parsing! {
    RecRecord {
        record_type: (0, 3, required, one_of(&["REC"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        release_date: (19, 27, optional),
        constant: (27, 87, required),
        release_duration: (87, 93, optional),
        constant2: (93, 98, required),
        album_title: (98, 158, optional),
        album_label: (158, 218, optional),
        release_catalog_num: (218, 236, optional),
        ean: (236, 249, optional),
        isrc: (249, 261, optional),
        recording_format: (261, 262, optional),
        recording_technique: (262, 263, optional),
        media_type: (263, 266, optional),
        recording_title: (266, 326, optional),
        version_title: (326, 386, optional),
        display_artist: (386, 446, optional),
        record_label: (446, 506, optional),
        isrc_validity: (506, 526, optional),
        submitter_recording_identifier: (526, 540, optional),
    }
}
