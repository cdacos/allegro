use crate::error::CwrParseError;
use crate::records::*;
use log::{error, info};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek};
use crate::cwr_record::CwrRecord;
use crate::util::get_cwr_version;

// Context struct to hold file-level metadata like CWR version
#[derive(Debug, Clone, serde::Serialize)]
pub struct ParsingContext {
    pub cwr_version: f32, // e.g., 2.0, 2.1, 2.2
    pub file_id: i64,     // Database file_id for this file
                          // Add other metadata like charset later if needed
}

/// Represents a parsed CWR record with its metadata
#[derive(Debug, Clone, serde::Serialize)]
pub struct ParsedRecord {
    pub line_number: usize,
    pub record: CwrRecord,
    pub context: ParsingContext,
    pub warnings: Vec<String>,
}

/// Parses a single CWR line and returns the parsed record
fn parse_cwr_line(line: &str, line_number: usize, context: &ParsingContext) -> Result<ParsedRecord, CwrParseError> {
    let record_type = line.get(0..3).ok_or_else(|| {
        CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number))
    })?;

    // Parse into the appropriate record struct
    let (record, warnings) = match record_type {
        "HDR" => {
            let result = HdrRecord::from_cwr_line(line)?;
            (CwrRecord::Hdr(result.record), result.warnings)
        },
        "GRH" => {
            let result = GrhRecord::from_cwr_line(line)?;
            (CwrRecord::Grh(result.record), result.warnings)
        },
        "GRT" => {
            let result = GrtRecord::from_cwr_line(line)?;
            (CwrRecord::Grt(result.record), result.warnings)
        },
        "TRL" => {
            let result = TrlRecord::from_cwr_line(line)?;
            (CwrRecord::Trl(result.record), result.warnings)
        },
        "AGR" => {
            let result = AgrRecord::from_cwr_line(line)?;
            (CwrRecord::Agr(result.record), result.warnings)
        },
        "NWR" | "REV" | "ISW" | "EXC" => {
            let result = NwrRecord::from_cwr_line(line)?;
            (CwrRecord::Nwr(result.record), result.warnings)
        },
        "ACK" => {
            let result = AckRecord::from_cwr_line(line)?;
            (CwrRecord::Ack(result.record), result.warnings)
        },
        "TER" => {
            let result = TerRecord::from_cwr_line(line)?;
            (CwrRecord::Ter(result.record), result.warnings)
        },
        "IPA" => {
            let result = IpaRecord::from_cwr_line(line)?;
            (CwrRecord::Ipa(result.record), result.warnings)
        },
        "NPA" => {
            let result = NpaRecord::from_cwr_line(line)?;
            (CwrRecord::Npa(result.record), result.warnings)
        },
        "SPU" | "OPU" => {
            let result = SpuRecord::from_cwr_line(line)?;
            (CwrRecord::Spu(result.record), result.warnings)
        },
        "NPN" => {
            let result = NpnRecord::from_cwr_line(line)?;
            (CwrRecord::Npn(result.record), result.warnings)
        },
        "SPT" | "OPT" => {
            let result = SptRecord::from_cwr_line(line)?;
            (CwrRecord::Spt(result.record), result.warnings)
        },
        "SWR" | "OWR" => {
            let result = SwrRecord::from_cwr_line(line)?;
            (CwrRecord::Swr(result.record), result.warnings)
        },
        "NWN" => {
            let result = NwnRecord::from_cwr_line(line)?;
            (CwrRecord::Nwn(result.record), result.warnings)
        },
        "SWT" | "OWT" => {
            let result = SwtRecord::from_cwr_line(line)?;
            (CwrRecord::Swt(result.record), result.warnings)
        },
        "PWR" => {
            let result = PwrRecord::from_cwr_line(line)?;
            (CwrRecord::Pwr(result.record), result.warnings)
        },
        "ALT" => {
            let result = AltRecord::from_cwr_line(line)?;
            (CwrRecord::Alt(result.record), result.warnings)
        },
        "NAT" => {
            let result = NatRecord::from_cwr_line(line)?;
            (CwrRecord::Nat(result.record), result.warnings)
        },
        "EWT" => {
            let result = EwtRecord::from_cwr_line(line)?;
            (CwrRecord::Ewt(result.record), result.warnings)
        },
        "VER" => {
            let result = VerRecord::from_cwr_line(line)?;
            (CwrRecord::Ver(result.record), result.warnings)
        },
        "PER" => {
            let result = PerRecord::from_cwr_line(line)?;
            (CwrRecord::Per(result.record), result.warnings)
        },
        "NPR" => {
            let result = NprRecord::from_cwr_line(line)?;
            (CwrRecord::Npr(result.record), result.warnings)
        },
        "REC" => {
            let result = RecRecord::from_cwr_line(line)?;
            (CwrRecord::Rec(result.record), result.warnings)
        },
        "ORN" => {
            let result = OrnRecord::from_cwr_line(line)?;
            (CwrRecord::Orn(result.record), result.warnings)
        },
        "INS" => {
            let result = InsRecord::from_cwr_line(line)?;
            (CwrRecord::Ins(result.record), result.warnings)
        },
        "IND" => {
            let result = IndRecord::from_cwr_line(line)?;
            (CwrRecord::Ind(result.record), result.warnings)
        },
        "COM" => {
            let result = ComRecord::from_cwr_line(line)?;
            (CwrRecord::Com(result.record), result.warnings)
        },
        "MSG" => {
            let result = MsgRecord::from_cwr_line(line)?;
            (CwrRecord::Msg(result.record), result.warnings)
        },
        "NET" | "NCT" | "NVT" => {
            let result = NetRecord::from_cwr_line(line)?;
            (CwrRecord::Net(result.record), result.warnings)
        },
        "NOW" => {
            let result = NowRecord::from_cwr_line(line)?;
            (CwrRecord::Now(result.record), result.warnings)
        },
        "ARI" => {
            let result = AriRecord::from_cwr_line(line)?;
            (CwrRecord::Ari(result.record), result.warnings)
        },
        "XRF" => {
            let result = XrfRecord::from_cwr_line(line)?;
            (CwrRecord::Xrf(result.record), result.warnings)
        },
        _ => {
            return Err(CwrParseError::BadFormat(format!("Unrecognized record type '{}'", record_type)));
        }
    };

    Ok(ParsedRecord { line_number, record, context: context.clone(), warnings })
}

/// Returns an iterator that processes CWR lines and yields parsed records
pub fn process_cwr_stream(input_filename: &str) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
    process_cwr_stream_with_version(input_filename, None)
}

/// Returns an iterator that processes CWR lines and yields parsed records with optional version hint
pub fn process_cwr_stream_with_version(input_filename: &str, version_hint: Option<f32>) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
    let file = File::open(input_filename)?;
    let mut reader = BufReader::new(file);

    // Read the first line to determine CWR version
    let mut first_line = String::new();
    let bytes_read = reader.read_line(&mut first_line)?;
    if bytes_read == 0 {
        return Err(CwrParseError::BadFormat("File is empty".to_string()));
    }
    let hdr_line = first_line.trim_end();

    if !hdr_line.starts_with("HDR") {
        return Err(CwrParseError::BadFormat(format!("File does not start with HDR record. Found: '{}'", hdr_line.get(0..std::cmp::min(hdr_line.len(), 50)).unwrap_or(""))));
    }

    let cwr_version = get_cwr_version(input_filename, hdr_line, version_hint)?;
    info!("Determined CWR version: {}", cwr_version);

    let context = ParsingContext { cwr_version, file_id: 0 };

    // Reset to start of file
    reader.seek(io::SeekFrom::Start(0))?;

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
            Err(io_err) => {
                error!("IO error reading line {}: {}", line_number, io_err);
                Err(CwrParseError::Io(io_err))
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
        let version = get_cwr_version("test_file.cwr", hdr_line, None).unwrap();
        assert_eq!(version, 2.1);
    }

    #[test]
    fn test_get_cwr_version_cli_override() {
        // CLI version should override auto-detection
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let version = get_cwr_version("test_file.cwr", hdr_line, Some(2.0)).unwrap();
        assert_eq!(version, 2.0);
    }

    #[test]
    fn test_get_cwr_version_filename_detection() {
        // Filename version should be detected when no CLI version specified
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let version = get_cwr_version("CW060001EMI_044.V21", hdr_line, None).unwrap();
        assert_eq!(version, 2.1);
    }

    #[test]
    fn test_get_cwr_version_cli_vs_filename() {
        // CLI version should override filename version
        let hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let version = get_cwr_version("CW060001EMI_044.V21", hdr_line, Some(2.2)).unwrap();
        assert_eq!(version, 2.2);
    }

    #[test]
    fn test_get_cwr_version_filename_vs_hdr() {
        // Filename version should override HDR version when no CLI version
        let mut hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2"); // HDR says 2.2
        hdr_line.push(' ');

        let version = get_cwr_version("CW060001EMI_044.V21", &hdr_line, None).unwrap(); // filename says 2.1
        assert_eq!(version, 2.1); // filename takes precedence
    }

    #[test]
    fn test_get_cwr_version_explicit_v22() {
        // Create a v2.2 line with explicit version at position 101-104
        let mut hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        // Pad to position 101
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2"); // Add version at position 101-104
        hdr_line.push(' '); // Make length > 104

        let version = get_cwr_version("test_file.cwr", &hdr_line, None).unwrap();
        assert_eq!(version, 2.2);
    }

    #[test]
    fn test_get_cwr_version_explicit_conflicts_with_cli() {
        // Test warning when explicit version conflicts with CLI version
        let mut hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
        while hdr_line.len() < 101 {
            hdr_line.push(' ');
        }
        hdr_line.push_str("2.2");
        hdr_line.push(' ');

        // CLI specifies 2.1 but file has explicit 2.2 - should use CLI version with warning
        let version = get_cwr_version("test_file.cwr", &hdr_line, Some(2.1)).unwrap();
        assert_eq!(version, 2.1);
    }

    #[test]
    fn test_get_cwr_version_invalid_explicit() {
        // Invalid explicit version should return error
        let mut hdr_line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221".to_string();
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
        let context = ParsingContext { cwr_version: 2.2, file_id: 0 };
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
        let context = ParsingContext { cwr_version: 2.2, file_id: 0 };
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
        let context = ParsingContext { cwr_version: 2.0, file_id: 0 };
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
        use crate::domain_types::Date;
        use crate::records::HdrRecord;
        use chrono::NaiveDate;

        let hdr = HdrRecord {
            record_type: "HDR".to_string(),
            sender_type: "01".to_string(),
            sender_id: "BMI".to_string(),
            sender_name: "BMI MUSIC".to_string(),
            edi_standard_version_number: "01.10".to_string(),
            creation_date: Date(NaiveDate::from_ymd_opt(2005, 1, 1)),
            creation_time: "120000".to_string(),
            transmission_date: Date(NaiveDate::from_ymd_opt(2005, 1, 1)),
            character_set: None,
            version: None,
            revision: None,
            software_package: None,
            software_package_version: None,
        };
        let cwr_record = CwrRecord::Hdr(hdr);
        assert_eq!(cwr_record.record_type(), "HDR");
    }

    fn create_temp_cwr_file(content: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let thread_id = std::thread::current().id();
        let file_path = temp_dir.join(format!("test_{}_{:?}.cwr", timestamp, thread_id));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        drop(file);
        file_path.to_string_lossy().to_string()
    }

    #[test]
    fn test_process_cwr_stream_empty_file() {
        let temp_file = create_temp_cwr_file("");
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
        let temp_file = create_temp_cwr_file(content);
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
        let temp_file = create_temp_cwr_file(content);
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
        let temp_file = create_temp_cwr_file(content);
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
}
