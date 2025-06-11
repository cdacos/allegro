use crate::error::CwrParseError;
use crate::parsing::CwrFieldParse;
use crate::util::get_cwr_version;
use std::io::{BufRead, BufReader, Read, Write};

fn should_validate_ascii(character_set: &Option<crate::domain_types::CharacterSet>) -> bool {
    use crate::domain_types::CharacterSet;
    match character_set {
        None | Some(CharacterSet::ASCII) => true,
        Some(CharacterSet::UTF8) | Some(CharacterSet::Unicode) => false,
        Some(CharacterSet::TraditionalBig5) | Some(CharacterSet::SimplifiedGb) => false,
        Some(CharacterSet::Unknown(_)) => true, // Be conservative with unknown sets
    }
}

#[derive(Debug, Clone)]
pub struct CwrHeaderInfo {
    pub header_line: String,
    pub version: f32,
    pub character_set: Option<crate::domain_types::CharacterSet>,
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

        let line_bytes = first_line.as_bytes();

        // Check for BOM at start of file
        let (bom_detected, content_start) = self.detect_bom(line_bytes);
        if bom_detected.is_some() {
            // Log the BOM detection but continue parsing
            eprintln!("BOM detected in CWR file: {} (CWR files should be ASCII only)", bom_detected.as_ref().unwrap());
        }

        // Get the line content after any BOM
        let content_bytes = &line_bytes[content_start..];
        let line_content = if content_start > 0 {
            std::str::from_utf8(content_bytes)
                .map_err(|_| CwrParseError::BadFormat("Invalid UTF-8 after BOM".to_string()))?
        } else {
            &first_line
        };

        // Validate ASCII in header line (after BOM removal)
        for (pos, byte) in content_bytes.iter().enumerate() {
            if *byte > 127 {
                return Err(CwrParseError::NonAsciiInput {
                    line_num: 1,
                    byte_pos: pos + content_start,
                    byte_value: *byte,
                });
            }
        }

        let line = line_content.trim_end_matches('\n').trim_end_matches('\r');
        if !line.starts_with("HDR") {
            return Err(CwrParseError::InvalidHeader {
                found_bytes: line.chars().take(3).collect::<String>().into_bytes(),
            });
        }

        Ok(line.to_string())
    }

    fn detect_bom(&self, bytes: &[u8]) -> (Option<String>, usize) {
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            (Some("UTF-8".to_string()), 3)
        } else if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
            (Some("UTF-16 LE".to_string()), 2)
        } else if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
            (Some("UTF-16 BE".to_string()), 2)
        } else if bytes.len() >= 4 && bytes[0] == 0xFF && bytes[1] == 0xFE && bytes[2] == 0x00 && bytes[3] == 0x00 {
            (Some("UTF-32 LE".to_string()), 4)
        } else if bytes.len() >= 4 && bytes[0] == 0x00 && bytes[1] == 0x00 && bytes[2] == 0xFE && bytes[3] == 0xFF {
            (Some("UTF-32 BE".to_string()), 4)
        } else {
            (None, 0)
        }
    }

    pub fn validate_and_detect_version(
        &mut self, filename: &str, cli_version: Option<f32>,
    ) -> Result<&CwrHeaderInfo, CwrParseError> {
        if self.cached_header_info.is_some() {
            return Ok(self.cached_header_info.as_ref().unwrap());
        }

        let line = self.read_and_validate_header_line()?;
        let version = get_cwr_version(filename, &line, cli_version)?;

        // Extract character set from HDR record if version >= 2.1
        let character_set = if version >= 2.1 && line.len() >= 101 {
            let charset_field = line.get(86..101).unwrap_or("").trim();
            if charset_field.is_empty() {
                None
            } else {
                let (charset, _) = <Option<crate::domain_types::CharacterSet>>::parse_cwr_field(
                    charset_field,
                    "character_set",
                    "Character set",
                );
                charset
            }
        } else {
            None
        };

        let header_info = CwrHeaderInfo { header_line: line, version, character_set };

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
    character_set: Option<crate::domain_types::CharacterSet>,
}

impl<R: Read> AsciiLineReader<R> {
    pub fn new(inner: R) -> Self {
        Self { buf_reader: BufReader::new(inner), character_set: None }
    }

    pub fn with_character_set(inner: R, character_set: Option<crate::domain_types::CharacterSet>) -> Self {
        Self { buf_reader: BufReader::new(inner), character_set }
    }

    pub fn lines(self) -> impl Iterator<Item = Result<String, CwrParseError>> {
        AsciiLineIterator { buf_reader: self.buf_reader, line_num: 0, character_set: self.character_set }
    }
}

struct AsciiLineIterator<R: Read> {
    buf_reader: BufReader<R>,
    line_num: usize,
    character_set: Option<crate::domain_types::CharacterSet>,
}

impl<R: Read> AsciiLineIterator<R> {
    fn detect_bom(&self, bytes: &[u8]) -> (Option<String>, usize) {
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            (Some("UTF-8".to_string()), 3)
        } else if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
            (Some("UTF-16 LE".to_string()), 2)
        } else if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
            (Some("UTF-16 BE".to_string()), 2)
        } else if bytes.len() >= 4 && bytes[0] == 0xFF && bytes[1] == 0xFE && bytes[2] == 0x00 && bytes[3] == 0x00 {
            (Some("UTF-32 LE".to_string()), 4)
        } else if bytes.len() >= 4 && bytes[0] == 0x00 && bytes[1] == 0x00 && bytes[2] == 0xFE && bytes[3] == 0xFF {
            (Some("UTF-32 BE".to_string()), 4)
        } else {
            (None, 0)
        }
    }
}

impl<R: Read> Iterator for AsciiLineIterator<R> {
    type Item = Result<String, CwrParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_num += 1;
        let mut line = String::new();

        match self.buf_reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                let line_bytes = line.as_bytes();

                // Check for BOM on first line only (silently handle it)
                let content_start = if self.line_num == 1 {
                    let (_, bom_bytes) = self.detect_bom(line_bytes);
                    bom_bytes
                } else {
                    0
                };

                // Validate character encoding based on character set (skip BOM bytes if present)
                let content_bytes = &line_bytes[content_start..];
                if should_validate_ascii(&self.character_set) {
                    for (pos, byte) in content_bytes.iter().enumerate() {
                        if *byte > 127 {
                            return Some(Err(CwrParseError::NonAsciiInput {
                                line_num: self.line_num,
                                byte_pos: pos + content_start,
                                byte_value: *byte,
                            }));
                        }
                    }
                }

                // Get content after BOM (if any) and remove trailing newlines
                let line_content = if content_start > 0 {
                    match std::str::from_utf8(content_bytes) {
                        Ok(s) => s,
                        Err(_) => return Some(Err(CwrParseError::BadFormat("Invalid UTF-8 after BOM".to_string()))),
                    }
                } else {
                    &line
                };

                let trimmed = line_content.trim_end_matches('\n').trim_end_matches('\r');
                Some(Ok(trimmed.to_string()))
            }
            Err(e) => Some(Err(CwrParseError::Io(e))),
        }
    }
}

pub struct AsciiWriter<W: Write> {
    inner: W,
    character_set: Option<crate::domain_types::CharacterSet>,
}

impl<W: Write> AsciiWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner, character_set: None }
    }

    pub fn with_character_set(inner: W, character_set: Option<crate::domain_types::CharacterSet>) -> Self {
        Self { inner, character_set }
    }

    pub fn write_line(&mut self, utf8_line: &str) -> Result<(), CwrParseError> {
        // Validate character encoding based on character set
        if should_validate_ascii(&self.character_set) {
            for (pos, ch) in utf8_line.char_indices() {
                if !ch.is_ascii() {
                    return Err(CwrParseError::NonAsciiOutput { char: ch, position: pos });
                }
            }
        }

        // Write as UTF-8 bytes + \r\n (UTF-8 is backwards compatible with ASCII)
        self.inner.write_all(utf8_line.as_bytes())?;
        self.inner.write_all(b"\r\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_types::CharacterSet;
    use std::io::Cursor;

    #[test]
    fn test_ascii_line_reader_with_ascii_charset() {
        let data = "HDR01\r\nASCII LINE\r\n";
        let cursor = Cursor::new(data.as_bytes());
        let reader = AsciiLineReader::with_character_set(cursor, Some(CharacterSet::ASCII));

        let lines: Result<Vec<_>, _> = reader.lines().collect();
        assert!(lines.is_ok());
        let lines = lines.unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "HDR01");
        assert_eq!(lines[1], "ASCII LINE");
    }

    #[test]
    fn test_ascii_line_reader_with_utf8_charset_allows_non_ascii() {
        let data = "HDR01\r\nLINE WITH UNICODE: café\r\n";
        let cursor = Cursor::new(data.as_bytes());
        let reader = AsciiLineReader::with_character_set(cursor, Some(CharacterSet::UTF8));

        let lines: Result<Vec<_>, _> = reader.lines().collect();
        assert!(lines.is_ok());
        let lines = lines.unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "HDR01");
        assert_eq!(lines[1], "LINE WITH UNICODE: café");
    }

    #[test]
    fn test_ascii_line_reader_with_ascii_charset_rejects_non_ascii() {
        let data = "HDR01\r\nLINE WITH UNICODE: café\r\n";
        let cursor = Cursor::new(data.as_bytes());
        let reader = AsciiLineReader::with_character_set(cursor, Some(CharacterSet::ASCII));

        let lines: Vec<_> = reader.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].is_ok());
        assert!(lines[1].is_err());

        if let Err(CwrParseError::NonAsciiInput { line_num, byte_pos: _, byte_value: _ }) = &lines[1] {
            assert_eq!(*line_num, 2);
        } else {
            panic!("Expected NonAsciiInput error");
        }
    }

    #[test]
    fn test_ascii_writer_with_ascii_charset() {
        let mut output = Vec::new();
        let mut writer = AsciiWriter::with_character_set(&mut output, Some(CharacterSet::ASCII));

        assert!(writer.write_line("ASCII LINE").is_ok());
        assert!(writer.write_line("Line with unicode: café").is_err());

        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "ASCII LINE\r\n");
    }

    #[test]
    fn test_ascii_writer_with_utf8_charset() {
        let mut output = Vec::new();
        let mut writer = AsciiWriter::with_character_set(&mut output, Some(CharacterSet::UTF8));

        assert!(writer.write_line("ASCII LINE").is_ok());
        assert!(writer.write_line("Line with unicode: café").is_ok());

        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "ASCII LINE\r\nLine with unicode: café\r\n");
    }

    #[test]
    fn test_ascii_writer_with_unicode_charset() {
        let mut output = Vec::new();
        let mut writer = AsciiWriter::with_character_set(&mut output, Some(CharacterSet::Unicode));

        assert!(writer.write_line("ASCII LINE").is_ok());
        assert!(writer.write_line("Line with unicode: café").is_ok());
        assert!(writer.write_line("Line with Chinese: 你好").is_ok());

        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "ASCII LINE\r\nLine with unicode: café\r\nLine with Chinese: 你好\r\n");
    }

    #[test]
    fn test_ascii_writer_with_unknown_charset_conservative() {
        let mut output = Vec::new();
        let mut writer =
            AsciiWriter::with_character_set(&mut output, Some(CharacterSet::Unknown("ISO-8859-1".to_string())));

        assert!(writer.write_line("ASCII LINE").is_ok());
        assert!(writer.write_line("Line with unicode: café").is_err());

        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "ASCII LINE\r\n");
    }
}
