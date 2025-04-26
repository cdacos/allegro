use crate::db::log_error;
use crate::error::CwrParseError;
use crate::{db, error, record_handlers};
use rusqlite::{Connection, params}; // Added params
use std::fs::File;
use std::io::{self, BufRead, BufReader}; // Added io

// Context struct to hold file-level metadata like CWR version
#[derive(Debug, Clone)] // Added Clone for easier use if needed later
pub struct ParsingContext {
    pub cwr_version: String, // e.g., "2.0", "2.1", "2.2"
    // Add other metadata like charset later if needed
}


pub fn process_and_load_file(input_filename: &str, db_filename: &str) -> Result<usize, CwrParseError> {
    let file = File::open(input_filename)?;
    // Use BufReader::new directly, we need to consume the first line separately
    let mut reader = BufReader::new(file);

    // --- Read the first line to determine CWR version ---
    let mut first_line = String::new();
    let bytes_read = reader.read_line(&mut first_line)?;
    if bytes_read == 0 {
        return Err(CwrParseError::BadFormat("File is empty".to_string()));
    }
    let hdr_line = first_line.trim_end(); // Remove trailing newline chars

    if !hdr_line.starts_with("HDR") {
        return Err(CwrParseError::BadFormat(format!(
            "File does not start with HDR record. Found: '{}'",
            hdr_line.get(0..std::cmp::min(hdr_line.len(), 50)).unwrap_or("") // Show first 50 chars
        )));
    }

    // Determine version (Default to 2.0)
    let mut cwr_version = "2.0".to_string();
    // Check for explicit v2.2 version field (positions 101-104, 0-based)
    if let Some(version_str) = hdr_line.get(101..104) {
        if version_str == "022" { // CWR 2.2 spec uses "022"
             cwr_version = "2.2".to_string();
        } else if version_str == "021" { // Allow for explicit 2.1
             cwr_version = "2.1".to_string();
        }
        // Ignore other values for now, stick with default or length check
    }

    // If not explicitly 2.2 or 2.1, check length for implicit 2.1 (charset field starts at 86)
    // If length >= 87 (0-based index 86), it implies at least v2.1 structure
    if cwr_version == "2.0" && hdr_line.len() >= 87 {
         cwr_version = "2.1".to_string();
    }

    let context = ParsingContext { cwr_version };
    println!("Determined CWR Version: {}", context.cwr_version); // Log detected version

    // --- Setup Database and Transaction ---
    let mut conn = Connection::open(db_filename)?;
    // Set PRAGMAs before transaction
    conn.pragma_update(None, "journal_mode", "OFF")?;
    conn.pragma_update(None, "synchronous", "OFF")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;

    conn.pragma_update(None, "synchronous", "OFF")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;

    // Start transaction *before* preparing statements
    let tx = conn.transaction()?;
    let mut line_number: usize = 0; // Start at 0, HDR is line 1
    let mut processed_records: usize = 0;

    // Prepare all statements *using the transaction*
    // Keep the struct mutable as inserts happen within the handlers
    let mut prepared_statements = db::PreparedStatements {
        error_stmt: tx.prepare("INSERT INTO error (line_number, description) VALUES (?1, ?2)")?,
        file_stmt: tx.prepare("INSERT INTO file (line_number, record_type, record_id) VALUES (?1, ?2, ?3)")?,
            hdr_stmt: tx.prepare("INSERT INTO cwr_hdr (record_type, sender_type, sender_id, sender_name, edi_standard_version_number, creation_date, creation_time, transmission_date, character_set, version, revision, software_package, software_package_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)")?,
            grh_stmt: tx.prepare("INSERT INTO cwr_grh (record_type, transaction_type, group_id, version_number_for_this_transaction_type, batch_request, submission_distribution_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
            grt_stmt: tx.prepare("INSERT INTO cwr_grt (record_type, group_id, transaction_count, record_count, currency_indicator, total_monetary_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
            trl_stmt: tx.prepare("INSERT INTO cwr_trl (record_type, group_count, transaction_count, record_count) VALUES (?1, ?2, ?3, ?4)")?,
            agr_stmt: tx.prepare("INSERT INTO cwr_agr (record_type, transaction_sequence_num, record_sequence_num, submitter_agreement_number, international_standard_agreement_code, agreement_type, agreement_start_date, agreement_end_date, retention_end_date, prior_royalty_status, prior_royalty_start_date, post_term_collection_status, post_term_collection_end_date, date_of_signature_of_agreement, number_of_works, sales_manufacture_clause, shares_change, advance_given, society_assigned_agreement_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)")?,
            nwr_stmt: tx.prepare("INSERT INTO cwr_nwr (record_type, transaction_sequence_num, record_sequence_num, work_title, language_code, submitter_work_num, iswc, copyright_date, copyright_number, musical_work_distribution_category, duration, recorded_indicator, text_music_relationship, composite_type, version_type, excerpt_type, music_arrangement, lyric_adaptation, contact_name, contact_id, cwr_work_type, grand_rights_ind, composite_component_count, date_of_publication_of_printed_edition, exceptional_clause, opus_number, catalogue_number, priority_flag) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28)")?,
            ack_stmt: tx.prepare("INSERT INTO cwr_ack (record_type, transaction_sequence_num, record_sequence_num, creation_date, creation_time, original_group_id, original_transaction_sequence_num, original_transaction_type, creation_title, submitter_creation_num, recipient_creation_num, processing_date, transaction_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)")?,
            ter_stmt: tx.prepare("INSERT INTO cwr_ter (record_type, transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code) VALUES (?1, ?2, ?3, ?4, ?5)")?,
            ipa_stmt: tx.prepare("INSERT INTO cwr_ipa (record_type, transaction_sequence_num, record_sequence_num, agreement_role_code, interested_party_ipi_name_num, ipi_base_number, interested_party_num, interested_party_last_name, interested_party_writer_first_name, pr_affiliation_society, pr_share, mr_affiliation_society, mr_share, sr_affiliation_society, sr_share) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)")?,
            npa_stmt: tx.prepare("INSERT INTO cwr_npa (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, interested_party_name, interested_party_writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            spu_stmt: tx.prepare("INSERT INTO cwr_spu (record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, publisher_unknown_indicator, publisher_type, tax_id_num, publisher_ipi_name_num, submitter_agreement_number, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, special_agreements_indicator, first_recording_refusal_ind, filler, publisher_ipi_base_number, international_standard_agreement_code, society_assigned_agreement_number, agreement_type, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)")?,
            npn_stmt: tx.prepare("INSERT INTO cwr_npn (record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            spt_stmt: tx.prepare("INSERT INTO cwr_spt (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, constant_spaces, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)")?,
            swr_stmt: tx.prepare("INSERT INTO cwr_swr (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, writer_unknown_indicator, writer_designation_code, tax_id_num, writer_ipi_name_num, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, reversionary_indicator, first_recording_refusal_ind, work_for_hire_indicator, filler, writer_ipi_base_number, personal_number, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)")?,
            nwn_stmt: tx.prepare("INSERT INTO cwr_nwn (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            swt_stmt: tx.prepare("INSERT INTO cwr_swt (record_type, transaction_sequence_num, record_sequence_num, interested_party_num, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)")?,
            pwr_stmt: tx.prepare("INSERT INTO cwr_pwr (record_type, transaction_sequence_num, record_sequence_num, publisher_ip_num, publisher_name, submitter_agreement_number, society_assigned_agreement_number, writer_ip_num, publisher_sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?,
            alt_stmt: tx.prepare("INSERT INTO cwr_alt (record_type, transaction_sequence_num, record_sequence_num, alternate_title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
            nat_stmt: tx.prepare("INSERT INTO cwr_nat (record_type, transaction_sequence_num, record_sequence_num, title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
            ewt_stmt: tx.prepare("INSERT INTO cwr_ewt (record_type, transaction_sequence_num, record_sequence_num, entire_work_title, iswc_of_entire_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)")?,
            ver_stmt: tx.prepare("INSERT INTO cwr_ver (record_type, transaction_sequence_num, record_sequence_num, original_work_title, iswc_of_original_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)")?,
            per_stmt: tx.prepare("INSERT INTO cwr_per (record_type, transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            npr_stmt: tx.prepare("INSERT INTO cwr_npr (record_type, transaction_sequence_num, record_sequence_num, performing_artist_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number, language_code, performance_language, performance_dialect) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?,
            rec_stmt: tx.prepare("INSERT INTO cwr_rec (record_type, transaction_sequence_num, record_sequence_num, release_date, constant_blanks_1, release_duration, constant_blanks_2, album_title, album_label, release_catalog_num, ean, isrc, recording_format, recording_technique, media_type, recording_title, version_title, display_artist, record_label, isrc_validity, submitter_recording_identifier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)")?,
            orn_stmt: tx.prepare("INSERT INTO cwr_orn (record_type, transaction_sequence_num, record_sequence_num, intended_purpose, production_title, cd_identifier, cut_number, library, bltvr, filler_reserved, production_num, episode_title, episode_num, year_of_production, avi_society_code, audio_visual_number, v_isan_isan, v_isan_episode, v_isan_check_digit_1, v_isan_version, v_isan_check_digit_2, eidr, eidr_check_digit) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)")?,
            ins_stmt: tx.prepare("INSERT INTO cwr_ins (record_type, transaction_sequence_num, record_sequence_num, number_of_voices, standard_instrumentation_type, instrumentation_description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
            ind_stmt: tx.prepare("INSERT INTO cwr_ind (record_type, transaction_sequence_num, record_sequence_num, instrument_code, number_of_players) VALUES (?1, ?2, ?3, ?4, ?5)")?,
            com_stmt: tx.prepare("INSERT INTO cwr_com (record_type, transaction_sequence_num, record_sequence_num, title, iswc_of_component, submitter_work_num, duration, writer_1_last_name, writer_1_first_name, writer_1_ipi_name_num, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_1_ipi_base_number, writer_2_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)")?,
            msg_stmt: tx.prepare("INSERT INTO cwr_msg (record_type, transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, msg_record_type, message_level, validation_number, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?,
            net_stmt: tx.prepare("INSERT INTO cwr_net (record_type, transaction_sequence_num, record_sequence_num, title, language_code) VALUES (?1, ?2, ?3, ?4, ?5)")?,
            now_stmt: tx.prepare("INSERT INTO cwr_now (record_type, transaction_sequence_num, record_sequence_num, writer_name, writer_first_name, language_code, writer_position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
            ari_stmt: tx.prepare("INSERT INTO cwr_ari (record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
            xrf_stmt: tx.prepare("INSERT INTO cwr_xrf (record_type, transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        };

    // --- Process the first HDR line ---
    line_number = 1; // First line is line 1
    { // Scope for hdr_safe_slice
        // Define safe_slice specifically for the HDR line we already read
        let hdr_safe_slice = |start: usize, end: usize| -> Result<Option<String>, CwrParseError> {
             let slice_opt = if end > hdr_line.len() {
                 if start >= hdr_line.len() { None } else { hdr_line.get(start..hdr_line.len()) }
             } else {
                 hdr_line.get(start..end)
             };
             match slice_opt {
                 Some(slice) => {
                     let trimmed = slice.trim();
                     if trimmed.is_empty() { Ok(None) } else { Ok(Some(trimmed.to_string())) }
                 },
                 None => Ok(None)
             }
        };

        // Call the HDR handler for the first line, passing the context
        match record_handlers::parse_and_insert_hdr(line_number, &tx, &mut prepared_statements, &context, &hdr_safe_slice) {
            Ok(_) => processed_records += 1,
            Err(e) => {
                // Log the error for the HDR line
                if let Err(log_err) = error::log_cwr_parse_error(&mut prepared_statements, line_number, &e) {
                     eprintln!("CRITICAL Error: Failed to log HDR error to database on line {}: {} (Original error was: {})", line_number, log_err, e);
                     return Err(CwrParseError::Db(log_err)); // Fatal DB error during logging
                }
                // If the error was BadFormat, we logged it but might continue if desired (though HDR error is usually fatal)
                // If it was DB or IO, it's fatal anyway. We return the original error to rollback.
                eprintln!("Error processing HDR record (line {}): {}. Aborting.", line_number, e);
                return Err(e); // Abort on any HDR processing error
            }
        }
    } // End scope for hdr_safe_slice

    // --- Process remaining lines ---
    // Note: The loop starts from the second line because the first was consumed by read_line earlier.
    { // Scope for the main processing loop and prepared statements
        for line_result in reader.lines() {
            line_number += 1; // Increment for subsequent lines
            let line = match line_result {
                Ok(l) => l,
                Err(io_err) => {
                    // Log IO error reading the line
                    let parse_err = CwrParseError::Io(io_err);
                     if let Err(log_err) = error::log_cwr_parse_error(&mut prepared_statements, line_number, &parse_err) {
                         eprintln!("CRITICAL Error: Failed to log IO error to database on line {}: {} (Original error was: {})", line_number, log_err, parse_err);
                         return Err(CwrParseError::Db(log_err));
                     }
                     eprintln!("Aborting transaction due to IO error reading line {}: {}", line_number, parse_err);
                     return Err(parse_err); // Abort on IO Error
                }
            };


            if line.is_empty() || line.trim().is_empty() {
                continue;
            }

            if line.len() < 3 {
                log_error(&mut prepared_statements.error_stmt, line_number, format!("Line {} is too short (less than 3 chars), skipping.", line_number))?;
                continue;
            }

            let record_type = &line[0..3];

            // --- Define the Safe Slice Helper Closure for the current line ---
            let safe_slice = |start: usize, end: usize| -> Result<Option<String>, CwrParseError> {
                let slice_opt = if end > line.len() {
                    if start >= line.len() { None } else { line.get(start..line.len()) }
                } else {
                    line.get(start..end)
                };
                match slice_opt {
                    Some(slice) => {
                        let trimmed = slice.trim();
                        if trimmed.is_empty() { Ok(None) } else { Ok(Some(trimmed.to_string())) }
                    },
                    None => Ok(None)
                }
            };
            // --- End of Safe Slice Definition ---

            // Use a separate scope to handle the result processing cleanly
            let mut known_record_processed = false; // Flag to track if a known handler was called
            let process_result: Result<(), CwrParseError> = {
                match record_type {
                    // HDR should not appear again, treat as error or skip? Skipping for now.
                    "HDR" => { // This case should ideally not be hit in the loop
                        log_error(&mut prepared_statements.error_stmt, line_number, "Unexpected HDR record found after first line, skipping.".to_string())?;
                        // If the handler returns Ok, set the flag
                        known_record_processed = true; // Still mark as processed if we just log/skip
                        Ok(())
                    }
                    // Apply the pattern: call handler, set flag on success
                    // NOTE: These calls WILL FAIL TO COMPILE until their signatures are updated
                    //       to accept the `context` argument.
                    "GRH" => { record_handlers::parse_and_insert_grh(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "GRT" => { record_handlers::parse_and_insert_grt(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "TRL" => { record_handlers::parse_and_insert_trl(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "AGR" => { record_handlers::parse_and_insert_agr(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NWR" | "REV" | "ISW" | "EXC" => { record_handlers::parse_and_insert_nwr(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "ACK" => { record_handlers::parse_and_insert_ack(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "TER" => { record_handlers::parse_and_insert_ter(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "IPA" => { record_handlers::parse_and_insert_ipa(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NPA" => { record_handlers::parse_and_insert_npa(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "SPU" | "OPU" => { record_handlers::parse_and_insert_spu(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NPN" => { record_handlers::parse_and_insert_npn(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "SPT" | "OPT" => { record_handlers::parse_and_insert_spt(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "SWR" | "OWR" => { record_handlers::parse_and_insert_swr(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NWN" => { record_handlers::parse_and_insert_nwn(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "SWT" | "OWT" => { record_handlers::parse_and_insert_swt(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "PWR" => { record_handlers::parse_and_insert_pwr(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "ALT" => { record_handlers::parse_and_insert_alt(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NAT" => { record_handlers::parse_and_insert_nat(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "EWT" => { record_handlers::parse_and_insert_ewt(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "VER" => { record_handlers::parse_and_insert_ver(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "PER" => { record_handlers::parse_and_insert_per(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NPR" => { record_handlers::parse_and_insert_npr(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "REC" => { record_handlers::parse_and_insert_rec(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "ORN" => { record_handlers::parse_and_insert_orn(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "INS" => { record_handlers::parse_and_insert_ins(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "IND" => { record_handlers::parse_and_insert_ind(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "COM" => { record_handlers::parse_and_insert_com(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "MSG" => { record_handlers::parse_and_insert_msg(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NET" | "NCT" | "NVT" => { record_handlers::parse_and_insert_net(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "NOW" => { record_handlers::parse_and_insert_now(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "ARI" => { record_handlers::parse_and_insert_ari(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    "XRF" => { record_handlers::parse_and_insert_xrf(line_number, &tx, &mut prepared_statements, &safe_slice)?; known_record_processed = true; Ok(()) },
                    _ => { // Unrecognized record type
                        log_error(&mut prepared_statements.error_stmt, line_number, format!("Unrecognized record type '{}', skipping.", record_type))?;
                        // known_record_processed remains false
                        Ok(()) // Still Ok overall, just skipped this line
                    }
                }
            }; // End of inner scope for process_result

            // Check the result of processing this line
            match process_result {
                Ok(_) => {
                    // Only increment if a known record type was processed successfully
                    if known_record_processed {
                        processed_records += 1;
                    }
                    // If !known_record_processed, it was an unknown type, already logged, do nothing more.
                }
                Err(e) => {
                    // An error occurred processing this line (e.g., BadFormat from validation, or DB error from macro)

                    if let Err(log_err) = error::log_cwr_parse_error(&mut prepared_statements, line_number, &e) {
                        // If logging *itself* fails, we have a serious problem (likely DB issue). Abort immediately.
                        eprintln!(
                            "CRITICAL Error: Failed to log error to database on line {}: {} (Original error was: {})",
                            line_number, log_err, e
                        );
                        // Return the database error that occurred during logging.
                        return Err(CwrParseError::Db(log_err));
                    }

                    // Logging succeeded. Now decide if the *original* error warrants stopping.
                    match e {
                        // BadFormat errors are logged per record, but we continue processing the file.
                        CwrParseError::BadFormat(_) => {
                            // Logged above. Continue to the next line.
                            // No action needed here, the loop will just continue.
                        }
                        // IO errors (reading the file) or DB errors (during parsing/insertion, *not* during logging)
                        // are usually fatal for the whole process.
                        CwrParseError::Io(_) | CwrParseError::Db(_) => {
                            eprintln!("Aborting transaction due to unrecoverable error: {}", e);
                            // Propagate the original error to trigger transaction rollback.
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    // Commit the transaction *only if* all lines were processed without error
    tx.commit()?;

    Ok(processed_records)
}

// Helper macro for mandatory fields. Logs error to DB (using prepared statement) and returns "" if missing/empty.
// Propagates DB errors or fundamental slice errors.
#[macro_export]
macro_rules! get_mandatory_field {
    ($stmts:expr, $slice_fn:expr, $start:expr, $end:expr, $line_num:expr, $rec_type:expr, $field_name:expr) => {
        // Match on the result of the slice function
        match $slice_fn($start, $end) {
            // Case 1: Slice function itself returned an error (rare with current safe_slice, but good practice)
            Err(slice_err) => Err(slice_err), // Propagate the underlying error

            // Case 2: Slice succeeded and found a non-empty value
            Ok(Some(value)) => Ok(value), // Return the found value

            // Case 3: Slice succeeded but returned None (missing or empty/whitespace field)
            Ok(None) => {
                // Construct the error description
                let error_description = format!(
                    "{} missing or empty mandatory field '{}' (Expected at {}-{}). Using fallback ''.",
                    $rec_type, $field_name, $start + 1, $end // Use 1-based indexing for user message
                );

                match $stmts.error_stmt.execute(params![$line_num as i64, error_description]) {
                    // Subcase 3a: Database insertion failed
                    Err(db_err) => Err(CwrParseError::Db(db_err)), // Propagate the DB error
                    // Subcase 3b: Database insertion succeeded
                    Ok(_) => Ok(String::new()), // Return the fallback empty string
                }
            }
        }? // Use '?' *after* the match block to propagate any Err returned from the match arms
          // This ensures the macro returns Result<String, CwrParseError>
    };
}
