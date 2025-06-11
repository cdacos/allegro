use std::fs;
use std::io::{self, Read};
use std::process;
use std::time::Instant;

use log::{error, info};

#[derive(Default)]
pub struct BaseConfig {
    pub input_files: Vec<String>,
    pub cwr_version: Option<f32>,
    pub read_stdin: bool,
}

impl BaseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_cwr_version(&mut self, version_str: &str) -> Result<(), String> {
        let version: f32 = version_str
            .parse()
            .map_err(|_| format!("Invalid CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version_str))?;

        if ![2.0, 2.1, 2.2].contains(&version) {
            return Err(format!("Unsupported CWR version '{}'. Valid versions: 2.0, 2.1, 2.2", version));
        }

        self.cwr_version = Some(version);
        Ok(())
    }

    pub fn add_input_file(&mut self, file: String) {
        self.input_files.push(file);
    }

    pub fn finalize(&mut self) {
        if self.input_files.is_empty() {
            self.read_stdin = true;
        }
    }
}

pub fn get_value(parser: &mut lexopt::Parser, arg_name: &str) -> Result<String, String> {
    parser
        .value()
        .map(|val| val.to_string_lossy().to_string())
        .map_err(|e| format!("Missing value for --{}: {}", arg_name, e))
}

pub fn process_stdin_with_temp_file<F, T>(temp_file_prefix: &str, processor: F, start_time: Instant) -> T
where
    F: FnOnce(&str, Instant) -> T,
{
    info!("Reading CWR data from stdin");

    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        eprintln!("Error reading from stdin: {}", e);
        process::exit(1);
    }

    let temp_file = std::env::temp_dir().join(format!("{}.tmp", temp_file_prefix));
    if let Err(e) = fs::write(&temp_file, &buffer) {
        eprintln!("Error writing temporary file: {}", e);
        process::exit(1);
    }

    let temp_path = temp_file.to_string_lossy();
    let result = processor(&temp_path, start_time);

    let _ = fs::remove_file(&temp_file);
    result
}

pub fn init_logging_and_parse_args<F, T>(parser_fn: F) -> T
where
    F: FnOnce() -> Result<T, String>,
{
    env_logger::init();

    match parser_fn() {
        Ok(config) => config,
        Err(e) => {
            error!("Configuration error: {}", e);
            process::exit(1);
        }
    }
}

/// Finds the next available filename by incrementing the index
pub fn find_next_available_filename(base_name: &str, start_index: usize) -> String {
    let mut index = start_index;
    loop {
        let candidate = if let Some(dot_pos) = base_name.rfind('.') {
            format!("{}.{}{}", &base_name[..dot_pos], index, &base_name[dot_pos..])
        } else {
            format!("{}.{}", base_name, index)
        };
        if !std::path::Path::new(&candidate).exists() {
            return candidate;
        }
        index += 1;
    }
}

/// Generates a default output filename based on input filename and extension
/// Finds the next available filename to avoid overwriting existing files
pub fn generate_default_output_filename(input_filename: &str, extension: &str) -> String {
    let base_name = format!("{}.{}", input_filename, extension);
    if !std::path::Path::new(&base_name).exists() {
        base_name
    } else {
        find_next_available_filename(&base_name, 1)
    }
}

/// Determines the output filename for a given input file when processing files
/// For single files: uses the exact filename (overwrites if exists) or None if no output specified
/// For multiple files: finds the next available filename by incrementing the index, or generates default if no output specified
/// Returns the output filename or None if no output filename was specified and single file
pub fn get_output_filename_for_multiple_files(
    base_output_filename: Option<&str>, input_file_count: usize, current_file_index: usize,
) -> Option<String> {
    match (base_output_filename, input_file_count > 1) {
        (Some(base_name), true) => {
            // Multiple files with -o: use incremental naming
            Some(find_next_available_filename(base_name, current_file_index + 1))
        }
        (Some(base_name), false) => {
            // Single file with -o: use exact filename (overwrite if exists)
            Some(base_name.to_string())
        }
        (None, _) => None,
    }
}

/// Determines the output filename, generating defaults for multiple files when no -o specified
/// For single files without -o: returns None (stdout/default behavior)
/// For multiple files without -o: generates default output filenames with the given extension
/// For files with -o: uses get_output_filename_for_multiple_files logic
pub fn get_output_filename_with_default_extension(
    base_output_filename: Option<&str>, input_filename: &str, input_file_count: usize, current_file_index: usize,
    default_extension: &str,
) -> Option<String> {
    match (base_output_filename, input_file_count > 1) {
        (Some(_), _) => {
            // Use existing logic when -o is specified
            get_output_filename_for_multiple_files(base_output_filename, input_file_count, current_file_index)
        }
        (None, true) => {
            // Multiple files without -o: generate default output filename
            Some(generate_default_output_filename(input_filename, default_extension))
        }
        (None, false) => {
            // Single file without -o: use default behavior (stdout/etc)
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_available_filename_with_extension() {
        // Test with extension - should insert index before extension
        let result = find_next_available_filename("test.json", 1);
        assert_eq!(result, "test.1.json");
    }

    #[test]
    fn test_find_next_available_filename_no_extension() {
        // Test without extension - should append index
        let result = find_next_available_filename("test", 1);
        assert_eq!(result, "test.1");
    }

    #[test]
    fn test_find_next_available_filename_multiple_dots() {
        // Test with multiple dots - should use rightmost dot
        let result = find_next_available_filename("my.test.json", 1);
        assert_eq!(result, "my.test.1.json");
    }

    #[test]
    fn test_find_next_available_filename_dot_at_start() {
        // Test with dot at start - rfind finds the dot, so it inserts before it
        let result = find_next_available_filename(".config", 1);
        assert_eq!(result, ".1.config");
    }

    #[test]
    fn test_find_next_available_filename_hidden_file_with_extension() {
        // Test hidden file with extension
        let result = find_next_available_filename(".test.json", 1);
        assert_eq!(result, ".test.1.json");
    }
}
