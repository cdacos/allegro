use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// REC - Recording Detail Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    validator = rec_custom_validate,
    test_data = "REC000000000000002720191004                                                            000306     WASTED ON YOU - SINGLE                                      INDEPENDENT                                                                                                                                                                                                                                                                                                                                                                                                        "
)]
pub struct RecRecord {
    #[cwr(title = "Always 'REC'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Release date YYYYMMDD (optional)", start = 19, len = 8)]
    pub release_date: Option<Date>,

    #[cwr(title = "Constant - spaces", start = 27, len = 60)]
    pub constant: String,

    #[cwr(title = "Release duration HHMMSS (optional)", start = 87, len = 6)]
    pub release_duration: Option<Time>,

    #[cwr(title = "Constant - spaces", start = 93, len = 5)]
    pub constant2: String,

    #[cwr(title = "Album title (optional)", start = 98, len = 60)]
    pub album_title: Option<String>,

    #[cwr(title = "Album label (optional)", start = 158, len = 60)]
    pub album_label: Option<String>,

    #[cwr(title = "Release catalog number (optional)", start = 218, len = 18)]
    pub release_catalog_num: Option<String>,

    #[cwr(title = "EAN (optional)", start = 236, len = 13)]
    pub ean: Option<Ean>,

    #[cwr(title = "ISRC (optional)", start = 249, len = 12)]
    pub isrc: Option<Isrc>,

    #[cwr(title = "Recording format (1 char, optional)", start = 261, len = 1)]
    pub recording_format: Option<RecordingFormat>,

    #[cwr(title = "Recording technique (1 char, optional)", start = 262, len = 1)]
    pub recording_technique: Option<RecordingTechnique>,

    #[cwr(title = "Media type (optional, v2.1+)", start = 263, len = 3, min_version = 2.1)]
    pub media_type: Option<MediaType>,

    #[cwr(title = "Recording title (optional, v2.2+)", start = 266, len = 60, min_version = 2.2)]
    pub recording_title: Option<String>,

    #[cwr(title = "Version title (optional, v2.2+)", start = 326, len = 60, min_version = 2.2)]
    pub version_title: Option<String>,

    #[cwr(title = "Display artist (optional, v2.2+)", start = 386, len = 60, min_version = 2.2)]
    pub display_artist: Option<String>,

    #[cwr(title = "Record label (optional, v2.2+)", start = 446, len = 60, min_version = 2.2)]
    pub record_label: Option<String>,

    #[cwr(title = "ISRC validity (conditional, v2.2+)", start = 506, len = 20, min_version = 2.2)]
    pub isrc_validity: Option<IsrcValidityIndicator>,

    #[cwr(title = "Submitter recording identifier (optional, v2.2+)", start = 526, len = 14, min_version = 2.2)]
    pub submitter_recording_identifier: Option<String>,
}

// Custom validation function for REC record
fn rec_custom_validate(record: &mut RecRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Release date should not be in the future
    if let Some(ref release_date) = record.release_date {
        let current_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        if release_date.to_timestamp() > current_timestamp {
            warnings.push(CwrWarning { field_name: "release_date", field_title: "Release date YYYYMMDD (optional)", source_str: std::borrow::Cow::Owned(release_date.as_str()), level: WarningLevel::Warning, description: format!("Release date {} is in the future", release_date.as_str()) });
        }
    }

    // TODO: Additional business rules requiring broader context:
    // - Must follow a NWR/REV record (requires parsing context)

    warnings
}
