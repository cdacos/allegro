//! SQLite database operations for CWR (Common Works Registration) files
//!
//! This crate provides database setup, schema management, and record operations
//! for storing and querying CWR file data in SQLite databases.

pub mod connection;
pub mod domain_conversions;
pub mod error;
pub mod operations;
pub mod record_handlers;
pub mod report;
pub mod statements;

// Re-export main types and functions
pub use connection::{CwrDatabase, determine_db_filename, setup_database};
pub use error::CwrDbError;
pub use operations::{CwrRecordInserter, count_errors_by_record_type, count_records_by_type, insert_file_line_record, insert_file_record, log_error};
pub use statements::PreparedStatements;

/// Result type for database operations
pub type Result<T> = std::result::Result<T, CwrDbError>;

/// SQLite implementation of CwrHandler trait
pub struct SqliteHandler {
    conn: rusqlite::Connection,
    tx: Option<rusqlite::Transaction<'static>>,
    file_id: i64,
    processed_count: usize,
    error_count: usize,
    db_filename: String,
    batch_size: usize,
    statements: Option<statements::PreparedStatements<'static>>,
}

impl SqliteHandler {
    pub fn new(input_filename: &str, db_filename: &str) -> Result<Self> {
        Self::new_with_batch_size(input_filename, db_filename, 1000)
    }

    pub fn new_with_batch_size(input_filename: &str, db_filename: &str, batch_size: usize) -> Result<Self> {
        use statements::get_prepared_statements;

        // Setup database
        setup_database(db_filename)?;

        // Open connection and setup transaction
        let mut conn = rusqlite::Connection::open(db_filename)?;
        conn.pragma_update(None, "journal_mode", "OFF")?;
        conn.pragma_update(None, "synchronous", "OFF")?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;

        // We need to work around the lifetime issue with transactions
        // For now, let's use a simpler approach without holding the transaction
        let file_id = {
            let tx = conn.transaction()?;
            let mut prepared_statements = get_prepared_statements(&tx)?;
            let file_id = insert_file_record(&tx, &mut prepared_statements.file_insert_stmt, input_filename)?;
            drop(prepared_statements); // Drop before commit to release borrow
            tx.commit()?;
            file_id
        };

        Ok(SqliteHandler { conn, tx: None, file_id, processed_count: 0, error_count: 0, db_filename: db_filename.to_string(), batch_size, statements: None })
    }

    fn start_batch(&mut self) -> Result<()> {
        if self.tx.is_none() {
            // Start transaction
            let tx = self.conn.transaction()?;
            // We need to use unsafe to extend the lifetime
            let tx: rusqlite::Transaction<'static> = unsafe { std::mem::transmute(tx) };
            let statements = statements::get_prepared_statements(&tx)?;
            let statements: statements::PreparedStatements<'static> = unsafe { std::mem::transmute(statements) };

            self.tx = Some(tx);
            self.statements = Some(statements);
        }
        Ok(())
    }

    fn commit_batch(&mut self) -> Result<()> {
        if let Some(tx) = self.tx.take() {
            self.statements = None;
            tx.commit()?;
        }
        Ok(())
    }

    fn should_commit_batch(&self) -> bool {
        self.processed_count % self.batch_size == 0
    }
}

impl allegro_cwr::CwrHandler for SqliteHandler {
    type Error = CwrDbError;

    fn process_record(&mut self, parsed_record: allegro_cwr::ParsedRecord) -> std::result::Result<(), Self::Error> {
        self.start_batch()?;

        if let Some(ref tx) = self.tx {
            if let Some(ref mut statements) = self.statements {
                let record_id = match &parsed_record.record {
                    allegro_cwr::cwr_record::CwrRecord::Hdr(hdr) => {
                        statements.hdr_stmt.execute(rusqlite::params![
                            self.file_id,
                            "HDR",
                            hdr.sender_type,
                            hdr.sender_id,
                            hdr.sender_name,
                            hdr.edi_standard_version_number,
                            hdr.creation_date.as_str(),
                            hdr.creation_time,
                            hdr.transmission_date.as_str(),
                            hdr.character_set,
                            hdr.version,
                            hdr.revision,
                            hdr.software_package,
                            hdr.software_package_version
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Grh(grh) => {
                        statements.grh_stmt.execute(rusqlite::params![
                            self.file_id,
                            "GRH",
                            grh.transaction_type,
                            grh.group_id,
                            grh.version_number,
                            grh.batch_request,
                            grh.submission_distribution_type
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Grt(grt) => {
                        statements.grt_stmt.execute(rusqlite::params![
                            self.file_id,
                            "GRT",
                            grt.group_id,
                            grt.transaction_count,
                            grt.record_count,
                            grt.currency_indicator,
                            grt.total_monetary_value
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Trl(trl) => {
                        statements.trl_stmt.execute(rusqlite::params![
                            self.file_id,
                            "TRL",
                            trl.group_count,
                            trl.transaction_count,
                            trl.record_count
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Agr(agr) => {
                        statements.agr_stmt.execute(rusqlite::params![
                            self.file_id,
                            "AGR",
                            agr.transaction_sequence_num,
                            agr.record_sequence_num,
                            agr.submitter_agreement_number,
                            agr.international_standard_agreement_code,
                            agr.agreement_type,
                            agr.agreement_start_date.as_str(),
                            agr.agreement_end_date.as_ref().map(|d| d.as_str()),
                            agr.retention_end_date.as_ref().map(|d| d.as_str()),
                            agr.prior_royalty_status.as_str(),
                            agr.prior_royalty_start_date.as_ref().map(|d| d.as_str()),
                            agr.post_term_collection_status.as_str(),
                            agr.post_term_collection_end_date.as_ref().map(|d| d.as_str()),
                            agr.date_of_signature_of_agreement.as_ref().map(|d| d.as_str()),
                            agr.number_of_works.as_str(),
                            agr.sales_manufacture_clause,
                            agr.shares_change,
                            agr.advance_given,
                            agr.society_assigned_agreement_number
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Nwr(nwr) => {
                        statements.nwr_stmt.execute(rusqlite::params![
                            self.file_id,
                            nwr.record_type,
                            nwr.transaction_sequence_num,
                            nwr.record_sequence_num,
                            nwr.work_title,
                            nwr.language_code,
                            nwr.submitter_work_num,
                            nwr.iswc,
                            nwr.copyright_date,
                            nwr.copyright_number,
                            nwr.musical_work_distribution_category,
                            nwr.duration,
                            nwr.recorded_indicator,
                            nwr.text_music_relationship,
                            nwr.composite_type,
                            nwr.version_type,
                            nwr.excerpt_type,
                            nwr.music_arrangement,
                            nwr.lyric_adaptation,
                            nwr.contact_name,
                            nwr.contact_id,
                            nwr.cwr_work_type,
                            nwr.grand_rights_ind,
                            nwr.composite_component_count,
                            nwr.date_of_publication_of_printed_edition,
                            nwr.exceptional_clause,
                            nwr.opus_number,
                            nwr.catalogue_number,
                            nwr.priority_flag
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Spu(spu) => {
                        statements.spu_stmt.execute(rusqlite::params![
                            self.file_id,
                            spu.record_type,
                            spu.transaction_sequence_num,
                            spu.record_sequence_num,
                            spu.publisher_sequence_num,
                            spu.interested_party_num,
                            spu.publisher_name,
                            spu.publisher_unknown_indicator,
                            spu.publisher_type,
                            spu.tax_id_num,
                            spu.publisher_ipi_name_num,
                            spu.submitter_agreement_number,
                            spu.pr_affiliation_society_num,
                            spu.pr_ownership_share,
                            spu.mr_society,
                            spu.mr_ownership_share,
                            spu.sr_society,
                            spu.sr_ownership_share,
                            spu.special_agreements_indicator,
                            spu.first_recording_refusal_ind,
                            spu.filler,
                            spu.publisher_ipi_base_number,
                            spu.international_standard_agreement_code,
                            spu.society_assigned_agreement_number,
                            spu.agreement_type,
                            spu.usa_license_ind
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Swr(swr) => {
                        statements.swr_stmt.execute(rusqlite::params![
                            self.file_id,
                            swr.record_type,
                            swr.transaction_sequence_num,
                            swr.record_sequence_num,
                            swr.interested_party_num,
                            swr.writer_last_name,
                            swr.writer_first_name,
                            swr.writer_unknown_indicator,
                            swr.writer_designation_code,
                            swr.tax_id_num,
                            swr.writer_ipi_name_num,
                            swr.pr_affiliation_society_num,
                            swr.pr_ownership_share,
                            swr.mr_society,
                            swr.mr_ownership_share,
                            swr.sr_society,
                            swr.sr_ownership_share,
                            swr.reversionary_indicator,
                            swr.first_recording_refusal_ind,
                            swr.work_for_hire_indicator,
                            swr.filler,
                            swr.writer_ipi_base_number,
                            swr.personal_number,
                            swr.usa_license_ind
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Alt(alt) => {
                        statements.alt_stmt.execute(rusqlite::params![
                            self.file_id,
                            "ALT",
                            alt.transaction_sequence_num,
                            alt.record_sequence_num,
                            alt.alternate_title,
                            alt.title_type,
                            alt.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Per(per) => {
                        statements.per_stmt.execute(rusqlite::params![
                            self.file_id,
                            "PER",
                            per.transaction_sequence_num,
                            per.record_sequence_num,
                            per.performing_artist_last_name,
                            per.performing_artist_first_name,
                            per.performing_artist_ipi_name_num,
                            per.performing_artist_ipi_base_number
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Rec(rec) => {
                        statements.rec_stmt.execute(rusqlite::params![
                            self.file_id,
                            "REC",
                            rec.transaction_sequence_num,
                            rec.record_sequence_num,
                            rec.release_date,
                            rec.constant,
                            rec.release_duration,
                            rec.constant2,
                            rec.album_title,
                            rec.album_label,
                            rec.release_catalog_num,
                            rec.ean,
                            rec.isrc,
                            rec.recording_format,
                            rec.recording_technique,
                            rec.media_type,
                            rec.recording_title,
                            rec.version_title,
                            rec.display_artist,
                            rec.record_label,
                            rec.isrc_validity,
                            rec.submitter_recording_identifier
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ack(ack) => {
                        statements.ack_stmt.execute(rusqlite::params![
                            self.file_id,
                            "ACK",
                            ack.transaction_sequence_num,
                            ack.record_sequence_num,
                            ack.creation_date,
                            ack.creation_time,
                            ack.original_group_id,
                            ack.original_transaction_sequence_num,
                            ack.original_transaction_type,
                            ack.creation_title,
                            ack.submitter_creation_num,
                            ack.recipient_creation_num,
                            ack.processing_date,
                            ack.transaction_status
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ter(ter) => {
                        statements.ter_stmt.execute(rusqlite::params![
                            self.file_id,
                            "TER",
                            ter.transaction_sequence_num,
                            ter.record_sequence_num,
                            ter.inclusion_exclusion_indicator,
                            ter.tis_numeric_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ipa(ipa) => {
                        statements.ipa_stmt.execute(rusqlite::params![
                            self.file_id,
                            "IPA",
                            ipa.transaction_sequence_num,
                            ipa.record_sequence_num,
                            ipa.agreement_role_code,
                            ipa.interested_party_ipi_name_num,
                            ipa.ipi_base_number,
                            ipa.interested_party_num,
                            ipa.interested_party_last_name,
                            ipa.interested_party_writer_first_name,
                            ipa.pr_affiliation_society,
                            ipa.pr_share,
                            ipa.mr_affiliation_society,
                            ipa.mr_share,
                            ipa.sr_affiliation_society,
                            ipa.sr_share
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Npa(npa) => {
                        statements.npa_stmt.execute(rusqlite::params![
                            self.file_id,
                            "NPA",
                            npa.transaction_sequence_num,
                            npa.record_sequence_num,
                            npa.interested_party_num,
                            npa.interested_party_name,
                            npa.interested_party_writer_first_name,
                            npa.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Npn(npn) => {
                        statements.npn_stmt.execute(rusqlite::params![
                            self.file_id,
                            "NPN",
                            npn.transaction_sequence_num,
                            npn.record_sequence_num,
                            npn.publisher_sequence_num,
                            npn.interested_party_num,
                            npn.publisher_name,
                            npn.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Npr(npr) => {
                        statements.npr_stmt.execute(rusqlite::params![
                            self.file_id,
                            "NPR",
                            npr.transaction_sequence_num,
                            npr.record_sequence_num,
                            npr.performing_artist_name,
                            npr.performing_artist_first_name,
                            npr.performing_artist_ipi_name_num,
                            npr.performing_artist_ipi_base_number,
                            npr.language_code,
                            npr.performance_language,
                            npr.performance_dialect
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Spt(spt) => {
                        statements.spt_stmt.execute(rusqlite::params![
                            self.file_id, spt.record_type, spt.transaction_sequence_num, spt.record_sequence_num,
                            spt.interested_party_num, spt.constant, spt.pr_collection_share, spt.mr_collection_share,
                            spt.sr_collection_share, spt.inclusion_exclusion_indicator, spt.tis_numeric_code,
                            spt.shares_change, spt.sequence_num
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Nwn(nwn) => {
                        statements.nwn_stmt.execute(rusqlite::params![
                            self.file_id, "NWN", nwn.transaction_sequence_num, nwn.record_sequence_num,
                            nwn.interested_party_num, nwn.writer_last_name, nwn.writer_first_name, nwn.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Swt(swt) => {
                        statements.swt_stmt.execute(rusqlite::params![
                            self.file_id, swt.record_type, swt.transaction_sequence_num, swt.record_sequence_num,
                            swt.interested_party_num, swt.pr_collection_share, swt.mr_collection_share,
                            swt.sr_collection_share, swt.inclusion_exclusion_indicator, swt.tis_numeric_code,
                            swt.shares_change, swt.sequence_num
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Pwr(pwr) => {
                        statements.pwr_stmt.execute(rusqlite::params![
                            self.file_id, "PWR", pwr.transaction_sequence_num, pwr.record_sequence_num,
                            pwr.publisher_ip_num, pwr.publisher_name, pwr.submitter_agreement_number,
                            pwr.society_assigned_agreement_number, pwr.writer_ip_num, pwr.publisher_sequence_num
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Nat(nat) => {
                        statements.nat_stmt.execute(rusqlite::params![
                            self.file_id, "NAT", nat.transaction_sequence_num, nat.record_sequence_num,
                            nat.title, nat.title_type, nat.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ewt(ewt) => {
                        statements.ewt_stmt.execute(rusqlite::params![
                            self.file_id, "EWT", ewt.transaction_sequence_num, ewt.record_sequence_num,
                            ewt.entire_work_title, ewt.iswc_of_entire_work, ewt.language_code, ewt.writer_1_last_name,
                            ewt.writer_1_first_name, ewt.source, ewt.writer_1_ipi_name_num, ewt.writer_1_ipi_base_number,
                            ewt.writer_2_last_name, ewt.writer_2_first_name, ewt.writer_2_ipi_name_num,
                            ewt.writer_2_ipi_base_number, ewt.submitter_work_num
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ver(ver) => {
                        statements.ver_stmt.execute(rusqlite::params![
                            self.file_id, "VER", ver.transaction_sequence_num, ver.record_sequence_num,
                            ver.original_work_title, ver.iswc_of_original_work, ver.language_code, ver.writer_1_last_name,
                            ver.writer_1_first_name, ver.source, ver.writer_1_ipi_name_num, ver.writer_1_ipi_base_number,
                            ver.writer_2_last_name, ver.writer_2_first_name, ver.writer_2_ipi_name_num,
                            ver.writer_2_ipi_base_number, ver.submitter_work_num
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Orn(orn) => {
                        statements.orn_stmt.execute(rusqlite::params![
                            self.file_id, "ORN", orn.transaction_sequence_num, orn.record_sequence_num,
                            orn.intended_purpose, orn.production_title, orn.cd_identifier, orn.cut_number, orn.library,
                            orn.bltvr, orn.filler, orn.production_num, orn.episode_title, orn.episode_num,
                            orn.year_of_production, orn.avi_society_code, orn.audio_visual_number, orn.v_isan_isan,
                            orn.v_isan_episode, orn.v_isan_check_digit_1, orn.v_isan_version, orn.v_isan_check_digit_2,
                            orn.eidr, orn.eidr_check_digit
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ins(ins) => {
                        statements.ins_stmt.execute(rusqlite::params![
                            self.file_id, "INS", ins.transaction_sequence_num, ins.record_sequence_num,
                            ins.number_of_voices, ins.standard_instrumentation_type, ins.instrumentation_description
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ind(ind) => {
                        statements.ind_stmt.execute(rusqlite::params![
                            self.file_id, "IND", ind.transaction_sequence_num, ind.record_sequence_num,
                            ind.instrument_code, ind.number_of_players
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Com(com) => {
                        statements.com_stmt.execute(rusqlite::params![
                            self.file_id, "COM", com.transaction_sequence_num, com.record_sequence_num, com.title,
                            com.iswc_of_component, com.submitter_work_num, com.duration, com.writer_1_last_name,
                            com.writer_1_first_name, com.writer_1_ipi_name_num, com.writer_2_last_name,
                            com.writer_2_first_name, com.writer_2_ipi_name_num, com.writer_1_ipi_base_number,
                            com.writer_2_ipi_base_number
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Msg(msg) => {
                        statements.msg_stmt.execute(rusqlite::params![
                            self.file_id, "MSG", msg.transaction_sequence_num, msg.record_sequence_num,
                            msg.message_type, msg.original_record_sequence_num, msg.record_type_field,
                            msg.message_level, msg.validation_number, msg.message_text
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Net(net) => {
                        statements.net_stmt.execute(rusqlite::params![
                            self.file_id, net.record_type, net.transaction_sequence_num, net.record_sequence_num,
                            net.title, net.language_code
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Now(now) => {
                        statements.now_stmt.execute(rusqlite::params![
                            self.file_id, "NOW", now.transaction_sequence_num, now.record_sequence_num,
                            now.writer_name, now.writer_first_name, now.language_code, now.writer_position
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Ari(ari) => {
                        statements.ari_stmt.execute(rusqlite::params![
                            self.file_id, "ARI", ari.transaction_sequence_num, ari.record_sequence_num,
                            ari.society_num, ari.work_num, ari.type_of_right, ari.subject_code, ari.note
                        ])?;
                        tx.last_insert_rowid()
                    }
                    allegro_cwr::cwr_record::CwrRecord::Xrf(xrf) => {
                        statements.xrf_stmt.execute(rusqlite::params![
                            self.file_id, "XRF", xrf.transaction_sequence_num, xrf.record_sequence_num,
                            xrf.organisation_code, xrf.identifier, xrf.identifier_type, xrf.validity
                        ])?;
                        tx.last_insert_rowid()
                    }
                };

                // Insert into file_line table for tracking
                insert_file_line_record(&mut statements.file_stmt, self.file_id, parsed_record.line_number, parsed_record.record.record_type(), record_id)?;
            }
        }

        self.processed_count += 1;

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn handle_parse_error(&mut self, line_number: usize, error: &allegro_cwr::CwrParseError) -> std::result::Result<(), Self::Error> {
        self.start_batch()?;

        if let Some(ref mut statements) = self.statements {
            log_error(&mut statements.error_stmt, self.file_id, line_number, error.to_string())?;
        }

        self.error_count += 1;

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn handle_warnings(&mut self, line_number: usize, record_type: &str, warnings: &[String]) -> std::result::Result<(), Self::Error> {
        if warnings.is_empty() {
            return Ok(());
        }

        self.start_batch()?;

        if let Some(ref mut statements) = self.statements {
            for warning in warnings {
                // Store warnings in the error table with "WARNING:" prefix to distinguish from errors
                let warning_description = format!("WARNING [{}]: {}", record_type, warning);
                log_error(&mut statements.error_stmt, self.file_id, line_number, warning_description)?;
                self.error_count += 1;
            }
        }

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn finalize(&mut self) -> std::result::Result<(), Self::Error> {
        // Commit any remaining batch
        self.commit_batch()?;
        Ok(())
    }

    fn get_report(&self) -> String {
        format!("SQLite processing complete:\n  Database: {}\n  Records processed: {}\n  Errors: {}", self.db_filename, self.processed_count, self.error_count)
    }
}

/// Convenience function to process CWR file with SQLite handler
pub fn process_cwr_to_sqlite(input_filename: &str, db_filename: &str) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    process_cwr_to_sqlite_with_version(input_filename, db_filename, None)
}

/// Convenience function to process CWR file with SQLite handler and optional version hint
pub fn process_cwr_to_sqlite_with_version(input_filename: &str, db_filename: &str, version_hint: Option<f32>) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    let handler = SqliteHandler::new(input_filename, db_filename)?;
    let file_id = handler.file_id;
    let report = allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?;

    // Extract count from report (simple parsing for now)
    let processed_count = report.lines().find(|line| line.contains("Records processed:")).and_then(|line| line.split(':').nth(1)).and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);

    Ok((file_id, processed_count, report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_records_are_actually_inserted_into_database() {
        // Create a temporary CWR file with a few records
        let temp_dir = tempdir().unwrap();
        let cwr_file_path = temp_dir.path().join("test.cwr");
        let db_file_path = temp_dir.path().join("test.db");
        
        let mut file = File::create(&cwr_file_path).unwrap();
        
        // Use working test data - focus on demonstrating the scope of missing functionality
        writeln!(file, "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221").unwrap();
        writeln!(file, "GRHAGR0000102.10            ").unwrap();
        writeln!(file, "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ").unwrap();
        
        // Add a few more key record types to demonstrate the scope
        writeln!(file, "SPU0000000100000002000000011357924680SAMPLE PUBLISHER                    N  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "SWR0000000100000003000000013579SAMPLE WRITER              JOHN            A  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "ALT0000000100000004ALTERNATE TITLE                         AT EN           ").unwrap();
        writeln!(file, "PER0000000100000005SAMPLE PERFORMER            1357924680123456789012345678").unwrap();
        writeln!(file, "REC0000000100000006         0000000SAMPLE ALBUM                      SAMPLE LABEL         1234567890123EAN1234567890ISRCCD   SAMPLE RECORDING             SAMPLE VERSION               SAMPLE ARTIST                SAMPLE RECORD LABEL          Y12345678901234567890").unwrap();
        
        // Add many unhandled record types to show the scope of missing functionality
        writeln!(file, "ACK0000000100000007202212211254110000001TRK                                                 20221221A").unwrap();
        writeln!(file, "AGR0000000100000008AGR12345                    C 20221221                                                                                                                                                                                                                  ").unwrap();
        writeln!(file, "ARI0000000100000009123456789012345678901234567890000000PA                ").unwrap();
        writeln!(file, "COM0000000100000010COMPONENT WORK                                         12345670000000WRITER1             JOHN            1234567890123456WRITER2             JANE            1234567890123456123456789012345612345678901234561234567890123456").unwrap();
        writeln!(file, "EWT0000000100000011ENTIRE WORK TITLE                                      EN ENTIRE WORK WRITER   JOHN            SRC 1234567890123456123456789012345SECOND WRITER        JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "IND0000000100000012PI 005").unwrap();
        writeln!(file, "INS0000000100000013010ST                                    ").unwrap();
        writeln!(file, "IPA0000000100000014E01234567890123456123456789012345678901234567890INTERESTED PARTY        FIRST               123456789012345678901234567890         123456789012345678901234567890         123456789012345678901234567890         ").unwrap();
        writeln!(file, "MSG000000010000001501 1HDR000SAMPLE MESSAGE TEXT                                                                                                                                                                                                                                                                                      ").unwrap();
        writeln!(file, "NAT000000010000001601NATIONAL TITLE                         NT EN           ").unwrap();
        writeln!(file, "NET000000010000001701NET TITLE                              EN              ").unwrap();
        writeln!(file, "NOW000000010000001801NOW WRITER NAME         FIRST           EN  1").unwrap();
        writeln!(file, "NPA000000010000001901123456789012345678901234567890INTERESTED PARTY NAME           FIRST           EN  ").unwrap();
        writeln!(file, "NPN000000010000002001001123456789012345678901234567890PUBLISHER NAME                  EN              ").unwrap();
        writeln!(file, "NPR000000010000002101PERFORMING ARTIST       FIRST           1234567890123456123456789012345EN  EN  EN  ").unwrap();
        writeln!(file, "NWN000000010000002201123456789012345678901234567890WRITER NAME             FIRST           EN              ").unwrap();
        writeln!(file, "ORN000000010000002301LSAMPLE PRODUCTION                                                                                                                                                                                    2022123456789012345678901234567890123456789012345612345678901234561234567890123456").unwrap();
        writeln!(file, "PWR000000010000002401123456789012345678901234567890PUBLISHER NAME                                            123456789012345678901234567890001").unwrap();
        writeln!(file, "SPT000000010000002501123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "SWT000000010000002601123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "TER000000010000002701I000000000").unwrap();
        writeln!(file, "VER000000010000002801VERSION WORK TITLE                                   12345670000000EN VERSION WRITER       JOHN            SRC 1234567890123456123456789012345VERSION WRITER 2     JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "XRF000000010000002901123456789012345ABCD1234567890123456789012345678901234567890YV ").unwrap();
        
        writeln!(file, "GRT000010000000010000027").unwrap();
        writeln!(file, "TRL00001000000010000027").unwrap();
        
        // Process the file
        let (file_id, processed_count, _report) = process_cwr_to_sqlite(
            cwr_file_path.to_str().unwrap(),
            db_file_path.to_str().unwrap()
        ).unwrap();
        
        // Verify processing happened  
        assert_eq!(processed_count, 33, "Should have processed 33 records");
        
        // Connect to database and verify records were actually inserted
        let conn = rusqlite::Connection::open(&db_file_path).unwrap();
        
        // Check file_line table - should have entries for each record type
        let mut stmt = conn.prepare("SELECT record_type, COUNT(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type").unwrap();
        let rows: std::result::Result<Vec<(String, i64)>, rusqlite::Error> = stmt.query_map([file_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        }).unwrap().collect();
        
        let record_counts = rows.unwrap();
        println!("Record counts from file_line table: {:?}", record_counts);
        
        // Should have one of each record type (all 31 types from records/mod.rs)
        let expected_records = vec![
            ("ACK".to_string(), 1i64),
            ("AGR".to_string(), 1i64),
            ("ALT".to_string(), 1i64),
            ("ARI".to_string(), 1i64),
            ("COM".to_string(), 1i64),
            ("EWT".to_string(), 1i64),
            ("GRH".to_string(), 1i64),
            ("GRT".to_string(), 1i64),
            ("HDR".to_string(), 1i64),
            ("IND".to_string(), 1i64),
            ("INS".to_string(), 1i64),
            ("IPA".to_string(), 1i64),
            ("MSG".to_string(), 1i64),
            ("NAT".to_string(), 1i64),
            ("NET".to_string(), 1i64),
            ("NOW".to_string(), 1i64),
            ("NPA".to_string(), 1i64),
            ("NPN".to_string(), 1i64),
            ("NPR".to_string(), 1i64),
            ("NWN".to_string(), 1i64),
            ("NWR".to_string(), 1i64),
            ("ORN".to_string(), 1i64),
            ("PER".to_string(), 1i64),
            ("PWR".to_string(), 1i64),
            ("REC".to_string(), 1i64),
            ("SPT".to_string(), 1i64),
            ("SPU".to_string(), 1i64),
            ("SWR".to_string(), 1i64),
            ("SWT".to_string(), 1i64),
            ("TER".to_string(), 1i64),
            ("TRL".to_string(), 1i64),
            ("VER".to_string(), 1i64),
            ("XRF".to_string(), 1i64),
        ];
        assert_eq!(record_counts, expected_records, "file_line table should track all record types");
        
        // MORE IMPORTANTLY: Check that actual record data was inserted into specific tables
        
        // Get all cwr_ table names dynamically
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%' ORDER BY name").unwrap();
        let table_names: Vec<String> = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        }).unwrap().collect::<std::result::Result<Vec<_>, _>>().unwrap();
        
        println!("Found {} cwr_ tables: {:?}", table_names.len(), table_names);
        
        // Check count in each cwr_ table
        let mut total_records_in_tables = 0i64;
        let mut implemented_tables = Vec::new();
        let mut unimplemented_tables = Vec::new();
        
        for table_name in &table_names {
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {} WHERE file_id = ?1", table_name),
                [file_id],
                |row| row.get(0)
            ).unwrap();
            
            total_records_in_tables += count;
            
            if count == 1 {
                implemented_tables.push(table_name.clone());
                println!("✅ {}: {} records", table_name, count);
            } else {
                unimplemented_tables.push(table_name.clone());
                println!("❌ {}: {} records (not implemented)", table_name, count);
            }
        }
        
        // The key assertion: total should equal 33 when all are implemented
        println!("📊 Total records in cwr_ tables: {} / 33", total_records_in_tables);
        assert_eq!(total_records_in_tables, 33, "Should be 33 one for each record type");
        
        // Verify we have the expected number of tables (should be 33 corresponding to all record types)
        assert_eq!(table_names.len(), 33, "Should have 33 cwr_ tables for all record types");
        
        // Verify the HDR record actually contains the parsed data
        let (sender_name, creation_date): (String, String) = conn.query_row(
            "SELECT sender_name, creation_date FROM cwr_hdr WHERE file_id = ?1",
            [file_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        ).unwrap();
        
        assert_eq!(sender_name, "WARNER CHAPPELL MUSIC PUBLISHING LTD", "HDR should contain actual parsed sender name");
        assert_eq!(creation_date, "20221221", "HDR should contain parsed creation date");
        
        println!("🚨 This test demonstrates the missing functionality!");
        println!("📊 Records tracked in file_line: {} types", record_counts.len());
        println!("✅ Implemented tables: {:?}", implemented_tables);
        println!("❌ Unimplemented tables: {:?}", unimplemented_tables);
        println!("🎯 Goal: All 33 record types should sum to 33 total records in cwr_ tables");
    }
}
