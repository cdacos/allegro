use std::collections::HashMap;

use allegro_cwr::process_cwr_stream_with_version;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoundtripError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CWR parsing error: {0}")]
    CwrParsing(String),
}

/// Check round-trip integrity by parsing CWR records and serializing them back
pub fn check_roundtrip_integrity(input_path: &str, cwr_version: Option<f32>) -> Result<usize, RoundtripError> {
    let mut record_count = 0;
    let mut diff_map: HashMap<String, Vec<usize>> = HashMap::new(); // key: diff description, value: line numbers
    let mut diff_examples: HashMap<String, (String, String, usize)> = HashMap::new(); // key: diff description, value: (original, serialized, line_number)
    let mut extra_chars_map: HashMap<String, Vec<usize>> = HashMap::new(); // key: "record_type:extra_char", value: line numbers
    let mut detected_version: Option<f32> = None;
    let mut warning_counts: HashMap<String, Vec<usize>> = HashMap::new(); // key: warning description, value: line numbers

    // Read original lines for comparison
    let original_lines: Vec<String> = std::fs::read_to_string(input_path)?.lines().map(|s| s.to_string()).collect();

    // Use the allegro_cwr streaming parser
    let record_stream = process_cwr_stream_with_version(input_path, cwr_version).map_err(|e| RoundtripError::CwrParsing(format!("Failed to open CWR file: {}", e)))?;

    for parsed_result in record_stream {
        match parsed_result {
            Ok(parsed_record) => {
                // Capture the detected version from the first record
                if detected_version.is_none() {
                    detected_version = Some(parsed_record.context.cwr_version);
                    println!("Detected CWR version: {}", parsed_record.context.cwr_version);
                }

                let line_index = parsed_record.line_number - 1; // Convert to 0-based index
                if line_index < original_lines.len() {
                    let original_line = &original_lines[line_index];

                    // Serialize the parsed record back to CWR line
                    let version = allegro_cwr::domain_types::CwrVersion(parsed_record.context.cwr_version);
                    let serialized_line = parsed_record.record.to_cwr_line(&version);

                    // Check for character differences
                    check_character_differences(original_line, &serialized_line, parsed_record.record.record_type(), parsed_record.line_number, &mut diff_map, &mut diff_examples, &mut extra_chars_map);
                }

                // Collect warnings from this record
                for warning in &parsed_record.warnings {
                    // Prefix warning with record type for consistent formatting
                    let record_type = parsed_record.record.record_type();
                    let formatted_warning = format!("{}: {}", record_type, warning);
                    warning_counts.entry(formatted_warning).or_default().push(parsed_record.line_number);
                }

                record_count += 1;
            }
            Err(e) => {
                return Err(RoundtripError::CwrParsing(format!("Parse error: {}", e)));
            }
        }
    }
    println!();

    // Report all warnings in a consolidated section
    if !warning_counts.is_empty() || !extra_chars_map.is_empty() {
        let total_issues = warning_counts.len() + extra_chars_map.len();
        println!("WARNINGS: Found {} distinct types of validation issues:", total_issues);

        // First show parsing warnings with consistent formatting
        if !warning_counts.is_empty() {
            // Sort warnings by count (descending) and then by description
            let mut sorted_warnings: Vec<_> = warning_counts.iter().collect();
            sorted_warnings.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));

            for (warning, line_numbers) in sorted_warnings {
                let display_lines = if line_numbers.len() <= 5 { format!("lines {:?}", line_numbers) } else { format!("{} occurrences (first few: {})", line_numbers.len(), line_numbers.iter().take(3).map(|n| n.to_string()).collect::<Vec<_>>().join(", ")) };
                println!("{}: {}", warning, display_lines);
            }
        }

        // Then show format differences (missing optional fields, extra chars, etc.)
        if !extra_chars_map.is_empty() {
            let mut sorted_extra: Vec<_> = extra_chars_map.iter().collect();
            sorted_extra.sort_by_key(|(key, lines)| (key.as_str(), lines.len()));

            if sorted_extra.len() > 0 {
                println!("\nAMBIGUOUS:");
            }

            for (extra_key, line_numbers) in sorted_extra {
                let parts: Vec<&str> = extra_key.split(':').collect();
                let record_type = parts[0];
                let extra_info = parts.get(1).unwrap_or(&"?");
                let display_lines = if line_numbers.len() <= 5 { format!("lines {:?}", line_numbers) } else { format!("{} occurrences (first few: {})", line_numbers.len(), line_numbers.iter().take(3).map(|n| n.to_string()).collect::<Vec<_>>().join(", ")) };

                if *extra_info == "missing_optional_fields" {
                    println!("{}: missing optional fields (serializer adds proper padding): {}", record_type, display_lines);
                } else {
                    println!("{}: records with extra '{}': {}", record_type, extra_info, display_lines);
                }
            }
        }
        println!();
    }

    if !diff_map.is_empty() {
        println!("ROUNDTRIP FAILED: Found {} distinct diff types across {} total errors:", diff_map.len(), diff_map.values().map(|v| v.len()).sum::<usize>());
        let mut sorted_diffs: Vec<_> = diff_map.iter().collect();
        sorted_diffs.sort_by_key(|(key, _)| key.as_str());

        for (diff_key, line_numbers) in sorted_diffs {
            let display_lines = if line_numbers.len() <= 5 { format!("{:?}", line_numbers) } else { format!("[{}, {}, {}, ...]", line_numbers[0], line_numbers[1], line_numbers[2]) };
            println!("  {}: {} occurrences on lines {}", diff_key, line_numbers.len(), display_lines);

            // Show visual diff for the first example
            if let Some((original, serialized, line_num)) = diff_examples.get(diff_key) {
                eprintln!("    Example from line {}:", line_num);
                eprintln!("    Original:   {}", original);
                eprintln!("    Serialized: {}", serialized);

                // Create visual diff indicator
                let mut diff_indicator = String::new();
                let original_chars: Vec<char> = original.chars().collect();
                let serialized_chars: Vec<char> = serialized.chars().collect();
                let max_len = original_chars.len().max(serialized_chars.len());

                for i in 0..max_len {
                    let orig_char = original_chars.get(i);
                    let ser_char = serialized_chars.get(i);

                    if orig_char != ser_char {
                        diff_indicator.push('^');
                        break;
                    } else {
                        diff_indicator.push('-');
                    }
                }

                eprintln!("    Diff:       {}", diff_indicator);
                eprintln!();
            }
        }
        return Err(RoundtripError::CwrParsing(format!("Round-trip integrity check failed with {} distinct error types", diff_map.len())));
    }

    println!("ROUNDTRIP PASSED: All {} records maintain round-trip integrity", record_count);
    Ok(record_count)
}

/// Check for character differences between original and round-trip serialized lines
fn check_character_differences(original: &str, serialized: &str, record_type: &str, line_number: usize, diff_map: &mut HashMap<String, Vec<usize>>, diff_examples: &mut HashMap<String, (String, String, usize)>, extra_chars_map: &mut HashMap<String, Vec<usize>>) {
    // Check length difference first
    if original.len() != serialized.len() {
        // Special handling for cases where original file is longer than CWR spec allows
        if original.len() > serialized.len() {
            // Check if the extra characters in the original are just trailing content
            let common_len = serialized.len();
            let common_original = &original[..common_len];

            // If the common part matches exactly, this is likely just extra trailing characters
            if common_original == serialized {
                let extra_chars = &original[common_len..];
                // Group by record type and extra character content
                let extra_key = format!("{}:{}", record_type, extra_chars);
                extra_chars_map.entry(extra_key).or_default().push(line_number);
                return; // Don't treat this as an error
            }
        }
        // Special handling for cases where original file is shorter (missing optional fields)
        else if original.len() < serialized.len() {
            // Check if the original matches the beginning of the serialized output
            if serialized.starts_with(original) {
                let missing_chars = &serialized[original.len()..];
                // If the missing part is just spaces/padding, this is expected behavior
                if missing_chars.chars().all(|c| c == ' ') {
                    let missing_key = format!("{}:missing_optional_fields", record_type);
                    extra_chars_map.entry(missing_key).or_default().push(line_number);
                    return; // Don't treat this as an error
                }
            }
        }

        let explanation = if original.len() > serialized.len() { " - source file may have extra characters beyond CWR specification" } else { " - source file missing version-specific fields, serializer adds required padding" };
        let diff_key = format!("record: {}, LENGTH_MISMATCH (original: {}, serialized: {}){}", record_type, original.len(), serialized.len(), explanation);
        diff_map.entry(diff_key.clone()).or_default().push(line_number);
        // Store example if this is the first occurrence
        diff_examples.entry(diff_key).or_insert_with(|| (original.to_string(), serialized.to_string(), line_number));
        return;
    }

    let original_chars: Vec<char> = original.chars().collect();
    let serialized_chars: Vec<char> = serialized.chars().collect();

    for (index, (orig_char, ser_char)) in original_chars.iter().zip(serialized_chars.iter()).enumerate() {
        if orig_char != ser_char {
            let diff_key = format!("record: {}, index: {}", record_type, index);
            diff_map.entry(diff_key.clone()).or_default().push(line_number);
            // Store example if this is the first occurrence
            diff_examples.entry(diff_key).or_insert_with(|| (original.to_string(), serialized.to_string(), line_number));
        }
    }
}
