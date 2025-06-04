//! CWR Record Type Definitions
//! 
//! This module contains typed representations of CWR records that mirror the 
//! wire format structure. These serve as an intermediate layer between raw
//! CWR lines and business domain objects.

pub mod ack;
pub mod agr;
pub mod alt;
pub mod ari;
pub mod com;
pub mod ewt;
pub mod grh;
pub mod grt;
pub mod hdr;
pub mod ind;
pub mod ins;
pub mod ipa;
pub mod msg;
pub mod nat;
pub mod net;
pub mod now;
pub mod npa;
pub mod npn;
pub mod npr;
pub mod nwr;
pub mod nwn;
pub mod orn;
pub mod per;
pub mod pwr;
pub mod rec;
pub mod spu;
pub mod spt;
pub mod swr;
pub mod swt;
pub mod ter;
pub mod trl;
pub mod ver;
pub mod xrf;

// Re-export all record types
pub use ack::AckRecord;
pub use agr::AgrRecord;
pub use alt::AltRecord;
pub use ari::AriRecord;
pub use com::ComRecord;
pub use ewt::EwtRecord;
pub use grh::GrhRecord;
pub use grt::GrtRecord;
pub use hdr::HdrRecord;
pub use ind::IndRecord;
pub use ins::InsRecord;
pub use ipa::IpaRecord;
pub use msg::MsgRecord;
pub use nat::NatRecord;
pub use net::NetRecord;
pub use now::NowRecord;
pub use npa::NpaRecord;
pub use npn::NpnRecord;
pub use npr::NprRecord;
pub use nwr::NwrRecord;
pub use nwn::NwnRecord;
pub use orn::OrnRecord;
pub use per::PerRecord;
pub use pwr::PwrRecord;
pub use rec::RecRecord;
pub use spu::SpuRecord;
pub use spt::SptRecord;
pub use swr::SwrRecord;
pub use swt::SwtRecord;
pub use ter::TerRecord;
pub use trl::TrlRecord;
pub use ver::VerRecord;
pub use xrf::XrfRecord;