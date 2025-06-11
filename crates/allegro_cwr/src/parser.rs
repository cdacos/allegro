use crate::ascii_io::{AsciiLineReader, AsciiStreamSniffer};
use crate::cwr_registry::CwrRegistry;
use crate::error::CwrParseError;
use log::{error, info};
use std::fs::File;

#[cfg(test)]
use crate::util::get_cwr_version;
#[cfg(test)]
use std::io;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ParsingContext {
    pub cwr_version: f32,
    pub file_id: i64,
    pub character_set: Option<crate::domain_types::CharacterSet>,
}

/// Represents a parsed CWR record with its metadata
#[derive(Debug, Clone, serde::Serialize)]
pub struct ParsedRecord {
    pub line_number: usize,
    pub record: CwrRegistry,
    pub context: ParsingContext,
    pub warnings: Vec<String>,
}

/// Checks if a file is a CWR file by validating the header
/// Returns true if the file starts with "HDR", false otherwise
/// Returns an error if the file cannot be read or has invalid content
pub fn is_cwr_file(filename: &str) -> Result<bool, CwrParseError> {
    let file = File::open(filename)?;
    let mut sniffer = AsciiStreamSniffer::new(file);

    match sniffer.validate_cwr_header() {
        Ok(()) => Ok(true),
        Err(CwrParseError::InvalidHeader { .. }) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Parses a single CWR line and returns the parsed record
fn parse_cwr_line(line: &str, line_number: usize, context: &ParsingContext) -> Result<ParsedRecord, CwrParseError> {
    let record_type = line
        .get(0..3)
        .ok_or_else(|| CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number)))?;

    let (record, warnings) = crate::cwr_registry::parse_by_record_type(record_type, line)?;

    Ok(ParsedRecord { line_number, record, context: context.clone(), warnings })
}

/// Returns an iterator that processes CWR lines and yields parsed records
pub fn process_cwr_stream(
    input_filename: &str,
) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
    process_cwr_stream_with_version(input_filename, None)
}

/// Returns an iterator that processes CWR lines and yields parsed records with optional version hint and character set override
pub fn process_cwr_stream_with_version_and_charset(
    input_filename: &str, version_hint: Option<f32>, charset_override: Option<&str>,
) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
    // Validate header and detect version in one operation!
    let file = File::open(input_filename)?;
    let mut sniffer = AsciiStreamSniffer::new(file);
    let mut header_info = match sniffer.validate_and_detect_version(input_filename, version_hint) {
        Err(CwrParseError::InvalidHeader { found_bytes }) if found_bytes.is_empty() => {
            return Err(CwrParseError::BadFormat("File is empty".to_string()));
        }
        Err(CwrParseError::InvalidHeader { found_bytes }) => {
            return Err(CwrParseError::BadFormat(format!(
                "File does not start with HDR record. Found: {:?}",
                String::from_utf8_lossy(&found_bytes)
            )));
        }
        Err(e) => return Err(e),
        Ok(info) => info.clone(),
    };

    // Override character set if provided
    if let Some(charset_str) = charset_override {
        use crate::parsing::CwrFieldParse;
        let (charset_opt, _) = <Option<crate::domain_types::CharacterSet>>::parse_cwr_field(
            charset_str,
            "character_set_override",
            "Character set override",
        );
        header_info.character_set = charset_opt;
        info!("Character set overridden to: {:?}", header_info.character_set);
    }

    let cwr_version = header_info.version;
    info!("Determined CWR version: {}", cwr_version);

    let context = ParsingContext { cwr_version, file_id: 0, character_set: header_info.character_set.clone() };

    // Create a new reader for the full iteration with character set context
    let file = File::open(input_filename)?;
    let reader = AsciiLineReader::with_character_set(file, header_info.character_set.clone());

    Ok(reader.lines().enumerate().map(move |(idx, line_result)| {
        let line_number = idx + 1;
        match line_result {
            Ok(line) => {
                if line.is_empty() || line.trim().is_empty() {
                    Err(CwrParseError::BadFormat(format!("Line {} is empty", line_number)))
                } else if line.len() < 3 {
                    Err(CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number)))
                } else {
                    parse_cwr_line(&line, line_number, &context)
                }
            }
            Err(parse_err) => {
                error!("Parse error at line {}: {}", line_number, parse_err);
                Err(parse_err)
            }
        }
    }))
}

/// Returns an iterator that processes CWR lines and yields parsed records with optional version hint
pub fn process_cwr_stream_with_version(
    input_filename: &str, version_hint: Option<f32>,
) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
    // Validate header and detect version in one operation!
    let file = File::open(input_filename)?;
    let mut sniffer = AsciiStreamSniffer::new(file);
    let header_info = match sniffer.validate_and_detect_version(input_filename, version_hint) {
        Err(CwrParseError::InvalidHeader { found_bytes }) if found_bytes.is_empty() => {
            return Err(CwrParseError::BadFormat("File is empty".to_string()));
        }
        Err(CwrParseError::InvalidHeader { found_bytes }) => {
            return Err(CwrParseError::BadFormat(format!(
                "File does not start with HDR record. Found: {:?}",
                String::from_utf8_lossy(&found_bytes)
            )));
        }
        Err(e) => return Err(e),
        Ok(info) => info.clone(),
    };

    let cwr_version = header_info.version;
    info!("Determined CWR version: {}", cwr_version);

    let context = ParsingContext { cwr_version, file_id: 0, character_set: header_info.character_set.clone() };

    // Create a new reader for the full iteration with character set context
    let file = File::open(input_filename)?;
    let reader = AsciiLineReader::with_character_set(file, header_info.character_set.clone());

    Ok(reader.lines().enumerate().map(move |(idx, line_result)| {
        let line_number = idx + 1;
        match line_result {
            Ok(line) => {
                if line.is_empty() || line.trim().is_empty() {
                    Err(CwrParseError::BadFormat(format!("Line {} is empty", line_number)))
                } else if line.len() < 3 {
                    Err(CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number)))
                } else {
                    parse_cwr_line(&line, line_number, &context)
                }
            }
            Err(parse_err) => {
                error!("Parse error at line {}: {}", line_number, parse_err);
                Err(parse_err)
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_get_cwr_version_v21_auto_detect() {
        // TestSample.V21 HDR line - should auto-detect as 2.1 by heuristics
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = get_cwr_version("test_file.cwr", hdr_line, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.1);
    }

    #[test]
    fn test_get_cwr_version_cli_override() {
        // CLI version should override auto-detection
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = get_cwr_version("test_file.cwr", hdr_line, Some(2.0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_get_cwr_version_filename_detection() {
        // Filename version should be detected when no CLI version specified
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = get_cwr_version("CW060001EMI_044.V21", hdr_line, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.1);
    }

    #[test]
    fn test_get_cwr_version_cli_vs_filename() {
        // CLI version should override filename version
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = get_cwr_version("CW060001EMI_044.V21", hdr_line, Some(2.2));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.2);
    }

    #[test]
    fn test_get_cwr_version_filename_vs_hdr() {
        // Filename version should override HDR version when no CLI version
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2"); // HDR says 2.2
        hdr_line.push(' ');

        let result = get_cwr_version("CW060001EMI_044.V21", &hdr_line, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.1); // filename takes precedence
    }

    #[test]
    fn test_get_cwr_version_explicit_v22() {
        // Create a v2.2 line with explicit version at position 101-104
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        // Pad to position 101
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2"); // Add version at position 101-104
        hdr_line.push(' '); // Make length > 104

        let result = get_cwr_version("test_file.cwr", &hdr_line, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.2);
    }

    #[test]
    fn test_get_cwr_version_explicit_conflicts_with_cli() {
        // Test warning when explicit version conflicts with CLI version
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2");
        hdr_line.push(' ');

        // CLI specifies 2.1 but file has explicit 2.2 - should use CLI version with warning
        let result = get_cwr_version("test_file.cwr", &hdr_line, Some(2.1));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.1);
    }

    #[test]
    fn test_get_cwr_version_invalid_explicit() {
        // Invalid explicit version should return error
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("9.9");
        hdr_line.push(' ');

        let result = get_cwr_version("test_file.cwr", &hdr_line, None);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert!(msg.contains("Invalid CWR version in header: 9.9"));
            }
            _ => panic!("Expected BadFormat error"),
        }
    }

    #[test]
    fn test_parse_cwr_line_too_short() {
        let context = ParsingContext { cwr_version: 2.2, file_id: 0, character_set: None };
        let result = parse_cwr_line("AB", 1, &context);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert_eq!(msg, "Line 1 is too short (less than 3 chars)");
            }
            _ => panic!("Expected BadFormat error"),
        }
    }

    #[test]
    fn test_parse_cwr_line_unknown_record_type() {
        let context = ParsingContext { cwr_version: 2.2, file_id: 0, character_set: None };
        let result = parse_cwr_line("XYZ00000001000000012005010112000000001000000001NWR", 1, &context);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert_eq!(msg, "Unrecognized record type 'XYZ'");
            }
            _ => panic!("Expected BadFormat error"),
        }
    }

    #[test]
    fn test_parse_cwr_line_valid_hdr() {
        let context = ParsingContext { cwr_version: 2.0, file_id: 0, character_set: None };
        // Real HDR line from TestSample.V21
        let line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = parse_cwr_line(line, 1, &context);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.line_number, 1);
        assert_eq!(parsed.record.record_type(), "HDR");
    }

    #[test]
    fn test_cwr_record_type_mapping() {
        use crate::domain_types::*;
        use crate::records::HdrRecord;
        use chrono::{NaiveDate, NaiveTime};

        let hdr = HdrRecord {
            record_type: "HDR".to_string(),
            sender_type: SenderType::NumericPrefix("01".to_string()),
            sender_id: SenderId("BMI".to_string()),
            sender_name: SenderName("BMI MUSIC".to_string()),
            edi_standard_version_number: EdiStandardVersion("01.10".to_string()),
            creation_date: Date(NaiveDate::from_ymd_opt(2005, 1, 1).unwrap()),
            creation_time: Time(NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
            transmission_date: Date(NaiveDate::from_ymd_opt(2005, 1, 1).unwrap()),
            character_set: None,
            version: None,
            revision: None,
            software_package: None,
            software_package_version: None,
        };
        let cwr_record = CwrRegistry::Hdr(hdr);
        assert_eq!(cwr_record.record_type(), "HDR");
    }

    fn create_temp_cwr_file(content: &str) -> Result<String, io::Error> {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| io::Error::other("System time error"))?
            .as_nanos();
        let thread_id = std::thread::current().id();
        let file_path = temp_dir.join(format!("test_{}_{:?}.cwr", timestamp, thread_id));
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        drop(file);
        Ok(file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_process_cwr_stream_empty_file() {
        let temp_file = create_temp_cwr_file("").unwrap();
        let result = process_cwr_stream(&temp_file);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert_eq!(msg, "File is empty");
            }
            _ => panic!("Expected BadFormat error"),
        }
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_no_hdr() {
        let content = "TRL00000001000000012005010100";
        let temp_file = create_temp_cwr_file(content).unwrap();
        let result = process_cwr_stream(&temp_file);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert!(msg.starts_with("File does not start with HDR record"));
            }
            _ => panic!("Expected BadFormat error"),
        }
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_valid_file() {
        let content = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221\nGRHNWR0000102.100000000000  \nTRL00000002000000022022122100                                                                                                                                                                                                                                                                                                                                                                                   ";
        let temp_file = create_temp_cwr_file(content).unwrap();
        let result = process_cwr_stream(&temp_file);
        assert!(result.is_ok());

        let records: Vec<_> = result.unwrap().collect();
        assert_eq!(records.len(), 3);

        assert!(records[0].is_ok());
        assert!(records[1].is_ok());
        assert!(records[2].is_ok());

        let first_record = records[0].as_ref().unwrap();
        assert_eq!(first_record.line_number, 1);
        assert_eq!(first_record.record.record_type(), "HDR");
        assert_eq!(first_record.context.cwr_version, 2.1);

        let second_record = records[1].as_ref().unwrap();
        assert_eq!(second_record.line_number, 2);
        assert_eq!(second_record.record.record_type(), "GRH");

        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_empty_line() {
        let content = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221\n\nTRL00000002000000022022122100                                                                                                                                                                                                                                                                                                                                                                                   ";
        let temp_file = create_temp_cwr_file(content).unwrap();
        let result = process_cwr_stream(&temp_file);
        assert!(result.is_ok());

        let records: Vec<_> = result.unwrap().collect();
        assert_eq!(records.len(), 3);

        assert!(records[0].is_ok());
        assert!(records[1].is_err());
        assert!(records[2].is_ok());

        match &records[1] {
            Err(CwrParseError::BadFormat(msg)) => {
                assert_eq!(msg, "Line 2 is empty");
            }
            _ => panic!("Expected BadFormat error for empty line"),
        }

        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_real_sample() {
        // Test with actual sample file
        let result = process_cwr_stream("../../.me/TestSample.V21");
        assert!(result.is_ok());

        let records: Vec<_> = result.unwrap().take(10).collect(); // Just test first 10 records
        assert!(!records.is_empty());

        // Verify first record is HDR
        assert!(records[0].is_ok());
        let first_record = records[0].as_ref().unwrap();
        assert_eq!(first_record.record.record_type(), "HDR");
        assert_eq!(first_record.context.cwr_version, 2.1);
    }

    #[test]
    fn test_is_cwr_file() {
        // Test with HDR file
        let hdr_content = "HDRTEST";
        let hdr_file = create_temp_cwr_file(hdr_content).unwrap();
        let result = is_cwr_file(&hdr_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        fs::remove_file(&hdr_file).ok();

        // Test with non-HDR file
        let non_hdr_content = "NOT A CWR FILE";
        let non_hdr_file = create_temp_cwr_file(non_hdr_content).unwrap();
        let result = is_cwr_file(&non_hdr_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        fs::remove_file(&non_hdr_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_with_utf8_charset_and_non_ascii() {
        use crate::domain_types::CharacterSet;

        // Create a CWR file with UTF-8 character set and non-ASCII data
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        // Pad to position 86 (character set field)
        while hdr_line.len() < 86 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("UTF-8          "); // Character set field (15 chars)

        let content = format!(
            "{}\nGRHNWR0000102.100000000000  \nTRL00000002000000022022122100                                                                                                                                                                                                                                                                                                                                                                                   ",
            hdr_line
        );
        let temp_file = create_temp_cwr_file(&content).unwrap();

        let result = process_cwr_stream(&temp_file);
        assert!(result.is_ok());

        let records: Vec<_> = result.unwrap().collect();
        assert_eq!(records.len(), 3);

        // Check that all records parsed successfully
        assert!(records[0].is_ok());
        assert!(records[1].is_ok());
        assert!(records[2].is_ok());

        // Check that character set was detected properly
        let first_record = records[0].as_ref().unwrap();
        assert_eq!(first_record.context.character_set, Some(CharacterSet::UTF8));

        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_ascii_charset_rejects_non_ascii() {
        // Create a CWR file with ASCII character set and attempt to include non-ASCII data
        let mut hdr_line =
            "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        // Pad to position 86 (character set field)
        while hdr_line.len() < 86 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("ASCII          "); // Character set field (15 chars)

        // Include a line with non-ASCII characters
        let content = format!(
            "{}\nLine with café\nTRL00000002000000022022122100                                                                                                                                                                                                                                                                                                                                                                                   ",
            hdr_line
        );
        let temp_file = create_temp_cwr_file(&content).unwrap();

        let result = process_cwr_stream(&temp_file);
        assert!(result.is_ok());

        let records: Vec<_> = result.unwrap().collect();
        assert_eq!(records.len(), 3);

        // First record (HDR) should be OK
        assert!(records[0].is_ok());

        // Second record should fail due to non-ASCII characters
        assert!(records[1].is_err());
        if let Err(CwrParseError::NonAsciiInput { line_num, byte_pos: _, byte_value: _ }) = &records[1] {
            assert_eq!(*line_num, 2);
        } else {
            panic!("Expected NonAsciiInput error");
        }

        // Third record should be OK
        assert!(records[2].is_ok());

        fs::remove_file(&temp_file).ok();
    }
}
