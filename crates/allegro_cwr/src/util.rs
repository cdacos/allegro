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
