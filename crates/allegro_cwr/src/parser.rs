use crate::error::CwrParseError;
use crate::records::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek};

// Context struct to hold file-level metadata like CWR version
#[derive(Debug, Clone)]
pub struct ParsingContext {
    pub cwr_version: f32, // e.g., 2.0, 2.1, 2.2
    pub file_id: i64,     // Database file_id for this file
                          // Add other metadata like charset later if needed
}

fn get_cwr_version(hdr_line: &str) -> Result<f32, CwrParseError> {
    // Define valid CWR versions
    let valid_cwr_versions = [2.0, 2.1, 2.2];

    // Determine version based on header line length
    let cwr_version = if hdr_line.len() < 87 {
        2.0
    } else if hdr_line.len() > 104 {
        // Try to parse version from specific position in header
        if let Some(version_str) = hdr_line.get(101..104) {
            match version_str.trim().parse::<f32>() {
                Ok(version) => version,
                Err(_) => return Err(CwrParseError::BadFormat(format!("Invalid CWR version value: {}", version_str))),
            }
        } else {
            return Err(CwrParseError::BadFormat("Unable to extract CWR version from header".to_string()));
        }
    } else {
        2.1
    };

    // Validate the version
    if valid_cwr_versions.contains(&cwr_version) { Ok(cwr_version) } else { Err(CwrParseError::BadFormat(format!("Invalid CWR version: {}", cwr_version))) }
}

/// Enum containing all possible parsed CWR record types
#[derive(Debug, Clone)]
pub enum CwrRecord {
    Hdr(HdrRecord),
    Grh(GrhRecord),
    Grt(GrtRecord),
    Trl(TrlRecord),
    Agr(AgrRecord),
    Nwr(NwrRecord),
    Ack(AckRecord),
    Ter(TerRecord),
    Ipa(IpaRecord),
    Npa(NpaRecord),
    Spu(SpuRecord),
    Npn(NpnRecord),
    Spt(SptRecord),
    Swr(SwrRecord),
    Nwn(NwnRecord),
    Swt(SwtRecord),
    Pwr(PwrRecord),
    Alt(AltRecord),
    Nat(NatRecord),
    Ewt(EwtRecord),
    Ver(VerRecord),
    Per(PerRecord),
    Npr(NprRecord),
    Rec(RecRecord),
    Orn(OrnRecord),
    Ins(InsRecord),
    Ind(IndRecord),
    Com(ComRecord),
    Msg(MsgRecord),
    Net(NetRecord),
    Now(NowRecord),
    Ari(AriRecord),
    Xrf(XrfRecord),
}

impl CwrRecord {
    pub fn record_type(&self) -> &str {
        match self {
            CwrRecord::Hdr(_) => "HDR",
            CwrRecord::Grh(_) => "GRH",
            CwrRecord::Grt(_) => "GRT",
            CwrRecord::Trl(_) => "TRL",
            CwrRecord::Agr(_) => "AGR",
            CwrRecord::Nwr(_) => "NWR",
            CwrRecord::Ack(_) => "ACK",
            CwrRecord::Ter(_) => "TER",
            CwrRecord::Ipa(_) => "IPA",
            CwrRecord::Npa(_) => "NPA",
            CwrRecord::Spu(_) => "SPU",
            CwrRecord::Npn(_) => "NPN",
            CwrRecord::Spt(_) => "SPT",
            CwrRecord::Swr(_) => "SWR",
            CwrRecord::Nwn(_) => "NWN",
            CwrRecord::Swt(_) => "SWT",
            CwrRecord::Pwr(_) => "PWR",
            CwrRecord::Alt(_) => "ALT",
            CwrRecord::Nat(_) => "NAT",
            CwrRecord::Ewt(_) => "EWT",
            CwrRecord::Ver(_) => "VER",
            CwrRecord::Per(_) => "PER",
            CwrRecord::Npr(_) => "NPR",
            CwrRecord::Rec(_) => "REC",
            CwrRecord::Orn(_) => "ORN",
            CwrRecord::Ins(_) => "INS",
            CwrRecord::Ind(_) => "IND",
            CwrRecord::Com(_) => "COM",
            CwrRecord::Msg(_) => "MSG",
            CwrRecord::Net(_) => "NET",
            CwrRecord::Now(_) => "NOW",
            CwrRecord::Ari(_) => "ARI",
            CwrRecord::Xrf(_) => "XRF",
        }
    }
}

/// Represents a parsed CWR record with its metadata
#[derive(Debug, Clone)]
pub struct ParsedRecord {
    pub line_number: usize,
    pub record: CwrRecord,
    pub context: ParsingContext,
}

/// Parses a single CWR line and returns the parsed record
fn parse_cwr_line(line: &str, line_number: usize, context: &ParsingContext) -> Result<ParsedRecord, CwrParseError> {
    if line.len() < 3 {
        return Err(CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number)));
    }

    let record_type = &line[0..3];

    // Parse into the appropriate record struct
    let record = match record_type {
        "HDR" => CwrRecord::Hdr(HdrRecord::from_cwr_line(line)?),
        "GRH" => CwrRecord::Grh(GrhRecord::from_cwr_line(line)?),
        "GRT" => CwrRecord::Grt(GrtRecord::from_cwr_line(line)?),
        "TRL" => CwrRecord::Trl(TrlRecord::from_cwr_line(line)?),
        "AGR" => CwrRecord::Agr(AgrRecord::from_cwr_line(line)?),
        "NWR" | "REV" | "ISW" | "EXC" => CwrRecord::Nwr(NwrRecord::from_cwr_line(line)?),
        "ACK" => CwrRecord::Ack(AckRecord::from_cwr_line(line)?),
        "TER" => CwrRecord::Ter(TerRecord::from_cwr_line(line)?),
        "IPA" => CwrRecord::Ipa(IpaRecord::from_cwr_line(line)?),
        "NPA" => CwrRecord::Npa(NpaRecord::from_cwr_line(line)?),
        "SPU" | "OPU" => CwrRecord::Spu(SpuRecord::from_cwr_line(line)?),
        "NPN" => CwrRecord::Npn(NpnRecord::from_cwr_line(line)?),
        "SPT" | "OPT" => CwrRecord::Spt(SptRecord::from_cwr_line(line)?),
        "SWR" | "OWR" => CwrRecord::Swr(SwrRecord::from_cwr_line(line)?),
        "NWN" => CwrRecord::Nwn(NwnRecord::from_cwr_line(line)?),
        "SWT" | "OWT" => CwrRecord::Swt(SwtRecord::from_cwr_line(line)?),
        "PWR" => CwrRecord::Pwr(PwrRecord::from_cwr_line(line)?),
        "ALT" => CwrRecord::Alt(AltRecord::from_cwr_line(line)?),
        "NAT" => CwrRecord::Nat(NatRecord::from_cwr_line(line)?),
        "EWT" => CwrRecord::Ewt(EwtRecord::from_cwr_line(line)?),
        "VER" => CwrRecord::Ver(VerRecord::from_cwr_line(line)?),
        "PER" => CwrRecord::Per(PerRecord::from_cwr_line(line)?),
        "NPR" => CwrRecord::Npr(NprRecord::from_cwr_line(line)?),
        "REC" => CwrRecord::Rec(RecRecord::from_cwr_line(line)?),
        "ORN" => CwrRecord::Orn(OrnRecord::from_cwr_line(line)?),
        "INS" => CwrRecord::Ins(InsRecord::from_cwr_line(line)?),
        "IND" => CwrRecord::Ind(IndRecord::from_cwr_line(line)?),
        "COM" => CwrRecord::Com(ComRecord::from_cwr_line(line)?),
        "MSG" => CwrRecord::Msg(MsgRecord::from_cwr_line(line)?),
        "NET" | "NCT" | "NVT" => CwrRecord::Net(NetRecord::from_cwr_line(line)?),
        "NOW" => CwrRecord::Now(NowRecord::from_cwr_line(line)?),
        "ARI" => CwrRecord::Ari(AriRecord::from_cwr_line(line)?),
        "XRF" => CwrRecord::Xrf(XrfRecord::from_cwr_line(line)?),
        _ => {
            return Err(CwrParseError::BadFormat(format!("Unrecognized record type '{}'", record_type)));
        }
    };

    Ok(ParsedRecord {
        line_number,
        record,
        context: context.clone(),
    })
}

/// Returns an iterator that processes CWR lines and yields parsed records
pub fn process_cwr_stream(input_filename: &str) -> Result<impl Iterator<Item = Result<ParsedRecord, CwrParseError>>, CwrParseError> {
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
        return Err(CwrParseError::BadFormat(format!(
            "File does not start with HDR record. Found: '{}'",
            hdr_line.get(0..std::cmp::min(hdr_line.len(), 50)).unwrap_or("")
        )));
    }

    let cwr_version = get_cwr_version(hdr_line)?;
    println!("Determined CWR Version: {}", cwr_version);

    let context = ParsingContext { cwr_version, file_id: 0 };

    // Reset to start of file
    reader.seek(io::SeekFrom::Start(0))?;

    Ok(reader.lines()
        .enumerate()
        .filter_map(move |(idx, line_result)| {
            let line_number = idx + 1;
            match line_result {
                Ok(line) => {
                    if line.is_empty() || line.trim().is_empty() {
                        Some(Err(CwrParseError::BadFormat(format!("Line {} is empty", line_number))))
                    } else if line.len() < 3 {
                        Some(Err(CwrParseError::BadFormat(format!("Line {} is too short (less than 3 chars)", line_number))))
                    } else {
                        Some(parse_cwr_line(&line, line_number, &context))
                    }
                }
                Err(io_err) => {
                    eprintln!("IO error reading line {}: {}", line_number, io_err);
                    Some(Err(CwrParseError::Io(io_err)))
                }
            }
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_get_cwr_version_v20() {
        let hdr_line = "HDR01BMI      BMI MUSIC                             01.1020050101120000 ";
        let version = get_cwr_version(hdr_line).unwrap();
        assert_eq!(version, 2.0);
    }

    #[test]
    fn test_get_cwr_version_v21() {
        let hdr_line = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101        ";
        let version = get_cwr_version(hdr_line).unwrap();
        assert_eq!(version, 2.1);
    }

    #[test]
    fn test_get_cwr_version_v22() {
        let hdr_line = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    2.2";
        let version = get_cwr_version(hdr_line).unwrap();
        assert_eq!(version, 2.2);
    }

    #[test]
    fn test_get_cwr_version_invalid() {
        let hdr_line = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    9.9";
        let result = get_cwr_version(hdr_line);
        assert!(result.is_err());
        match result {
            Err(CwrParseError::BadFormat(msg)) => {
                assert_eq!(msg, "Invalid CWR version: 9.9");
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
        let context = ParsingContext { cwr_version: 2.2, file_id: 0 };
        let line = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    2.2";
        let result = parse_cwr_line(line, 1, &context);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.line_number, 1);
        assert_eq!(parsed.record.record_type(), "HDR");
    }

    #[test]
    fn test_cwr_record_type_mapping() {
        use crate::records::HdrRecord;
        let hdr = HdrRecord::new("01".to_string(), "BMI".to_string(), "BMI MUSIC".to_string(), "01.10".to_string(), "20050101".to_string(), "120000".to_string(), "20050101".to_string());
        let cwr_record = CwrRecord::Hdr(hdr);
        assert_eq!(cwr_record.record_type(), "HDR");
    }

    fn create_temp_cwr_file(content: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file_path = temp_dir.join(format!("test_{}.cwr", timestamp));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
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
        let content = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    2.2\nTRL00000001000000012005010100                                                                                                                                                                                                                                                                                                                                                                                   ";
        let temp_file = create_temp_cwr_file(content);
        let result = process_cwr_stream(&temp_file);
        assert!(result.is_ok());
        
        let records: Vec<_> = result.unwrap().collect();
        assert_eq!(records.len(), 2);
        
        assert!(records[0].is_ok());
        assert!(records[1].is_ok());
        
        let first_record = records[0].as_ref().unwrap();
        assert_eq!(first_record.line_number, 1);
        assert_eq!(first_record.record.record_type(), "HDR");
        assert_eq!(first_record.context.cwr_version, 2.2);
        
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_process_cwr_stream_empty_line() {
        let content = "HDR01BMI      BMI MUSIC                             01.1020050101120000 20050101                    2.2\n\nTRL00000001000000012005010100                                                                                                                                                                                                                                                                                                                                                                                   ";
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
}

