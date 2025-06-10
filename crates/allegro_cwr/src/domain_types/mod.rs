//! Domain types for CWR field parsing
//!
//! This module has been refactored into separate modules for better organization.
//! All types are re-exported from this module to maintain compatibility.

mod common;
mod date;
mod enums;
mod identifiers;
mod numeric;
mod text;
mod time;

// Re-export all types to maintain compatibility with existing imports
pub use common::*;
pub use date::*;
pub use enums::*;
pub use identifiers::*;
pub use numeric::*;
// pub use text::*;  // No exports from text module yet
pub use time::*;
