//! NWR - New Work Registration Record
//!
//! Also handles REV (Revised Registration), ISW (ISWC Notification), and EXC (Existing Work in Conflict).

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// NWR - New Work Registration Record
///
/// Used for NWR, REV, ISW, and EXC record types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NwrRecord {
    /// "NWR", "REV", "ISW", or "EXC"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Work title (60 chars)
    pub work_title: String,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Submitter work number (14 chars)
    pub submitter_work_num: String,

    /// ISWC (11 chars, optional)
    pub iswc: Option<String>,

    /// Copyright date (8 chars, optional)
    pub copyright_date: Option<String>,

    /// Copyright number (12 chars, optional)
    pub copyright_number: Option<String>,

    /// Musical work distribution category (3 chars)
    pub musical_work_distribution_category: String,

    /// Duration HHMMSS (6 chars, conditional)
    pub duration: Option<String>,

    /// Recorded indicator (1 char)
    pub recorded_indicator: String,

    /// Text music relationship (3 chars, optional)
    pub text_music_relationship: Option<String>,

    /// Composite type (3 chars, optional)
    pub composite_type: Option<String>,

    /// Version type (3 chars)
    pub version_type: String,

    /// Excerpt type (3 chars, optional)
    pub excerpt_type: Option<String>,

    /// Music arrangement (3 chars, conditional)
    pub music_arrangement: Option<String>,

    /// Lyric adaptation (3 chars, conditional)
    pub lyric_adaptation: Option<String>,

    /// Contact name (30 chars, optional)
    pub contact_name: Option<String>,

    /// Contact ID (10 chars, optional)
    pub contact_id: Option<String>,

    /// CWR work type (2 chars, optional)
    pub cwr_work_type: Option<String>,

    /// Grand rights indicator (1 char, conditional)
    pub grand_rights_ind: Option<String>,

    /// Composite component count (3 chars, conditional)
    pub composite_component_count: Option<String>,

    /// Date of publication of printed edition (8 chars, optional)
    pub date_of_publication_of_printed_edition: Option<String>,

    /// Exceptional clause (1 char, optional)
    pub exceptional_clause: Option<String>,

    /// Opus number (25 chars, optional)
    pub opus_number: Option<String>,

    /// Catalogue number (25 chars, optional)
    pub catalogue_number: Option<String>,

    /// Priority flag (1 char, optional, v2.1+)
    pub priority_flag: Option<String>,
}


impl_cwr_parsing! {
    NwrRecord {
        record_type: (0, 3, required, one_of(&["NWR", "REV", "ISW", "EXC"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        work_title: (19, 79, required),
        language_code: (79, 81, optional),
        submitter_work_num: (81, 95, required),
        iswc: (95, 106, optional),
        copyright_date: (106, 114, optional),
        copyright_number: (114, 126, optional),
        musical_work_distribution_category: (126, 129, required),
        duration: (129, 135, optional),
        recorded_indicator: (135, 136, required),
        text_music_relationship: (136, 139, optional),
        composite_type: (139, 142, optional),
        version_type: (142, 145, required),
        excerpt_type: (145, 148, optional),
        music_arrangement: (148, 151, optional),
        lyric_adaptation: (151, 154, optional),
        contact_name: (154, 184, optional),
        contact_id: (184, 194, optional),
        cwr_work_type: (194, 196, optional),
        grand_rights_ind: (196, 197, optional),
        composite_component_count: (197, 200, optional),
        date_of_publication_of_printed_edition: (200, 208, optional),
        exceptional_clause: (208, 209, optional),
        opus_number: (209, 234, optional),
        catalogue_number: (234, 259, optional),
        priority_flag: (259, 260, optional),
    }
    with_test_data ["NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               "]
}

