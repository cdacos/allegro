use std::io;

/// Core parsing errors - focused on parsing issues only
#[derive(Debug)]
pub enum CwrParseError {
    Io(io::Error),
    BadFormat(String),
    MissingOptionalField { field_name: String, record_type: String },
}

#[derive(Debug)]
pub struct CwrParseResult<T> {
    pub record: T,
    pub warnings: Vec<String>,
}

impl From<io::Error> for CwrParseError {
    fn from(err: io::Error) -> CwrParseError {
        CwrParseError::Io(err)
    }
}

impl std::fmt::Display for CwrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CwrParseError::Io(err) => write!(f, "IO Error: {}", err),
            CwrParseError::BadFormat(msg) => write!(f, "{}", msg),
            CwrParseError::MissingOptionalField { field_name, record_type } => {
                write!(f, "{} record missing optional field: {}", record_type, field_name)
            }
        }
    }
}

impl std::error::Error for CwrParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CwrParseError::Io(err) => Some(err),
            CwrParseError::BadFormat(_) => None,
            CwrParseError::MissingOptionalField { .. } => None,
        }
    }
}
