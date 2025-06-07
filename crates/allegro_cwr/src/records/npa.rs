use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// NPA - Non-Roman Alphabet Publisher Name Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(
    test_data = "NPA000000010000000212345678 PLACEHOLDER INTERESTED PARTY NAME                                                                                                                               PLACEHOLDER FIRST NAME                                                                                                                                          EN"
)]
pub struct NpaRecord {
    #[cwr(title = "Always 'NPA'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Interested party number (conditional)", start = 19, len = 9)]
    pub interested_party_num: Option<String>,

    #[cwr(title = "Interested party name", start = 28, len = 160)]
    pub interested_party_name: String,

    #[cwr(title = "Interested party writer first name", start = 188, len = 160)]
    pub interested_party_writer_first_name: String,

    #[cwr(title = "Language code (optional)", start = 348, len = 2)]
    pub language_code: Option<String>,
}
