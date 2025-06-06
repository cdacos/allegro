//! ORN - Work Origin Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// ORN - Work Origin Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
pub struct OrnRecord {
    #[cwr(title = "Always 'ORN'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Intended purpose", start = 19, len = 3)]
    pub intended_purpose: String,

    #[cwr(title = "Production title (60 chars, conditional)", start = 22, len = 60)]
    pub production_title: Option<String>,

    #[cwr(title = "CD identifier (15 chars, conditional)", start = 82, len = 15)]
    pub cd_identifier: Option<String>,

    #[cwr(title = "Cut number (4 chars, optional)", start = 97, len = 4)]
    pub cut_number: Option<String>,

    #[cwr(title = "Library (60 chars, conditional, v2.1+)", start = 101, len = 60)]
    pub library: Option<String>,

    #[cwr(title = "BLTVR (1 char, optional, v2.1+)", start = 161, len = 1)]
    pub bltvr: Option<String>,

    #[cwr(title = "Filler (25 chars, optional, v2.1+)", start = 162, len = 25)]
    pub filler: Option<String>,

    #[cwr(title = "Production number (12 chars, optional, v2.1+)", start = 187, len = 12)]
    pub production_num: Option<String>,

    #[cwr(title = "Episode title (60 chars, optional, v2.1+)", start = 199, len = 60)]
    pub episode_title: Option<String>,

    #[cwr(title = "Episode number (20 chars, optional, v2.1+)", start = 259, len = 20)]
    pub episode_num: Option<String>,

    #[cwr(title = "Year of production (4 chars, optional, v2.1+)", start = 279, len = 4)]
    pub year_of_production: Option<String>,

    #[cwr(title = "AVI society code (3 chars, optional, v2.1+)", start = 283, len = 3)]
    pub avi_society_code: Option<String>,

    #[cwr(title = "Audio-visual number (15 chars, optional, v2.1+)", start = 286, len = 15)]
    pub audio_visual_number: Option<String>,

    #[cwr(title = "V-ISAN/ISAN (12 chars, optional, v2.2+)", start = 301, len = 12)]
    pub v_isan_isan: Option<String>,

    #[cwr(title = "V-ISAN/Episode (4 chars, optional, v2.2+)", start = 313, len = 4)]
    pub v_isan_episode: Option<String>,

    #[cwr(title = "V-ISAN/Check Digit 1 (1 char, optional, v2.2+)", start = 317, len = 1)]
    pub v_isan_check_digit_1: Option<String>,

    #[cwr(title = "V-ISAN/Version (8 chars, optional, v2.2+)", start = 318, len = 8)]
    pub v_isan_version: Option<String>,

    #[cwr(title = "V-ISAN/Check Digit 2 (1 char, optional, v2.2+)", start = 326, len = 1)]
    pub v_isan_check_digit_2: Option<String>,

    #[cwr(title = "EIDR (20 chars, optional, v2.2+)", start = 327, len = 20)]
    pub eidr: Option<String>,

    #[cwr(title = "EIDR/Check Digit (1 char, optional, v2.2+)", start = 347, len = 1)]
    pub eidr_check_digit: Option<String>,

}