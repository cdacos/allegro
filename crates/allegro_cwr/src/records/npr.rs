//! NPR - Non-Roman Alphabet Performing Artist Record

use crate::validators::one_of;
use crate::{impl_cwr_parsing, impl_cwr_parsing_test_roundtrip};
use serde::{Deserialize, Serialize};

/// NPR - Non-Roman Alphabet Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NprRecord {
    /// Always "NPR"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Performing artist name (160 chars, conditional)
    pub performing_artist_name: Option<String>,

    /// Performing artist first name (160 chars, optional)
    pub performing_artist_first_name: Option<String>,

    /// Performing artist IPI name number (11 chars, optional)
    pub performing_artist_ipi_name_num: Option<String>,

    /// Performing artist IPI base number (13 chars, optional)
    pub performing_artist_ipi_base_number: Option<String>,

    /// Language code (2 chars, optional)
    pub language_code: Option<String>,

    /// Performance language (2 chars, conditional, v2.1+)
    pub performance_language: Option<String>,

    /// Performance dialect (3 chars, conditional, v2.1+)
    pub performance_dialect: Option<String>,
}


impl_cwr_parsing! {
    NprRecord {
        record_type: (0, 3, required, one_of(&["NPR"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        performing_artist_name: (19, 179, optional),
        performing_artist_first_name: (179, 339, optional),
        performing_artist_ipi_name_num: (339, 350, optional),
        performing_artist_ipi_base_number: (350, 363, optional),
        language_code: (363, 365, optional),
        performance_language: (365, 367, optional),
        performance_dialect: (367, 370, optional),
    }
}

impl_cwr_parsing_test_roundtrip!(
    NprRecord,
    [
        "NPR0000000100000001ARTIST NAME                                                                                                                                                             FIRST NAME                                                                                                                                                              01234567890123456789012ENFRGER"
    ]
);
