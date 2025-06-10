use crate::domain_types::*;
use allegro_cwr_derive::CwrRecord;
use serde::{Deserialize, Serialize};

/// Used for NWR, REV, ISW, and EXC record types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(codes = ["NWR", "REV", "ISW", "EXC"], validator = nwr_custom_validate, test_data = "NWR0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ")]
pub struct NwrRecord {
    #[cwr(title = "'NWR', 'REV', 'ISW', or 'EXC'", start = 0, len = 3)]
    pub record_type: String,

    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: Number,

    #[cwr(title = "Record sequence number", start = 11, len = 8)]
    pub record_sequence_num: Number,

    #[cwr(title = "Work title", start = 19, len = 60)]
    pub work_title: String,

    #[cwr(title = "Language code (optional)", start = 79, len = 2)]
    pub language_code: Option<LanguageCode>,

    #[cwr(title = "Submitter work number", start = 81, len = 14)]
    pub submitter_work_num: String,

    #[cwr(title = "ISWC (optional)", start = 95, len = 11)]
    pub iswc: Option<String>,

    #[cwr(title = "Copyright date (optional)", start = 106, len = 8)]
    pub copyright_date: Option<Date>,

    #[cwr(title = "Copyright number (optional)", start = 114, len = 12)]
    pub copyright_number: Option<String>,

    #[cwr(title = "Musical work distribution category", start = 126, len = 3)]
    pub musical_work_distribution_category: MusicalWorkDistributionCategory,

    #[cwr(title = "Duration HHMMSS (conditional)", start = 129, len = 6)]
    pub duration: Option<Time>,

    #[cwr(title = "Recorded indicator (1 char)", start = 135, len = 1)]
    pub recorded_indicator: Flag,

    #[cwr(title = "Text music relationship (optional)", start = 136, len = 3)]
    pub text_music_relationship: Option<TextMusicRelationship>,

    #[cwr(title = "Composite type (optional)", start = 139, len = 3)]
    pub composite_type: Option<CompositeType>,

    #[cwr(title = "Version type", start = 142, len = 3)]
    pub version_type: VersionType,

    #[cwr(title = "Excerpt type (optional)", start = 145, len = 3)]
    pub excerpt_type: Option<ExcerptType>,

    #[cwr(title = "Music arrangement (conditional)", start = 148, len = 3)]
    pub music_arrangement: Option<MusicArrangement>,

    #[cwr(title = "Lyric adaptation (conditional)", start = 151, len = 3)]
    pub lyric_adaptation: Option<LyricAdaptation>,

    #[cwr(title = "Contact name (optional)", start = 154, len = 30)]
    pub contact_name: Option<String>,

    #[cwr(title = "Contact ID (optional)", start = 184, len = 10)]
    pub contact_id: Option<String>,

    #[cwr(title = "CWR work type (optional)", start = 194, len = 2)]
    pub cwr_work_type: Option<WorkType>,

    #[cwr(title = "Grand rights indicator (1 char, conditional)", start = 196, len = 1)]
    pub grand_rights_ind: Option<Flag>,

    #[cwr(title = "Composite component count (conditional)", start = 197, len = 3)]
    pub composite_component_count: Option<CompositeComponentCount>,

    #[cwr(title = "Date of publication of printed edition (optional)", start = 200, len = 8)]
    pub date_of_publication_of_printed_edition: Option<Date>,

    #[cwr(title = "Exceptional clause (1 char, optional)", start = 208, len = 1)]
    pub exceptional_clause: Option<Flag>,

    #[cwr(title = "Opus number (optional)", start = 209, len = 25)]
    pub opus_number: Option<String>,

    #[cwr(title = "Catalogue number (optional)", start = 234, len = 25)]
    pub catalogue_number: Option<String>,

    #[cwr(title = "Priority flag (1 char, optional, v2.1+)", start = 259, len = 1, min_version = 2.1)]
    pub priority_flag: Option<Flag>,
}

// Custom validation function for NWR record
fn nwr_custom_validate(record: &mut NwrRecord) -> Vec<CwrWarning<'static>> {
    let mut warnings = Vec::new();

    // Business rule: Duration required if Musical Work Distribution Category = "SER"
    if record.musical_work_distribution_category.as_str() == "SER" && record.duration.is_none() {
        warnings.push(CwrWarning {
            field_name: "duration",
            field_title: "Duration HHMMSS (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Critical,
            description: "Duration is required when Musical Work Distribution Category is 'SER'".to_string(),
        });
    }

    // Business rule: Duration must be > 0 if present
    if let Some(ref duration) = record.duration {
        let seconds = duration.duration_since_midnight();
        if seconds == 0.0 {
            warnings.push(CwrWarning {
                field_name: "duration",
                field_title: "Duration HHMMSS (conditional)",
                source_str: std::borrow::Cow::Owned(duration.as_str()),
                level: WarningLevel::Warning,
                description: "Duration should be greater than 00:00:00 if specified".to_string(),
            });
        }
    }

    // Business rule: Music Arrangement required if Version Type = "MOD"
    if record.version_type.as_str() == "MOD" {
        if record.music_arrangement.is_none()
            || record.music_arrangement.as_ref().is_none_or(|s| s.as_str().trim().is_empty())
        {
            warnings.push(CwrWarning {
                field_name: "music_arrangement",
                field_title: "Music arrangement (conditional)",
                source_str: std::borrow::Cow::Borrowed(""),
                level: WarningLevel::Critical,
                description: "Music Arrangement is required when Version Type is 'MOD'".to_string(),
            });
        }

        if record.lyric_adaptation.is_none()
            || record.lyric_adaptation.as_ref().is_none_or(|s| s.as_str().trim().is_empty())
        {
            warnings.push(CwrWarning {
                field_name: "lyric_adaptation",
                field_title: "Lyric adaptation (conditional)",
                source_str: std::borrow::Cow::Borrowed(""),
                level: WarningLevel::Critical,
                description: "Lyric Adaptation is required when Version Type is 'MOD'".to_string(),
            });
        }
    }

    // Business rule: Composite Component Count required for ASCAP when Composite Type is present
    if record.composite_type.is_some()
        && record.composite_type.as_ref().is_some_and(|s| !s.as_str().trim().is_empty())
        && (record.composite_component_count.is_none()
            || record.composite_component_count.as_ref().is_some_and(|c| c.0 == 0))
    {
        warnings.push(CwrWarning {
            field_name: "composite_component_count",
            field_title: "Composite component count (conditional)",
            source_str: std::borrow::Cow::Borrowed(""),
            level: WarningLevel::Warning,
            description:
                "Composite Component Count should be specified when Composite Type is present (required for ASCAP)"
                    .to_string(),
        });
    }

    // TODO: Additional business rules requiring broader context:
    // - Grand Rights Indicator required for UK societies
    // - Some societies (BMI) may require duration for "JAZ" category
    // - Submitter Work # must be unique per publisher (requires context)
    // - ISWC format validation

    warnings
}

#[cfg(test)]
mod roundtrip_test {
    use super::*;
    use crate::domain_types::CwrVersion;

    #[test]
    fn test_roundtrip_character_shift_issue() {
        // Use a complete NWR line - pad the fragment to proper NWR length
        let fragment = "T700080427519610301 EU 660110  POP";
        let original = format!("{:260}", fragment); // Pad to 260 characters

        // Parse the original line
        let (record, warnings) = NwrRecord::parse(&original);

        println!("Original:    '{}'", original);
        println!("Parsed warnings: {:?}", warnings);

        // Generate the line back
        let version = CwrVersion(2.2);
        let serialized = record.to_cwr_line(&version);

        println!("Serialized:  '{}'", serialized);

        // Check character by character differences
        let orig_chars: Vec<char> = original.chars().collect();
        let ser_chars: Vec<char> = serialized.chars().collect();

        println!("\nCharacter-by-character comparison:");
        for (i, (o, s)) in orig_chars.iter().zip(ser_chars.iter()).enumerate() {
            if o != s {
                println!("Pos {}: '{}' → '{}' (original → serialized)", i, o, s);
            }
        }

        // Show positions 110-125 specifically
        println!("\nPositions 110-125:");
        if original.len() > 110 {
            let orig_slice = &original[110..original.len().min(125)];
            println!("Original:    '{}'", orig_slice);
        }
        if serialized.len() > 110 {
            let ser_slice = &serialized[110..serialized.len().min(125)];
            println!("Serialized:  '{}'", ser_slice);
        }

        // Show field boundaries for debugging
        println!("\nField analysis:");
        println!("copyright_date (106-113): '{}'", &original[106..114]);
        println!("copyright_number (114-125): '{}'", &original[114..126]);
        println!("musical_work_dist_cat (126-128): '{}'", &original[126..129]);
    }
}
