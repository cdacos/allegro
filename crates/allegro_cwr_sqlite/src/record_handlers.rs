use allegro_cwr::{CwrParseError, ParsingContext};
use allegro_cwr::records::*;
use crate::{PreparedStatements, insert_file_line_record, log_error};
use rusqlite::{Transaction, params};

/// Helper function to reconstruct the full line from safe_slice calls
/// This allows us to use the existing record structs instead of the dangerous get_mandatory_field! macro
fn reconstruct_line_from_safe_slice(safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<String, CwrParseError> {
    // Try to get a large slice that should contain the entire line
    // Most CWR lines are under 1000 characters, so this should work for all record types
    match safe_slice(0, 2000) {
        Ok(Some(line)) => Ok(line),
        Ok(None) => Err(CwrParseError::BadFormat("Empty line".to_string())),
        Err(e) => Err(e),
    }
}

/// Generic handler that uses the record structs instead of field-by-field parsing
fn handle_record_with_struct<T, F>(line_number: usize, tx: &Transaction, stmts: &mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>, parse_fn: fn(&str) -> Result<T, CwrParseError>, insert_fn: F) -> Result<(), crate::CwrDbError>
where
    F: FnOnce(&T, &mut PreparedStatements, i64) -> Result<(), rusqlite::Error>,
{
    // Reconstruct the full line
    let line = reconstruct_line_from_safe_slice(safe_slice)?;

    // Parse using the existing record struct
    match parse_fn(&line) {
        Ok(record) => {
            // Insert using the provided function
            insert_fn(&record, stmts, context.file_id)?;
            let record_id = tx.last_insert_rowid();

            // Log to file_line table
            let record_type = line.get(0..3).unwrap_or("UNK");
            insert_file_line_record(&mut stmts.file_stmt, context.file_id, line_number, record_type, record_id)?;

            Ok(())
        }
        Err(e) => {
            // Log error but continue processing (recoverable error)
            log_error(&mut stmts.error_stmt, context.file_id, line_number, e.to_string())?;
            Ok(())
        }
    }
}

/// Generic handler for records that return CwrParseResult (with warnings)
fn handle_record_with_warnings<T, F>(line_number: usize, tx: &Transaction, stmts: &mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>, parse_fn: fn(&str) -> Result<allegro_cwr::error::CwrParseResult<T>, CwrParseError>, insert_fn: F) -> Result<(), crate::CwrDbError>
where
    F: FnOnce(&T, &mut PreparedStatements, i64) -> Result<(), rusqlite::Error>,
{
    // Reconstruct the full line
    let line = reconstruct_line_from_safe_slice(safe_slice)?;

    // Parse using the provided function
    match parse_fn(&line) {
        Ok(parse_result) => {
            // Log any warnings
            for warning in &parse_result.warnings {
                log::warn!("Line {}: {}", line_number, warning);
            }
            
            // Insert using the provided function
            insert_fn(&parse_result.record, stmts, context.file_id)?;
            let record_id = tx.last_insert_rowid();

            // Log to file_line table
            let record_type = line.get(0..3).unwrap_or("UNK");
            insert_file_line_record(&mut stmts.file_stmt, context.file_id, line_number, record_type, record_id)?;

            Ok(())
        }
        Err(e) => {
            // Log error but continue processing (recoverable error)
            log_error(&mut stmts.error_stmt, context.file_id, line_number, e.to_string())?;
            Ok(())
        }
    }
}

// HDR - Transmission Header
pub fn parse_and_insert_hdr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, HdrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.hdr_stmt.execute(params![
            file_id,
            record.record_type,
            record.sender_type,
            record.sender_id,
            record.sender_name,
            record.edi_standard_version_number,
            record.creation_date,
            record.creation_time,
            record.transmission_date,
            record.character_set.as_deref().unwrap_or(""),
            record.version.as_deref().unwrap_or(""),
            record.revision.as_deref().unwrap_or(""),
            record.software_package.as_deref().unwrap_or(""),
            record.software_package_version.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

// GRH - Group Header
pub fn parse_and_insert_grh<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, GrhRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.grh_stmt.execute(params![
            file_id,
            record.record_type,
            "0", // transaction_sequence_num - not in GRH record
            "0", // record_sequence_num - not in GRH record
            record.group_id,
            record.transaction_type,
            record.version_number,
            record.batch_request.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

// GRT - Group Trailer
pub fn parse_and_insert_grt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, GrtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.grt_stmt.execute(params![
            file_id,
            record.record_type,
            "0", // transaction_sequence_num - not in GRT record
            "0", // record_sequence_num - not in GRT record
            record.group_id,
            record.transaction_count,
            record.record_count,
        ])?;
        Ok(())
    })
}

// TRL - Transmission Trailer
pub fn parse_and_insert_trl<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, TrlRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.trl_stmt.execute(params![file_id, record.record_type, record.group_count, record.transaction_count, record.record_count,])?;
        Ok(())
    })
}

// ALT - Alternate Title
pub fn parse_and_insert_alt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, AltRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.alt_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.alternate_title, record.title_type, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

// Generate stubs for remaining record types - using the same pattern
// This dramatically reduces the code from 1200+ lines to ~40 lines per record type

// AGR - Agreement Transaction
pub fn parse_and_insert_agr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, AgrRecord::from_cwr_line_v2, |record, stmts, file_id| {
        stmts.agr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.submitter_agreement_number,
            record.international_standard_agreement_code.as_deref().unwrap_or(""),
            record.agreement_type,
            record.agreement_start_date.as_str(),
            record.agreement_end_date.as_deref().unwrap_or(""),
            record.retention_end_date.as_deref().unwrap_or(""),
            record.prior_royalty_status.as_str(),
            record.prior_royalty_start_date.as_deref().unwrap_or(""),
            record.post_term_collection_status.as_str(),
            record.post_term_collection_end_date.as_deref().unwrap_or(""),
            record.date_of_signature_of_agreement.as_deref().unwrap_or(""),
            record.number_of_works.as_str(),
            record.sales_manufacture_clause.as_deref().unwrap_or(""),
            record.shares_change.as_deref().unwrap_or(""),
            record.advance_given.as_deref().unwrap_or(""),
            record.society_assigned_agreement_number.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nwr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nwr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.work_title,
            record.language_code.as_deref().unwrap_or(""),
            record.submitter_work_num,
            record.iswc.as_deref().unwrap_or(""),
            record.copyright_date.as_deref().unwrap_or(""),
            record.copyright_number.as_deref().unwrap_or(""),
            record.musical_work_distribution_category,
            record.duration.as_deref().unwrap_or(""),
            record.recorded_indicator,
            record.text_music_relationship.as_deref().unwrap_or(""),
            record.composite_type.as_deref().unwrap_or(""),
            record.version_type,
            record.excerpt_type.as_deref().unwrap_or(""),
            record.music_arrangement.as_deref().unwrap_or(""),
            record.lyric_adaptation.as_deref().unwrap_or(""),
            record.contact_name.as_deref().unwrap_or(""),
            record.contact_id.as_deref().unwrap_or(""),
            record.cwr_work_type.as_deref().unwrap_or(""),
            record.grand_rights_ind.as_deref().unwrap_or(""),
            record.composite_component_count.as_deref().unwrap_or(""),
            record.date_of_publication_of_printed_edition.as_deref().unwrap_or(""),
            record.exceptional_clause.as_deref().unwrap_or(""),
            record.opus_number.as_deref().unwrap_or(""),
            record.catalogue_number.as_deref().unwrap_or(""),
            record.priority_flag.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ack<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, AckRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ack_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.creation_date,
            record.creation_time,
            record.original_group_id,
            record.original_transaction_sequence_num,
            record.original_transaction_type,
            record.creation_title.as_deref().unwrap_or(""),
            record.submitter_creation_num.as_deref().unwrap_or(""),
            record.recipient_creation_num.as_deref().unwrap_or(""),
            record.processing_date,
            record.transaction_status,
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ter<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, TerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ter_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.inclusion_exclusion_indicator, record.tis_numeric_code,])?;
        Ok(())
    })
}

pub fn parse_and_insert_ipa<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, IpaRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ipa_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.agreement_role_code,
            record.interested_party_ipi_name_num.as_deref().unwrap_or(""),
            record.ipi_base_number.as_deref().unwrap_or(""),
            record.interested_party_num,
            record.interested_party_last_name,
            record.interested_party_writer_first_name.as_deref().unwrap_or(""),
            record.pr_affiliation_society.as_deref().unwrap_or(""),
            record.pr_share.as_deref().unwrap_or(""),
            record.mr_affiliation_society.as_deref().unwrap_or(""),
            record.mr_share.as_deref().unwrap_or(""),
            record.sr_affiliation_society.as_deref().unwrap_or(""),
            record.sr_share.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_npa<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NpaRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npa_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.interested_party_num, record.interested_party_name, record.interested_party_writer_first_name.as_str(), record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_spu<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, SpuRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.spu_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.publisher_sequence_num,
            record.interested_party_num,
            record.publisher_name,
            record.publisher_unknown_indicator,
            record.publisher_type.as_deref().unwrap_or(""),
            record.tax_id_num.as_deref().unwrap_or(""),
            record.publisher_ipi_name_num.as_deref().unwrap_or(""),
            record.submitter_agreement_number.as_deref().unwrap_or(""),
            record.pr_affiliation_society_num.as_deref().unwrap_or(""),
            record.pr_ownership_share.as_deref().unwrap_or(""),
            record.mr_society.as_deref().unwrap_or(""),
            record.mr_ownership_share.as_deref().unwrap_or(""),
            record.sr_society.as_deref().unwrap_or(""),
            record.sr_ownership_share.as_deref().unwrap_or(""),
            record.special_agreements_indicator.as_deref().unwrap_or(""),
            record.first_recording_refusal_ind.as_deref().unwrap_or(""),
            record.filler.as_deref().unwrap_or(""),
            record.publisher_ipi_base_number.as_deref().unwrap_or(""),
            record.international_standard_agreement_code.as_deref().unwrap_or(""),
            record.society_assigned_agreement_number.as_deref().unwrap_or(""),
            record.agreement_type.as_deref().unwrap_or(""),
            record.usa_license_ind.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_npn<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NpnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npn_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.publisher_sequence_num, record.interested_party_num, record.publisher_name, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_spt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, SptRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.spt_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.interested_party_num,
            record.constant.as_str(),
            record.pr_collection_share.as_deref().unwrap_or(""),
            record.mr_collection_share.as_deref().unwrap_or(""),
            record.sr_collection_share.as_deref().unwrap_or(""),
            record.inclusion_exclusion_indicator.as_str(),
            record.tis_numeric_code.as_str(),
            record.shares_change.as_deref().unwrap_or(""),
            record.sequence_num.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_swr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, SwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.swr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.interested_party_num,
            record.writer_last_name.as_deref().unwrap_or(""),
            record.writer_first_name.as_deref().unwrap_or(""),
            record.writer_unknown_indicator.as_deref().unwrap_or(""),
            record.writer_designation_code.as_deref().unwrap_or(""),
            record.tax_id_num.as_deref().unwrap_or(""),
            record.writer_ipi_name_num.as_deref().unwrap_or(""),
            record.pr_affiliation_society_num.as_deref().unwrap_or(""),
            record.pr_ownership_share.as_deref().unwrap_or(""),
            record.mr_society.as_deref().unwrap_or(""),
            record.mr_ownership_share.as_deref().unwrap_or(""),
            record.sr_society.as_deref().unwrap_or(""),
            record.sr_ownership_share.as_deref().unwrap_or(""),
            record.reversionary_indicator.as_deref().unwrap_or(""),
            record.first_recording_refusal_ind.as_deref().unwrap_or(""),
            record.work_for_hire_indicator.as_deref().unwrap_or(""),
            record.filler.as_deref().unwrap_or(""),
            record.writer_ipi_base_number.as_deref().unwrap_or(""),
            record.personal_number.as_deref().unwrap_or(""),
            record.usa_license_ind.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nwn<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NwnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nwn_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.interested_party_num, record.writer_last_name, record.writer_first_name.as_deref().unwrap_or(""), record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_swt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, SwtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.swt_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.interested_party_num,
            record.pr_collection_share.as_deref().unwrap_or(""),
            record.mr_collection_share.as_deref().unwrap_or(""),
            record.sr_collection_share.as_deref().unwrap_or(""),
            record.inclusion_exclusion_indicator.as_str(),
            record.tis_numeric_code.as_str(),
            record.shares_change.as_deref().unwrap_or(""),
            record.sequence_num.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_pwr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, PwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.pwr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.publisher_ip_num,
            record.publisher_name,
            record.submitter_agreement_number.as_deref().unwrap_or(""),
            record.society_assigned_agreement_number.as_deref().unwrap_or(""),
            record.writer_ip_num,
            record.publisher_sequence_num.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nat<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NatRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nat_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.title, record.title_type, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ewt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, EwtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ewt_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.entire_work_title,
            record.iswc_of_entire_work.as_deref().unwrap_or(""),
            record.language_code.as_deref().unwrap_or(""),
            record.writer_1_last_name.as_deref().unwrap_or(""),
            record.writer_1_first_name.as_deref().unwrap_or(""),
            record.source.as_deref().unwrap_or(""),
            record.writer_1_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_1_ipi_base_number.as_deref().unwrap_or(""),
            record.writer_2_last_name.as_deref().unwrap_or(""),
            record.writer_2_first_name.as_deref().unwrap_or(""),
            record.writer_2_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_2_ipi_base_number.as_deref().unwrap_or(""),
            record.submitter_work_num.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ver<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, VerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ver_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.original_work_title,
            record.iswc_of_original_work.as_deref().unwrap_or(""),
            record.language_code.as_deref().unwrap_or(""),
            record.writer_1_last_name.as_deref().unwrap_or(""),
            record.writer_1_first_name.as_deref().unwrap_or(""),
            record.source.as_deref().unwrap_or(""),
            record.writer_1_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_1_ipi_base_number.as_deref().unwrap_or(""),
            record.writer_2_last_name.as_deref().unwrap_or(""),
            record.writer_2_first_name.as_deref().unwrap_or(""),
            record.writer_2_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_2_ipi_base_number.as_deref().unwrap_or(""),
            record.submitter_work_num.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_per<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, PerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.per_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.performing_artist_last_name,
            record.performing_artist_first_name.as_deref().unwrap_or(""),
            record.performing_artist_ipi_name_num.as_deref().unwrap_or(""),
            record.performing_artist_ipi_base_number.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_npr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NprRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.performing_artist_name,
            record.performing_artist_first_name.as_deref().unwrap_or(""),
            record.performing_artist_ipi_name_num.as_deref().unwrap_or(""),
            record.performing_artist_ipi_base_number.as_deref().unwrap_or(""),
            record.language_code.as_deref().unwrap_or(""),
            record.performance_language.as_deref().unwrap_or(""),
            record.performance_dialect.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_rec<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, RecRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.rec_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.release_date.as_deref().unwrap_or(""),
            record.constant.as_str(),
            record.release_duration.as_deref().unwrap_or(""),
            record.constant.as_str(),
            record.album_title.as_deref().unwrap_or(""),
            record.album_label.as_deref().unwrap_or(""),
            record.release_catalog_num.as_deref().unwrap_or(""),
            record.ean.as_deref().unwrap_or(""),
            record.isrc.as_deref().unwrap_or(""),
            record.recording_format.as_deref().unwrap_or(""),
            record.recording_technique.as_deref().unwrap_or(""),
            record.media_type.as_deref().unwrap_or(""),
            record.recording_title.as_deref().unwrap_or(""),
            record.version_title.as_deref().unwrap_or(""),
            record.display_artist.as_deref().unwrap_or(""),
            record.record_label.as_deref().unwrap_or(""),
            record.isrc_validity.as_deref().unwrap_or(""),
            record.submitter_recording_identifier.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_orn<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, OrnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.orn_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.intended_purpose.as_str(),
            record.production_title.as_deref().unwrap_or(""),
            record.cd_identifier.as_deref().unwrap_or(""),
            record.cut_number.as_deref().unwrap_or(""),
            record.library.as_deref().unwrap_or(""),
            record.bltvr.as_deref().unwrap_or(""),
            record.filler.as_deref().unwrap_or(""),
            record.production_num.as_deref().unwrap_or(""),
            record.episode_title.as_deref().unwrap_or(""),
            record.episode_num.as_deref().unwrap_or(""),
            record.year_of_production.as_deref().unwrap_or(""),
            record.avi_society_code.as_deref().unwrap_or(""),
            record.audio_visual_number.as_deref().unwrap_or(""),
            record.v_isan_isan.as_deref().unwrap_or(""),
            record.v_isan_episode.as_deref().unwrap_or(""),
            record.v_isan_check_digit_1.as_deref().unwrap_or(""),
            record.v_isan_version.as_deref().unwrap_or(""),
            record.v_isan_check_digit_2.as_deref().unwrap_or(""),
            record.eidr.as_deref().unwrap_or(""),
            record.eidr_check_digit.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ins<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, InsRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ins_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.number_of_voices.as_deref().unwrap_or(""), record.standard_instrumentation_type.as_deref().unwrap_or(""), record.instrumentation_description.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ind<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, IndRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ind_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.instrument_code, record.number_of_players.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_com<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, ComRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.com_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num,
            record.record_sequence_num,
            record.title,
            record.iswc_of_component.as_deref().unwrap_or(""),
            record.submitter_work_num.as_deref().unwrap_or(""),
            record.duration.as_deref().unwrap_or(""),
            record.writer_1_last_name.as_str(),
            record.writer_1_first_name.as_deref().unwrap_or(""),
            record.writer_1_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_2_last_name.as_deref().unwrap_or(""),
            record.writer_2_first_name.as_deref().unwrap_or(""),
            record.writer_2_ipi_name_num.as_deref().unwrap_or(""),
            record.writer_1_ipi_base_number.as_deref().unwrap_or(""),
            record.writer_2_ipi_base_number.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_msg<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, MsgRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.msg_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.message_type, record.original_record_sequence_num.as_str(), record.record_type_field.as_str(), record.message_level, record.validation_number.as_str(), record.message_text,])?;
        Ok(())
    })
}

pub fn parse_and_insert_net<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NetRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.net_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.title, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_now<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, NowRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.now_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.writer_name, record.writer_first_name.as_str(), record.language_code.as_deref().unwrap_or(""), record.writer_position.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ari<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, AriRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ari_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.society_num, record.work_num, record.type_of_right.as_str(), record.subject_code.as_deref().unwrap_or(""), record.note.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_xrf<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_struct(line_number, tx, stmts, context, safe_slice, XrfRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.xrf_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num, record.record_sequence_num, record.organisation_code, record.identifier, record.identifier_type.as_str(), record.validity.as_str(),])?;
        Ok(())
    })
}
