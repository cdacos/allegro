use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

use allegro_cwr::{process_cwr_stream_with_version, CwrRegistry};
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObfuscationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CWR parsing error: {0}")]
    CwrParsing(String),
}

/// Consistent obfuscation mappings for different types of data
#[derive(Debug, Default)]
pub struct ObfuscationMappings {
    /// Map original names to obfuscated names (publishers, writers, etc.)
    names: HashMap<String, String>,
    /// Map original titles to obfuscated titles
    titles: HashMap<String, String>,
    /// Map original IPI numbers to obfuscated IPI numbers
    ipis: HashMap<String, String>,
    /// Map original work numbers to obfuscated work numbers
    work_numbers: HashMap<String, String>,
    /// Map original ISWCs to obfuscated ISWCs
    iswcs: HashMap<String, String>,
}

impl ObfuscationMappings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or create an obfuscated name, ensuring consistency
    pub fn obfuscate_name(&mut self, original: &str) -> String {
        if original.trim().is_empty() {
            return original.to_string();
        }

        self.names.entry(original.to_string()).or_insert_with(|| generate_fake_name(original)).clone()
    }

    /// Get or create an obfuscated title, ensuring consistency
    pub fn obfuscate_title(&mut self, original: &str) -> String {
        if original.trim().is_empty() {
            return original.to_string();
        }

        self.titles.entry(original.to_string()).or_insert_with(|| generate_fake_title(original)).clone()
    }

    /// Get or create an obfuscated IPI number, ensuring consistency
    pub fn obfuscate_ipi(&mut self, original: &str) -> String {
        if original.trim().is_empty() {
            return original.to_string();
        }

        self.ipis.entry(original.to_string()).or_insert_with(|| generate_fake_ipi(original)).clone()
    }

    /// Get or create an obfuscated work number, ensuring consistency
    pub fn obfuscate_work_number(&mut self, original: &str) -> String {
        if original.trim().is_empty() {
            return original.to_string();
        }

        self.work_numbers.entry(original.to_string()).or_insert_with(|| generate_fake_work_number(original)).clone()
    }

    /// Get or create an obfuscated ISWC, ensuring consistency
    pub fn obfuscate_iswc(&mut self, original: &str) -> String {
        if original.trim().is_empty() {
            return original.to_string();
        }

        self.iswcs.entry(original.to_string()).or_insert_with(|| generate_fake_iswc(original)).clone()
    }
}

/// Generate a deterministic but obfuscated name based on original
fn generate_fake_name(original: &str) -> String {
    let hash = Sha256::digest(original.as_bytes());
    let seed = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Common fake publisher/writer names
    let prefixes = ["FAKE", "TEST", "DEMO", "SAMPLE", "MOCK"];
    let suffixes = ["MUSIC", "PUBLISHING", "RECORDS", "ENTERTAINMENT", "MEDIA", "WORKS"];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];
    let number = rng.gen_range(100..999);

    format!("{} {} {}", prefix, suffix, number)
}

/// Generate a deterministic but obfuscated title based on original
fn generate_fake_title(original: &str) -> String {
    let hash = Sha256::digest(original.as_bytes());
    let seed = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Common fake song title patterns
    let adjectives = ["DEMO", "TEST", "SAMPLE", "FAKE", "MOCK"];
    let nouns = ["SONG", "TRACK", "MELODY", "TUNE", "COMPOSITION"];

    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];
    let number = rng.gen_range(1..999);

    format!("{} {} {}", adjective, noun, number)
}

/// Generate a deterministic but obfuscated IPI number
fn generate_fake_ipi(original: &str) -> String {
    let hash = Sha256::digest(original.as_bytes());
    let seed = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Generate fake IPI maintaining same format (11 digits for IPI Name Number)
    if original.len() == 11 && original.chars().all(|c| c.is_ascii_digit()) {
        format!("{:011}", rng.gen_range(10000000000u64..99999999999u64))
    } else if original.len() == 13 {
        // IPI Base Number format (alphanumeric)
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
        (0..13).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
    } else {
        // Keep original format if we don't recognize it
        original.to_string()
    }
}

/// Generate a deterministic but obfuscated work number
fn generate_fake_work_number(original: &str) -> String {
    let hash = Sha256::digest(original.as_bytes());
    let seed = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Generate fake work number maintaining same length
    if original.chars().all(|c| c.is_ascii_digit()) {
        let num_digits = original.len();
        let min_val = 10_u64.pow((num_digits - 1) as u32);
        let max_val = 10_u64.pow(num_digits as u32) - 1;
        format!("{:0width$}", rng.gen_range(min_val..=max_val), width = num_digits)
    } else {
        // Keep original format for non-numeric work numbers
        original.to_string()
    }
}

/// Generate a deterministic but obfuscated ISWC
fn generate_fake_iswc(original: &str) -> String {
    let hash = Sha256::digest(original.as_bytes());
    let seed = u64::from_le_bytes(hash[0..8].try_into().unwrap());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // ISWC format: T-NNNNNNNN-C (T followed by 8 digits followed by check digit)
    if original.len() == 11 && original.starts_with('T') {
        let number = rng.gen_range(10000000u32..99999999u32);
        let check_digit = rng.gen_range(0..10);
        format!("T{:08}{}", number, check_digit)
    } else {
        // Keep original if format is unrecognized
        original.to_string()
    }
}

/// Process a CWR file and obfuscate sensitive information
pub fn process_cwr_obfuscation(input_path: &str, output_path: Option<&str>, cwr_version: Option<f32>) -> Result<usize, ObfuscationError> {
    let default_output = format!("{}.obfuscated", input_path);
    let output_path = output_path.unwrap_or(&default_output);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut mappings = ObfuscationMappings::new();
    let mut record_count = 0;

    // Use the allegro_cwr streaming parser
    let record_stream = process_cwr_stream_with_version(input_path, cwr_version).map_err(|e| ObfuscationError::CwrParsing(format!("Failed to open CWR file: {}", e)))?;

    for parsed_result in record_stream {
        match parsed_result {
            Ok(parsed_record) => {
                // Obfuscate the record
                let obfuscated_record = obfuscate_record(parsed_record.record, &mut mappings);

                // Convert back to CWR line and write
                let version = allegro_cwr::domain_types::CwrVersion(Some(parsed_record.context.cwr_version));
                let obfuscated_line = obfuscated_record.to_cwr_line(&version);
                writeln!(writer, "{}", obfuscated_line)?;
                record_count += 1;
            }
            Err(e) => {
                return Err(ObfuscationError::CwrParsing(format!("Parse error: {}", e)));
            }
        }
    }

    writer.flush()?;
    println!("Successfully obfuscated {} records to '{}'", record_count, output_path);

    Ok(record_count)
}

/// Obfuscate sensitive information in a CWR record
fn obfuscate_record(record: CwrRegistry, mappings: &mut ObfuscationMappings) -> CwrRegistry {
    match record {
        CwrRegistry::Hdr(mut hdr) => {
            // Obfuscate sender name (it's a domain type, access the inner string)
            let sender_name_str = &hdr.sender_name.0;
            let obfuscated = mappings.obfuscate_name(sender_name_str);
            // Create new SenderName with obfuscated value
            hdr.sender_name = allegro_cwr::domain_types::SenderName(obfuscated);
            CwrRegistry::Hdr(hdr)
        }
        CwrRegistry::Nwr(mut nwr) => {
            nwr.work_title = mappings.obfuscate_title(&nwr.work_title);
            nwr.submitter_work_num = mappings.obfuscate_work_number(&nwr.submitter_work_num);
            if let Some(ref iswc) = nwr.iswc {
                nwr.iswc = Some(mappings.obfuscate_iswc(iswc));
            }
            CwrRegistry::Nwr(nwr)
        }
        CwrRegistry::Swr(mut swr) => {
            if let Some(ref last_name) = swr.writer_last_name {
                swr.writer_last_name = Some(mappings.obfuscate_name(last_name));
            }
            if let Some(ref first_name) = swr.writer_first_name {
                swr.writer_first_name = Some(mappings.obfuscate_name(first_name));
            }
            if let Some(ref ipi_name) = swr.writer_ipi_name_num {
                swr.writer_ipi_name_num = Some(mappings.obfuscate_ipi(ipi_name));
            }
            if let Some(ref ipi_base) = swr.writer_ipi_base_number {
                swr.writer_ipi_base_number = Some(mappings.obfuscate_ipi(ipi_base));
            }
            CwrRegistry::Swr(swr)
        }
        CwrRegistry::Spu(mut spu) => {
            if let Some(ref pub_name) = spu.publisher_name {
                spu.publisher_name = Some(mappings.obfuscate_name(pub_name));
            }
            if let Some(ref ipi_name) = spu.publisher_ipi_name_num {
                spu.publisher_ipi_name_num = Some(mappings.obfuscate_ipi(ipi_name));
            }
            if let Some(ref ipi_base) = spu.publisher_ipi_base_number {
                spu.publisher_ipi_base_number = Some(mappings.obfuscate_ipi(ipi_base));
            }
            CwrRegistry::Spu(spu)
        }
        CwrRegistry::Alt(mut alt) => {
            alt.alternate_title = mappings.obfuscate_title(&alt.alternate_title);
            CwrRegistry::Alt(alt)
        }
        CwrRegistry::Per(mut per) => {
            per.performing_artist_last_name = mappings.obfuscate_name(&per.performing_artist_last_name);
            if let Some(ref first_name) = per.performing_artist_first_name {
                per.performing_artist_first_name = Some(mappings.obfuscate_name(first_name));
            }
            if let Some(ref ipi_name) = per.performing_artist_ipi_name_num {
                per.performing_artist_ipi_name_num = Some(mappings.obfuscate_ipi(ipi_name));
            }
            if let Some(ref ipi_base) = per.performing_artist_ipi_base_number {
                per.performing_artist_ipi_base_number = Some(mappings.obfuscate_ipi(ipi_base));
            }
            CwrRegistry::Per(per)
        }
        CwrRegistry::Rec(mut rec) => {
            if let Some(ref album_title) = rec.album_title {
                rec.album_title = Some(mappings.obfuscate_title(album_title));
            }
            if let Some(ref album_label) = rec.album_label {
                rec.album_label = Some(mappings.obfuscate_name(album_label));
            }
            if let Some(ref recording_title) = rec.recording_title {
                rec.recording_title = Some(mappings.obfuscate_title(recording_title));
            }
            if let Some(ref version_title) = rec.version_title {
                rec.version_title = Some(mappings.obfuscate_title(version_title));
            }
            if let Some(ref display_artist) = rec.display_artist {
                rec.display_artist = Some(mappings.obfuscate_name(display_artist));
            }
            if let Some(ref record_label) = rec.record_label {
                rec.record_label = Some(mappings.obfuscate_name(record_label));
            }
            CwrRegistry::Rec(rec)
        }
        CwrRegistry::Com(mut com) => {
            com.title = mappings.obfuscate_title(&com.title);
            if let Some(ref iswc) = com.iswc_of_component {
                com.iswc_of_component = Some(mappings.obfuscate_iswc(iswc));
            }
            if let Some(ref work_num) = com.submitter_work_num {
                com.submitter_work_num = Some(mappings.obfuscate_work_number(work_num));
            }
            com.writer_1_last_name = mappings.obfuscate_name(&com.writer_1_last_name);
            if let Some(ref first_name) = com.writer_1_first_name {
                com.writer_1_first_name = Some(mappings.obfuscate_name(first_name));
            }
            if let Some(ref writer_2_last) = com.writer_2_last_name {
                com.writer_2_last_name = Some(mappings.obfuscate_name(writer_2_last));
            }
            if let Some(ref writer_2_first) = com.writer_2_first_name {
                com.writer_2_first_name = Some(mappings.obfuscate_name(writer_2_first));
            }
            if let Some(ref ipi_name) = com.writer_1_ipi_name_num {
                com.writer_1_ipi_name_num = Some(mappings.obfuscate_ipi(ipi_name));
            }
            if let Some(ref ipi_name) = com.writer_2_ipi_name_num {
                com.writer_2_ipi_name_num = Some(mappings.obfuscate_ipi(ipi_name));
            }
            if let Some(ref ipi_base) = com.writer_1_ipi_base_number {
                com.writer_1_ipi_base_number = Some(mappings.obfuscate_ipi(ipi_base));
            }
            if let Some(ref ipi_base) = com.writer_2_ipi_base_number {
                com.writer_2_ipi_base_number = Some(mappings.obfuscate_ipi(ipi_base));
            }
            CwrRegistry::Com(com)
        }
        // For other record types that don't contain sensitive information,
        // pass them through unchanged
        _ => record,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_name_obfuscation() {
        let mut mappings = ObfuscationMappings::new();

        // Same name should always map to the same obfuscated value
        let original_name = "ACME PUBLISHING";
        let obfuscated1 = mappings.obfuscate_name(original_name);
        let obfuscated2 = mappings.obfuscate_name(original_name);

        assert_eq!(obfuscated1, obfuscated2);
        assert_ne!(obfuscated1, original_name);
    }

    #[test]
    fn test_consistent_title_obfuscation() {
        let mut mappings = ObfuscationMappings::new();

        // Same title should always map to the same obfuscated value
        let original_title = "MY AMAZING SONG";
        let obfuscated1 = mappings.obfuscate_title(original_title);
        let obfuscated2 = mappings.obfuscate_title(original_title);

        assert_eq!(obfuscated1, obfuscated2);
        assert_ne!(obfuscated1, original_title);
    }

    #[test]
    fn test_consistent_ipi_obfuscation() {
        let mut mappings = ObfuscationMappings::new();

        // Same IPI should always map to the same obfuscated value
        let original_ipi = "12345678901";
        let obfuscated1 = mappings.obfuscate_ipi(original_ipi);
        let obfuscated2 = mappings.obfuscate_ipi(original_ipi);

        assert_eq!(obfuscated1, obfuscated2);
        assert_ne!(obfuscated1, original_ipi);
        assert_eq!(obfuscated1.len(), original_ipi.len()); // Same length
    }

    #[test]
    fn test_different_names_get_different_obfuscations() {
        let mut mappings = ObfuscationMappings::new();

        let name1 = "ACME PUBLISHING";
        let name2 = "XYZ RECORDS";

        let obfuscated1 = mappings.obfuscate_name(name1);
        let obfuscated2 = mappings.obfuscate_name(name2);

        assert_ne!(obfuscated1, obfuscated2);
    }

    #[test]
    fn test_empty_strings_remain_empty() {
        let mut mappings = ObfuscationMappings::new();

        assert_eq!(mappings.obfuscate_name(""), "");
        assert_eq!(mappings.obfuscate_title("   "), "   ");
        assert_eq!(mappings.obfuscate_ipi(""), "");
    }

    #[test]
    fn test_ipi_format_preservation() {
        let mut mappings = ObfuscationMappings::new();

        // Test IPI Name Number (11 digits)
        let ipi_name = "12345678901";
        let obfuscated_name = mappings.obfuscate_ipi(ipi_name);
        assert_eq!(obfuscated_name.len(), 11);
        assert!(obfuscated_name.chars().all(|c| c.is_ascii_digit()));

        // Test IPI Base Number (13 alphanumeric)
        let ipi_base = "ABCD123456789";
        let obfuscated_base = mappings.obfuscate_ipi(ipi_base);
        assert_eq!(obfuscated_base.len(), 13);
        assert!(obfuscated_base.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_deterministic_obfuscation() {
        // Test that the same input always produces the same output
        // even across different ObfuscationMappings instances
        let mut mappings1 = ObfuscationMappings::new();
        let mut mappings2 = ObfuscationMappings::new();

        let original = "TEST PUBLISHER";
        let obfuscated1 = mappings1.obfuscate_name(original);
        let obfuscated2 = mappings2.obfuscate_name(original);

        assert_eq!(obfuscated1, obfuscated2);
    }
}
