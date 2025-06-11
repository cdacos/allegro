//! ISO 639-2 Language Dialect code

use crate::domain_types::CharacterSet;
use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use std::borrow::Cow;

/// ISO 639-2 Language Dialect code (3 characters)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LanguageDialect(pub String);

impl LanguageDialect {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for LanguageDialect {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for LanguageDialect {
    fn to_cwr_field_bytes(&self, _width: usize, _character_set: &CharacterSet) -> Vec<u8> {
        format_text(self.as_str(), _width).into_bytes()
    }
}

impl CwrFieldParse for LanguageDialect {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::language_dialect_codes::is_valid_language_dialect_code;

        let trimmed = source.trim().to_uppercase();
        let mut warnings = vec![];

        if !is_valid_language_dialect_code(&trimmed) {
            warnings.push(CwrWarning {
                field_name,
                field_title,
                source_str: Cow::Owned(source.to_string()),
                level: WarningLevel::Warning,
                description: format!("Language dialect code '{}' not found in ISO 639-2 table", trimmed),
            });
        }

        (LanguageDialect(trimmed), warnings)
    }
}

impl CwrFieldParse for Option<LanguageDialect> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (language_dialect, warnings) = LanguageDialect::parse_cwr_field(source, field_name, field_title);
            (Some(language_dialect), warnings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_language_dialect_codes() {
        let test_cases = [
            ("eng", "ENG"),
            ("fre", "FRE"),
            ("spa", "SPA"),
            ("alb", "ALB"), // Bibliographic variant
            ("sqi", "SQI"), // Terminologic variant
            ("arm", "ARM"),
            ("hye", "HYE"),
            ("bur", "BUR"),
            ("mya", "MYA"),
            ("chi", "CHI"),
            ("zho", "ZHO"),
        ];

        for (input, expected_code) in test_cases {
            let (result, warnings) = LanguageDialect::parse_cwr_field(input, "test_field", "Test Field");
            assert_eq!(result.as_str(), expected_code);
            assert!(warnings.is_empty(), "Expected no warnings for valid code '{}'", input);
        }
    }

    #[test]
    fn test_invalid_language_dialect_code() {
        let (result, warnings) = LanguageDialect::parse_cwr_field("XXX", "test_field", "Test Field");
        assert_eq!(result.as_str(), "XXX");
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].description.contains("not found in ISO 639-2 table"));
    }

    #[test]
    fn test_case_insensitive_parsing() {
        let (result, warnings) = LanguageDialect::parse_cwr_field("eng", "test_field", "Test Field");
        assert_eq!(result.as_str(), "ENG");
        assert!(warnings.is_empty());

        let (result, warnings) = LanguageDialect::parse_cwr_field("ENG", "test_field", "Test Field");
        assert_eq!(result.as_str(), "ENG");
        assert!(warnings.is_empty());

        let (result, warnings) = LanguageDialect::parse_cwr_field("Eng", "test_field", "Test Field");
        assert_eq!(result.as_str(), "ENG");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_whitespace_trimming() {
        let (result, warnings) = LanguageDialect::parse_cwr_field("  eng  ", "test_field", "Test Field");
        assert_eq!(result.as_str(), "ENG");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_optional_language_dialect_empty() {
        let (result, warnings) = Option::<LanguageDialect>::parse_cwr_field("", "test_field", "Test Field");
        assert!(result.is_none());
        assert!(warnings.is_empty());

        let (result, warnings) = Option::<LanguageDialect>::parse_cwr_field("   ", "test_field", "Test Field");
        assert!(result.is_none());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_optional_language_dialect_with_value() {
        let (result, warnings) = Option::<LanguageDialect>::parse_cwr_field("eng", "test_field", "Test Field");
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str(), "ENG");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_cwr_field_write() {
        use crate::domain_types::CharacterSet;
        let dialect = LanguageDialect("ENG".to_string());
        assert_eq!(dialect.to_cwr_field_bytes(3, &CharacterSet::ASCII), b"ENG");
    }

    #[test]
    fn test_deref() {
        let dialect = LanguageDialect("ENG".to_string());
        assert_eq!(&*dialect, "ENG");
        assert_eq!(dialect.as_str(), "ENG");
    }

    #[test]
    fn test_variants_both_b_and_t() {
        // Test that both bibliographic and terminologic variants are recognized
        let variants = [
            ("alb", "ALB"), // Albanian bibliographic
            ("sqi", "SQI"), // Albanian terminologic
            ("arm", "ARM"), // Armenian bibliographic
            ("hye", "HYE"), // Armenian terminologic
            ("baq", "BAQ"), // Basque bibliographic
            ("eus", "EUS"), // Basque terminologic
            ("bur", "BUR"), // Burmese bibliographic
            ("mya", "MYA"), // Burmese terminologic
            ("chi", "CHI"), // Chinese bibliographic
            ("zho", "ZHO"), // Chinese terminologic
            ("cze", "CZE"), // Czech bibliographic
            ("ces", "CES"), // Czech terminologic
            ("dut", "DUT"), // Dutch bibliographic
            ("nld", "NLD"), // Dutch terminologic
            ("fre", "FRE"), // French bibliographic
            ("fra", "FRA"), // French terminologic
            ("geo", "GEO"), // Georgian bibliographic
            ("kat", "KAT"), // Georgian terminologic
            ("ger", "GER"), // German bibliographic
            ("deu", "DEU"), // German terminologic
            ("gre", "GRE"), // Greek bibliographic
            ("ell", "ELL"), // Greek terminologic
            ("ice", "ICE"), // Icelandic bibliographic
            ("isl", "ISL"), // Icelandic terminologic
            ("mac", "MAC"), // Macedonian bibliographic
            ("mkd", "MKD"), // Macedonian terminologic
            ("mao", "MAO"), // Maori bibliographic
            ("mri", "MRI"), // Maori terminologic
            ("may", "MAY"), // Malay bibliographic
            ("msa", "MSA"), // Malay terminologic
            ("per", "PER"), // Persian bibliographic
            ("fas", "FAS"), // Persian terminologic
            ("rum", "RUM"), // Romanian bibliographic
            ("ron", "RON"), // Romanian terminologic
            ("slo", "SLO"), // Slovak bibliographic
            ("slk", "SLK"), // Slovak terminologic
            ("tib", "TIB"), // Tibetan bibliographic
            ("bod", "BOD"), // Tibetan terminologic
            ("wel", "WEL"), // Welsh bibliographic
            ("cym", "CYM"), // Welsh terminologic
        ];

        for (input, expected_code) in variants {
            let (result, warnings) = LanguageDialect::parse_cwr_field(input, "test_field", "Test Field");
            assert_eq!(result.as_str(), expected_code, "Failed for input '{}'", input);
            assert!(warnings.is_empty(), "Unexpected warnings for valid code '{}'", input);
        }
    }
}
