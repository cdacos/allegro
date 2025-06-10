use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel, format_text};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WriterPosition {
    #[default]
    First,
    Second,
}

impl WriterPosition {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            WriterPosition::First => "1",
            WriterPosition::Second => "2",
        }
    }
}

impl fmt::Display for WriterPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CwrFieldWrite for WriterPosition {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for WriterPosition {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "1" => (WriterPosition::First, vec![]),
            "2" => (WriterPosition::Second, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!(
                        "Invalid writer position '{}'. Must be '1' or '2', defaulting to '1'",
                        trimmed
                    ),
                }];
                (WriterPosition::First, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<WriterPosition> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (position, warnings) = WriterPosition::parse_cwr_field(source, field_name, field_title);
            (Some(position), warnings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_positions() {
        let (result, warnings) = WriterPosition::parse_cwr_field("1", "test", "test");
        assert_eq!(result, WriterPosition::First);
        assert!(warnings.is_empty());

        let (result, warnings) = WriterPosition::parse_cwr_field("2", "test", "test");
        assert_eq!(result, WriterPosition::Second);
        assert!(warnings.is_empty());

        let (result, warnings) = WriterPosition::parse_cwr_field(" 1 ", "test", "test");
        assert_eq!(result, WriterPosition::First);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_empty() {
        let (result, warnings) = Option::<WriterPosition>::parse_cwr_field("", "test", "test");
        assert_eq!(result, None);
        assert!(warnings.is_empty());

        let (result, warnings) = Option::<WriterPosition>::parse_cwr_field("   ", "test", "test");
        assert_eq!(result, None);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_invalid() {
        let (result, warnings) = WriterPosition::parse_cwr_field("0", "test", "test");
        assert_eq!(result, WriterPosition::First);
        assert_eq!(warnings.len(), 1);

        let (result, warnings) = WriterPosition::parse_cwr_field("3", "test", "test");
        assert_eq!(result, WriterPosition::First);
        assert_eq!(warnings.len(), 1);

        let (result, warnings) = WriterPosition::parse_cwr_field("A", "test", "test");
        assert_eq!(result, WriterPosition::First);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(WriterPosition::First.as_str(), "1");
        assert_eq!(WriterPosition::Second.as_str(), "2");
    }

    #[test]
    fn test_to_cwr_str() {
        assert_eq!(WriterPosition::First.to_cwr_str(1), "1");
        assert_eq!(WriterPosition::Second.to_cwr_str(1), "2");
    }

    #[test]
    fn test_default() {
        assert_eq!(WriterPosition::default(), WriterPosition::First);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", WriterPosition::First), "1");
        assert_eq!(format!("{}", WriterPosition::Second), "2");
    }
}
