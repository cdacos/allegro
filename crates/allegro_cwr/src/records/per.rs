//! PER - Performing Artist Record

use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// PER - Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "PER0000050400000429DEVVON TERRELL                                                                                     ")]
pub struct PerRecord {
    #[cwr(title = "Always 'PER'", start = 0, len = 3)]
    pub record_type: RecordType,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: String,

    #[cwr(title = "Performing artist last name", start = 19, len = 45)]
    pub performing_artist_last_name: String,

    #[cwr(title = "Performing artist first name (optional)", start = 64, len = 30)]
    pub performing_artist_first_name: Option<String>,

    #[cwr(title = "Performing artist IPI name number (optional)", start = 94, len = 11)]
    pub performing_artist_ipi_name_num: Option<String>,

    #[cwr(title = "Performing artist IPI base number (optional)", start = 105, len = 13)]
    pub performing_artist_ipi_base_number: Option<String>,
}
