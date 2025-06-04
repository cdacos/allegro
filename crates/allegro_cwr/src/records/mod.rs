//! CWR Record Type Definitions
//! 
//! This module contains typed representations of CWR records that mirror the 
//! wire format structure. These serve as an intermediate layer between raw
//! CWR lines and business domain objects.

pub mod agr;
pub mod grh;
pub mod grt;
pub mod hdr;
pub mod nwr;
pub mod swr;
pub mod trl;

// Re-export all record types
pub use agr::AgrRecord;
pub use grh::GrhRecord;
pub use grt::GrtRecord;
pub use hdr::HdrRecord;
pub use nwr::NwrRecord;
pub use swr::SwrRecord;
pub use trl::TrlRecord;