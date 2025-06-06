//! PER - Performing Artist Record

use crate::validators::one_of;
use crate::impl_cwr_parsing;
use serde::{Deserialize, Serialize};

/// PER - Performing Artist Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerRecord {
    /// Always "PER"
    pub record_type: String,

    /// Transaction sequence number (8 chars)
    pub transaction_sequence_num: String,

    /// Record sequence number (8 chars)
    pub record_sequence_num: String,

    /// Performing artist last name (45 chars)
    pub performing_artist_last_name: String,

    /// Performing artist first name (30 chars, optional)
    pub performing_artist_first_name: Option<String>,

    /// Performing artist IPI name number (11 chars, optional)
    pub performing_artist_ipi_name_num: Option<String>,

    /// Performing artist IPI base number (13 chars, optional)
    pub performing_artist_ipi_base_number: Option<String>,
}


impl_cwr_parsing! {
    PerRecord {
        record_type: (0, 3, required, one_of(&["PER"])),
        transaction_sequence_num: (3, 11, required),
        record_sequence_num: (11, 19, required),
        performing_artist_last_name: (19, 64, required),
        performing_artist_first_name: (64, 94, optional),
        performing_artist_ipi_name_num: (94, 105, optional),
        performing_artist_ipi_base_number: (105, 118, optional),
    }
    with_test_data ["PER0000000100000001SMITH                                        JOHN                          01234567890123456789012"]
}

