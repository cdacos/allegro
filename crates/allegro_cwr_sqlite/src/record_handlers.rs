use crate::domain_conversions::{CwrToSqlInt, CwrToSqlString, opt_domain_to_int, opt_domain_to_string};
use crate::{PreparedStatements, insert_file_line_record, log_error};
use allegro_cwr::records::*;
use allegro_cwr::{CwrParseError, ParsingContext};
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

/// Generic handler for records that return CwrParseResult (with warnings)
fn handle_record_with_warnings<T, F>(
    line_number: usize, tx: &Transaction, stmts: &mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>, parse_fn: fn(&str) -> Result<allegro_cwr::error::CwrParseResult<T>, CwrParseError>, insert_fn: F,
) -> Result<(), crate::CwrDbError>
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, HdrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.hdr_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.sender_type.as_str(),
            record.sender_id.as_str(),
            record.sender_name.as_str(),
            record.edi_standard_version_number.as_str(),
            record.creation_date.to_sql_string(),
            record.creation_time.as_str(),
            record.transmission_date.to_sql_string(),
            opt_domain_to_string(&record.character_set).as_deref().unwrap_or(""),
            record.version.as_ref().map(|v| v.as_str()).unwrap_or_default(),
            record.revision.as_ref().map(|r| r.as_str()).unwrap_or_default(),
            record.software_package.as_deref().unwrap_or(""),
            record.software_package_version.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

// GRH - Group Header
pub fn parse_and_insert_grh<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, GrhRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.grh_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            "0", // transaction_sequence_num - not in GRH record
            "0", // record_sequence_num - not in GRH record
            record.group_id.to_sql_int(),
            record.transaction_type.to_sql_string(),
            record.version_number.to_sql_string(),
            &opt_domain_to_int(&record.batch_request).unwrap_or(0).to_string(),
        ])?;
        Ok(())
    })
}

// GRT - Group Trailer
pub fn parse_and_insert_grt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, GrtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.grt_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            "0", // transaction_sequence_num - not in GRT record
            "0", // record_sequence_num - not in GRT record
            record.group_id.to_sql_int(),
            record.transaction_count.to_sql_int(),
            record.record_count.to_sql_int(),
        ])?;
        Ok(())
    })
}

// TRL - Transmission Trailer
pub fn parse_and_insert_trl<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, TrlRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.trl_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.group_count.to_sql_int(), record.transaction_count.to_sql_int(), record.record_count.to_sql_int()])?;
        Ok(())
    })
}

// ALT - Alternate Title
pub fn parse_and_insert_alt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, AltRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.alt_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.alternate_title, record.title_type.to_sql_string(), record.language_code.as_deref().unwrap_or("")])?;
        Ok(())
    })
}

// Generate stubs for remaining record types - using the same pattern
// This dramatically reduces the code from 1200+ lines to ~40 lines per record type

// AGR - Agreement Transaction
pub fn parse_and_insert_agr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, AgrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.agr_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.submitter_agreement_number,
            record.international_standard_agreement_code.as_deref().unwrap_or(""),
            record.agreement_type,
            &record.agreement_start_date.as_str(),
            &record.agreement_end_date.as_ref().map(|d| d.as_str()).unwrap_or_default(),
            &record.retention_end_date.as_ref().map(|d| d.as_str()).unwrap_or_default(),
            record.prior_royalty_status.as_str(),
            &record.prior_royalty_start_date.as_ref().map(|d| d.as_str()).unwrap_or_default(),
            record.post_term_collection_status.as_str(),
            &record.post_term_collection_end_date.as_ref().map(|d| d.as_str()).unwrap_or_default(),
            &record.date_of_signature_of_agreement.as_ref().map(|d| d.as_str()).unwrap_or_default(),
            &record.number_of_works.as_str(),
            record.sales_manufacture_clause.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.shares_change).unwrap_or_default(),
            &opt_domain_to_string(&record.advance_given).unwrap_or_default(),
            record.society_assigned_agreement_number.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nwr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nwr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.work_title,
            record.language_code.as_deref().unwrap_or(""),
            record.submitter_work_num,
            record.iswc.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.copyright_date).unwrap_or_default(),
            record.copyright_number.as_deref().unwrap_or(""),
            record.musical_work_distribution_category,
            &opt_domain_to_string(&record.duration).unwrap_or_default(),
            record.recorded_indicator.to_sql_string(),
            record.text_music_relationship.as_deref().unwrap_or(""),
            record.composite_type.as_deref().unwrap_or(""),
            record.version_type,
            record.excerpt_type.as_deref().unwrap_or(""),
            record.music_arrangement.as_deref().unwrap_or(""),
            record.lyric_adaptation.as_deref().unwrap_or(""),
            record.contact_name.as_deref().unwrap_or(""),
            record.contact_id.as_deref().unwrap_or(""),
            record.cwr_work_type.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.grand_rights_ind).unwrap_or_default(),
            &opt_domain_to_int(&record.composite_component_count).unwrap_or(0).to_string(),
            &opt_domain_to_string(&record.date_of_publication_of_printed_edition).unwrap_or_default(),
            &opt_domain_to_string(&record.exceptional_clause).unwrap_or_default(),
            record.opus_number.as_deref().unwrap_or(""),
            record.catalogue_number.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.priority_flag).unwrap_or_default(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ack<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, AckRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ack_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.creation_date.to_sql_string(),
            record.creation_time.to_sql_string(),
            record.original_group_id.to_sql_int(),
            record.original_transaction_sequence_num.to_sql_int(),
            record.original_transaction_type.to_sql_string(),
            record.creation_title.as_deref().unwrap_or(""),
            record.submitter_creation_num.as_deref().unwrap_or(""),
            record.recipient_creation_num.as_deref().unwrap_or(""),
            record.processing_date.to_sql_string(),
            record.transaction_status.to_sql_string(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ter<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, TerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ter_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.inclusion_exclusion_indicator.to_sql_string(), record.tis_numeric_code.to_sql_int(),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ipa<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, IpaRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ipa_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.agreement_role_code.to_sql_string(),
            record.interested_party_ipi_name_num.as_deref().unwrap_or(""),
            record.ipi_base_number.as_deref().unwrap_or(""),
            record.interested_party_num,
            record.interested_party_last_name,
            record.interested_party_writer_first_name.as_deref().unwrap_or(""),
            record.pr_affiliation_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.pr_share).unwrap_or(0).to_string(),
            record.mr_affiliation_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.mr_share).unwrap_or(0).to_string(),
            record.sr_affiliation_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.sr_share).unwrap_or(0).to_string(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_npa<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NpaRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npa_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.interested_party_num,
            record.interested_party_name,
            &record.interested_party_writer_first_name,
            record.language_code.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_spu<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, SpuRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.spu_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.publisher_sequence_num.to_sql_int(),
            record.interested_party_num,
            record.publisher_name,
            &opt_domain_to_string(&record.publisher_unknown_indicator).unwrap_or_default(),
            &opt_domain_to_string(&record.publisher_type).unwrap_or_default(),
            record.tax_id_num.as_deref().unwrap_or(""),
            record.publisher_ipi_name_num.as_deref().unwrap_or(""),
            record.submitter_agreement_number.as_deref().unwrap_or(""),
            record.pr_affiliation_society_num.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.pr_ownership_share).unwrap_or(0).to_string(),
            record.mr_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.mr_ownership_share).unwrap_or(0).to_string(),
            record.sr_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.sr_ownership_share).unwrap_or(0).to_string(),
            &opt_domain_to_string(&record.special_agreements_indicator).unwrap_or_default(),
            &opt_domain_to_string(&record.first_recording_refusal_ind).unwrap_or_default(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NpnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npn_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.publisher_sequence_num.to_sql_int(), record.interested_party_num, record.publisher_name, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_spt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, SptRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.spt_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.interested_party_num,
            record.constant.as_str(),
            &opt_domain_to_int(&record.pr_collection_share).unwrap_or(0).to_string(),
            &opt_domain_to_int(&record.mr_collection_share).unwrap_or(0).to_string(),
            &opt_domain_to_int(&record.sr_collection_share).unwrap_or(0).to_string(),
            record.inclusion_exclusion_indicator.as_str(),
            record.tis_numeric_code.as_str(),
            &opt_domain_to_string(&record.shares_change).unwrap_or_default(),
            &opt_domain_to_int(&record.sequence_num).unwrap_or(0).to_string(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_swr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, SwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.swr_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.interested_party_num,
            record.writer_last_name.as_deref().unwrap_or(""),
            record.writer_first_name.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.writer_unknown_indicator).unwrap_or_default(),
            record.writer_designation_code.as_deref().unwrap_or(""),
            record.tax_id_num.as_deref().unwrap_or(""),
            record.writer_ipi_name_num.as_deref().unwrap_or(""),
            record.pr_affiliation_society_num.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.pr_ownership_share).unwrap_or(0).to_string(),
            record.mr_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.mr_ownership_share).unwrap_or(0).to_string(),
            record.sr_society.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.sr_ownership_share).unwrap_or(0).to_string(),
            &opt_domain_to_string(&record.reversionary_indicator).unwrap_or_default(),
            &opt_domain_to_string(&record.first_recording_refusal_ind).unwrap_or_default(),
            &opt_domain_to_string(&record.work_for_hire_indicator).unwrap_or_default(),
            record.filler.as_deref().unwrap_or(""),
            record.writer_ipi_base_number.as_deref().unwrap_or(""),
            record.personal_number.as_deref().unwrap_or(""),
            record.usa_license_ind.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nwn<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NwnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nwn_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.interested_party_num,
            record.writer_last_name,
            record.writer_first_name.as_deref().unwrap_or(""),
            record.language_code.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_swt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, SwtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.swt_stmt.execute(params![
            file_id,
            record.record_type,
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.interested_party_num,
            &opt_domain_to_int(&record.pr_collection_share).unwrap_or(0).to_string(),
            &opt_domain_to_int(&record.mr_collection_share).unwrap_or(0).to_string(),
            &opt_domain_to_int(&record.sr_collection_share).unwrap_or(0).to_string(),
            record.inclusion_exclusion_indicator.as_str(),
            record.tis_numeric_code.as_str(),
            &opt_domain_to_string(&record.shares_change).unwrap_or_default(),
            &opt_domain_to_int(&record.sequence_num).unwrap_or(0).to_string(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_pwr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, PwrRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.pwr_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.publisher_ip_num,
            record.publisher_name,
            record.submitter_agreement_number.as_deref().unwrap_or(""),
            record.society_assigned_agreement_number.as_deref().unwrap_or(""),
            record.writer_ip_num,
            &opt_domain_to_int(&record.publisher_sequence_num).unwrap_or(0).to_string(),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_nat<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NatRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.nat_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.title, record.title_type.to_sql_string(), record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ewt<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, EwtRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ewt_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, VerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ver_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, PerRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.per_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.performing_artist_last_name,
            record.performing_artist_first_name.as_deref().unwrap_or(""),
            record.performing_artist_ipi_name_num.as_deref().unwrap_or(""),
            record.performing_artist_ipi_base_number.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_npr<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NprRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.npr_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, RecRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.rec_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            &opt_domain_to_string(&record.release_date).unwrap_or_default(),
            record.constant.as_str(),
            &opt_domain_to_string(&record.release_duration).unwrap_or_default(),
            record.constant.as_str(),
            record.album_title.as_deref().unwrap_or(""),
            record.album_label.as_deref().unwrap_or(""),
            record.release_catalog_num.as_deref().unwrap_or(""),
            record.ean.as_deref().unwrap_or(""),
            record.isrc.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.recording_format).unwrap_or_default(),
            &opt_domain_to_string(&record.recording_technique).unwrap_or_default(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, OrnRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.orn_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.intended_purpose.as_str(),
            record.production_title.as_deref().unwrap_or(""),
            record.cd_identifier.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.cut_number).unwrap_or(0).to_string(),
            record.library.as_deref().unwrap_or(""),
            record.bltvr.as_deref().unwrap_or(""),
            record.filler.as_deref().unwrap_or(""),
            record.production_num.as_deref().unwrap_or(""),
            record.episode_title.as_deref().unwrap_or(""),
            record.episode_num.as_deref().unwrap_or(""),
            &opt_domain_to_int(&record.year_of_production).unwrap_or(0).to_string(),
            &opt_domain_to_int(&record.avi_society_code).unwrap_or(0).to_string(),
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, InsRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ins_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            &opt_domain_to_int(&record.number_of_voices).unwrap_or(0).to_string(),
            record.standard_instrumentation_type.as_deref().unwrap_or(""),
            record.instrumentation_description.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_ind<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, IndRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ind_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.instrument_code, &opt_domain_to_int(&record.number_of_players).unwrap_or(0).to_string(),])?;
        Ok(())
    })
}

pub fn parse_and_insert_com<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, ComRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.com_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.title,
            record.iswc_of_component.as_deref().unwrap_or(""),
            record.submitter_work_num.as_deref().unwrap_or(""),
            &opt_domain_to_string(&record.duration).unwrap_or_default(),
            &record.writer_1_last_name,
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
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, MsgRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.msg_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.message_type,
            record.original_record_sequence_num.to_sql_int(),
            &record.record_type_field,
            record.message_level,
            &record.validation_number,
            record.message_text,
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_net<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NetRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.net_stmt.execute(params![file_id, record.record_type, record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.title, record.language_code.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_now<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, NowRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.now_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.writer_name, &record.writer_first_name, record.language_code.as_deref().unwrap_or(""), record.writer_position.as_deref().unwrap_or(""),])?;
        Ok(())
    })
}

pub fn parse_and_insert_ari<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, AriRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.ari_stmt.execute(params![
            file_id,
            record.record_type.to_sql_string(),
            record.transaction_sequence_num.to_sql_int(),
            record.record_sequence_num.to_sql_int(),
            record.society_num,
            record.work_num,
            record.type_of_right.as_str(),
            record.subject_code.as_deref().unwrap_or(""),
            record.note.as_deref().unwrap_or(""),
        ])?;
        Ok(())
    })
}

pub fn parse_and_insert_xrf<'a>(line_number: usize, tx: &'a Transaction, stmts: &'a mut PreparedStatements, context: &ParsingContext, safe_slice: &impl Fn(usize, usize) -> Result<Option<String>, CwrParseError>) -> Result<(), crate::CwrDbError> {
    handle_record_with_warnings(line_number, tx, stmts, context, safe_slice, XrfRecord::from_cwr_line, |record, stmts, file_id| {
        stmts.xrf_stmt.execute(params![file_id, record.record_type.to_sql_string(), record.transaction_sequence_num.to_sql_int(), record.record_sequence_num.to_sql_int(), record.organisation_code, record.identifier, record.identifier_type.as_str(), record.validity.as_str(),])?;
        Ok(())
    })
}
