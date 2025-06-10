//! Domain types for CWR field parsing
//!
//! Each type has its own module following Rust idiom of many small files
//! with single responsibility principle.

mod boolean;
mod composite_component_count;
mod currency_code;
mod cwr_revision;
mod cwr_version;
mod cwr_version_number;
mod date;
mod edi_standard_version;
mod enums;
mod flag;
mod group_count;
mod group_id;
mod number;
mod ownership_share;
mod publisher_sequence_number;
mod record_count;
mod sender_id;
mod sender_name;
mod text;
mod time;
mod tis_numeric_code;
mod transaction_count;
mod works_count;

// Re-export all types to maintain compatibility with existing imports
pub use boolean::*;
pub use composite_component_count::*;
pub use currency_code::*;
// Re-export parsing traits and types that domain types depend on
pub use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
pub use cwr_revision::*;
pub use cwr_version::*;
pub use cwr_version_number::*;
pub use date::*;
pub use edi_standard_version::*;
pub use enums::*;
pub use flag::*;
pub use group_count::*;
pub use group_id::*;
pub use number::*;
pub use ownership_share::*;
pub use publisher_sequence_number::*;
pub use record_count::*;
pub use sender_id::*;
pub use sender_name::*;
// pub use text::*;  // No exports from text module yet
pub use time::*;
pub use tis_numeric_code::*;
pub use transaction_count::*;
pub use works_count::*;
