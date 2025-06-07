//! CWR Record Type Definitions
//!
//! This module contains typed representations of CWR records that mirror the
//! wire format structure. These serve as an intermediate layer between raw
//! CWR lines and business domain objects.

use crate::cwr_registry::CwrRegistry;
use crate::error::CwrParseError;

/// Result type returned by record parsing functions
#[derive(Debug)]
pub struct ParseResult<T> {
    pub record: T,
    pub warnings: Vec<String>,
}

/// Trait for getting the record type from any record instance
pub trait RecordType {
    fn record_type(&self) -> &str;
}

/// Trait that all CWR record types must implement
pub trait CwrRecord {
    /// The 3-character record type codes this record handles
    fn record_codes() -> &'static [&'static str];

    /// Parse a CWR line into this specific record type
    fn from_cwr_line(line: &str) -> Result<ParseResult<Self>, CwrParseError>
    where
        Self: Sized;

    /// Convert this record into the registry enum variant
    fn into_registry(self) -> CwrRegistry;
}

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
pub mod nwn;
pub mod nwr;
pub mod orn;
pub mod per;
pub mod pwr;
pub mod rec;
pub mod spt;
pub mod spu;
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
pub use nwn::NwnRecord;
pub use nwr::NwrRecord;
pub use orn::OrnRecord;
pub use per::PerRecord;
pub use pwr::PwrRecord;
pub use rec::RecRecord;
pub use spt::SptRecord;
pub use spu::SpuRecord;
pub use swr::SwrRecord;
pub use swt::SwtRecord;
pub use ter::TerRecord;
pub use trl::TrlRecord;
pub use ver::VerRecord;
pub use xrf::XrfRecord;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdr_record_trait() {
        // Test that HdrRecord implements CwrRecord trait correctly
        let codes = HdrRecord::record_codes();
        assert_eq!(codes, &["HDR"]);

        // Test parsing
        let line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = HdrRecord::from_cwr_line(line);
        assert!(result.is_ok());

        let parse_result = result.unwrap();
        let registry = parse_result.record.into_registry();
        assert_eq!(registry.record_type(), "HDR");
    }

    #[test]
    fn test_nwr_record_trait() {
        // Test that NwrRecord implements CwrRecord trait with multiple codes
        let codes = NwrRecord::record_codes();
        assert_eq!(codes, &["NWR", "REV", "ISW", "EXC"]);

        // Test parsing
        let line = "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ";
        let result = NwrRecord::from_cwr_line(line);
        assert!(result.is_ok());

        let parse_result = result.unwrap();
        let registry = parse_result.record.into_registry();
        assert_eq!(registry.record_type(), "NWR");
    }

    #[test]
    fn test_spu_record_trait() {
        // Test that SpuRecord implements CwrRecord trait with multiple codes
        let codes = SpuRecord::record_codes();
        assert_eq!(codes, &["SPU", "OPU"]);
    }
}
