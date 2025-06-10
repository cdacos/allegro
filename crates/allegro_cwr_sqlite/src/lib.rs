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

use allegro_cwr::domain_types::{
    Boolean, Flag, LanguageCode, MusicalWorkDistributionCategory, Number, SocietyCode, TransactionStatus, VersionType,
};
use domain_conversions::{
    CwrFromSqlString, CwrToSqlInt, CwrToSqlString, opt_domain_to_string, opt_string_to_domain, opt_string_to_numeric,
};

/// Trait for inserting CWR records into SQLite
pub trait SqliteInsertable {
    /// Get the table name for this record type (e.g., "cwr_hdr")
    fn table_name(&self) -> &'static str;

    /// Convert record fields to SQL parameters
    fn to_sql_params(&self, file_id: i64) -> Vec<Box<dyn rusqlite::types::ToSql>>;

    /// Execute insertion using appropriate prepared statement
    fn execute_insert(
        &self, statements: &mut PreparedStatements, tx: &rusqlite::Transaction, file_id: i64,
    ) -> Result<i64>;
}

/// Trait for querying CWR records from SQLite
pub trait SqliteQueryable: Sized {
    /// Get the table name for this record type
    fn table_name() -> &'static str;

    /// Construct a record from a SQL row
    fn from_sql_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
}

// Implementation of SqliteInsertable for CwrRegistry - this centralizes the 33-case match logic
impl SqliteInsertable for allegro_cwr::CwrRegistry {
    fn table_name(&self) -> &'static str {
        match self {
            allegro_cwr::CwrRegistry::Hdr(_) => "cwr_hdr",
            allegro_cwr::CwrRegistry::Grh(_) => "cwr_grh",
            allegro_cwr::CwrRegistry::Grt(_) => "cwr_grt",
            allegro_cwr::CwrRegistry::Trl(_) => "cwr_trl",
            allegro_cwr::CwrRegistry::Agr(_) => "cwr_agr",
            allegro_cwr::CwrRegistry::Nwr(_) => "cwr_nwr",
            allegro_cwr::CwrRegistry::Ack(_) => "cwr_ack",
            allegro_cwr::CwrRegistry::Ter(_) => "cwr_ter",
            allegro_cwr::CwrRegistry::Ipa(_) => "cwr_ipa",
            allegro_cwr::CwrRegistry::Npa(_) => "cwr_npa",
            allegro_cwr::CwrRegistry::Spu(_) => "cwr_spu",
            allegro_cwr::CwrRegistry::Npn(_) => "cwr_npn",
            allegro_cwr::CwrRegistry::Spt(_) => "cwr_spt",
            allegro_cwr::CwrRegistry::Swr(_) => "cwr_swr",
            allegro_cwr::CwrRegistry::Nwn(_) => "cwr_nwn",
            allegro_cwr::CwrRegistry::Swt(_) => "cwr_swt",
            allegro_cwr::CwrRegistry::Pwr(_) => "cwr_pwr",
            allegro_cwr::CwrRegistry::Alt(_) => "cwr_alt",
            allegro_cwr::CwrRegistry::Nat(_) => "cwr_nat",
            allegro_cwr::CwrRegistry::Ewt(_) => "cwr_ewt",
            allegro_cwr::CwrRegistry::Ver(_) => "cwr_ver",
            allegro_cwr::CwrRegistry::Per(_) => "cwr_per",
            allegro_cwr::CwrRegistry::Npr(_) => "cwr_npr",
            allegro_cwr::CwrRegistry::Rec(_) => "cwr_rec",
            allegro_cwr::CwrRegistry::Orn(_) => "cwr_orn",
            allegro_cwr::CwrRegistry::Ins(_) => "cwr_ins",
            allegro_cwr::CwrRegistry::Ind(_) => "cwr_ind",
            allegro_cwr::CwrRegistry::Com(_) => "cwr_com",
            allegro_cwr::CwrRegistry::Msg(_) => "cwr_msg",
            allegro_cwr::CwrRegistry::Net(_) => "cwr_net",
            allegro_cwr::CwrRegistry::Now(_) => "cwr_now",
            allegro_cwr::CwrRegistry::Ari(_) => "cwr_ari",
            allegro_cwr::CwrRegistry::Xrf(_) => "cwr_xrf",
        }
    }

    fn to_sql_params(&self, _file_id: i64) -> Vec<Box<dyn rusqlite::types::ToSql>> {
        // This will be implemented via execute_insert for now
        // since the parameter structure varies significantly between record types
        vec![]
    }

    fn execute_insert(
        &self, statements: &mut PreparedStatements, tx: &rusqlite::Transaction, file_id: i64,
    ) -> Result<i64> {
        use rusqlite::params;

        match self {
            allegro_cwr::CwrRegistry::Hdr(hdr) => {
                statements.hdr_stmt.execute(params![
                    file_id,
                    "HDR",
                    hdr.sender_type.as_str(),
                    hdr.sender_id.as_str(),
                    hdr.sender_name.as_str(),
                    hdr.edi_standard_version_number.as_str(),
                    hdr.creation_date.as_str(),
                    hdr.creation_time.as_str(),
                    hdr.transmission_date.as_str(),
                    opt_domain_to_string(&hdr.character_set),
                    hdr.version.as_ref().map(|v| v.as_str()),
                    hdr.revision.as_ref().map(|r| r.as_str()),
                    hdr.software_package,
                    hdr.software_package_version
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Grh(grh) => {
                statements.grh_stmt.execute(params![
                    file_id,
                    "GRH",
                    grh.transaction_type.to_sql_string(),
                    grh.group_id.to_sql_int(),
                    grh.version_number.as_str(),
                    grh.batch_request.as_ref().map(|n| n.to_string()).as_deref(),
                    grh.submission_distribution_type
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Grt(grt) => {
                statements.grt_stmt.execute(params![
                    file_id,
                    "GRT",
                    grt.group_id.to_sql_int(),
                    grt.transaction_count.to_sql_int(),
                    grt.record_count.to_sql_int(),
                    grt.currency_indicator.as_ref().map(|c| c.to_sql_string()),
                    grt.total_monetary_value.as_ref().map(|n| n.to_string()).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Trl(trl) => {
                statements.trl_stmt.execute(params![
                    file_id,
                    "TRL",
                    trl.group_count.to_sql_int(),
                    trl.transaction_count.to_sql_int(),
                    trl.record_count.to_sql_int()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Agr(agr) => {
                statements.agr_stmt.execute(params![
                    file_id,
                    "AGR",
                    agr.transaction_sequence_num.as_str(),
                    agr.record_sequence_num.as_str(),
                    agr.submitter_agreement_number.as_str(),
                    agr.international_standard_agreement_code.as_deref(),
                    agr.agreement_type.to_sql_string(),
                    agr.agreement_start_date.as_str(),
                    agr.agreement_end_date.as_ref().map(|d| d.as_str()),
                    agr.retention_end_date.as_ref().map(|d| d.as_str()),
                    agr.prior_royalty_status.to_sql_string(),
                    agr.prior_royalty_start_date.as_ref().map(|d| d.as_str()),
                    agr.post_term_collection_status.to_sql_string(),
                    agr.post_term_collection_end_date.as_ref().map(|d| d.as_str()),
                    agr.date_of_signature_of_agreement.as_ref().map(|d| d.as_str()),
                    agr.number_of_works.to_sql_int(),
                    agr.sales_manufacture_clause.as_ref().map(|c| c.as_str()),
                    opt_domain_to_string(&agr.shares_change).as_deref(),
                    opt_domain_to_string(&agr.advance_given).as_deref(),
                    agr.society_assigned_agreement_number.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Nwr(nwr) => {
                statements.nwr_stmt.execute(params![
                    file_id,
                    "NWR",
                    nwr.transaction_sequence_num.as_str(),
                    nwr.record_sequence_num.as_str(),
                    nwr.work_title.as_str(),
                    &opt_domain_to_string(&nwr.language_code),
                    nwr.submitter_work_num.as_str(),
                    nwr.iswc.as_deref(),
                    opt_domain_to_string(&nwr.copyright_date).as_deref(),
                    nwr.copyright_number.as_deref(),
                    nwr.musical_work_distribution_category.to_sql_string(),
                    opt_domain_to_string(&nwr.duration).as_deref(),
                    nwr.recorded_indicator.to_sql_string(),
                    &opt_domain_to_string(&nwr.text_music_relationship),
                    &opt_domain_to_string(&nwr.composite_type),
                    nwr.version_type.to_sql_string(),
                    &opt_domain_to_string(&nwr.excerpt_type),
                    &opt_domain_to_string(&nwr.music_arrangement),
                    &opt_domain_to_string(&nwr.lyric_adaptation),
                    nwr.contact_name.as_deref(),
                    nwr.contact_id.as_deref(),
                    &opt_domain_to_string(&nwr.cwr_work_type),
                    opt_domain_to_string(&nwr.grand_rights_ind).as_deref(),
                    nwr.composite_component_count.as_ref().map(|c| c.to_sql_int()),
                    opt_domain_to_string(&nwr.date_of_publication_of_printed_edition).as_deref(),
                    opt_domain_to_string(&nwr.exceptional_clause).as_deref(),
                    nwr.opus_number.as_deref(),
                    nwr.catalogue_number.as_deref(),
                    opt_domain_to_string(&nwr.priority_flag).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ack(ack) => {
                statements.ack_stmt.execute(params![
                    file_id,
                    "ACK",
                    ack.transaction_sequence_num.as_str(),
                    ack.record_sequence_num.as_str(),
                    ack.creation_date.as_str(),
                    ack.creation_time.as_str(),
                    ack.original_group_id.to_sql_int(),
                    ack.original_transaction_sequence_num.as_str(),
                    ack.original_transaction_type.to_sql_string(),
                    ack.creation_title.as_deref(),
                    ack.submitter_creation_num.as_deref(),
                    ack.recipient_creation_num.as_deref(),
                    ack.processing_date.as_str(),
                    ack.transaction_status.to_sql_string()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ter(ter) => {
                statements.ter_stmt.execute(params![
                    file_id,
                    "TER",
                    ter.transaction_sequence_num.as_str(),
                    ter.record_sequence_num.as_str(),
                    ter.inclusion_exclusion_indicator.to_sql_string(),
                    ter.tis_numeric_code.to_sql_int()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ipa(ipa) => {
                statements.ipa_stmt.execute(params![
                    file_id,
                    "IPA",
                    ipa.transaction_sequence_num.as_str(),
                    ipa.record_sequence_num.as_str(),
                    ipa.agreement_role_code.to_sql_string(),
                    ipa.interested_party_ipi_name_num.as_deref(),
                    ipa.ipi_base_number.as_deref(),
                    ipa.interested_party_num.as_str(),
                    ipa.interested_party_last_name.as_str(),
                    ipa.interested_party_writer_first_name.as_deref(),
                    ipa.pr_affiliation_society.as_deref(),
                    ipa.pr_share.as_ref().map(|s| s.to_sql_int()),
                    ipa.mr_affiliation_society.as_deref(),
                    ipa.mr_share.as_ref().map(|s| s.to_sql_int()),
                    ipa.sr_affiliation_society.as_deref(),
                    ipa.sr_share.as_ref().map(|s| s.to_sql_int())
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Npa(npa) => {
                statements.npa_stmt.execute(params![
                    file_id,
                    "NPA",
                    npa.transaction_sequence_num.as_str(),
                    npa.record_sequence_num.as_str(),
                    npa.interested_party_num.as_deref(),
                    npa.interested_party_name.as_str(),
                    npa.interested_party_writer_first_name.as_str(),
                    npa.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Spu(spu) => {
                statements.spu_stmt.execute(params![
                    file_id,
                    "SPU",
                    spu.transaction_sequence_num.as_str(),
                    spu.record_sequence_num.as_str(),
                    spu.publisher_sequence_num.to_sql_int(),
                    spu.interested_party_num.as_deref(),
                    spu.publisher_name.as_deref(),
                    opt_domain_to_string(&spu.publisher_unknown_indicator).as_deref(),
                    spu.publisher_type.as_ref().map(|p| p.to_sql_string()).as_deref(),
                    spu.tax_id_num.as_deref(),
                    spu.publisher_ipi_name_num.as_deref(),
                    spu.submitter_agreement_number.as_deref(),
                    spu.pr_affiliation_society_num.as_deref(),
                    spu.pr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    spu.mr_society.as_deref(),
                    spu.mr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    spu.sr_society.as_deref(),
                    spu.sr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    opt_domain_to_string(&spu.special_agreements_indicator).as_deref(),
                    opt_domain_to_string(&spu.first_recording_refusal_ind).as_deref(),
                    spu.filler.as_ref().map(|n| n.to_string()).as_deref(),
                    spu.publisher_ipi_base_number.as_deref(),
                    spu.international_standard_agreement_code.as_deref(),
                    spu.society_assigned_agreement_number.as_deref(),
                    spu.agreement_type.as_ref().map(|x| x.as_str()),
                    opt_domain_to_string(&spu.usa_license_ind).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Npn(npn) => {
                statements.npn_stmt.execute(params![
                    file_id,
                    "NPN",
                    npn.transaction_sequence_num.as_str(),
                    npn.record_sequence_num.as_str(),
                    npn.publisher_sequence_num.to_sql_int(),
                    npn.interested_party_num.as_str(),
                    npn.publisher_name.as_str(),
                    npn.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Spt(spt) => {
                statements.spt_stmt.execute(params![
                    file_id,
                    "SPT",
                    spt.transaction_sequence_num.as_str(),
                    spt.record_sequence_num.as_str(),
                    spt.interested_party_num.as_str(),
                    "", // constant_spaces
                    spt.pr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    spt.mr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    spt.sr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    spt.inclusion_exclusion_indicator.to_sql_string(),
                    spt.tis_numeric_code.to_sql_int(),
                    opt_domain_to_string(&spt.shares_change).as_deref(),
                    spt.sequence_num.as_ref().map(|n| n.to_string()).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Swr(swr) => {
                statements.swr_stmt.execute(params![
                    file_id,
                    "SWR",
                    swr.transaction_sequence_num.as_str(),
                    swr.record_sequence_num.as_str(),
                    swr.interested_party_num.as_deref(),
                    swr.writer_last_name.as_deref(),
                    swr.writer_first_name.as_deref(),
                    opt_domain_to_string(&swr.writer_unknown_indicator).as_deref(),
                    swr.writer_designation_code.as_ref().map(|x| x.as_str()),
                    swr.tax_id_num.as_deref(),
                    swr.writer_ipi_name_num.as_deref(),
                    swr.pr_affiliation_society_num.as_deref(),
                    swr.pr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    swr.mr_society.as_deref(),
                    swr.mr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    swr.sr_society.as_deref(),
                    swr.sr_ownership_share.as_ref().map(|s| s.to_sql_int()),
                    opt_domain_to_string(&swr.reversionary_indicator).as_deref(),
                    opt_domain_to_string(&swr.first_recording_refusal_ind).as_deref(),
                    opt_domain_to_string(&swr.work_for_hire_indicator).as_deref(),
                    swr.filler.as_ref().map(|n| n.to_string()).as_deref(),
                    swr.writer_ipi_base_number.as_deref(),
                    swr.personal_number.as_ref().map(|n| n.to_string()).as_deref(),
                    opt_domain_to_string(&swr.usa_license_ind).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Nwn(nwn) => {
                statements.nwn_stmt.execute(params![
                    file_id,
                    "NWN",
                    nwn.transaction_sequence_num.as_str(),
                    nwn.record_sequence_num.as_str(),
                    nwn.interested_party_num.as_deref(),
                    nwn.writer_last_name.as_str(),
                    nwn.writer_first_name.as_deref(),
                    nwn.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Swt(swt) => {
                statements.swt_stmt.execute(params![
                    file_id,
                    "SWT",
                    swt.transaction_sequence_num.as_str(),
                    swt.record_sequence_num.as_str(),
                    swt.interested_party_num.as_deref(),
                    swt.pr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    swt.mr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    swt.sr_collection_share.as_ref().map(|s| s.to_sql_int()),
                    swt.inclusion_exclusion_indicator.to_sql_string(),
                    swt.tis_numeric_code.to_sql_int(),
                    opt_domain_to_string(&swt.shares_change).as_deref(),
                    swt.sequence_num.as_ref().map(|n| n.to_string()).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Pwr(pwr) => {
                statements.pwr_stmt.execute(params![
                    file_id,
                    "PWR",
                    pwr.transaction_sequence_num.as_str(),
                    pwr.record_sequence_num.as_str(),
                    pwr.publisher_ip_num.as_deref(),
                    pwr.publisher_name.as_deref(),
                    pwr.submitter_agreement_number.as_deref(),
                    pwr.society_assigned_agreement_number.as_deref(),
                    pwr.writer_ip_num.as_deref(),
                    pwr.publisher_sequence_num.as_ref().map(|s| s.to_sql_int())
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Alt(alt) => {
                statements.alt_stmt.execute(params![
                    file_id,
                    "ALT",
                    alt.transaction_sequence_num.as_str(),
                    alt.record_sequence_num.as_str(),
                    alt.alternate_title.as_str(),
                    alt.title_type.to_sql_string(),
                    alt.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Nat(nat) => {
                statements.nat_stmt.execute(params![
                    file_id,
                    "NAT",
                    nat.transaction_sequence_num.as_str(),
                    nat.record_sequence_num.as_str(),
                    nat.title.as_str(),
                    nat.title_type.to_sql_string(),
                    nat.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ewt(ewt) => {
                statements.ewt_stmt.execute(params![
                    file_id,
                    "EWT",
                    ewt.transaction_sequence_num.as_str(),
                    ewt.record_sequence_num.as_str(),
                    ewt.entire_work_title.as_str(),
                    ewt.iswc_of_entire_work.as_deref(),
                    ewt.language_code.as_deref(),
                    ewt.writer_1_last_name.as_deref(),
                    ewt.writer_1_first_name.as_deref(),
                    ewt.source.as_deref(),
                    ewt.writer_1_ipi_name_num.as_deref(),
                    ewt.writer_1_ipi_base_number.as_deref(),
                    ewt.writer_2_last_name.as_deref(),
                    ewt.writer_2_first_name.as_deref(),
                    ewt.writer_2_ipi_name_num.as_deref(),
                    ewt.writer_2_ipi_base_number.as_deref(),
                    ewt.submitter_work_num.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ver(ver) => {
                statements.ver_stmt.execute(params![
                    file_id,
                    "VER",
                    ver.transaction_sequence_num.as_str(),
                    ver.record_sequence_num.as_str(),
                    ver.original_work_title.as_str(),
                    ver.iswc_of_original_work.as_deref(),
                    ver.language_code.as_deref(),
                    ver.writer_1_last_name.as_deref(),
                    ver.writer_1_first_name.as_deref(),
                    ver.source.as_deref(),
                    ver.writer_1_ipi_name_num.as_deref(),
                    ver.writer_1_ipi_base_number.as_deref(),
                    ver.writer_2_last_name.as_deref(),
                    ver.writer_2_first_name.as_deref(),
                    ver.writer_2_ipi_name_num.as_deref(),
                    ver.writer_2_ipi_base_number.as_deref(),
                    ver.submitter_work_num.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Per(per) => {
                statements.per_stmt.execute(params![
                    file_id,
                    "PER",
                    per.transaction_sequence_num.as_str(),
                    per.record_sequence_num.as_str(),
                    per.performing_artist_last_name.as_str(),
                    per.performing_artist_first_name.as_deref(),
                    per.performing_artist_ipi_name_num.as_deref(),
                    per.performing_artist_ipi_base_number.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Npr(npr) => {
                statements.npr_stmt.execute(params![
                    file_id,
                    "NPR",
                    npr.transaction_sequence_num.as_str(),
                    npr.record_sequence_num.as_str(),
                    npr.performing_artist_name.as_deref(),
                    npr.performing_artist_first_name.as_deref(),
                    npr.performing_artist_ipi_name_num.as_deref(),
                    npr.performing_artist_ipi_base_number.as_deref(),
                    npr.language_code.as_deref(),
                    npr.performance_language.as_deref(),
                    npr.performance_dialect.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Rec(rec) => {
                statements.rec_stmt.execute(params![
                    file_id,
                    "REC",
                    rec.transaction_sequence_num.as_str(),
                    rec.record_sequence_num.as_str(),
                    rec.release_date.as_ref().map(|d| d.as_str()),
                    "", // constant_blanks_1
                    opt_domain_to_string(&rec.release_duration).as_deref(),
                    "", // constant_blanks_2
                    rec.album_title.as_deref(),
                    rec.album_label.as_deref(),
                    rec.release_catalog_num.as_deref(),
                    rec.ean.as_deref(),
                    rec.isrc.as_deref(),
                    opt_domain_to_string(&rec.recording_format).as_deref(),
                    opt_domain_to_string(&rec.recording_technique).as_deref(),
                    rec.media_type.as_deref(),
                    rec.recording_title.as_deref(),
                    rec.version_title.as_deref(),
                    rec.display_artist.as_deref(),
                    rec.record_label.as_deref(),
                    rec.isrc_validity.as_ref().map(|x| x.as_str()),
                    rec.submitter_recording_identifier.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Orn(orn) => {
                statements.orn_stmt.execute(params![
                    file_id,
                    "ORN",
                    orn.transaction_sequence_num.as_str(),
                    orn.record_sequence_num.as_str(),
                    orn.intended_purpose.to_sql_string(),
                    orn.production_title.as_deref(),
                    orn.cd_identifier.as_deref(),
                    orn.cut_number.as_ref().map(|n| n.to_string()).as_deref(),
                    orn.library.as_deref(),
                    opt_domain_to_string(&orn.bltvr).as_deref(),
                    orn.filler.as_ref().map(|n| n.to_string()).as_deref(),
                    orn.production_num.as_deref(),
                    orn.episode_title.as_deref(),
                    orn.episode_num.as_deref(),
                    orn.year_of_production.as_ref().map(|n| n.to_string()).as_deref(),
                    orn.avi_society_code.as_ref().map(|n| n.to_string()).as_deref(),
                    orn.audio_visual_number.as_deref(),
                    orn.v_isan_isan.as_deref(),
                    orn.v_isan_episode.as_deref(),
                    orn.v_isan_check_digit_1.as_deref(),
                    orn.v_isan_version.as_deref(),
                    orn.v_isan_check_digit_2.as_deref(),
                    orn.eidr.as_deref(),
                    orn.eidr_check_digit.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ins(ins) => {
                statements.ins_stmt.execute(params![
                    file_id,
                    "INS",
                    ins.transaction_sequence_num.as_str(),
                    ins.record_sequence_num.as_str(),
                    ins.number_of_voices.as_ref().map(|n| n.to_string()).as_deref(),
                    ins.standard_instrumentation_type.as_deref(),
                    ins.instrumentation_description.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ind(ind) => {
                statements.ind_stmt.execute(params![
                    file_id,
                    "IND",
                    ind.transaction_sequence_num.as_str(),
                    ind.record_sequence_num.as_str(),
                    ind.instrument_code.to_sql_string(),
                    ind.number_of_players.as_ref().map(|n| n.to_string()).as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Com(com) => {
                statements.com_stmt.execute(params![
                    file_id,
                    "COM",
                    com.transaction_sequence_num.as_str(),
                    com.record_sequence_num.as_str(),
                    com.title.as_str(),
                    com.iswc_of_component.as_deref(),
                    com.submitter_work_num.as_deref(),
                    opt_domain_to_string(&com.duration).as_deref(),
                    com.writer_1_last_name.as_str(),
                    com.writer_1_first_name.as_deref(),
                    com.writer_1_ipi_name_num.as_deref(),
                    com.writer_2_last_name.as_deref(),
                    com.writer_2_first_name.as_deref(),
                    com.writer_2_ipi_name_num.as_deref(),
                    com.writer_1_ipi_base_number.as_deref(),
                    com.writer_2_ipi_base_number.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Msg(msg) => {
                statements.msg_stmt.execute(params![
                    file_id,
                    "MSG",
                    msg.transaction_sequence_num.as_str(),
                    msg.record_sequence_num.as_str(),
                    msg.message_type.to_sql_string(),
                    msg.original_record_sequence_num.as_str(),
                    msg.record_type_field.as_str(),
                    msg.message_level.to_sql_string(),
                    msg.validation_number.as_str(),
                    msg.message_text.as_str()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Net(net) => {
                statements.net_stmt.execute(params![
                    file_id,
                    "NET",
                    net.transaction_sequence_num.as_str(),
                    net.record_sequence_num.as_str(),
                    net.title.as_str(),
                    net.language_code.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Now(now) => {
                statements.now_stmt.execute(params![
                    file_id,
                    "NOW",
                    now.transaction_sequence_num.as_str(),
                    now.record_sequence_num.as_str(),
                    now.writer_name.as_str(),
                    now.writer_first_name.as_str(),
                    now.language_code.as_deref(),
                    now.writer_position.as_ref().map(|p| p.as_str())
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Ari(ari) => {
                statements.ari_stmt.execute(params![
                    file_id,
                    "ARI",
                    ari.transaction_sequence_num.as_str(),
                    ari.record_sequence_num.as_str(),
                    ari.society_num.to_sql_string(),
                    ari.work_num.as_deref(),
                    ari.type_of_right.to_sql_string(),
                    ari.subject_code.as_ref().map(|x| x.as_str()),
                    ari.note.as_deref()
                ])?;
                Ok(tx.last_insert_rowid())
            }
            allegro_cwr::CwrRegistry::Xrf(xrf) => {
                statements.xrf_stmt.execute(params![
                    file_id,
                    "XRF",
                    xrf.transaction_sequence_num.as_str(),
                    xrf.record_sequence_num.as_str(),
                    xrf.organisation_code.to_sql_string(),
                    xrf.identifier.as_str(),
                    xrf.identifier_type.to_sql_string(),
                    xrf.validity.to_sql_string()
                ])?;
                Ok(tx.last_insert_rowid())
            }
        }
    }
}

// Re-export main types and functions
pub use connection::{CwrDatabase, determine_db_filename, setup_database};
pub use error::CwrDbError;
pub use operations::{
    CwrRecordInserter, count_errors_by_record_type, count_records_by_type, insert_file_line_record, insert_file_record,
    log_error,
};
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

        Ok(SqliteHandler {
            conn,
            tx: None,
            file_id,
            processed_count: 0,
            error_count: 0,
            db_filename: db_filename.to_string(),
            batch_size,
            statements: None,
        })
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
                // Use the trait method to execute the insertion - replaces 434 lines of match statement!
                let record_id = parsed_record.record.execute_insert(statements, tx, self.file_id)?;

                // Insert into file_line table for tracking
                insert_file_line_record(
                    &mut statements.file_stmt,
                    self.file_id,
                    parsed_record.line_number,
                    parsed_record.record.record_type(),
                    record_id,
                )?;
            }
        }

        self.processed_count += 1;

        if self.should_commit_batch() {
            self.commit_batch()?;
        }

        Ok(())
    }

    fn handle_parse_error(
        &mut self, line_number: usize, error: &allegro_cwr::CwrParseError,
    ) -> std::result::Result<(), Self::Error> {
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

    fn handle_warnings(
        &mut self, line_number: usize, record_type: &str, warnings: &[String],
    ) -> std::result::Result<(), Self::Error> {
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
        format!(
            "SQLite processing complete:\n  Database: {}\n  Records processed: {}\n  Errors: {}",
            self.db_filename, self.processed_count, self.error_count
        )
    }
}

/// Convenience function to process CWR file with SQLite handler
pub fn process_cwr_to_sqlite(
    input_filename: &str, db_filename: &str,
) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    process_cwr_to_sqlite_with_version(input_filename, db_filename, None)
}

/// Convenience function to process CWR file with SQLite handler and optional version hint
pub fn process_cwr_to_sqlite_with_version(
    input_filename: &str, db_filename: &str, version_hint: Option<f32>,
) -> std::result::Result<(i64, usize, String), Box<dyn std::error::Error>> {
    let handler = SqliteHandler::new(input_filename, db_filename)?;
    let file_id = handler.file_id;
    let report = allegro_cwr::process_cwr_with_handler_and_version(input_filename, handler, version_hint)?;

    // Extract count from report (simple parsing for now)
    let processed_count = report
        .lines()
        .find(|line| line.contains("Records processed:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0);

    Ok((file_id, processed_count, report))
}

/// Convenience function to process SQLite database and output CWR with optional version hint and output file
/// This demonstrates the SQLite-to-CWR write pattern, following the same approach as JSON-to-CWR
pub fn process_sqlite_to_cwr_with_version_and_output(
    db_filename: &str, file_id: i64, version_hint: Option<f32>, output_filename: Option<&str>,
) -> std::result::Result<usize, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::{self, Write};

    // Open database connection
    let conn = rusqlite::Connection::open(db_filename)?;

    // Get CWR version from the database or use hint
    let _cwr_version = allegro_cwr::domain_types::CwrVersion(version_hint.or(Some(2.2)).expect("Hardcoded?!"));

    // Create output writer
    let mut output: Box<dyn Write> = match output_filename {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    };

    // For demonstration, let's implement a simple approach using the stored record lines
    // In a full implementation, we would query each record type and reconstruct the CWR lines
    // This follows the same streaming pattern as the JSON handler

    // Query records in order using file_line table to maintain original file order
    let mut stmt = conn.prepare(
        "
        SELECT fl.record_type, fl.record_id
        FROM file_line fl
        WHERE fl.file_id = ?1 
        ORDER BY fl.line_number, fl.insert_position
    ",
    )?;

    let record_rows = stmt.query_map([file_id], |row| {
        Ok((
            row.get::<_, String>(0)?, // record_type
            row.get::<_, i64>(1)?,    // record_id
        ))
    })?;

    let mut count = 0;
    for record_result in record_rows {
        let (record_type, record_id) = record_result?;

        // Query and reconstruct the actual record from database fields
        if let Some(cwr_record) = query_record_by_type(&conn, &record_type, record_id)? {
            let cwr_line = cwr_record.to_cwr_line(&_cwr_version);
            writeln!(output, "{}", cwr_line)?;
            count += 1; // Only count successfully reconstructed records
        }
        // Skip records that couldn't be reconstructed (not yet implemented)
    }

    output.flush()?;
    Ok(count)
}

/// Query a specific record by type and reconstruct it from database fields
/// This demonstrates the key challenge: converting database strings back to domain types
fn query_record_by_type(
    conn: &rusqlite::Connection, record_type: &str, record_id: i64,
) -> Result<Option<allegro_cwr::CwrRegistry>> {
    use rusqlite::params;

    match record_type {
        "HDR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_hdr WHERE cwr_hdr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                // Reconstruct HDR record from database fields
                // This is where we need to convert database strings back to domain types
                let hdr = allegro_cwr::records::HdrRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    sender_type: {
                        use allegro_cwr::domain_types::SenderType;
                        SenderType::from_sql_string(&row.get::<_, String>("sender_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sender_id: {
                        use allegro_cwr::domain_types::SenderId;
                        SenderId::from_sql_string(&row.get::<_, String>("sender_id")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sender_name: {
                        use allegro_cwr::domain_types::SenderName;
                        SenderName::from_sql_string(&row.get::<_, String>("sender_name")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    edi_standard_version_number: {
                        use allegro_cwr::domain_types::EdiStandardVersion;
                        EdiStandardVersion::from_sql_string(&row.get::<_, String>("edi_standard_version_number")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    creation_date: {
                        use allegro_cwr::domain_types::Date;
                        Date::from_sql_string(&row.get::<_, String>("creation_date")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    creation_time: {
                        use allegro_cwr::domain_types::Time;
                        Time::from_sql_string(&row.get::<_, String>("creation_time")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    transmission_date: {
                        use allegro_cwr::domain_types::Date;
                        Date::from_sql_string(&row.get::<_, String>("transmission_date")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    character_set: {
                        use allegro_cwr::domain_types::CharacterSet;
                        opt_string_to_domain::<CharacterSet>(row.get::<_, Option<String>>("character_set")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    version: {
                        use allegro_cwr::domain_types::CwrVersion;
                        match row.get::<_, Option<String>>("version")? {
                            Some(version_str) => {
                                Some(CwrVersion::from_sql_string(&version_str).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                    revision: {
                        use allegro_cwr::domain_types::CwrRevision;
                        match row.get::<_, Option<String>>("revision")? {
                            Some(revision_str) => {
                                Some(CwrRevision::from_sql_string(&revision_str).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                    software_package: row.get::<_, Option<String>>("software_package")?,
                    software_package_version: row.get::<_, Option<String>>("software_package_version")?,
                };
                Ok(hdr)
            }) {
                Ok(hdr) => Ok(Some(allegro_cwr::CwrRegistry::Hdr(hdr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "GRH" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_grh WHERE cwr_grh_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let grh = allegro_cwr::records::GrhRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_type: {
                        use allegro_cwr::domain_types::TransactionType;
                        TransactionType::from_sql_string(&row.get::<_, String>("transaction_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    group_id: {
                        use allegro_cwr::domain_types::GroupId;
                        GroupId::from_sql_string(&row.get::<_, String>("group_id")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    version_number: {
                        use allegro_cwr::domain_types::CwrVersionNumber;
                        CwrVersionNumber::from_sql_string(
                            &row.get::<_, String>("version_number_for_this_transaction_type")?,
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    batch_request: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("batch_request")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    submission_distribution_type: row.get::<_, Option<String>>("submission_distribution_type")?,
                };
                Ok(grh)
            }) {
                Ok(grh) => Ok(Some(allegro_cwr::CwrRegistry::Grh(grh))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "GRT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_grt WHERE cwr_grt_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let grt = allegro_cwr::records::GrtRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    group_id: {
                        use allegro_cwr::domain_types::GroupId;
                        GroupId::from_sql_string(&row.get::<_, String>("group_id")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    transaction_count: {
                        use allegro_cwr::domain_types::TransactionCount;
                        TransactionCount::from_sql_string(&row.get::<_, String>("transaction_count")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_count: {
                        use allegro_cwr::domain_types::RecordCount;
                        RecordCount::from_sql_string(&row.get::<_, String>("record_count")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    currency_indicator: {
                        use allegro_cwr::domain_types::CurrencyCode;
                        match row.get::<_, Option<String>>("currency_indicator")? {
                            Some(currency_str) => {
                                Some(CurrencyCode::from_sql_string(&currency_str).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                    total_monetary_value: {
                        match row.get::<_, Option<String>>("total_monetary_value")? {
                            Some(total_monetary_value) => {
                                Some(Number::from_sql_string(&total_monetary_value).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                };
                Ok(grt)
            }) {
                Ok(grt) => Ok(Some(allegro_cwr::CwrRegistry::Grt(grt))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "TRL" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_trl WHERE cwr_trl_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let trl = allegro_cwr::records::TrlRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    group_count: {
                        use allegro_cwr::domain_types::GroupCount;
                        GroupCount::from_sql_string(&row.get::<_, String>("group_count")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    transaction_count: {
                        use allegro_cwr::domain_types::TransactionCount;
                        TransactionCount::from_sql_string(&row.get::<_, String>("transaction_count")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_count: {
                        use allegro_cwr::domain_types::RecordCount;
                        RecordCount::from_sql_string(&row.get::<_, String>("record_count")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(trl)
            }) {
                Ok(trl) => Ok(Some(allegro_cwr::CwrRegistry::Trl(trl))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NWR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_nwr WHERE cwr_nwr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let nwr = allegro_cwr::records::NwrRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    work_title: row.get::<_, String>("work_title")?,
                    language_code: opt_string_to_domain(row.get::<_, Option<String>>("language_code")?.as_deref())
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    submitter_work_num: row.get::<_, String>("submitter_work_num")?,
                    iswc: row.get::<_, Option<String>>("iswc")?,
                    copyright_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(row.get::<_, Option<String>>("copyright_date")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    copyright_number: row.get::<_, Option<String>>("copyright_number")?,
                    musical_work_distribution_category: MusicalWorkDistributionCategory::from_sql_string(
                        &row.get::<_, String>("musical_work_distribution_category")?,
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    duration: {
                        use allegro_cwr::domain_types::Time;
                        opt_string_to_domain::<Time>(row.get::<_, Option<String>>("duration")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    recorded_indicator: {
                        use allegro_cwr::domain_types::Flag;
                        Flag::from_sql_string(&row.get::<_, String>("recorded_indicator")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    text_music_relationship: opt_string_to_domain(
                        row.get::<_, Option<String>>("text_music_relationship")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    composite_type: opt_string_to_domain(row.get::<_, Option<String>>("composite_type")?.as_deref())
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    version_type: VersionType::from_sql_string(&row.get::<_, String>("version_type")?)
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    excerpt_type: opt_string_to_domain(row.get::<_, Option<String>>("excerpt_type")?.as_deref())
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    music_arrangement: opt_string_to_domain(
                        row.get::<_, Option<String>>("music_arrangement")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    lyric_adaptation: opt_string_to_domain(
                        row.get::<_, Option<String>>("lyric_adaptation")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    contact_name: row.get::<_, Option<String>>("contact_name")?,
                    contact_id: row.get::<_, Option<String>>("contact_id")?,
                    cwr_work_type: opt_string_to_domain(row.get::<_, Option<String>>("cwr_work_type")?.as_deref())
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    grand_rights_ind: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("grand_rights_ind")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    composite_component_count: {
                        let opt_int: Option<i64> = row.get::<_, Option<i64>>("composite_component_count")?;
                        opt_int
                            .map(|i| {
                                use allegro_cwr::domain_types::CompositeComponentCount;
                                CompositeComponentCount::from_sql_string(&i.to_string())
                            })
                            .transpose()
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    date_of_publication_of_printed_edition: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(
                            row.get::<_, Option<String>>("date_of_publication_of_printed_edition")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    exceptional_clause: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("exceptional_clause")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    opus_number: row.get::<_, Option<String>>("opus_number")?,
                    catalogue_number: row.get::<_, Option<String>>("catalogue_number")?,
                    priority_flag: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("priority_flag")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(nwr)
            }) {
                Ok(nwr) => Ok(Some(allegro_cwr::CwrRegistry::Nwr(nwr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "AGR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_agr WHERE cwr_agr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let agr = allegro_cwr::records::AgrRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    submitter_agreement_number: row.get::<_, String>("submitter_agreement_number")?,
                    international_standard_agreement_code: row
                        .get::<_, Option<String>>("international_standard_agreement_code")?,
                    agreement_type: {
                        use crate::domain_conversions::CwrFromSqlString;

                        use allegro_cwr::domain_types::AgreementType;

                        AgreementType::from_sql_string(&row.get::<_, String>("agreement_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    agreement_start_date: {
                        use allegro_cwr::domain_types::Date;
                        Date::from_sql_string(&row.get::<_, String>("agreement_start_date")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    agreement_end_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(row.get::<_, Option<String>>("agreement_end_date")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    retention_end_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(row.get::<_, Option<String>>("retention_end_date")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    prior_royalty_status: {
                        use allegro_cwr::domain_types::PriorRoyaltyStatus;
                        PriorRoyaltyStatus::from_sql_string(&row.get::<_, String>("prior_royalty_status")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    prior_royalty_start_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(
                            row.get::<_, Option<String>>("prior_royalty_start_date")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    post_term_collection_status: {
                        use allegro_cwr::domain_types::PostTermCollectionStatus;
                        PostTermCollectionStatus::from_sql_string(&row.get::<_, String>("post_term_collection_status")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    post_term_collection_end_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(
                            row.get::<_, Option<String>>("post_term_collection_end_date")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    date_of_signature_of_agreement: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(
                            row.get::<_, Option<String>>("date_of_signature_of_agreement")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    number_of_works: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("number_of_works")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sales_manufacture_clause: {
                        use crate::domain_conversions::opt_string_to_domain;
                        use allegro_cwr::domain_types::SalesManufactureClause;
                        opt_string_to_domain::<SalesManufactureClause>(
                            row.get::<_, Option<String>>("sales_manufacture_clause")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    shares_change: opt_string_to_domain::<Boolean>(
                        row.get::<_, Option<String>>("shares_change")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    advance_given: opt_string_to_domain::<Boolean>(
                        row.get::<_, Option<String>>("advance_given")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    society_assigned_agreement_number: row
                        .get::<_, Option<String>>("society_assigned_agreement_number")?,
                };
                Ok(agr)
            }) {
                Ok(agr) => Ok(Some(allegro_cwr::CwrRegistry::Agr(agr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "ACK" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ack WHERE cwr_ack_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ack = allegro_cwr::records::AckRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    creation_date: {
                        use allegro_cwr::domain_types::Date;
                        Date::from_sql_string(&row.get::<_, String>("creation_date")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    creation_time: {
                        use allegro_cwr::domain_types::Time;
                        Time::from_sql_string(&row.get::<_, String>("creation_time")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    original_group_id: {
                        use allegro_cwr::domain_types::GroupId;
                        GroupId::from_sql_string(&row.get::<_, String>("original_group_id")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    original_transaction_sequence_num: {
                        use allegro_cwr::domain_types::TransactionCount;
                        TransactionCount::from_sql_string(&row.get::<_, String>("original_transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    original_transaction_type: {
                        use allegro_cwr::domain_types::TransactionType;
                        TransactionType::from_sql_string(&row.get::<_, String>("original_transaction_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    creation_title: row.get::<_, Option<String>>("creation_title")?,
                    submitter_creation_num: row.get::<_, Option<String>>("submitter_creation_num")?,
                    recipient_creation_num: row.get::<_, Option<String>>("recipient_creation_num")?,
                    processing_date: {
                        use allegro_cwr::domain_types::Date;
                        Date::from_sql_string(&row.get::<_, String>("processing_date")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    transaction_status: {
                        use crate::domain_conversions::CwrFromSqlString;

                        TransactionStatus::from_sql_string(&row.get::<_, String>("transaction_status")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(ack)
            }) {
                Ok(ack) => Ok(Some(allegro_cwr::CwrRegistry::Ack(ack))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "TER" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ter WHERE cwr_ter_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ter = allegro_cwr::records::TerRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    inclusion_exclusion_indicator: {
                        use allegro_cwr::domain_types::InclusionExclusionIndicator;
                        InclusionExclusionIndicator::from_sql_string(
                            &row.get::<_, String>("inclusion_exclusion_indicator")?,
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    tis_numeric_code: {
                        use allegro_cwr::domain_types::TisNumericCode;
                        TisNumericCode::from_sql_string(&row.get::<_, String>("tis_numeric_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(ter)
            }) {
                Ok(ter) => Ok(Some(allegro_cwr::CwrRegistry::Ter(ter))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "IPA" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ipa WHERE cwr_ipa_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ipa = allegro_cwr::records::IpaRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    agreement_role_code: {
                        use allegro_cwr::domain_types::AgreementRoleCode;
                        AgreementRoleCode::from_sql_string(&row.get::<_, String>("agreement_role_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("interested_party_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, String>("interested_party_num")?,
                    interested_party_last_name: row.get::<_, String>("interested_party_last_name")?,
                    interested_party_writer_first_name: row
                        .get::<_, Option<String>>("interested_party_writer_first_name")?,
                    pr_affiliation_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(
                            row.get::<_, Option<String>>("pr_affiliation_society")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    pr_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(row.get::<_, Option<String>>("pr_share")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_affiliation_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(
                            row.get::<_, Option<String>>("mr_affiliation_society")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(row.get::<_, Option<String>>("mr_share")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_affiliation_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(
                            row.get::<_, Option<String>>("sr_affiliation_society")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(row.get::<_, Option<String>>("sr_share")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(ipa)
            }) {
                Ok(ipa) => Ok(Some(allegro_cwr::CwrRegistry::Ipa(ipa))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NPA" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_npa WHERE cwr_npa_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let npa = allegro_cwr::records::NpaRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, Option<String>>("interested_party_num")?,
                    interested_party_name: row.get::<_, String>("interested_party_name")?,
                    interested_party_writer_first_name: row.get::<_, String>("interested_party_writer_first_name")?,
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(npa)
            }) {
                Ok(npa) => Ok(Some(allegro_cwr::CwrRegistry::Npa(npa))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "SPU" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_spu WHERE cwr_spu_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let spu = allegro_cwr::records::SpuRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    publisher_sequence_num: {
                        use allegro_cwr::domain_types::PublisherSequenceNumber;
                        PublisherSequenceNumber::from_sql_string(&row.get::<_, String>("publisher_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, Option<String>>("interested_party_num")?,
                    publisher_name: row.get::<_, Option<String>>("publisher_name")?,
                    publisher_unknown_indicator: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(
                            row.get::<_, Option<String>>("publisher_unknown_indicator")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    publisher_type: {
                        use allegro_cwr::domain_types::PublisherType;
                        opt_string_to_domain::<PublisherType>(
                            row.get::<_, Option<String>>("publisher_type")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    tax_id_num: row.get::<_, Option<String>>("tax_id_num")?,
                    publisher_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("publisher_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    submitter_agreement_number: row.get::<_, Option<String>>("submitter_agreement_number")?,
                    pr_affiliation_society_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(
                            row.get::<_, Option<String>>("pr_affiliation_society_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    pr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("pr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(row.get::<_, Option<String>>("mr_society")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("mr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(row.get::<_, Option<String>>("sr_society")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("sr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    special_agreements_indicator: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(
                            row.get::<_, Option<String>>("special_agreements_indicator")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    first_recording_refusal_ind: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(
                            row.get::<_, Option<String>>("first_recording_refusal_ind")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    filler: row.get::<_, Option<String>>("filler")?,
                    publisher_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("publisher_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    international_standard_agreement_code: row
                        .get::<_, Option<String>>("international_standard_agreement_code")?,
                    society_assigned_agreement_number: row
                        .get::<_, Option<String>>("society_assigned_agreement_number")?,
                    agreement_type: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::AgreementType;

                        opt_string_to_domain::<AgreementType>(
                            row.get::<_, Option<String>>("agreement_type")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    usa_license_ind: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::UsaLicenseIndicator;

                        opt_string_to_domain::<UsaLicenseIndicator>(
                            row.get::<_, Option<String>>("usa_license_ind")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(spu)
            }) {
                Ok(spu) => Ok(Some(allegro_cwr::CwrRegistry::Spu(spu))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NPN" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_npn WHERE cwr_npn_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let npn = allegro_cwr::records::NpnRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    publisher_sequence_num: {
                        use allegro_cwr::domain_types::PublisherSequenceNumber;
                        PublisherSequenceNumber::from_sql_string(&row.get::<_, String>("publisher_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, String>("interested_party_num")?,
                    publisher_name: row.get::<_, String>("publisher_name")?,
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(npn)
            }) {
                Ok(npn) => Ok(Some(allegro_cwr::CwrRegistry::Npn(npn))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "SPT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_spt WHERE cwr_spt_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let spt = allegro_cwr::records::SptRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, String>("interested_party_num")?,
                    constant: row.get::<_, String>("constant_spaces")?,
                    pr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("pr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("mr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("sr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    inclusion_exclusion_indicator: {
                        use allegro_cwr::domain_types::InclusionExclusionIndicator;
                        InclusionExclusionIndicator::from_sql_string(
                            &row.get::<_, String>("inclusion_exclusion_indicator")?,
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    tis_numeric_code: {
                        use allegro_cwr::domain_types::TisNumericCode;
                        TisNumericCode::from_sql_string(&row.get::<_, String>("tis_numeric_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    shares_change: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("shares_change")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sequence_num: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("sequence_num")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                };
                Ok(spt)
            }) {
                Ok(spt) => Ok(Some(allegro_cwr::CwrRegistry::Spt(spt))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "SWR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_swr WHERE cwr_swr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let swr = allegro_cwr::records::SwrRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, Option<String>>("interested_party_num")?,
                    writer_last_name: row.get::<_, Option<String>>("writer_last_name")?,
                    writer_first_name: row.get::<_, Option<String>>("writer_first_name")?,
                    writer_unknown_indicator: opt_string_to_domain::<Flag>(
                        row.get::<_, Option<String>>("writer_unknown_indicator")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    writer_designation_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::WriterDesignation;

                        opt_string_to_domain::<WriterDesignation>(
                            row.get::<_, Option<String>>("writer_designation_code")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    tax_id_num: row.get::<_, Option<String>>("tax_id_num")?,
                    writer_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    pr_affiliation_society_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(
                            row.get::<_, Option<String>>("pr_affiliation_society_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    pr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("pr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(row.get::<_, Option<String>>("mr_society")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("mr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_society: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<SocietyCode>(row.get::<_, Option<String>>("sr_society")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_ownership_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("sr_ownership_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    reversionary_indicator: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("reversionary_indicator")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    first_recording_refusal_ind: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(
                            row.get::<_, Option<String>>("first_recording_refusal_ind")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    work_for_hire_indicator: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(
                            row.get::<_, Option<String>>("work_for_hire_indicator")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    filler: row.get::<_, Option<String>>("filler")?,
                    writer_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    personal_number: {
                        match row.get::<_, Option<String>>("personal_number")? {
                            Some(personal_number) => {
                                Some(Number::from_sql_string(&personal_number).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                    usa_license_ind: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::UsaLicenseIndicator;

                        opt_string_to_domain::<UsaLicenseIndicator>(
                            row.get::<_, Option<String>>("usa_license_ind")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(swr)
            }) {
                Ok(swr) => Ok(Some(allegro_cwr::CwrRegistry::Swr(swr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NWN" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_nwn WHERE cwr_nwn_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let nwn = allegro_cwr::records::NwnRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, Option<String>>("interested_party_num")?,
                    writer_last_name: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::NonRomanAlphabet;
                        NonRomanAlphabet::from_sql_string(&row.get::<_, String>("writer_last_name")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_first_name: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::NonRomanAlphabet;

                        opt_string_to_domain::<NonRomanAlphabet>(
                            row.get::<_, Option<String>>("writer_first_name")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(nwn)
            }) {
                Ok(nwn) => Ok(Some(allegro_cwr::CwrRegistry::Nwn(nwn))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "SWT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_swt WHERE cwr_swt_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let swt = allegro_cwr::records::SwtRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    interested_party_num: row.get::<_, Option<String>>("interested_party_num")?,
                    pr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("pr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    mr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("mr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sr_collection_share: {
                        use allegro_cwr::domain_types::OwnershipShare;
                        opt_string_to_numeric::<OwnershipShare>(
                            row.get::<_, Option<String>>("sr_collection_share")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    inclusion_exclusion_indicator: {
                        use allegro_cwr::domain_types::InclusionExclusionIndicator;
                        InclusionExclusionIndicator::from_sql_string(
                            &row.get::<_, String>("inclusion_exclusion_indicator")?,
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    tis_numeric_code: {
                        use allegro_cwr::domain_types::TisNumericCode;
                        TisNumericCode::from_sql_string(&row.get::<_, String>("tis_numeric_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    shares_change: {
                        use allegro_cwr::domain_types::Flag;
                        opt_string_to_domain::<Flag>(row.get::<_, Option<String>>("shares_change")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    sequence_num: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("sequence_num")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                };
                Ok(swt)
            }) {
                Ok(swt) => Ok(Some(allegro_cwr::CwrRegistry::Swt(swt))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "PWR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_pwr WHERE cwr_pwr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let pwr = allegro_cwr::records::PwrRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    publisher_ip_num: row.get::<_, Option<String>>("publisher_ip_num")?,
                    publisher_name: row.get::<_, Option<String>>("publisher_name")?,
                    submitter_agreement_number: row.get::<_, Option<String>>("submitter_agreement_number")?,
                    society_assigned_agreement_number: row
                        .get::<_, Option<String>>("society_assigned_agreement_number")?,
                    writer_ip_num: row.get::<_, Option<String>>("writer_ip_num")?,
                    publisher_sequence_num: {
                        use allegro_cwr::domain_types::PublisherSequenceNumber;
                        opt_string_to_numeric::<PublisherSequenceNumber>(
                            row.get::<_, Option<String>>("publisher_sequence_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(pwr)
            }) {
                Ok(pwr) => Ok(Some(allegro_cwr::CwrRegistry::Pwr(pwr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "ALT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_alt WHERE cwr_alt_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let alt = allegro_cwr::records::AltRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    alternate_title: row.get::<_, String>("alternate_title")?,
                    title_type: {
                        use allegro_cwr::domain_types::TitleType;
                        TitleType::from_sql_string(&row.get::<_, String>("title_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(alt)
            }) {
                Ok(alt) => Ok(Some(allegro_cwr::CwrRegistry::Alt(alt))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NAT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_nat WHERE cwr_nat_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let nat = allegro_cwr::records::NatRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    title: row.get::<_, String>("title")?,
                    title_type: {
                        use allegro_cwr::domain_types::TitleType;
                        TitleType::from_sql_string(&row.get::<_, String>("title_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(nat)
            }) {
                Ok(nat) => Ok(Some(allegro_cwr::CwrRegistry::Nat(nat))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "EWT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ewt WHERE cwr_ewt_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ewt = allegro_cwr::records::EwtRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    entire_work_title: row.get::<_, String>("entire_work_title")?,
                    iswc_of_entire_work: row.get::<_, Option<String>>("iswc_of_entire_work")?,
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_1_last_name: row.get::<_, Option<String>>("writer_1_last_name")?,
                    writer_1_first_name: row.get::<_, Option<String>>("writer_1_first_name")?,
                    source: row.get::<_, Option<String>>("source")?,
                    writer_1_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_1_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_1_ipi_base_number: row.get::<_, Option<String>>("writer_1_ipi_base_number")?,
                    writer_2_last_name: row.get::<_, Option<String>>("writer_2_last_name")?,
                    writer_2_first_name: row.get::<_, Option<String>>("writer_2_first_name")?,
                    writer_2_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_2_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    submitter_work_num: row.get::<_, Option<String>>("submitter_work_num")?,
                };
                Ok(ewt)
            }) {
                Ok(ewt) => Ok(Some(allegro_cwr::CwrRegistry::Ewt(ewt))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "VER" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ver WHERE cwr_ver_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ver = allegro_cwr::records::VerRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    original_work_title: row.get::<_, String>("original_work_title")?,
                    iswc_of_original_work: row.get::<_, Option<String>>("iswc_of_original_work")?,
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_1_last_name: row.get::<_, Option<String>>("writer_1_last_name")?,
                    writer_1_first_name: row.get::<_, Option<String>>("writer_1_first_name")?,
                    source: row.get::<_, Option<String>>("source")?,
                    writer_1_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_1_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_1_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;
                        use allegro_cwr::domain_types::IpiBaseNumber;
                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_1_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_2_last_name: row.get::<_, Option<String>>("writer_2_last_name")?,
                    writer_2_first_name: row.get::<_, Option<String>>("writer_2_first_name")?,
                    writer_2_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_2_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    submitter_work_num: row.get::<_, Option<String>>("submitter_work_num")?,
                };
                Ok(ver)
            }) {
                Ok(ver) => Ok(Some(allegro_cwr::CwrRegistry::Ver(ver))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "PER" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_per WHERE cwr_per_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let per = allegro_cwr::records::PerRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performing_artist_last_name: row.get::<_, String>("performing_artist_last_name")?,
                    performing_artist_first_name: row.get::<_, Option<String>>("performing_artist_first_name")?,
                    performing_artist_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("performing_artist_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performing_artist_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("performing_artist_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(per)
            }) {
                Ok(per) => Ok(Some(allegro_cwr::CwrRegistry::Per(per))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "NPR" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_npr WHERE cwr_npr_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let npr = allegro_cwr::records::NprRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performing_artist_name: row.get::<_, Option<String>>("performing_artist_name")?,
                    performing_artist_first_name: row.get::<_, Option<String>>("performing_artist_first_name")?,
                    performing_artist_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("performing_artist_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performing_artist_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("performing_artist_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performance_language: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::LanguageCode;

                        opt_string_to_domain::<LanguageCode>(
                            row.get::<_, Option<String>>("performance_language")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    performance_dialect: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::LookupPlaceholder;

                        opt_string_to_domain::<LookupPlaceholder>(
                            row.get::<_, Option<String>>("performance_dialect")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(npr)
            }) {
                Ok(npr) => Ok(Some(allegro_cwr::CwrRegistry::Npr(npr))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "REC" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_rec WHERE cwr_rec_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let rec = allegro_cwr::records::RecRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    release_date: {
                        use allegro_cwr::domain_types::Date;
                        opt_string_to_domain::<Date>(row.get::<_, Option<String>>("release_date")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    constant: row.get::<_, String>("constant_blanks_1")?,
                    release_duration: {
                        use allegro_cwr::domain_types::Time;
                        opt_string_to_domain::<Time>(row.get::<_, Option<String>>("release_duration")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    constant2: row.get::<_, String>("constant_blanks_2")?,
                    album_title: row.get::<_, Option<String>>("album_title")?,
                    album_label: row.get::<_, Option<String>>("album_label")?,
                    release_catalog_num: row.get::<_, Option<String>>("release_catalog_num")?,
                    ean: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::Ean;

                        opt_string_to_domain::<Ean>(row.get::<_, Option<String>>("ean")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    isrc: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::Isrc;

                        opt_string_to_domain::<Isrc>(row.get::<_, Option<String>>("isrc")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    recording_format: {
                        use allegro_cwr::domain_types::RecordingFormat;
                        opt_string_to_domain::<RecordingFormat>(
                            row.get::<_, Option<String>>("recording_format")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    recording_technique: {
                        use allegro_cwr::domain_types::RecordingTechnique;
                        opt_string_to_domain::<RecordingTechnique>(
                            row.get::<_, Option<String>>("recording_technique")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    media_type: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::MediaType;

                        opt_string_to_domain::<MediaType>(row.get::<_, Option<String>>("media_type")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    recording_title: row.get::<_, Option<String>>("recording_title")?,
                    version_title: row.get::<_, Option<String>>("version_title")?,
                    display_artist: row.get::<_, Option<String>>("display_artist")?,
                    record_label: row.get::<_, Option<String>>("record_label")?,
                    isrc_validity: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IsrcValidityIndicator;

                        opt_string_to_domain::<IsrcValidityIndicator>(
                            row.get::<_, Option<String>>("isrc_validity")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    submitter_recording_identifier: row.get::<_, Option<String>>("submitter_recording_identifier")?,
                };
                Ok(rec)
            }) {
                Ok(rec) => Ok(Some(allegro_cwr::CwrRegistry::Rec(rec))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "ORN" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_orn WHERE cwr_orn_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let orn = allegro_cwr::records::OrnRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    intended_purpose: {
                        use crate::domain_conversions::CwrFromSqlString;

                        use allegro_cwr::domain_types::IntendedPurpose;

                        IntendedPurpose::from_sql_string(&row.get::<_, String>("intended_purpose")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    production_title: row.get::<_, Option<String>>("production_title")?,
                    cd_identifier: row.get::<_, Option<String>>("cd_identifier")?,
                    cut_number: opt_string_to_numeric::<Number>(row.get::<_, Option<String>>("cut_number")?.as_deref())
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    library: row.get::<_, Option<String>>("library")?,
                    bltvr: row.get::<_, Option<String>>("bltvr")?,
                    filler: {
                        match row.get::<_, Option<String>>("filler")? {
                            Some(filler) => {
                                Some(Number::from_sql_string(&filler).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            None => None,
                        }
                    },
                    production_num: row.get::<_, Option<String>>("production_num")?,
                    episode_title: row.get::<_, Option<String>>("episode_title")?,
                    episode_num: row.get::<_, Option<String>>("episode_num")?,
                    year_of_production: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("year_of_production")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    avi_society_code: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("avi_society_code")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    audio_visual_number: row.get::<_, Option<String>>("audio_visual_number")?,
                    v_isan_isan: row.get::<_, Option<String>>("v_isan_isan")?,
                    v_isan_episode: row.get::<_, Option<String>>("v_isan_episode")?,
                    v_isan_check_digit_1: row.get::<_, Option<String>>("v_isan_check_digit_1")?,
                    v_isan_version: row.get::<_, Option<String>>("v_isan_version")?,
                    v_isan_check_digit_2: row.get::<_, Option<String>>("v_isan_check_digit_2")?,
                    eidr: row.get::<_, Option<String>>("eidr")?,
                    eidr_check_digit: row.get::<_, Option<String>>("eidr_check_digit")?,
                };
                Ok(orn)
            }) {
                Ok(orn) => Ok(Some(allegro_cwr::CwrRegistry::Orn(orn))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }
        "INS" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ins WHERE cwr_ins_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ins = allegro_cwr::records::InsRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    number_of_voices: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("number_of_voices")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                    standard_instrumentation_type: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::StandardInstrumentationType;

                        opt_string_to_domain::<StandardInstrumentationType>(
                            row.get::<_, Option<String>>("standard_instrumentation_type")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    instrumentation_description: row.get::<_, Option<String>>("instrumentation_description")?,
                };
                Ok(ins)
            }) {
                Ok(ins) => Ok(Some(allegro_cwr::CwrRegistry::Ins(ins))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "IND" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ind WHERE cwr_ind_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ind = allegro_cwr::records::IndRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    instrument_code: {
                        use crate::domain_conversions::CwrFromSqlString;

                        use allegro_cwr::domain_types::InstrumentCode;

                        InstrumentCode::from_sql_string(&row.get::<_, String>("instrument_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    number_of_players: opt_string_to_numeric::<Number>(
                        row.get::<_, Option<String>>("number_of_players")?.as_deref(),
                    )
                    .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?,
                };
                Ok(ind)
            }) {
                Ok(ind) => Ok(Some(allegro_cwr::CwrRegistry::Ind(ind))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "COM" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_com WHERE cwr_com_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let com = allegro_cwr::records::ComRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    title: row.get::<_, String>("title")?,
                    iswc_of_component: row.get::<_, Option<String>>("iswc_of_component")?,
                    submitter_work_num: row.get::<_, Option<String>>("submitter_work_num")?,
                    duration: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::Time;
                        match row.get::<_, Option<String>>("duration")? {
                            Some(duration_str) if !duration_str.trim().is_empty() => {
                                Some(Time::from_sql_string(&duration_str).map_err(|e| {
                                    rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text)
                                })?)
                            }
                            _ => None,
                        }
                    },
                    writer_1_last_name: row.get::<_, String>("writer_1_last_name")?,
                    writer_1_first_name: row.get::<_, Option<String>>("writer_1_first_name")?,
                    writer_1_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_1_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_2_last_name: row.get::<_, Option<String>>("writer_2_last_name")?,
                    writer_2_first_name: row.get::<_, Option<String>>("writer_2_first_name")?,
                    writer_2_ipi_name_num: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiNameNumber;

                        opt_string_to_domain::<IpiNameNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_name_num")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_1_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;
                        use allegro_cwr::domain_types::IpiBaseNumber;
                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_1_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_2_ipi_base_number: {
                        use crate::domain_conversions::opt_string_to_domain;

                        use allegro_cwr::domain_types::IpiBaseNumber;

                        opt_string_to_domain::<IpiBaseNumber>(
                            row.get::<_, Option<String>>("writer_2_ipi_base_number")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(com)
            }) {
                Ok(com) => Ok(Some(allegro_cwr::CwrRegistry::Com(com))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "MSG" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_msg WHERE cwr_msg_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let msg = allegro_cwr::records::MsgRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    message_type: {
                        use crate::domain_conversions::CwrFromSqlString;

                        use allegro_cwr::domain_types::MessageType;

                        MessageType::from_sql_string(&row.get::<_, String>("message_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    original_record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("original_record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_type_field: row.get::<_, String>("record_type_field")?,
                    message_level: {
                        use crate::domain_conversions::CwrFromSqlString;

                        use allegro_cwr::domain_types::MessageLevel;

                        MessageLevel::from_sql_string(&row.get::<_, String>("message_level")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    validation_number: row.get::<_, String>("validation_number")?,
                    message_text: row.get::<_, String>("message_text")?,
                };
                Ok(msg)
            }) {
                Ok(msg) => Ok(Some(allegro_cwr::CwrRegistry::Msg(msg))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "NET" | "NCT" | "NVT" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_net WHERE cwr_net_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let net = allegro_cwr::records::NetRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    title: row.get::<_, String>("title")?,
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(net)
            }) {
                Ok(net) => Ok(Some(allegro_cwr::CwrRegistry::Net(net))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "NOW" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_now WHERE cwr_now_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let now = allegro_cwr::records::NowRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_name: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::NonRomanAlphabet;
                        NonRomanAlphabet::from_sql_string(&row.get::<_, String>("writer_name")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_first_name: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::NonRomanAlphabet;
                        NonRomanAlphabet::from_sql_string(&row.get::<_, String>("writer_first_name")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    language_code: {
                        use crate::domain_conversions::opt_string_to_domain;

                        opt_string_to_domain::<LanguageCode>(row.get::<_, Option<String>>("language_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    writer_position: {
                        use crate::domain_conversions::opt_string_to_domain;
                        use allegro_cwr::domain_types::WriterPosition;
                        opt_string_to_domain::<WriterPosition>(
                            row.get::<_, Option<String>>("writer_position")?.as_deref(),
                        )
                        .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(now)
            }) {
                Ok(now) => Ok(Some(allegro_cwr::CwrRegistry::Now(now))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "ARI" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_ari WHERE cwr_ari_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let ari = allegro_cwr::records::AriRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    society_num: allegro_cwr::domain_types::SocietyCode(row.get::<_, String>("society_num")?),
                    work_num: row.get::<_, Option<String>>("work_num")?,
                    type_of_right: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::TypeOfRight;
                        TypeOfRight::from_sql_string(&row.get::<_, String>("type_of_right")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    subject_code: {
                        use crate::domain_conversions::opt_string_to_domain;
                        use allegro_cwr::domain_types::SubjectCode;
                        opt_string_to_domain::<SubjectCode>(row.get::<_, Option<String>>("subject_code")?.as_deref())
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    note: row.get::<_, Option<String>>("note")?,
                };
                Ok(ari)
            }) {
                Ok(ari) => Ok(Some(allegro_cwr::CwrRegistry::Ari(ari))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        "XRF" => {
            let mut stmt = conn.prepare("SELECT * FROM cwr_xrf WHERE cwr_xrf_id = ?1")?;
            match stmt.query_row(params![record_id], |row| {
                let xrf = allegro_cwr::records::XrfRecord {
                    record_type: row.get::<_, String>("record_type")?,
                    transaction_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("transaction_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    record_sequence_num: {
                        use allegro_cwr::domain_types::Number;
                        Number::from_sql_string(&row.get::<_, String>("record_sequence_num")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    organisation_code: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::SocietyCode;
                        SocietyCode::from_sql_string(&row.get::<_, String>("organisation_code")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    identifier: row.get::<_, String>("identifier")?,
                    identifier_type: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::IdentifierType;
                        IdentifierType::from_sql_string(&row.get::<_, String>("identifier_type")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                    validity: {
                        use crate::domain_conversions::CwrFromSqlString;
                        use allegro_cwr::domain_types::Flag;
                        Flag::from_sql_string(&row.get::<_, String>("validity")?)
                            .map_err(|e| rusqlite::Error::InvalidColumnType(0, e, rusqlite::types::Type::Text))?
                    },
                };
                Ok(xrf)
            }) {
                Ok(xrf) => Ok(Some(allegro_cwr::CwrRegistry::Xrf(xrf))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(error::CwrDbError::Sqlite(e)),
            }
        }

        // Skip unimplemented record types for now
        _ => Ok(None),
    }
}

/// Helper function to parse domain types from database strings using our CwrFromSqlString trait
/// This leverages the existing CWR parsing logic to reconstruct domain types from stored values
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
        writeln!(file, "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221")
            .unwrap();
        writeln!(file, "GRHAGR0000102.10            ").unwrap();
        writeln!(file, "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ").unwrap();

        // Add a few more key record types to demonstrate the scope
        writeln!(file, "SPU0000000100000002000000011357924680SAMPLE PUBLISHER                    N  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "SWR0000000100000003000000013579SAMPLE WRITER              JOHN            A  01.1012345678901357924680123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890                    0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        writeln!(file, "ALT0000000100000004ALTERNATE TITLE                         AT EN           ").unwrap();
        writeln!(file, "PER0000000100000005SAMPLE PERFORMER            1357924680123456789012345678").unwrap();
        writeln!(file, "REC0000000100000006         0000000SAMPLE ALBUM                      SAMPLE LABEL         1234567890123EAN1234567890ISRCCD   SAMPLE RECORDING             SAMPLE VERSION               SAMPLE ARTIST                SAMPLE RECORD LABEL          Y12345678901234567890").unwrap();

        // Add many unhandled record types to show the scope of missing functionality
        writeln!(
            file,
            "ACK0000000100000007202212211254110000001TRK                                                 20221221A"
        )
        .unwrap();
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
        writeln!(
            file,
            "NPA000000010000001901123456789012345678901234567890INTERESTED PARTY NAME           FIRST           EN  "
        )
        .unwrap();
        writeln!(
            file,
            "NPN000000010000002001001123456789012345678901234567890PUBLISHER NAME                  EN              "
        )
        .unwrap();
        writeln!(
            file,
            "NPR000000010000002101PERFORMING ARTIST       FIRST           1234567890123456123456789012345EN  EN  EN  "
        )
        .unwrap();
        writeln!(file, "NWN000000010000002201123456789012345678901234567890WRITER NAME             FIRST           EN              ").unwrap();
        writeln!(
            file,
            "ORN000000010000002301LSAMPLE PRODUCTION                                                                                                                                                                                    2022123456789012345678901234567890123456789012345612345678901234561234567890123456"
        )
        .unwrap();
        writeln!(file, "PWR000000010000002401123456789012345678901234567890PUBLISHER NAME                                            123456789012345678901234567890001").unwrap();
        writeln!(file, "SPT000000010000002501123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "SWT000000010000002601123456789012345678901234567890                                                        I000000000        001").unwrap();
        writeln!(file, "TER000000010000002701I000000000").unwrap();
        writeln!(file, "VER000000010000002801VERSION WORK TITLE                                   12345670000000EN VERSION WRITER       JOHN            SRC 1234567890123456123456789012345VERSION WRITER 2     JANE            1234567890123456123456789012345612345678901234567890").unwrap();
        writeln!(file, "XRF000000010000002901123456789012345ABCD1234567890123456789012345678901234567890YV ").unwrap();

        writeln!(file, "GRT000010000000010000027").unwrap();
        writeln!(file, "TRL00001000000010000027").unwrap();

        // Process the file
        let (file_id, processed_count, _report) =
            process_cwr_to_sqlite(cwr_file_path.to_str().unwrap(), db_file_path.to_str().unwrap()).unwrap();

        // Verify processing happened
        assert_eq!(processed_count, 33, "Should have processed 33 records");

        // Connect to database and verify records were actually inserted
        let conn = rusqlite::Connection::open(&db_file_path).unwrap();

        // Check file_line table - should have entries for each record type
        let mut stmt = conn.prepare("SELECT record_type, COUNT(*) FROM file_line WHERE file_id = ?1 GROUP BY record_type ORDER BY record_type").unwrap();
        let rows: std::result::Result<Vec<(String, i64)>, rusqlite::Error> =
            stmt.query_map([file_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))).unwrap().collect();

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
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'cwr_%' ORDER BY name")
            .unwrap();
        let table_names: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();

        println!("Found {} cwr_ tables: {:?}", table_names.len(), table_names);

        // Check count in each cwr_ table
        let mut total_records_in_tables = 0i64;
        let mut implemented_tables = Vec::new();
        let mut unimplemented_tables = Vec::new();

        for table_name in &table_names {
            let count: i64 = conn
                .query_row(&format!("SELECT COUNT(*) FROM {} WHERE file_id = ?1", table_name), [file_id], |row| {
                    row.get(0)
                })
                .unwrap();

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
        let (sender_name, creation_date): (String, String) = conn
            .query_row("SELECT sender_name, creation_date FROM cwr_hdr WHERE file_id = ?1", [file_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap();

        assert_eq!(sender_name, "WARNER CHAPPELL MUSIC PUBLISHING LTD", "HDR should contain actual parsed sender name");
        assert_eq!(creation_date, "20221221", "HDR should contain parsed creation date");

        println!("🚨 This test demonstrates the missing functionality!");
        println!("📊 Records tracked in file_line: {} types", record_counts.len());
        println!("✅ Implemented tables: {:?}", implemented_tables);
        println!("❌ Unimplemented tables: {:?}", unimplemented_tables);
        println!("🎯 Goal: All 33 record types should sum to 33 total records in cwr_ tables");
    }

    #[test]
    fn test_sqlite_to_cwr_write_pattern() {
        // Test the bidirectional SQLite-to-CWR conversion pattern (similar to JSON-to-CWR)
        let temp_dir = tempfile::tempdir().unwrap();
        let cwr_file_path = temp_dir.path().join("test.cwr");
        let db_file_path = temp_dir.path().join("test.db");
        let output_file_path = temp_dir.path().join("output.cwr");

        let mut file = File::create(&cwr_file_path).unwrap();

        // Create test CWR data with key record types
        writeln!(file, "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221")
            .unwrap();
        writeln!(file, "GRHAGR0000102.10            ").unwrap();
        writeln!(file, "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ").unwrap();
        writeln!(file, "GRT000010000001400000365             ").unwrap();
        writeln!(file, "TRL00001000000010000027").unwrap();

        // First: Process CWR file to SQLite (demonstrates write pattern from CWR)
        let (file_id, processed_count, _report) =
            process_cwr_to_sqlite(cwr_file_path.to_str().unwrap(), db_file_path.to_str().unwrap()).unwrap();
        assert_eq!(processed_count, 5, "Should have processed 5 records");

        // Second: Process SQLite to CWR (demonstrates read pattern back to CWR)
        let output_count = process_sqlite_to_cwr_with_version_and_output(
            db_file_path.to_str().unwrap(),
            file_id,
            Some(2.2),
            Some(output_file_path.to_str().unwrap()),
        )
        .unwrap();

        // We implemented HDR, GRH, GRT, TRL, and NWR reconstruction
        assert_eq!(output_count, 5, "Should have output 5 CWR lines");

        // Verify the output file was created and contains CWR-like data
        let output_content = std::fs::read_to_string(&output_file_path).unwrap();
        let lines: Vec<&str> = output_content.trim().split('\n').collect();

        assert_eq!(lines.len(), 5, "Output should have 5 lines");

        // Verify each line starts with the expected record type
        assert!(lines[0].starts_with("HDR"), "First line should be HDR record");
        assert!(lines[1].starts_with("GRH"), "Second line should be GRH record");
        assert!(lines[2].starts_with("NWR"), "Third line should be NWR record");
        assert!(lines[3].starts_with("GRT"), "Fourth line should be GRT record");
        assert!(lines[4].starts_with("TRL"), "Fifth line should be TRL record");

        // Show what was actually reconstructed
        println!("📄 Reconstructed CWR content:");
        for (i, line) in lines.iter().enumerate() {
            println!("  {}: {}", i + 1, line);
        }

        // ✅ Implemented: HDR, GRH, NWR, GRT, TRL
        // TODO: Add implementations for remaining 28 record types:
        // - AGR, ACK, TER, IPA, NPA, SPU, NPN, SPT, SWR, NWN, SWT, PWR
        // - ALT, NAT, EWT, VER, PER, NPR, REC, ORN, INS, IND, COM, MSG
        // - NET, NOW, ARI, XRF

        println!("✅ Successfully demonstrated bidirectional SQLite ↔ CWR conversion pattern!");
        println!("📝 Original CWR → SQLite → CWR conversion completed");
        println!("🔄 This follows the same streaming pattern as JSON ↔ CWR conversion");
    }
}
