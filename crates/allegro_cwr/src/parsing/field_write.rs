//! Trait for converting CWR fields to their string representation for writing

/// Trait for converting CWR fields to their string representation for writing
pub trait CwrFieldWrite {
    fn to_cwr_str(&self) -> String;
}

impl CwrFieldWrite for String {
    fn to_cwr_str(&self) -> String {
        self.clone()
    }
}

impl<T: CwrFieldWrite> CwrFieldWrite for Option<T> {
    fn to_cwr_str(&self) -> String {
        match self {
            Some(val) => val.to_cwr_str(),
            None => String::new(),
        }
    }
}