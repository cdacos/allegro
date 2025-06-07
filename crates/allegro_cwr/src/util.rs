use crate::CwrParseError;

/// Formats an integer with commas as thousands separators.
pub fn format_int_with_commas(num: i64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        result.push(c);
        let pos = len - 1 - i;
        if pos > 0 && pos % 3 == 0 {
            result.push(',');
        }
    }
    result
}

/// Extract CWR version from filename according to the spec:
/// CWyynnnnsss_rrr.Vxx where Vxx is the version (e.g., V21 = 2.1, V22 = 2.2)
pub fn extract_version_from_filename(filename: &str) -> Option<f32> {
    let path = std::path::Path::new(filename);
    let filename_only = path.file_name()?.to_str()?;

    // Look for .Vxx pattern - we need to be careful about multiple extensions
    // e.g., CW060001EMI_044.V21.zip should detect V21, not zip
    let mut search_str = filename_only;

    // If it ends with common archive extensions, remove them first
    if let Some(base) = search_str.strip_suffix(".zip") {
        search_str = base;
    } else if let Some(base) = search_str.strip_suffix(".cwr") {
        search_str = base;
    }

    // Look for .Vxx pattern at the end
    if let Some(dot_pos) = search_str.rfind('.') {
        let version_part = search_str.get(dot_pos + 1..)?;

        if version_part.starts_with('V') && version_part.len() >= 3 {
            let version_digits = version_part.get(1..3)?;

            match version_digits {
                "20" => Some(2.0),
                "21" => Some(2.1),
                "22" => Some(2.2),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[must_use]
pub fn get_cwr_version(filename: &str, hdr_line: &str, cli_version: Option<f32>) -> Result<f32, CwrParseError> {
    use log::{info, warn};

    // Try to detect version from HDR line using enhanced heuristics
    let hdr_detected_version = detect_version_from_hdr(hdr_line)?;

    // Try to extract version from filename
    let filename_version = extract_version_from_filename(filename);

    // Determine final version with precedence: CLI > filename > HDR explicit > HDR heuristics
    match (cli_version, filename_version, hdr_detected_version) {
        // CLI version specified
        (Some(cli_ver), Some(filename_ver), _) => {
            if cli_ver != filename_ver {
                info!("CLI specified CWR version {} but filename suggests {}. Using CLI version {}", cli_ver, filename_ver, cli_ver);
            }
            Ok(cli_ver)
        }
        (Some(cli_ver), None, Some(hdr_ver)) => {
            if cli_ver != hdr_ver {
                warn!("CLI specified CWR version {} but file contains explicit version {}. Using CLI version {}", cli_ver, hdr_ver, cli_ver);
            }
            Ok(cli_ver)
        }
        (Some(cli_ver), None, None) => Ok(cli_ver),

        // No CLI version - use filename version if available
        (None, Some(filename_ver), Some(hdr_ver)) => {
            if filename_ver != hdr_ver {
                info!("Filename suggests CWR version {} but file contains explicit version {}. Using filename version {}", filename_ver, hdr_ver, filename_ver);
            }
            Ok(filename_ver)
        }
        (None, Some(filename_ver), None) => {
            info!("Detected CWR version {} from filename", filename_ver);
            Ok(filename_ver)
        }

        // No CLI or filename version - use HDR version if available
        (None, None, Some(hdr_ver)) => Ok(hdr_ver),

        // Fall back to heuristics
        (None, None, None) => {
            let heuristic_version = detect_version_by_heuristics(hdr_line);
            info!("Auto-detected CWR version: {}", heuristic_version);
            Ok(heuristic_version)
        }
    }
}

fn detect_version_from_hdr(hdr_line: &str) -> Result<Option<f32>, CwrParseError> {
    // Check for explicit version field at position 101-104 (CWR 2.2+)
    if hdr_line.len() > 104 {
        if let Some(version_str) = hdr_line.get(101..104) {
            let trimmed = version_str.trim();
            if !trimmed.is_empty() {
                match trimmed.parse::<f32>() {
                    Ok(version) => {
                        if [2.0, 2.1, 2.2].contains(&version) {
                            return Ok(Some(version));
                        } else {
                            return Err(CwrParseError::BadFormat(format!("Invalid CWR version in header: {}", version)));
                        }
                    }
                    Err(_) => return Err(CwrParseError::BadFormat(format!("Invalid CWR version format in header: {}", trimmed))),
                }
            }
        }
    }

    // Check for character set field presence (positions 87-89, indicates 2.1+)
    if hdr_line.len() >= 89 {
        if let Some(charset_field) = hdr_line.get(87..89) {
            let trimmed = charset_field.trim();
            if !trimmed.is_empty() {
                // Character set field present suggests 2.1+, but we can't distinguish 2.1 from 2.2 without explicit version
                return Ok(None); // Let heuristics handle it
            }
        }
    }

    Ok(None)
}

fn detect_version_by_heuristics(hdr_line: &str) -> f32 {
    let len = hdr_line.len();

    if len > 104 {
        2.2
    } else if len >= 80 {
        2.1
    } else {
        2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version_from_filename_v21() {
        // Example from spec: CW060001EMI_044.V21
        assert_eq!(extract_version_from_filename("CW060001EMI_044.V21"), Some(2.1));
        assert_eq!(extract_version_from_filename("/path/to/CW060001EMI_044.V21"), Some(2.1));
        assert_eq!(extract_version_from_filename("CW060001EMI_044.V21.cwr"), Some(2.1));
    }

    #[test]
    fn test_extract_version_from_filename_v22() {
        assert_eq!(extract_version_from_filename("CW230001ABC_123.V22"), Some(2.2));
        assert_eq!(extract_version_from_filename("CW230001ABC_123.V22.zip"), Some(2.2));
    }

    #[test]
    fn test_extract_version_from_filename_v20() {
        assert_eq!(extract_version_from_filename("CW050001XYZ_999.V20"), Some(2.0));
    }

    #[test]
    fn test_extract_version_from_filename_no_version() {
        assert_eq!(extract_version_from_filename("CW060001EMI_044"), None);
        assert_eq!(extract_version_from_filename("test.cwr"), None);
        assert_eq!(extract_version_from_filename("some_file.txt"), None);
    }

    #[test]
    fn test_extract_version_from_filename_invalid_version() {
        assert_eq!(extract_version_from_filename("CW060001EMI_044.V99"), None);
        assert_eq!(extract_version_from_filename("CW060001EMI_044.V1"), None);
        assert_eq!(extract_version_from_filename("CW060001EMI_044.X21"), None);
    }

    #[test]
    fn test_extract_version_from_filename_edge_cases() {
        assert_eq!(extract_version_from_filename(""), None);
        assert_eq!(extract_version_from_filename(".V21"), Some(2.1));
        // According to spec, version should be at the end (before archive extension)
        assert_eq!(extract_version_from_filename("file.V21.extra"), None);
        assert_eq!(extract_version_from_filename("file.V21.zip"), Some(2.1));
    }
}
