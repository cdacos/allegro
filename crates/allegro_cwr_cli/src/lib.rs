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
