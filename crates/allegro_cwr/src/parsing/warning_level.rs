//! Warning levels for CWR parsing

/// Warning levels for CWR parsing
#[derive(Debug, Clone, PartialEq)]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}
