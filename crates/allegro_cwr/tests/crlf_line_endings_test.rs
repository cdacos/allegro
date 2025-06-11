//! Integration tests for CRLF line endings in CWR output
//!
//! The CWR specification requires that all lines end with CRLF (\r\n).
//! These tests verify that the to_cwr_line method produces output with proper line endings.

use allegro_cwr::domain_types::{CwrVersion, Number, PublisherSequenceNumber};
use allegro_cwr::records::pwr::PwrRecord;
use allegro_cwr::records::hdr::HdrRecord;
use allegro_cwr::records::nwr::NwrRecord; 

#[test]
fn test_pwr_record_has_crlf_ending() {
    let pwr = PwrRecord {
        record_type: "PWR".to_string(),
        transaction_sequence_num: Number(1),
        record_sequence_num: Number(2),
        publisher_ip_num: Some("TESTPUB  ".to_string()),
        publisher_name: Some("TEST PUBLISHER                        ".to_string()),
        submitter_agreement_number: None,
        society_assigned_agreement_number: None,
        writer_ip_num: Some("WRITER   ".to_string()),
        publisher_sequence_num: Some(PublisherSequenceNumber(1)),
    };

    let version = CwrVersion(2.2);
    let output = pwr.to_cwr_line(&version);
    
    assert!(
        output.ends_with("\r\n"),
        "PWR record output should end with CRLF (\\r\\n), but got: {:?}",
        output.chars().rev().take(5).collect::<String>().chars().rev().collect::<String>()
    );
}

#[test]
fn test_hdr_record_has_crlf_ending() {
    // Create minimal HDR record for testing
    let test_data = "HDRPB00000000019900101TESTDATA            01.1020230101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    let (hdr, _warnings) = HdrRecord::parse(test_data);
    
    let version = CwrVersion(2.2);
    let output = hdr.to_cwr_line(&version);
    
    assert!(
        output.ends_with("\r\n"),
        "HDR record output should end with CRLF (\\r\\n), but got: {:?}",
        output.chars().rev().take(5).collect::<String>().chars().rev().collect::<String>()
    );
}

#[test]
fn test_nwr_record_has_crlf_ending() {
    // Create minimal NWR record for testing
    let test_data = "NWR0000000100000001WORK TITLE                                                            T037369871400000000001990010120230101                                        UNC000000000000000000000000000000Y";
    let (nwr, _warnings) = NwrRecord::parse(test_data);
    
    let version = CwrVersion(2.2);
    let output = nwr.to_cwr_line(&version);
    
    assert!(
        output.ends_with("\r\n"),
        "NWR record output should end with CRLF (\\r\\n), but got: {:?}",
        output.chars().rev().take(5).collect::<String>().chars().rev().collect::<String>()
    );
}

#[test]
fn test_multiple_records_all_have_crlf_endings() {
    // Test that multiple different record types all produce CRLF endings
    let pwr = PwrRecord {
        record_type: "PWR".to_string(),
        transaction_sequence_num: Number(1),
        record_sequence_num: Number(2),
        publisher_ip_num: Some("TESTPUB  ".to_string()),
        publisher_name: Some("TEST PUBLISHER                        ".to_string()),
        submitter_agreement_number: None,
        society_assigned_agreement_number: None,
        writer_ip_num: Some("WRITER   ".to_string()),
        publisher_sequence_num: Some(PublisherSequenceNumber(1)),
    };

    let hdr_test_data = "HDRPB00000000019900101TESTDATA            01.1020230101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    let (hdr, _) = HdrRecord::parse(hdr_test_data);

    let nwr_test_data = "NWR0000000100000001WORK TITLE                                                            T037369871400000000001990010120230101                                        UNC000000000000000000000000000000Y";
    let (nwr, _) = NwrRecord::parse(nwr_test_data);

    let version = CwrVersion(2.2);
    
    let records_and_names = vec![
        (pwr.to_cwr_line(&version), "PWR"),
        (hdr.to_cwr_line(&version), "HDR"),
        (nwr.to_cwr_line(&version), "NWR"),
    ];
    
    for (output, record_type) in records_and_names {
        assert!(
            output.ends_with("\r\n"),
            "{} record output should end with CRLF (\\r\\n), but got: {:?}",
            record_type,
            output.chars().rev().take(5).collect::<String>().chars().rev().collect::<String>()
        );
    }
}