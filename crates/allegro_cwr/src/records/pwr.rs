use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// PWR - Publisher for Writer Record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(validator = pwr_custom_validate, test_data = "PWR0000000000000325ABKC     ABKCO MUSIC INC.                                                         WOMA     01")]
pub struct PwrRecord {
    #[cwr(title = "Always 'PWR'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Publisher IP number (conditional)", start = 19, len = 9)]
    pub publisher_ip_num: Option<String>,

    #[cwr(title = "Publisher name (conditional)", start = 28, len = 45)]
    pub publisher_name: Option<String>,

    #[cwr(title = "Submitter agreement number (optional)", start = 73, len = 14)]
    pub submitter_agreement_number: Option<String>,

    #[cwr(title = "Society-assigned agreement number (optional)", start = 87, len = 14)]
    pub society_assigned_agreement_number: Option<String>,

    #[cwr(title = "Writer IP number (conditional, v2.1+)", start = 101, len = 9, min_version = 2.1)]
    pub writer_ip_num: Option<String>,

    #[cwr(title = "Publisher sequence number (v2.2+)", start = 110, len = 2, min_version = 2.2)]
    pub publisher_sequence_num: Option<PublisherSequenceNumber>,
}

// Custom validation function for PWR record
fn pwr_custom_validate(record: &mut PwrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Publisher identification - either Publisher IP Number or Publisher Name required
    if (record.publisher_ip_num.is_none() || record.publisher_ip_num.as_ref().is_none_or(|s| s.trim().is_empty()))
        && (record.publisher_name.is_none() || record.publisher_name.as_ref().is_none_or(|s| s.trim().is_empty()))
    {
        warnings.push(CwrWarning {
            field_name: "publisher_ip_num",
            field_title: "Publisher IP number (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Either Publisher IP Number or Publisher Name must be provided".to_string(),
        });
    }

    // Business rule: Publisher sequence numbers must be sequential within a work
    // This requires cross-record validation which would be implemented in a post-processing step

    // TODO: Additional business rules requiring broader context:
    // - Must follow a SWR record (requires parsing context)
    // - Publisher IP numbers must be valid IPI numbers (requires IPI lookup)
    // - Agreement numbers must follow proper format (submitter vs society-assigned)
    // - Writer IP number cross-validation with preceding SWR record
    // - Publisher sequence numbers must be sequential starting from 01

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_types::CwrVersion;

    #[test]
    fn test_version_aware_writing() {
        // Create a test PWR record
        let pwr = PwrRecord {
            record_type: "PWR".to_string(),
            transaction_sequence_num: Number(3),
            record_sequence_num: Number(25),
            publisher_ip_num: Some("ABKC     ".to_string()),
            publisher_name: Some("ABKCO MUSIC INC.                     ".to_string()),
            submitter_agreement_number: None,
            society_assigned_agreement_number: None,
            writer_ip_num: Some("WOMA     ".to_string()),
            publisher_sequence_num: Some(PublisherSequenceNumber(1)),
        };

        // Test with different versions
        let version_20 = CwrVersion(2.0);
        let version_21 = CwrVersion(2.1);
        let version_22 = CwrVersion(2.2);

        let line_20 = pwr.to_cwr_line_without_newline(&version_20);
        let line_21 = pwr.to_cwr_line_without_newline(&version_21);
        let line_22 = pwr.to_cwr_line_without_newline(&version_22);

        println!("Version 2.0 line: {} (length: {})", line_20, line_20.len());
        println!("Version 2.1 line: {} (length: {})", line_21, line_21.len());
        println!("Version 2.2 line: {} (length: {})", line_22, line_22.len());

        // Version 2.0 should be shortest (no version-specific fields)
        assert!(line_20.len() < line_21.len());

        // Version 2.1 should include writer_ip_num but not publisher_sequence_num
        assert!(line_21.len() < line_22.len());

        // Version 2.2 should be longest (includes all fields)
        assert!(line_22.len() > line_21.len());

        // Check that the version-specific fields are correctly included/excluded
        // This is a basic test - exact positioning would require more detailed verification
        assert!(!line_20.contains("WOMA")); // v2.0 shouldn't have writer_ip_num
        assert!(line_21.contains("WOMA")); // v2.1 should have writer_ip_num
        assert!(line_22.contains("WOMA")); // v2.2 should have writer_ip_num
        assert!(line_22.contains("01")); // v2.2 should have publisher_sequence_num
    }
}
