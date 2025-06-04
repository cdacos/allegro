use std::fmt;

#[derive(Debug)]
pub enum CwrDbError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    Setup(String),
}

impl From<rusqlite::Error> for CwrDbError {
    fn from(err: rusqlite::Error) -> Self {
        CwrDbError::Sqlite(err)
    }
}

impl From<std::io::Error> for CwrDbError {
    fn from(err: std::io::Error) -> Self {
        CwrDbError::Io(err)
    }
}

impl fmt::Display for CwrDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CwrDbError::Sqlite(err) => write!(f, "SQLite error: {}", err),
            CwrDbError::Io(err) => write!(f, "IO error: {}", err),
            CwrDbError::Setup(msg) => write!(f, "Database setup error: {}", msg),
        }
    }
}

impl std::error::Error for CwrDbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CwrDbError::Sqlite(err) => Some(err),
            CwrDbError::Io(err) => Some(err),
            CwrDbError::Setup(_) => None,
        }
    }
}
