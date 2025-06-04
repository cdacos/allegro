//! CWR Record Type Definitions
//! 
//! This module contains typed representations of CWR records that mirror the 
//! wire format structure. These serve as an intermediate layer between raw
//! CWR lines and business domain objects.

pub mod ack;
pub mod agr;
pub mod alt;
pub mod grh;
pub mod grt;
pub mod hdr;
pub mod ipa;
pub mod nwr;
pub mod per;
pub mod pwr;
pub mod rec;
pub mod spu;
pub mod spt;
pub mod swr;
pub mod swt;
pub mod ter;
pub mod trl;

// Re-export all record types
pub use ack::AckRecord;
pub use agr::AgrRecord;
pub use alt::AltRecord;
pub use grh::GrhRecord;
pub use grt::GrtRecord;
pub use hdr::HdrRecord;
pub use ipa::IpaRecord;
pub use nwr::NwrRecord;
pub use per::PerRecord;
pub use pwr::PwrRecord;
pub use rec::RecRecord;
pub use spu::SpuRecord;
pub use spt::SptRecord;
pub use swr::SwrRecord;
pub use swt::SwtRecord;
pub use ter::TerRecord;
pub use trl::TrlRecord;