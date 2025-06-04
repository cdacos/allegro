//! CWR Record Type Definitions
//! 
//! This module contains typed representations of CWR records that mirror the 
//! wire format structure. These serve as an intermediate layer between raw
//! CWR lines and business domain objects.

pub mod grh;

// Re-export all record types
pub use grh::GrhRecord;