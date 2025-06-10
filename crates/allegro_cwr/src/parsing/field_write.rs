//! Trait for converting CWR fields to their string representation for writing

/// Trait for converting CWR fields to their string representation for writing
pub trait CwrFieldWrite {
    fn to_cwr_str(&self, width: usize) -> String;
}

/// Helper function for formatting numeric values with zero-padding
pub fn format_number(value: impl std::fmt::Display, width: usize) -> String {
    format!("{:0width$}", value, width = width)
}

/// Helper function for formatting text values with space-padding on the right
pub fn format_text(value: &str, width: usize) -> String {
    format!("{:width$}", value, width = width)
}

/// Trait for numeric fields that need zero-padding to a specific width
pub trait CwrNumericField {
    /// Returns the unpadded numeric value as a string (e.g., "5" instead of "00000005")
    fn to_numeric_str(&self) -> String;
    
    /// Returns true if this is an empty/None value that should be rendered as "0"
    fn is_empty_numeric(&self) -> bool {
        false
    }
}

impl CwrFieldWrite for String {
    fn to_cwr_str(&self, width: usize) -> String {
        format_text(self, width)
    }
}

impl<T: CwrFieldWrite + 'static> CwrFieldWrite for Option<T> {
    fn to_cwr_str(&self, width: usize) -> String {
        match self {
            Some(val) => val.to_cwr_str(width),
            None => {
                // For numeric types, return zeros; for others, return empty string (space-padded)
                // MonetaryValue is a special case that should be space-padded
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<crate::domain_types::MonetaryValue>() {
                    format!("{:width$}", "", width = width)
                } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<crate::domain_types::Number>() ||
                         std::any::TypeId::of::<T>() == std::any::TypeId::of::<crate::domain_types::OwnershipShare>() {
                    format!("{:0width$}", 0, width = width)
                } else {
                    format!("{:width$}", "", width = width)
                }
            }
        }
    }
}
