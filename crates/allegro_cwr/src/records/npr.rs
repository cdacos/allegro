//! NPR - Non-Roman Alphabet Performing Artist Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPR - Non-Roman Alphabet Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "NPR0000000100000002PLACEHOLDER PERFORMING ARTIST                                                                                                                                   PLACEHOLDER FIRST NAME                                                                                                                                          12345678901123456789012ENENABC ")]
pub struct NprRecord {
    #[cwr(title = "Always 'NPR'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Performing artist name (conditional)", start = 19, len = 160)]
    pub performing_artist_name: Option<String>,

    #[cwr(title = "Performing artist first name (optional)", start = 179, len = 160)]
    pub performing_artist_first_name: Option<String>,

    #[cwr(title = "Performing artist IPI name number (optional)", start = 339, len = 11)]
    pub performing_artist_ipi_name_num: Option<String>,

    #[cwr(title = "Performing artist IPI base number (optional)", start = 350, len = 13)]
    pub performing_artist_ipi_base_number: Option<String>,

    #[cwr(title = "Language code (optional)", start = 363, len = 2)]
    pub language_code: Option<String>,

    #[cwr(title = "Performance language (conditional, v2.1+)", start = 365, len = 2)]
    pub performance_language: Option<String>,

    #[cwr(title = "Performance dialect (conditional, v2.1+)", start = 367, len = 3)]
    pub performance_dialect: Option<String>,
}
