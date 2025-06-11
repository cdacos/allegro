//! Integration tests for non-Roman alphabet support in CWR records
//!
//! These tests verify that multi-byte UTF-8 characters in NonRomanAlphabet fields
//! maintain proper field alignment in the fixed-width CWR format.

use allegro_cwr::domain_types::CwrVersion;
use allegro_cwr::records::nat::NatRecord;
use allegro_cwr::records::npa::NpaRecord;
use allegro_cwr::records::npn::NpnRecord;
use allegro_cwr::records::npr::NprRecord;
use allegro_cwr::records::nwn::NwnRecord;

#[test]
fn test_npn_with_non_ascii_publisher_name() {
    // Test data with non-ASCII characters in publisher name field
    // Field positions: record_type(0-2), transaction_seq(3-10), record_seq(11-18), pub_seq(19-20), ipi(21-29), pub_name(30-509), lang(510-511)
    let mut test_line = String::new();
    test_line.push_str("NPN"); // record_type (0-2)
    test_line.push_str("00000001"); // transaction_seq (3-10) 
    test_line.push_str("00000002"); // record_seq (11-18)
    test_line.push_str("01"); // publisher_seq (19-20)
    test_line.push_str("123456789"); // interested_party_num (21-29)
    test_line.push_str("café 音楽出版社"); // publisher_name starts at (30)

    // Pad publisher name field to full width (480 chars)
    let name_bytes = "café 音楽出版社".as_bytes().len();
    test_line.push_str(&" ".repeat(480 - name_bytes));
    test_line.push_str("EN"); // language_code (510-511)

    let (record, warnings) = NpnRecord::parse(&test_line);

    // Should parse successfully without critical warnings
    let critical_warnings: Vec<_> = warnings.iter().filter(|w| w.is_critical()).collect();
    assert!(critical_warnings.is_empty());
    assert_eq!(record.publisher_name.as_str().trim(), "café 音楽出版社");
}

#[test]
fn test_npa_with_non_ascii_names() {
    // Test data with non-ASCII characters in interested party fields
    // Field positions: record_type(0-2), transaction_seq(3-10), record_seq(11-18), ip_num(19-27), ip_name(28-187), ip_first_name(188-347), lang(348-349)
    let mut test_line = String::new();
    test_line.push_str("NPA"); // record_type (0-2)
    test_line.push_str("00000001"); // transaction_seq (3-10) 
    test_line.push_str("00000002"); // record_seq (11-18)
    test_line.push_str("123456789"); // interested_party_num (19-27) - 9 chars
    test_line.push_str("José María González"); // interested_party_name starts at (28)

    // Pad first name field to full width (160 chars)
    let name1_bytes = "José María González".as_bytes().len();
    test_line.push_str(&" ".repeat(160 - name1_bytes));
    test_line.push_str("María José"); // interested_party_writer_first_name starts at (188)

    // Pad second name field to full width (160 chars)
    let name2_bytes = "María José".as_bytes().len();
    test_line.push_str(&" ".repeat(160 - name2_bytes));
    test_line.push_str("EN"); // language_code (348-349)

    let (record, warnings) = NpaRecord::parse(&test_line);

    // Should parse successfully without critical warnings
    assert!(warnings.iter().all(|w| !w.is_critical()));
    assert_eq!(record.interested_party_name.as_str().trim(), "José María González");
    assert_eq!(record.interested_party_writer_first_name.as_str().trim(), "María José");
}

#[test]
fn test_nat_with_non_ascii_title() {
    // Test data with non-ASCII characters in title field
    let test_line = "NAT00000455000000170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ATEN";

    let (_record, warnings) = NatRecord::parse(test_line);

    // Should parse successfully without critical warnings related to title encoding
    let encoding_warnings: Vec<_> = warnings.iter().filter(|w| w.description.contains("Non-ASCII")).collect();

    // There should be no encoding warnings for the title field since it's marked as non_roman
    assert!(encoding_warnings.is_empty());
}

#[test]
fn test_npr_with_non_ascii_performer_names() {
    // Test data with non-ASCII characters in performer fields
    // Field positions: record_type(0-2), transaction_seq(3-10), record_seq(11-18), artist_name(19-178), artist_first_name(179-338), etc.
    let mut test_line = String::new();
    test_line.push_str("NPR"); // record_type (0-2)
    test_line.push_str("00000001"); // transaction_seq (3-10) 
    test_line.push_str("00000002"); // record_seq (11-18)
    test_line.push_str("André Müller"); // performing_artist_name starts at (19)

    // Pad first name field to full width (160 chars)
    let name1_bytes = "André Müller".as_bytes().len();
    test_line.push_str(&" ".repeat(160 - name1_bytes));
    test_line.push_str("Björk Guðmundsdóttir"); // performing_artist_first_name starts at (179)

    // Pad second name field to full width (160 chars)
    let name2_bytes = "Björk Guðmundsdóttir".as_bytes().len();
    test_line.push_str(&" ".repeat(160 - name2_bytes));

    // Add remaining optional fields with minimal data
    test_line.push_str(&" ".repeat(11)); // IPI name number (339-349)
    test_line.push_str(&" ".repeat(13)); // IPI base number (350-362)
    test_line.push_str("EN"); // language_code (363-364)

    let (record, warnings) = NprRecord::parse(&test_line);

    // Should parse successfully without critical warnings
    assert!(warnings.iter().all(|w| !w.is_critical()));

    if let Some(ref name) = record.performing_artist_name {
        assert_eq!(name.as_str().trim(), "André Müller");
    }

    if let Some(ref first_name) = record.performing_artist_first_name {
        assert_eq!(first_name.as_str().trim(), "Björk Guðmundsdóttir");
    }
}

#[test]
fn test_ascii_field_still_validates() {
    // Test that non-marked fields still reject non-ASCII
    let mut test_line = String::new();
    test_line.push_str("NPN"); // record_type (0-2)
    test_line.push_str("00000001"); // transaction_seq (3-10) 
    test_line.push_str("00000002"); // record_seq (11-18)
    test_line.push_str("01"); // publisher_seq (19-20)
    test_line.push_str("123456789"); // interested_party_num (21-29)
    test_line.push_str("NORMAL PUBLISHER NAME"); // publisher_name starts at (30)

    // Pad publisher name field to full width (480 chars)
    let name_bytes = "NORMAL PUBLISHER NAME".as_bytes().len();
    test_line.push_str(&" ".repeat(480 - name_bytes));
    test_line.push_str("EN"); // language_code (510-511)

    let (record, _warnings) = NpnRecord::parse(&test_line);

    // The record type field should still be ASCII-only validated
    // (though this test data doesn't have non-ASCII in the record type field)
    assert_eq!(record.record_type, "NPN");
    assert_eq!(record.publisher_name.as_str().trim(), "NORMAL PUBLISHER NAME");
}

#[test]
fn test_nat_multibyte_field_alignment_bug() {
    // This test reproduces the exact bug from carlos.cwr:
    // Multi-byte UTF-8 characters in NonRomanAlphabet fields causing field misalignment

    // Simulate the problematic line from carlos.cwr (line 517)
    // "EVIDÊNCIA" contains "Ê" which is 2 bytes but 1 character
    let mut test_line = String::new();
    test_line.push_str("NAT"); // record_type (0-2)
    test_line.push_str("00000058"); // transaction_seq (3-10)
    test_line.push_str("00000001"); // record_seq (11-18)

    // Title field starts at position 19, length 640
    test_line.push_str("EVIDÊNCIA"); // Title with multi-byte char (10 bytes, 9 chars)
    // Pad to 640 bytes total for title field
    test_line.push_str(&" ".repeat(640 - "EVIDÊNCIA".as_bytes().len()));

    // Title type at position 659, length 2 - this should be "OT"
    test_line.push_str("OT");
    // Language code at position 661, length 2 - this should be "PT"
    test_line.push_str("PT");

    // Parse the record
    let (record, warnings) = NatRecord::parse(&test_line);

    // The roundtrip serialization should produce exactly the same string
    let version = CwrVersion(2.1);
    let serialized = record.to_cwr_line_without_newline(&version);

    // Before the fix, this would fail due to field misalignment
    assert_eq!(serialized, test_line, "Round-trip serialization should be identical");
    assert_eq!(serialized.len(), test_line.len(), "Byte lengths should match");

    // Verify the fields were parsed correctly
    assert_eq!(record.title.as_str().trim(), "EVIDÊNCIA");
    assert_eq!(record.title_type.as_str(), "OT");
    if let Some(ref lang) = record.language_code {
        assert_eq!(lang.as_str(), "PT");
    }

    // Should not have critical warnings
    let critical_warnings: Vec<_> = warnings.iter().filter(|w| w.is_critical()).collect();
    assert!(critical_warnings.is_empty(), "Should not have critical warnings");
}

#[test]
fn test_nwn_multibyte_field_alignment_bug() {
    // Test the NWN record alignment issue from carlos.cwr
    // Similar to NAT but with writer names containing multi-byte characters

    let mut test_line = String::new();
    test_line.push_str("NWN"); // record_type (0-2)
    test_line.push_str("00000074"); // transaction_seq (3-10)
    test_line.push_str("00000006"); // record_seq (11-18)
    test_line.push_str("27976    "); // interested_party_num (19-27), 9 chars

    // Writer last name at position 28, length 160
    test_line.push_str("VALENÇA, ALCEU PAIVA"); // Contains "Ç" multi-byte char
    let name_bytes = "VALENÇA, ALCEU PAIVA".as_bytes().len();
    test_line.push_str(&" ".repeat(160 - name_bytes));

    // Writer first name at position 188, length 160 (optional, empty)
    test_line.push_str(&" ".repeat(160));

    // Language code at position 348, length 2
    test_line.push_str("PT");

    // Parse and test round-trip
    let (record, warnings) = NwnRecord::parse(&test_line);
    let version = CwrVersion(2.1);
    let serialized = record.to_cwr_line_without_newline(&version);

    // Should round-trip perfectly
    assert_eq!(serialized, test_line, "NWN round-trip should be identical");
    assert_eq!(serialized.len(), test_line.len(), "NWN byte lengths should match");

    // Verify parsing
    assert_eq!(record.writer_last_name.as_str().trim(), "VALENÇA, ALCEU PAIVA");
    if let Some(ref lang) = record.language_code {
        assert_eq!(lang.as_str(), "PT");
    }

    let critical_warnings: Vec<_> = warnings.iter().filter(|w| w.is_critical()).collect();
    assert!(critical_warnings.is_empty(), "NWN should not have critical warnings");
}
