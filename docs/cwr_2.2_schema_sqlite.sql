-- noinspection SqlNoDataSourceInspectionForFile

PRAGMA journal_mode = OFF;
PRAGMA synchronous = OFF;
PRAGMA temp_store = MEMORY;

CREATE TABLE error (
    error_id INTEGER PRIMARY KEY,
    line_number INTEGER NOT NULL,
    description TEXT
);

CREATE TABLE file (
    line_number INTEGER NOT NULL,
    insert_position INTEGER NOT NULL DEFAULT 0,
    record_type VARCHAR(3) NOT NULL,
    record_id INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_file_line_pos ON file(line_number, insert_position);

-- SQLITE DDL for CWR 2.2 Record Types

-- Transmission Header
CREATE TABLE cwr_hdr (
    cwr_hdr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    sender_type VARCHAR(2) NOT NULL,
    sender_id VARCHAR(9) NOT NULL,
    sender_name VARCHAR(45) NOT NULL,
    edi_standard_version_number VARCHAR(5) NOT NULL,
    creation_date VARCHAR(8) NOT NULL,
    creation_time VARCHAR(6) NOT NULL,
    transmission_date VARCHAR(8) NOT NULL,
    character_set VARCHAR(15), -- v2.1
    version VARCHAR(3), -- v2.2
    revision VARCHAR(3), -- v2.2
    software_package VARCHAR(30), -- v2.2
    software_package_version VARCHAR(30) -- v2.2
);

-- Group Header
CREATE TABLE cwr_grh (
    cwr_grh_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_type VARCHAR(3) NOT NULL,
    group_id VARCHAR(5) NOT NULL,
    version_number_for_this_transaction_type VARCHAR(5) NOT NULL,
    batch_request VARCHAR(10),
    submission_distribution_type VARCHAR(2)
);

-- Group Trailer
CREATE TABLE cwr_grt (
    cwr_grt_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    group_id VARCHAR(5) NOT NULL,
    transaction_count VARCHAR(8) NOT NULL,
    record_count VARCHAR(8) NOT NULL,
    currency_indicator VARCHAR(3),
    total_monetary_value VARCHAR(10)
);

-- Transmission Trailer
CREATE TABLE cwr_trl (
    cwr_trl_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    group_count VARCHAR(5) NOT NULL,
    transaction_count VARCHAR(8) NOT NULL,
    record_count VARCHAR(8) NOT NULL
);

-- Agreement Transaction
CREATE TABLE cwr_agr (
    cwr_agr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    submitter_agreement_number VARCHAR(14) NOT NULL,
    international_standard_agreement_code VARCHAR(14),
    agreement_type VARCHAR(2) NOT NULL,
    agreement_start_date VARCHAR(8) NOT NULL,
    agreement_end_date VARCHAR(8),
    retention_end_date VARCHAR(8),
    prior_royalty_status VARCHAR(1) NOT NULL,
    prior_royalty_start_date VARCHAR(8),
    post_term_collection_status VARCHAR(1) NOT NULL,
    post_term_collection_end_date VARCHAR(8),
    date_of_signature_of_agreement VARCHAR(8),
    number_of_works VARCHAR(5) NOT NULL,
    sales_manufacture_clause VARCHAR(1),
    shares_change VARCHAR(1),
    advance_given VARCHAR(1),
    society_assigned_agreement_number VARCHAR(14) -- v2.1
);

-- New Work Registration / Revised Registration / ISWC Notification / Existing Work in Conflict
CREATE TABLE cwr_nwr (
    cwr_nwr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    work_title VARCHAR(60) NOT NULL,
    language_code VARCHAR(2),
    submitter_work_num VARCHAR(14) NOT NULL,
    iswc VARCHAR(11),
    copyright_date VARCHAR(8),
    copyright_number VARCHAR(12),
    musical_work_distribution_category VARCHAR(3) NOT NULL,
    duration VARCHAR(6),
    recorded_indicator VARCHAR(1) NOT NULL,
    text_music_relationship VARCHAR(3),
    composite_type VARCHAR(3),
    version_type VARCHAR(3) NOT NULL,
    excerpt_type VARCHAR(3),
    music_arrangement VARCHAR(3),
    lyric_adaptation VARCHAR(3),
    contact_name VARCHAR(30),
    contact_id VARCHAR(10),
    cwr_work_type VARCHAR(2),
    grand_rights_ind VARCHAR(1),
    composite_component_count VARCHAR(3),
    date_of_publication_of_printed_edition VARCHAR(8),
    exceptional_clause VARCHAR(1),
    opus_number VARCHAR(25),
    catalogue_number VARCHAR(25),
    priority_flag VARCHAR(1) -- v2.1
);

-- Acknowledgement of Transaction
CREATE TABLE cwr_ack (
    cwr_ack_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    creation_date VARCHAR(8) NOT NULL,
    creation_time VARCHAR(6) NOT NULL,
    original_group_id VARCHAR(5) NOT NULL,
    original_transaction_sequence_num VARCHAR(8) NOT NULL,
    original_transaction_type VARCHAR(3) NOT NULL,
    creation_title VARCHAR(60),
    submitter_creation_num VARCHAR(20),
    recipient_creation_num VARCHAR(20),
    processing_date VARCHAR(8) NOT NULL,
    transaction_status VARCHAR(2) NOT NULL
);

-- Territory in Agreement
CREATE TABLE cwr_ter (
    cwr_ter_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    inclusion_exclusion_indicator VARCHAR(1) NOT NULL,
    tis_numeric_code VARCHAR(4) NOT NULL
);

-- Interested Party of Agreement
CREATE TABLE cwr_ipa (
    cwr_ipa_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    agreement_role_code VARCHAR(2) NOT NULL,
    interested_party_ipi_name_num VARCHAR(11),
    ipi_base_number VARCHAR(13),
    interested_party_num VARCHAR(9) NOT NULL,
    interested_party_last_name VARCHAR(45) NOT NULL,
    interested_party_writer_first_name VARCHAR(30),
    pr_affiliation_society VARCHAR(3),
    pr_share VARCHAR(5),
    mr_affiliation_society VARCHAR(3),
    mr_share VARCHAR(5),
    sr_affiliation_society VARCHAR(3),
    sr_share VARCHAR(5)
);

-- Non-Roman Alphabet Interested Party Name (associated with IPA)
CREATE TABLE cwr_npa (
    cwr_npa_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    interested_party_num VARCHAR(9),
    interested_party_name VARCHAR(160) NOT NULL,
    interested_party_writer_first_name VARCHAR(160) NOT NULL,
    language_code VARCHAR(2)
);

-- Publisher Controlled by Submitter / Other Publisher
CREATE TABLE cwr_spu (
    cwr_spu_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    publisher_sequence_num VARCHAR(2) NOT NULL,
    interested_party_num VARCHAR(9),
    publisher_name VARCHAR(45),
    publisher_unknown_indicator VARCHAR(1),
    publisher_type VARCHAR(2),
    tax_id_num VARCHAR(9),
    publisher_ipi_name_num VARCHAR(11),
    submitter_agreement_number VARCHAR(14),
    pr_affiliation_society_num VARCHAR(3),
    pr_ownership_share VARCHAR(5),
    mr_society VARCHAR(3),
    mr_ownership_share VARCHAR(5),
    sr_society VARCHAR(3),
    sr_ownership_share VARCHAR(5),
    special_agreements_indicator VARCHAR(1),
    first_recording_refusal_ind VARCHAR(1),
    filler VARCHAR(1),
    publisher_ipi_base_number VARCHAR(13),
    international_standard_agreement_code VARCHAR(14),
    society_assigned_agreement_number VARCHAR(14),
    agreement_type VARCHAR(2), -- v2.1
    usa_license_ind VARCHAR(1) -- v2.1
);

-- Non-Roman Alphabet Publisher Name
CREATE TABLE cwr_npn (
    cwr_npn_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    publisher_sequence_num VARCHAR(2) NOT NULL,
    interested_party_num VARCHAR(9) NOT NULL,
    publisher_name VARCHAR(480) NOT NULL,
    language_code VARCHAR(2)
);

-- Publisher Territory of Control / Other Publisher Territory
CREATE TABLE cwr_spt (
    cwr_spt_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    interested_party_num VARCHAR(9) NOT NULL,
    constant_spaces VARCHAR(6),
    pr_collection_share VARCHAR(5),
    mr_collection_share VARCHAR(5),
    sr_collection_share VARCHAR(5),
    inclusion_exclusion_indicator VARCHAR(1) NOT NULL,
    tis_numeric_code VARCHAR(4) NOT NULL,
    shares_change VARCHAR(1),
    sequence_num VARCHAR(3) -- v2.1 Mandatory
);

-- Writer Controlled by Submitter / Other Writer
CREATE TABLE cwr_swr (
    cwr_swr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    interested_party_num VARCHAR(9),
    writer_last_name VARCHAR(45),
    writer_first_name VARCHAR(30),
    writer_unknown_indicator VARCHAR(1),
    writer_designation_code VARCHAR(2),
    tax_id_num VARCHAR(9),
    writer_ipi_name_num VARCHAR(11),
    pr_affiliation_society_num VARCHAR(3),
    pr_ownership_share VARCHAR(5),
    mr_society VARCHAR(3),
    mr_ownership_share VARCHAR(5),
    sr_society VARCHAR(3),
    sr_ownership_share VARCHAR(5),
    reversionary_indicator VARCHAR(1),
    first_recording_refusal_ind VARCHAR(1),
    work_for_hire_indicator VARCHAR(1),
    filler VARCHAR(1),
    writer_ipi_base_number VARCHAR(13),
    personal_number VARCHAR(12),
    usa_license_ind VARCHAR(1) -- v2.1
);

-- Non-Roman Alphabet Writer Name
CREATE TABLE cwr_nwn (
    cwr_nwn_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    interested_party_num VARCHAR(9),
    writer_last_name VARCHAR(160) NOT NULL,
    writer_first_name VARCHAR(160),
    language_code VARCHAR(2)
);

-- Writer Territory of Control / Other Writer Territory
CREATE TABLE cwr_swt (
    cwr_swt_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    interested_party_num VARCHAR(9),
    pr_collection_share VARCHAR(5),
    mr_collection_share VARCHAR(5),
    sr_collection_share VARCHAR(5),
    inclusion_exclusion_indicator VARCHAR(1) NOT NULL,
    tis_numeric_code VARCHAR(4) NOT NULL,
    shares_change VARCHAR(1),
    sequence_num VARCHAR(3) -- v2.1 Mandatory
);

-- Publisher for Writer relationship
CREATE TABLE cwr_pwr (
    cwr_pwr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    publisher_ip_num VARCHAR(9),
    publisher_name VARCHAR(45),
    submitter_agreement_number VARCHAR(14),
    society_assigned_agreement_number VARCHAR(14),
    writer_ip_num VARCHAR(9), -- v2.1
    publisher_sequence_num VARCHAR(2) -- v2.2
);

-- Alternate Title
CREATE TABLE cwr_alt (
    cwr_alt_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    alternate_title VARCHAR(60) NOT NULL,
    title_type VARCHAR(2) NOT NULL,
    language_code VARCHAR(2)
);

-- Non-Roman Alphabet Title
CREATE TABLE cwr_nat (
    cwr_nat_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    title VARCHAR(640) NOT NULL,
    title_type VARCHAR(2) NOT NULL,
    language_code VARCHAR(2)
);

-- Entire Work Title for Excerpts
CREATE TABLE cwr_ewt (
    cwr_ewt_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    entire_work_title VARCHAR(60) NOT NULL,
    iswc_of_entire_work VARCHAR(11),
    language_code VARCHAR(2),
    writer_1_last_name VARCHAR(45),
    writer_1_first_name VARCHAR(30),
    source VARCHAR(60),
    writer_1_ipi_name_num VARCHAR(11),
    writer_1_ipi_base_number VARCHAR(13),
    writer_2_last_name VARCHAR(45),
    writer_2_first_name VARCHAR(30),
    writer_2_ipi_name_num VARCHAR(11),
    writer_2_ipi_base_number VARCHAR(13),
    submitter_work_num VARCHAR(14)
);

-- Original Work Title for Versions
CREATE TABLE cwr_ver (
    cwr_ver_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    original_work_title VARCHAR(60) NOT NULL,
    iswc_of_original_work VARCHAR(11),
    language_code VARCHAR(2),
    writer_1_last_name VARCHAR(45),
    writer_1_first_name VARCHAR(30),
    source VARCHAR(60),
    writer_1_ipi_name_num VARCHAR(11),
    writer_1_ipi_base_number VARCHAR(13),
    writer_2_last_name VARCHAR(45),
    writer_2_first_name VARCHAR(30),
    writer_2_ipi_name_num VARCHAR(11),
    writer_2_ipi_base_number VARCHAR(13),
    submitter_work_num VARCHAR(14)
);

-- Performing Artist
CREATE TABLE cwr_per (
    cwr_per_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    performing_artist_last_name VARCHAR(45) NOT NULL,
    performing_artist_first_name VARCHAR(30),
    performing_artist_ipi_name_num VARCHAR(11),
    performing_artist_ipi_base_number VARCHAR(13)
);

-- Non-Roman Alphabet Performing Artist Name
CREATE TABLE cwr_npr (
    cwr_npr_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    performing_artist_name VARCHAR(160),
    performing_artist_first_name VARCHAR(160),
    performing_artist_ipi_name_num VARCHAR(11),
    performing_artist_ipi_base_number VARCHAR(13),
    language_code VARCHAR(2),
    performance_language VARCHAR(2), -- v2.1
    performance_dialect VARCHAR(3) -- v2.1
);

-- Recording Detail
CREATE TABLE cwr_rec (
    cwr_rec_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    release_date VARCHAR(8),
    constant_blanks_1 VARCHAR(60),
    release_duration VARCHAR(6),
    constant_blanks_2 VARCHAR(5),
    album_title VARCHAR(60),
    album_label VARCHAR(60),
    release_catalog_num VARCHAR(18),
    ean VARCHAR(13),
    isrc VARCHAR(12),
    recording_format VARCHAR(1),
    recording_technique VARCHAR(1),
    media_type VARCHAR(3), -- v2.1
    recording_title VARCHAR(60), -- v2.2
    version_title VARCHAR(60), -- v2.2
    display_artist VARCHAR(60), -- v2.2
    record_label VARCHAR(60), -- v2.2
    isrc_validity VARCHAR(20), -- v2.2
    submitter_recording_identifier VARCHAR(14) -- v2.2
);

-- Work Origin
CREATE TABLE cwr_orn (
    cwr_orn_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    intended_purpose VARCHAR(3) NOT NULL,
    production_title VARCHAR(60),
    cd_identifier VARCHAR(15),
    cut_number VARCHAR(4),
    library VARCHAR(60), -- v2.1
    bltvr VARCHAR(1), -- v2.1
    filler_reserved VARCHAR(25), -- v2.1
    production_num VARCHAR(12), -- v2.1
    episode_title VARCHAR(60), -- v2.1
    episode_num VARCHAR(20), -- v2.1
    year_of_production VARCHAR(4), -- v2.1
    avi_society_code VARCHAR(3), -- v2.1
    audio_visual_number VARCHAR(15), -- v2.1
    v_isan_isan VARCHAR(12), -- v2.2
    v_isan_episode VARCHAR(4), -- v2.2
    v_isan_check_digit_1 VARCHAR(1), -- v2.2
    v_isan_version VARCHAR(8), -- v2.2
    v_isan_check_digit_2 VARCHAR(1), -- v2.2
    eidr VARCHAR(20), -- v2.2
    eidr_check_digit VARCHAR(1) -- v2.2
);

-- Instrumentation Summary
CREATE TABLE cwr_ins (
    cwr_ins_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    number_of_voices VARCHAR(3),
    standard_instrumentation_type VARCHAR(3),
    instrumentation_description VARCHAR(50)
);

-- Instrumentation Detail
CREATE TABLE cwr_ind (
    cwr_ind_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    instrument_code VARCHAR(3) NOT NULL,
    number_of_players VARCHAR(3)
);

-- Composite Component
CREATE TABLE cwr_com (
    cwr_com_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    title VARCHAR(60) NOT NULL,
    iswc_of_component VARCHAR(11),
    submitter_work_num VARCHAR(14),
    duration VARCHAR(6),
    writer_1_last_name VARCHAR(45) NOT NULL,
    writer_1_first_name VARCHAR(30),
    writer_1_ipi_name_num VARCHAR(11),
    writer_2_last_name VARCHAR(45),
    writer_2_first_name VARCHAR(30),
    writer_2_ipi_name_num VARCHAR(11),
    writer_1_ipi_base_number VARCHAR(13),
    writer_2_ipi_base_number VARCHAR(13)
);

-- Message
CREATE TABLE cwr_msg (
    cwr_msg_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    message_type VARCHAR(1) NOT NULL,
    original_record_sequence_num VARCHAR(8) NOT NULL,
    msg_record_type VARCHAR(3) NOT NULL,
    message_level VARCHAR(1) NOT NULL,
    validation_number VARCHAR(3) NOT NULL,
    message_text VARCHAR(150) NOT NULL
);

-- Non-Roman Alphabet Entire Work Title / Component Title / Original Title (for EWT/COM/VER)
CREATE TABLE cwr_net (
    cwr_net_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    title VARCHAR(640) NOT NULL,
    language_code VARCHAR(2)
);

-- Non-Roman Alphabet Writer Name (for EWT/VER/COM)
CREATE TABLE cwr_now (
    cwr_now_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    writer_name VARCHAR(160) NOT NULL,
    writer_first_name VARCHAR(160) NOT NULL,
    language_code VARCHAR(2),
    writer_position VARCHAR(1)
);

-- Additional Related Information
CREATE TABLE cwr_ari (
    cwr_ari_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    society_num VARCHAR(3) NOT NULL,
    work_num VARCHAR(14),
    type_of_right VARCHAR(3) NOT NULL,
    subject_code VARCHAR(2),
    note VARCHAR(160)
);

-- Work ID Cross Reference
CREATE TABLE cwr_xrf (
    cwr_xrf_id INTEGER PRIMARY KEY,
    record_type VARCHAR(3) NOT NULL,
    transaction_sequence_num VARCHAR(8) NOT NULL,
    record_sequence_num VARCHAR(8) NOT NULL,
    organisation_code VARCHAR(3) NOT NULL,
    identifier VARCHAR(14) NOT NULL,
    identifier_type VARCHAR(1) NOT NULL,
    validity VARCHAR(1) NOT NULL
);