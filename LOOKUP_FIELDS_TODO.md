# CWR Lookup Fields (Fmt==L) Verification Checklist

This document tracks the verification and implementation of all 92 lookup fields found in the CWR 2.2 specification.

## Status Legend
- ✅ **DONE** - Field uses proper domain type
- 🔄 **IN PROGRESS** - Work started but not complete
- ❌ **TODO** - Field uses String, needs domain type
- 🔍 **CHECK** - Need to verify current implementation
- ⚠️ **ISSUE** - Known problem or blocker

---

## HDR (Transmission Header) Record - 1 field
- [ ] `character_set` (pos 87-101) - CharacterSet enum ✅ DONE

## GRH (Group Header) Record - 2 fields  
- [ ] `transaction_type` (pos 4-6) - TransactionType enum ✅ DONE
- [ ] `submission_distribution_type` (pos 27-28) - String (blank for CWR) 🔍 CHECK

## GRT (Group Trailer) Record - 1 field
- [ ] `currency_indicator` (pos 25-27) - CurrencyCode struct ✅ DONE

## AGR (Agreement) Record - 2 fields
- [x] `agreement_type` (pos 48-49) - AgreementType lookup ✅ DONE
- [ ] `sales_manufacture_clause` (pos 105) - String ❌ TODO (S/M values)

## NWR (New Work Registration) Record - 9 fields
- [x] `language_code` (pos 80-81) - LanguageCode lookup ✅ DONE
- [x] `musical_work_distribution_category` (pos 127-129) - MusicalWorkDistributionCategory lookup ✅ DONE
- [x] `text_music_relationship` (pos 137-139) - TextMusicRelationship lookup ✅ DONE
- [x] `composite_type` (pos 140-142) - CompositeType lookup ✅ DONE
- [x] `version_type` (pos 143-145) - VersionType lookup ✅ DONE
- [x] `excerpt_type` (pos 146-148) - ExcerptType lookup ✅ DONE
- [x] `music_arrangement` (pos 149-151) - MusicArrangement lookup ✅ DONE
- [x] `lyric_adaptation` (pos 152-154) - LyricAdaptation lookup ✅ DONE
- [x] `cwr_work_type` (pos 195-196) - WorkType lookup ✅ DONE

## ACK (Acknowledgement) Record - 2 fields
- [ ] `original_transaction_type` (pos 47-49) - TransactionType enum ✅ DONE
- [ ] `transaction_status` (pos 158-159) - String ❌ TODO (Transaction Status.csv)

## TER (Territory) Record - 2 fields
- [ ] `inclusion_exclusion_indicator` (pos 20) - InclusionExclusionIndicator enum ✅ DONE
- [ ] `tis_numeric_code` (pos 21-24) - TisNumericCode struct ✅ DONE

## IPA (Interested Party Agreement) Record - 6 fields
- [ ] `agreement_role_code` (pos 20-21) - AgreementRoleCode enum ✅ DONE
- [ ] `interested_party_ipi_name_num` (pos 22-32) - String ❌ TODO (IPI Database)
- [ ] `ipi_base_number` (pos 33-45) - String ❌ TODO (IPI Database)
- [ ] `pr_affiliation_society` (pos 130-132) - String ❌ TODO (Society Code.csv)
- [ ] `mr_affiliation_society` (pos 138-140) - String ❌ TODO (Society Code.csv)
- [ ] `sr_affiliation_society` (pos 146-148) - String ❌ TODO (Society Code.csv)

## NPA (Name Publisher) Record - 1 field
- [ ] `language_code` (pos 349-350) - String ❌ TODO (CIS Language Code.csv)

## SPU (Publisher Controlled by Submitter) Record - 10 fields
- [ ] `publisher_type` (pos 77-78) - PublisherType enum ✅ DONE
- [ ] `publisher_ipi_name_num` (pos 88-98) - String ❌ TODO (IPI Database)
- [ ] `pr_affiliation_society_num` (pos 113-115) - String ❌ TODO (Society Code.csv)
- [ ] `mr_society` (pos 121-123) - String ❌ TODO (Society Code.csv)
- [ ] `sr_society` (pos 129-131) - String ❌ TODO (Society Code.csv)
- [ ] `special_agreements_indicator` (pos 137) - FlagYNU enum ✅ DONE
- [ ] `publisher_ipi_base_number` (pos 140-152) - String ❌ TODO (IPI Database)
- [ ] `agreement_type` (pos 181-182) - String ❌ TODO (Agreement Type.csv)
- [ ] `usa_license_ind` (pos 183) - String ❌ TODO (USA License Indicator.csv)

## NPN (Name Publisher Name) Record - 1 field
- [ ] `language_code` (pos 511-512) - String ❌ TODO (CIS Language Code.csv)

## SPT (Publisher Territory) Record - 2 fields
- [ ] `inclusion_exclusion_indicator` (pos 50) - InclusionExclusionIndicator enum ✅ DONE
- [ ] `tis_numeric_code` (pos 51-54) - TisNumericCode struct ✅ DONE

## SWR (Writer Controlled by Submitter) Record - 6 fields
- [ ] `writer_designation_code` (pos 105-106) - String ❌ TODO (Writer Designation.csv)
- [ ] `writer_ipi_name_num` (pos 116-126) - String ❌ TODO (IPI Database)
- [ ] `pr_affiliation_society_num` (pos 127-129) - String ❌ TODO (Society Code.csv)
- [ ] `mr_society` (pos 135-137) - String ❌ TODO (Society Code.csv)
- [ ] `sr_society` (pos 143-145) - String ❌ TODO (Society Code.csv)
- [ ] `writer_ipi_base_number` (pos 155-167) - String ❌ TODO (IPI Database)
- [ ] `usa_license_ind` (pos 180) - String ❌ TODO (USA License Indicator.csv)

## NWN (Name Writer Name) Record - 1 field
- [ ] `language_code` (pos 349-350) - String ❌ TODO (CIS Language Code.csv)

## SWT (Writer Territory) Record - 2 fields
- [ ] `inclusion_exclusion_indicator` (pos 44) - InclusionExclusionIndicator enum ✅ DONE
- [ ] `tis_numeric_code` (pos 45-48) - TisNumericCode struct ✅ DONE

## ALT (Alternate Title) Record - 2 fields
- [ ] `title_type` (pos 80-81) - TitleType enum ✅ DONE
- [ ] `language_code` (pos 82-83) - String ❌ TODO (CIS Language Code.csv)

## NAT (Name Title) Record - 2 fields
- [ ] `title_type` (pos 660-661) - TitleType enum ✅ DONE
- [ ] `language_code` (pos 662-663) - String ❌ TODO (CIS Language Code.csv)

## EWT (Entire Work Title) Record - 5 fields
- [ ] `language_code` (pos 91-92) - String ❌ TODO (CIS Language Code.csv)
- [ ] `writer_1_ipi_name_num` (pos 228-238) - String ❌ TODO (IPI Database)
- [ ] `writer_1_ipi_base_number` (pos 239-251) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_name_num` (pos 327-337) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_base_number` (pos 338-350) - String ❌ TODO (IPI Database)

## VER (Version) Record - 5 fields
- [ ] `language_code` (pos 91-92) - String ❌ TODO (CIS Language Code.csv)
- [ ] `writer_1_ipi_name_num` (pos 228-238) - String ❌ TODO (IPI Database)
- [ ] `writer_1_ipi_base_number` (pos 239-251) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_name_num` (pos 327-337) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_base_number` (pos 338-350) - String ❌ TODO (IPI Database)

## PER (Performing Artist) Record - 2 fields
- [ ] `performing_artist_ipi_name_num` (pos 95-105) - String ❌ TODO (IPI Database)
- [ ] `performing_artist_ipi_base_number` (pos 106-118) - String ❌ TODO (IPI Database)

## NPR (Name Performing Artist) Record - 5 fields
- [ ] `performing_artist_ipi_name_num` (pos 340-350) - String ❌ TODO (IPI Database)
- [ ] `performing_artist_ipi_base_number` (pos 351-363) - String ❌ TODO (IPI Database)
- [ ] `language_code` (pos 364-365) - String ❌ TODO (CIS Language Code.csv)
- [ ] `performance_language` (pos 366-367) - String ❌ TODO (CIS Language Code.csv)
- [ ] `performance_dialect` (pos 368-370) - String ❌ TODO (ISO 639-2(T))

## REC (Recording Detail) Record - 6 fields
- [ ] `ean` (pos 237-249) - String ❌ TODO (EAN-13 format)
- [ ] `isrc` (pos 250-261) - String ❌ TODO (ISO 3901 format)
- [x] `recording_format` (pos 262) - RecordingFormat enum ✅ DONE
- [x] `recording_technique` (pos 263) - RecordingTechnique enum ✅ DONE
- [x] `media_type` (pos 264-266) - MediaType lookup ✅ DONE
- [ ] `isrc_validity` (pos 507-526) - String ❌ TODO (ISRC Validity Indicator.csv)

## ORN (Work Origin) Record - 1 field
- [x] `intended_purpose` (pos 20-22) - IntendedPurpose lookup ✅ DONE

## INS (Instrumentation Summary) Record - 1 field
- [x] `standard_instrumentation_type` (pos 23-25) - StandardInstrumentation lookup ✅ DONE

## IND (Instrumentation Detail) Record - 1 field
- [ ] `instrument_code` (pos 20-22) - String ❌ TODO (Instrument.csv)

## COM (Composite Component) Record - 4 fields
- [ ] `writer_1_ipi_name_num` (pos 186-196) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_name_num` (pos 272-282) - String ❌ TODO (IPI Database)
- [ ] `writer_1_ipi_base_number` (pos 283-295) - String ❌ TODO (IPI Database)
- [ ] `writer_2_ipi_base_number` (pos 296-308) - String ❌ TODO (IPI Database)

## MSG (Message) Record - 2 fields
- [ ] `message_type` (pos 20) - String ❌ TODO (F/R/T/G/E values)
- [ ] `message_level` (pos 32) - String ❌ TODO (E/G/T/R/F values)

## NET (Name Title) Record - 1 field
- [ ] `language_code` (pos 660-661) - String ❌ TODO (CIS Language Code.csv)

## NOW (Name Other Writer) Record - 2 fields
- [ ] `language_code` (pos 340-341) - String ❌ TODO (CIS Language Code.csv)
- [ ] `writer_position` (pos 342) - String ❌ TODO (Position values)

## ARI (Additional Related Info) Record - 3 fields
- [ ] `society_num` (pos 20-22) - String ❌ TODO (Society Code.csv)
- [ ] `type_of_right` (pos 37-39) - String ❌ TODO (Type of Right.csv)
- [ ] `subject_code` (pos 40-41) - String ❌ TODO (Subject Codes.csv)

## XRF (Work ID Cross Reference) Record - 2 fields
- [ ] `organisation_code` (pos 20-22) - String ❌ TODO (Transmitter Code Table)
- [ ] `identifier_type` (pos 37) - String ❌ TODO (W/R/P/V values)

---

## Summary Statistics
- **Total Fields**: 92
- **Already Done**: ~22 (domain types exist and in use)
- **Need Implementation**: ~70
- **Available CSV Files**: 31 lookup tables in docs/cwr/
- **Recent Progress**: Added 7 new lookup types (TextMusicRelationship, ExcerptType, MusicArrangement, LyricAdaptation, MediaType, IntendedPurpose, StandardInstrumentation)

## Implementation Priority
1. **High**: Core fields used in most transactions (Agreement Type, Language Code, Society Code)
2. **Medium**: Work registration fields (Work Type, Version Type, etc.)
3. **Low**: Technical/status fields (Message Type, Subject Codes, etc.)

## Next Steps
1. Start with Agreement Type (most critical)
2. Create domain types from CSV data
3. Update field definitions in records
4. Update SQLite integration
5. Add validation logic