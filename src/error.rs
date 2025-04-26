use std::io;
use rusqlite;

#[derive(Debug)]
pub enum CwrParseError {
    Io(io::Error),
    Db(rusqlite::Error),
    BadFormat(String),
}

impl From<io::Error> for CwrParseError {
    fn from(err: io::Error) -> CwrParseError { CwrParseError::Io(err) }
}

impl From<rusqlite::Error> for CwrParseError {
    fn from(err: rusqlite::Error) -> CwrParseError { CwrParseError::Db(err) }
}

impl std::fmt::Display for CwrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CwrParseError::Io(err) => write!(f, "IO Error: {}", err),
            CwrParseError::Db(err) => write!(f, "Database Error: {}", err),
            CwrParseError::BadFormat(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CwrParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CwrParseError::Io(err) => Some(err),
            CwrParseError::Db(err) => Some(err),
            CwrParseError::BadFormat(_) => None,
        }
    }
}
