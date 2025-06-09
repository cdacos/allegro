use std::collections::HashMap;

use allegro_cwr::{process_cwr_stream_with_version};
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
    let mut detected_version: Option<f32> = None;

    // Read original lines for comparison
    let original_lines: Vec<String> = std::fs::read_to_string(input_path)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    // Use the allegro_cwr streaming parser
    let record_stream = process_cwr_stream_with_version(input_path, cwr_version)
        .map_err(|e| RoundtripError::CwrParsing(format!("Failed to open CWR file: {}", e)))?;

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
                    let version = allegro_cwr::domain_types::CwrVersion(Some(parsed_record.context.cwr_version));
                    let serialized_line = parsed_record.record.to_cwr_line(&version);

                    // Check for character differences
                    check_character_differences(original_line, &serialized_line, parsed_record.record.record_type(), parsed_record.line_number, &mut diff_map, &mut diff_examples);
                }

                record_count += 1;
            }
            Err(e) => {
                return Err(RoundtripError::CwrParsing(format!("Parse error: {}", e)));
            }
        }
    }
    
    if !diff_map.is_empty() {
        println!("ROUNDTRIP FAILED: Found {} distinct diff types across {} total errors:", diff_map.len(), diff_map.values().map(|v| v.len()).sum::<usize>());
        let mut sorted_diffs: Vec<_> = diff_map.iter().collect();
        sorted_diffs.sort_by_key(|(key, _)| key.as_str());
        
        for (diff_key, line_numbers) in sorted_diffs {
            let display_lines = if line_numbers.len() <= 5 {
                format!("{:?}", line_numbers)
            } else {
                format!("[{}, {}, {}, ...]", line_numbers[0], line_numbers[1], line_numbers[2])
            };
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
fn check_character_differences(original: &str, serialized: &str, record_type: &str, line_number: usize, diff_map: &mut HashMap<String, Vec<usize>>, diff_examples: &mut HashMap<String, (String, String, usize)>) {
    // Check length difference first
    if original.len() != serialized.len() {
        let explanation = if original.len() > serialized.len() {
            " - source file may have extra characters beyond CWR specification"
        } else {
            " - source file missing version-specific fields, serializer adds required padding"
        };
        let diff_key = format!("record: {}, LENGTH_MISMATCH (original: {}, serialized: {}){}", record_type, original.len(), serialized.len(), explanation);
        diff_map.entry(diff_key.clone()).or_insert_with(Vec::new).push(line_number);
        // Store example if this is the first occurrence
        diff_examples.entry(diff_key).or_insert_with(|| (original.to_string(), serialized.to_string(), line_number));
        return;
    }

    let original_chars: Vec<char> = original.chars().collect();
    let serialized_chars: Vec<char> = serialized.chars().collect();

    for (index, (orig_char, ser_char)) in original_chars.iter().zip(serialized_chars.iter()).enumerate() {
        if orig_char != ser_char {
            let diff_key = format!("record: {}, index: {}", record_type, index);
            diff_map.entry(diff_key.clone()).or_insert_with(Vec::new).push(line_number);
            // Store example if this is the first occurrence
            diff_examples.entry(diff_key).or_insert_with(|| (original.to_string(), serialized.to_string(), line_number));
        }
    }
}