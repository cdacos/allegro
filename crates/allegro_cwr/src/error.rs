use allegro_cwr_sqlite::{PreparedStatements, log_error, CwrDbError};
use std::io;

#[derive(Debug)]
pub enum CwrParseError {
    Io(io::Error),
    Db(rusqlite::Error),
    BadFormat(String),
}

impl From<io::Error> for CwrParseError {
    fn from(err: io::Error) -> CwrParseError {
        CwrParseError::Io(err)
    }
}

impl From<rusqlite::Error> for CwrParseError {
    fn from(err: rusqlite::Error) -> CwrParseError {
        CwrParseError::Db(err)
    }
}

impl From<CwrDbError> for CwrParseError {
    fn from(err: CwrDbError) -> CwrParseError {
        match err {
            CwrDbError::Sqlite(e) => CwrParseError::Db(e),
            CwrDbError::Io(e) => CwrParseError::Io(e),
            CwrDbError::Setup(msg) => CwrParseError::BadFormat(format!("Database error: {}", msg)),
        }
    }
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

/// Logs a CwrParseError to stderr and the error table using prepared statements.
pub fn log_cwr_parse_error(stmts: &mut PreparedStatements, file_id: i64, line_number: usize, error: &CwrParseError) -> Result<(), CwrDbError> {
    let description = error.to_string();
    log_error(&mut stmts.error_stmt, file_id, line_number, description)
}
