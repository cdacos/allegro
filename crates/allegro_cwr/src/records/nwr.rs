use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Used for NWR, REV, ISW, and EXC record types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["NWR", "REV", "ISW", "EXC"], test_data = "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ")]
pub struct NwrRecord {
    #[cwr(title = "'NWR', 'REV', 'ISW', or 'EXC'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Work title", start = 19, len = 60)]
    pub work_title: String,

    #[cwr(title = "Language code (optional)", start = 79, len = 2)]
    pub language_code: Option<String>,

    #[cwr(title = "Submitter work number", start = 81, len = 14)]
    pub submitter_work_num: String,

    #[cwr(title = "ISWC (optional)", start = 95, len = 11)]
    pub iswc: Option<String>,

    #[cwr(title = "Copyright date (optional)", start = 106, len = 8)]
    pub copyright_date: Option<String>,

    #[cwr(title = "Copyright number (optional)", start = 114, len = 12)]
    pub copyright_number: Option<String>,

    #[cwr(title = "Musical work distribution category", start = 126, len = 3)]
    pub musical_work_distribution_category: String,

    #[cwr(title = "Duration HHMMSS (conditional)", start = 129, len = 6)]
    pub duration: Option<String>,

    #[cwr(title = "Recorded indicator (1 char)", start = 135, len = 1)]
    pub recorded_indicator: String,

    #[cwr(title = "Text music relationship (optional)", start = 136, len = 3)]
    pub text_music_relationship: Option<String>,

    #[cwr(title = "Composite type (optional)", start = 139, len = 3)]
    pub composite_type: Option<String>,

    #[cwr(title = "Version type", start = 142, len = 3)]
    pub version_type: String,

    #[cwr(title = "Excerpt type (optional)", start = 145, len = 3)]
    pub excerpt_type: Option<String>,

    #[cwr(title = "Music arrangement (conditional)", start = 148, len = 3)]
    pub music_arrangement: Option<String>,

    #[cwr(title = "Lyric adaptation (conditional)", start = 151, len = 3)]
    pub lyric_adaptation: Option<String>,

    #[cwr(title = "Contact name (optional)", start = 154, len = 30)]
    pub contact_name: Option<String>,

    #[cwr(title = "Contact ID (optional)", start = 184, len = 10)]
    pub contact_id: Option<String>,

    #[cwr(title = "CWR work type (optional)", start = 194, len = 2)]
    pub cwr_work_type: Option<String>,

    #[cwr(title = "Grand rights indicator (1 char, conditional)", start = 196, len = 1)]
    pub grand_rights_ind: Option<String>,

    #[cwr(title = "Composite component count (conditional)", start = 197, len = 3)]
    pub composite_component_count: Option<String>,

    #[cwr(title = "Date of publication of printed edition (optional)", start = 200, len = 8)]
    pub date_of_publication_of_printed_edition: Option<String>,

    #[cwr(title = "Exceptional clause (1 char, optional)", start = 208, len = 1)]
    pub exceptional_clause: Option<String>,

    #[cwr(title = "Opus number (optional)", start = 209, len = 25)]
    pub opus_number: Option<String>,

    #[cwr(title = "Catalogue number (optional)", start = 234, len = 25)]
    pub catalogue_number: Option<String>,

    #[cwr(title = "Priority flag (1 char, optional, v2.1+)", start = 259, len = 1)]
    pub priority_flag: Option<String>,
}
