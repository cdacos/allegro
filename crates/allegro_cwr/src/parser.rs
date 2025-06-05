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

// Helper macro for mandatory fields. Logs error to DB (using prepared statement) and returns "" if missing/empty.
// Propagates DB errors or fundamental slice errors.
#[macro_export]
macro_rules! get_mandatory_field {
    ($error_stmt:expr, $slice_fn:expr, $start:expr, $end:expr, $line_num:expr, $file_id:expr, $rec_type:expr, $field_name:expr) => {
        // Match on the result of the slice function
        match $slice_fn($start, $end) {
            // Case 1: Slice function itself returned an error (rare with current safe_slice, but good practice)
            Err(slice_err) => Err(slice_err), // Propagate the underlying error

            // Case 2: Slice succeeded and found a non-empty value
            Ok(Some(value)) => Ok(value), // Return the found value

            // Case 3: Slice succeeded but returned None (missing or empty/whitespace field)
            Ok(None) => {
                // Construct the error description
                let error_description = format!("{} missing or empty mandatory field '{}' (Expected at {}-{}). Using fallback ''.", $rec_type, $field_name, $start + 1, $end); // Use 1-based indexing for user message

                match $error_stmt.execute(rusqlite::params![$file_id, $line_num as i64, error_description]) {
                    // Subcase 3a: Database insertion failed
                    Err(db_err) => Err($crate::error::CwrParseError::Db(db_err)), // Propagate the DB error
                    // Subcase 3b: Database insertion succeeded
                    Ok(_) => Ok(String::new()), // Return the fallback empty string
                }
            }
        }? // Use '?' *after* the match block to propagate any Err returned from the match arms
        // This ensures the macro returns Result<String, CwrParseError>
    };
}
