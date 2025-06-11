use crate::error::CwrParseError;
use crate::util::get_cwr_version;
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Debug, Clone)]
pub struct CwrHeaderInfo {
    pub header_line: String,
    pub version: f32,
}

pub struct AsciiStreamSniffer<R: Read> {
    inner: BufReader<R>,
    cached_header_info: Option<CwrHeaderInfo>,
}

impl<R: Read> AsciiStreamSniffer<R> {
    pub fn new(inner: R) -> Self {
        Self { inner: BufReader::new(inner), cached_header_info: None }
    }

    fn read_and_validate_header_line(&mut self) -> Result<String, CwrParseError> {
        let mut first_line = String::new();
        let bytes_read = self.inner.read_line(&mut first_line)?;

        if bytes_read == 0 {
            return Err(CwrParseError::InvalidHeader { found_bytes: vec![] });
        }

        // Validate ASCII in header line
        for (pos, byte) in first_line.as_bytes().iter().enumerate() {
            if *byte > 127 {
                return Err(CwrParseError::NonAsciiInput { line_num: 1, byte_pos: pos, byte_value: *byte });
            }
        }

        let line = first_line.trim_end();
        if !line.starts_with("HDR") {
            return Err(CwrParseError::InvalidHeader {
                found_bytes: line.chars().take(3).collect::<String>().into_bytes(),
            });
        }

        Ok(line.to_string())
    }

    pub fn validate_and_detect_version(
        &mut self, filename: &str, cli_version: Option<f32>,
    ) -> Result<&CwrHeaderInfo, CwrParseError> {
        if self.cached_header_info.is_some() {
            return Ok(self.cached_header_info.as_ref().unwrap());
        }

        let line = self.read_and_validate_header_line()?;
        let version = get_cwr_version(filename, &line, cli_version)?;

        let header_info = CwrHeaderInfo { header_line: line, version };

        self.cached_header_info = Some(header_info);
        Ok(self.cached_header_info.as_ref().unwrap())
    }

    pub fn validate_cwr_header(&mut self) -> Result<(), CwrParseError> {
        // Simple validation for is_cwr_file() - no caching needed
        self.read_and_validate_header_line().map(|_| ())
    }

    pub fn get_cached_header(&self) -> Option<&str> {
        self.cached_header_info.as_ref().map(|info| info.header_line.as_str())
    }

    pub fn get_cached_header_info(&self) -> Option<&CwrHeaderInfo> {
        self.cached_header_info.as_ref()
    }
}

pub struct AsciiLineReader<R: Read> {
    buf_reader: BufReader<R>,
}

impl<R: Read> AsciiLineReader<R> {
    pub fn new(inner: R) -> Self {
        Self { buf_reader: BufReader::new(inner) }
    }

    pub fn lines(self) -> impl Iterator<Item = Result<String, CwrParseError>> {
        AsciiLineIterator { buf_reader: self.buf_reader, line_num: 0 }
    }
}

struct AsciiLineIterator<R: Read> {
    buf_reader: BufReader<R>,
    line_num: usize,
}

impl<R: Read> Iterator for AsciiLineIterator<R> {
    type Item = Result<String, CwrParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_num += 1;
        let mut line = String::new();

        match self.buf_reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                // Validate ASCII
                for (pos, byte) in line.as_bytes().iter().enumerate() {
                    if *byte > 127 {
                        return Some(Err(CwrParseError::NonAsciiInput {
                            line_num: self.line_num,
                            byte_pos: pos,
                            byte_value: *byte,
                        }));
                    }
                }

                // Remove trailing newline characters
                let trimmed = line.trim_end();
                Some(Ok(trimmed.to_string()))
            }
            Err(e) => Some(Err(CwrParseError::Io(e))),
        }
    }
}

#[allow(dead_code)]
pub struct AsciiWriter<W: Write> {
    inner: W,
}

#[allow(dead_code)]
impl<W: Write> AsciiWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn write_line(&mut self, utf8_line: &str) -> Result<(), CwrParseError> {
        // Validate all chars are ASCII
        for (pos, ch) in utf8_line.char_indices() {
            if !ch.is_ascii() {
                return Err(CwrParseError::NonAsciiOutput { char: ch, position: pos });
            }
        }

        // Write as ASCII bytes + \r\n
        self.inner.write_all(utf8_line.as_bytes())?;
        self.inner.write_all(b"\r\n")?;
        Ok(())
    }
}
