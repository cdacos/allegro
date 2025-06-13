use crate::error::CwrDbError;
use rusqlite::{Statement, Transaction};

/// Structure to hold all prepared statements for CWR record insertion
pub struct PreparedStatements<'conn> {
    pub error_stmt: Statement<'conn>,
    pub file_insert_stmt: Statement<'conn>,
    pub file_stmt: Statement<'conn>,
    pub hdr_stmt: Statement<'conn>,
    pub grh_stmt: Statement<'conn>,
    pub grt_stmt: Statement<'conn>,
    pub trl_stmt: Statement<'conn>,
    pub agr_stmt: Statement<'conn>,
    pub nwr_stmt: Statement<'conn>, // Covers NWR, REV, ISW, EXC
    pub ack_stmt: Statement<'conn>,
    pub ter_stmt: Statement<'conn>,
    pub ipa_stmt: Statement<'conn>,
    pub npa_stmt: Statement<'conn>,
    pub spu_stmt: Statement<'conn>, // Covers SPU, OPU
    pub npn_stmt: Statement<'conn>,
    pub spt_stmt: Statement<'conn>, // Covers SPT, OPT
    pub swr_stmt: Statement<'conn>, // Covers SWR, OWR
    pub nwn_stmt: Statement<'conn>,
    pub swt_stmt: Statement<'conn>, // Covers SWT, OWT
    pub pwr_stmt: Statement<'conn>,
    pub alt_stmt: Statement<'conn>,
    pub nat_stmt: Statement<'conn>,
    pub ewt_stmt: Statement<'conn>,
    pub ver_stmt: Statement<'conn>,
    pub per_stmt: Statement<'conn>,
    pub npr_stmt: Statement<'conn>,
    pub rec_stmt: Statement<'conn>,
    pub orn_stmt: Statement<'conn>,
    pub ins_stmt: Statement<'conn>,
    pub ind_stmt: Statement<'conn>,
    pub com_stmt: Statement<'conn>,
    pub msg_stmt: Statement<'conn>,
    pub net_stmt: Statement<'conn>, // Covers NET, NCT, NVT
    pub now_stmt: Statement<'conn>,
    pub ari_stmt: Statement<'conn>,
    pub xrf_stmt: Statement<'conn>,
}

/// Creates all prepared statements for CWR record insertion
pub fn get_prepared_statements<'a>(tx: &'a Transaction) -> Result<PreparedStatements<'a>, CwrDbError> {
    Ok(PreparedStatements {
        error_stmt: tx.prepare("INSERT INTO error (file_id, line_number, description) VALUES (?1, ?2, ?3)")?,
        file_insert_stmt: tx.prepare("INSERT INTO file (file_path, imported_on) VALUES (?1, DATETIME('now'))")?,
        file_stmt: tx.prepare("INSERT INTO file_line (file_id, line_number, record_type, record_id) VALUES (?1, ?2, ?3, ?4)")?,
        hdr_stmt: tx.prepare("INSERT INTO cwr_hdr (file_id, record_type, sender_type, sender_id, sender_name, edi_standard_version_number, creation_date, creation_time, transmission_date, character_set, version, revision, software_package, software_package_version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)")?,
        grh_stmt: tx.prepare("INSERT INTO cwr_grh (file_id, record_type, transaction_type, group_id, version_number_for_this_transaction_type, batch_request, submission_distribution_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        grt_stmt: tx.prepare("INSERT INTO cwr_grt (file_id, record_type, group_id, transaction_count, record_count, currency_indicator, total_monetary_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        trl_stmt: tx.prepare("INSERT INTO cwr_trl (file_id, record_type, group_count, transaction_count, record_count) VALUES (?1, ?2, ?3, ?4, ?5)")?,
        agr_stmt: tx.prepare("INSERT INTO cwr_agr (file_id, record_type, transaction_sequence_num, record_sequence_num, submitter_agreement_number, international_standard_agreement_code, agreement_type, agreement_start_date, agreement_end_date, retention_end_date, prior_royalty_status, prior_royalty_start_date, post_term_collection_status, post_term_collection_end_date, date_of_signature_of_agreement, number_of_works, sales_manufacture_clause, shares_change, advance_given, society_assigned_agreement_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)")?,
        nwr_stmt: tx.prepare("INSERT INTO cwr_nwr (file_id, record_type, transaction_sequence_num, record_sequence_num, work_title, language_code, submitter_work_num, iswc, copyright_date, copyright_number, musical_work_distribution_category, duration, recorded_indicator, text_music_relationship, composite_type, version_type, excerpt_type, music_arrangement, lyric_adaptation, contact_name, contact_id, cwr_work_type, grand_rights_ind, composite_component_count, date_of_publication_of_printed_edition, exceptional_clause, opus_number, catalogue_number, priority_flag) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29)")?,
        ack_stmt: tx.prepare("INSERT INTO cwr_ack (file_id, record_type, transaction_sequence_num, record_sequence_num, creation_date, creation_time, original_group_id, original_transaction_sequence_num, original_transaction_type, creation_title, submitter_creation_num, recipient_creation_num, processing_date, transaction_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)")?,
        ter_stmt: tx.prepare("INSERT INTO cwr_ter (file_id, record_type, transaction_sequence_num, record_sequence_num, inclusion_exclusion_indicator, tis_numeric_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
        ipa_stmt: tx.prepare("INSERT INTO cwr_ipa (file_id, record_type, transaction_sequence_num, record_sequence_num, agreement_role_code, interested_party_ipi_name_num, ipi_base_number, interested_party_num, interested_party_last_name, interested_party_writer_first_name, pr_affiliation_society, pr_share, mr_affiliation_society, mr_share, sr_affiliation_society, sr_share) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)")?,
        npa_stmt: tx.prepare("INSERT INTO cwr_npa (file_id, record_type, transaction_sequence_num, record_sequence_num, interested_party_num, interested_party_name, interested_party_writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
        spu_stmt: tx.prepare("INSERT INTO cwr_spu (file_id, record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, publisher_unknown_indicator, publisher_type, tax_id_num, publisher_ipi_name_num, submitter_agreement_number, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, special_agreements_indicator, first_recording_refusal_ind, filler, publisher_ipi_base_number, international_standard_agreement_code, society_assigned_agreement_number, agreement_type, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)")?,
        npn_stmt: tx.prepare("INSERT INTO cwr_npn (file_id, record_type, transaction_sequence_num, record_sequence_num, publisher_sequence_num, interested_party_num, publisher_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
        spt_stmt: tx.prepare("INSERT INTO cwr_spt (file_id, record_type, transaction_sequence_num, record_sequence_num, interested_party_num, constant_spaces, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)")?,
        swr_stmt: tx.prepare("INSERT INTO cwr_swr (file_id, record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, writer_unknown_indicator, writer_designation_code, tax_id_num, writer_ipi_name_num, pr_affiliation_society_num, pr_ownership_share, mr_society, mr_ownership_share, sr_society, sr_ownership_share, reversionary_indicator, first_recording_refusal_ind, work_for_hire_indicator, filler, writer_ipi_base_number, personal_number, usa_license_ind) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)")?,
        nwn_stmt: tx.prepare("INSERT INTO cwr_nwn (file_id, record_type, transaction_sequence_num, record_sequence_num, interested_party_num, writer_last_name, writer_first_name, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
        swt_stmt: tx.prepare("INSERT INTO cwr_swt (file_id, record_type, transaction_sequence_num, record_sequence_num, interested_party_num, pr_collection_share, mr_collection_share, sr_collection_share, inclusion_exclusion_indicator, tis_numeric_code, shares_change, sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)")?,
        pwr_stmt: tx.prepare("INSERT INTO cwr_pwr (file_id, record_type, transaction_sequence_num, record_sequence_num, publisher_ip_num, publisher_name, submitter_agreement_number, society_assigned_agreement_number, writer_ip_num, publisher_sequence_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?,
        alt_stmt: tx.prepare("INSERT INTO cwr_alt (file_id, record_type, transaction_sequence_num, record_sequence_num, alternate_title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        nat_stmt: tx.prepare("INSERT INTO cwr_nat (file_id, record_type, transaction_sequence_num, record_sequence_num, title, title_type, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        ewt_stmt: tx.prepare("INSERT INTO cwr_ewt (file_id, record_type, transaction_sequence_num, record_sequence_num, entire_work_title, iswc_of_entire_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)")?,
        ver_stmt: tx.prepare("INSERT INTO cwr_ver (file_id, record_type, transaction_sequence_num, record_sequence_num, original_work_title, iswc_of_original_work, language_code, writer_1_last_name, writer_1_first_name, source, writer_1_ipi_name_num, writer_1_ipi_base_number, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_2_ipi_base_number, submitter_work_num) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)")?,
        per_stmt: tx.prepare("INSERT INTO cwr_per (file_id, record_type, transaction_sequence_num, record_sequence_num, performing_artist_last_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
        npr_stmt: tx.prepare("INSERT INTO cwr_npr (file_id, record_type, transaction_sequence_num, record_sequence_num, performing_artist_name, performing_artist_first_name, performing_artist_ipi_name_num, performing_artist_ipi_base_number, language_code, performance_language, performance_dialect) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)")?,
        rec_stmt: tx.prepare("INSERT INTO cwr_rec (file_id, record_type, transaction_sequence_num, record_sequence_num, release_date, constant_blanks_1, release_duration, constant_blanks_2, album_title, album_label, release_catalog_num, ean, isrc, recording_format, recording_technique, media_type, recording_title, version_title, display_artist, record_label, isrc_validity, submitter_recording_identifier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)")?,
        orn_stmt: tx.prepare("INSERT INTO cwr_orn (file_id, record_type, transaction_sequence_num, record_sequence_num, intended_purpose, production_title, cd_identifier, cut_number, library, bltvr, filler_reserved, production_num, episode_title, episode_num, year_of_production, avi_society_code, audio_visual_number, v_isan_isan, v_isan_episode, v_isan_check_digit_1, v_isan_version, v_isan_check_digit_2, eidr, eidr_check_digit) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)")?,
        ins_stmt: tx.prepare("INSERT INTO cwr_ins (file_id, record_type, transaction_sequence_num, record_sequence_num, number_of_voices, standard_instrumentation_type, instrumentation_description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?,
        ind_stmt: tx.prepare("INSERT INTO cwr_ind (file_id, record_type, transaction_sequence_num, record_sequence_num, instrument_code, number_of_players) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
        com_stmt: tx.prepare("INSERT INTO cwr_com (file_id, record_type, transaction_sequence_num, record_sequence_num, title, iswc_of_component, submitter_work_num, duration, writer_1_last_name, writer_1_first_name, writer_1_ipi_name_num, writer_2_last_name, writer_2_first_name, writer_2_ipi_name_num, writer_1_ipi_base_number, writer_2_ipi_base_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)")?,
        msg_stmt: tx.prepare("INSERT INTO cwr_msg (file_id, record_type, transaction_sequence_num, record_sequence_num, message_type, original_record_sequence_num, msg_record_type, message_level, validation_number, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")?,
        net_stmt: tx.prepare("INSERT INTO cwr_net (file_id, record_type, transaction_sequence_num, record_sequence_num, title, language_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?,
        now_stmt: tx.prepare("INSERT INTO cwr_now (file_id, record_type, transaction_sequence_num, record_sequence_num, writer_name, writer_first_name, language_code, writer_position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
        ari_stmt: tx.prepare("INSERT INTO cwr_ari (file_id, record_type, transaction_sequence_num, record_sequence_num, society_num, work_num, type_of_right, subject_code, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?,
        xrf_stmt: tx.prepare("INSERT INTO cwr_xrf (file_id, record_type, transaction_sequence_num, record_sequence_num, organisation_code, identifier, identifier_type, validity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?,
    })
}
