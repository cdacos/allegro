//! ORN - Work Origin Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// ORN - Work Origin Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrnRecord {
    /// Always "ORN"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Intended purpose (3 chars)
    pub intended_purpose: String,

    /// Production title (60 chars, conditional)
    pub production_title: Option<String>,

    /// CD identifier (15 chars, conditional)
    pub cd_identifier: Option<String>,

    /// Cut number (4 chars, optional)
    pub cut_number: Option<String>,

    /// Library (60 chars, conditional, v2.1+)
    pub library: Option<String>,

    /// BLTVR (1 char, optional, v2.1+)
    pub bltvr: Option<String>,

    /// Filler (25 chars, optional, v2.1+)
    pub filler: Option<String>,

    /// Production number (12 chars, optional, v2.1+)
    pub production_num: Option<String>,

    /// Episode title (60 chars, optional, v2.1+)
    pub episode_title: Option<String>,

    /// Episode number (20 chars, optional, v2.1+)
    pub episode_num: Option<String>,

    /// Year of production (4 chars, optional, v2.1+)
    pub year_of_production: Option<String>,

    /// AVI society code (3 chars, optional, v2.1+)
    pub avi_society_code: Option<String>,

    /// Audio-visual number (15 chars, optional, v2.1+)
    pub audio_visual_number: Option<String>,

    /// V-ISAN/ISAN (12 chars, optional, v2.2+)
    pub v_isan_isan: Option<String>,

    /// V-ISAN/Episode (4 chars, optional, v2.2+)
    pub v_isan_episode: Option<String>,

    /// V-ISAN/Check Digit 1 (1 char, optional, v2.2+)
    pub v_isan_check_digit_1: Option<String>,

    /// V-ISAN/Version (8 chars, optional, v2.2+)
    pub v_isan_version: Option<String>,

    /// V-ISAN/Check Digit 2 (1 char, optional, v2.2+)
    pub v_isan_check_digit_2: Option<String>,

    /// EIDR (20 chars, optional, v2.2+)
    pub eidr: Option<String>,

    /// EIDR/Check Digit (1 char, optional, v2.2+)
    pub eidr_check_digit: Option<String>,
}


impl_cwr_parsing! {
    OrnRecord {
        record_type: (0, 3, required, one_of(&["ORN"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        intended_purpose: (19, 22, required),
        production_title: (22, 82, optional),
        cd_identifier: (82, 97, optional),
        cut_number: (97, 101, optional),
        library: (101, 161, optional),
        bltvr: (161, 162, optional),
        filler: (162, 187, optional),
        production_num: (187, 199, optional),
        episode_title: (199, 259, optional),
        episode_num: (259, 279, optional),
        year_of_production: (279, 283, optional),
        avi_society_code: (283, 286, optional),
        audio_visual_number: (286, 301, optional),
        v_isan_isan: (301, 313, optional),
        v_isan_episode: (313, 317, optional),
        v_isan_check_digit_1: (317, 318, optional),
        v_isan_version: (318, 326, optional),
        v_isan_check_digit_2: (326, 327, optional),
        eidr: (327, 347, optional),
        eidr_check_digit: (347, 348, optional),
    }
}
