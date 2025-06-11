use std::io;

/// Core parsing errors - focused on parsing issues only
#[derive(Debug)]
pub enum CwrParseError {
    Io(io::Error),
    BadFormat(String),
    NonAsciiInput { line_num: usize, byte_pos: usize, byte_value: u8 },
    NonAsciiOutput { char: char, position: usize },
    InvalidHeader { found_bytes: Vec<u8> },
    BomDetected { bom_type: String },
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
            CwrParseError::NonAsciiInput { line_num, byte_pos, byte_value } => {
                write!(f, "Non-ASCII byte 0x{:02X} at line {}, position {}", byte_value, line_num, byte_pos)
            }
            CwrParseError::NonAsciiOutput { char, position } => {
                write!(f, "Non-ASCII character '{}' at position {}", char, position)
            }
            CwrParseError::InvalidHeader { found_bytes } => {
                write!(f, "Invalid CWR header, expected 'HDR' but found: {:?}", found_bytes)
            }
            CwrParseError::BomDetected { bom_type } => {
                write!(f, "BOM detected: {} (CWR files should be ASCII only)", bom_type)
            }
        }
    }
}

impl std::error::Error for CwrParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CwrParseError::Io(err) => Some(err),
            CwrParseError::BadFormat(_)
            | CwrParseError::NonAsciiInput { .. }
            | CwrParseError::NonAsciiOutput { .. }
            | CwrParseError::InvalidHeader { .. }
            | CwrParseError::BomDetected { .. } => None,
        }
    }
}
