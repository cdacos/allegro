use crate::parsing::{format_text, format_number, CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SalesManufactureClause {
    #[default]
    Sales,
    Manufacture,
}

impl SalesManufactureClause {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            SalesManufactureClause::Sales => "S",
            SalesManufactureClause::Manufacture => "M",
        }
    }
}

impl fmt::Display for SalesManufactureClause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl CwrFieldWrite for SalesManufactureClause {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for SalesManufactureClause {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "S" => (SalesManufactureClause::Sales, vec![]),
            "M" => (SalesManufactureClause::Manufacture, vec![]),
            _ => {
                let warnings = vec![CwrWarning {
                    field_name,
                    field_title,
                    source_str: Cow::Owned(source.to_string()),
                    level: WarningLevel::Warning,
                    description: format!(
                        "Invalid sales/manufacture clause '{}'. Must be 'S' (Sales) or 'M' (Manufacture), defaulting to 'S'",
                        trimmed
                    ),
                }];
                (SalesManufactureClause::Sales, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<SalesManufactureClause> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (clause, warnings) = SalesManufactureClause::parse_cwr_field(source, field_name, field_title);
            (Some(clause), warnings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_clauses() {
        let (result, warnings) = SalesManufactureClause::parse_cwr_field("S", "test", "test");
        assert_eq!(result, SalesManufactureClause::Sales);
        assert!(warnings.is_empty());

        let (result, warnings) = SalesManufactureClause::parse_cwr_field("M", "test", "test");
        assert_eq!(result, SalesManufactureClause::Manufacture);
        assert!(warnings.is_empty());

        let (result, warnings) = SalesManufactureClause::parse_cwr_field(" S ", "test", "test");
        assert_eq!(result, SalesManufactureClause::Sales);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_empty() {
        let (result, warnings) = Option::<SalesManufactureClause>::parse_cwr_field("", "test", "test");
        assert_eq!(result, None);
        assert!(warnings.is_empty());

        let (result, warnings) = Option::<SalesManufactureClause>::parse_cwr_field("   ", "test", "test");
        assert_eq!(result, None);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_invalid() {
        let (result, warnings) = SalesManufactureClause::parse_cwr_field("X", "test", "test");
        assert_eq!(result, SalesManufactureClause::Sales);
        assert_eq!(warnings.len(), 1);

        let (result, warnings) = SalesManufactureClause::parse_cwr_field("T", "test", "test");
        assert_eq!(result, SalesManufactureClause::Sales);
        assert_eq!(warnings.len(), 1);

        let (result, warnings) = SalesManufactureClause::parse_cwr_field("12", "test", "test");
        assert_eq!(result, SalesManufactureClause::Sales);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(SalesManufactureClause::Sales.as_str(), "S");
        assert_eq!(SalesManufactureClause::Manufacture.as_str(), "M");
    }

    #[test]
    fn test_to_cwr_str() {
        assert_eq!(SalesManufactureClause::Sales.to_cwr_str(), "S");
        assert_eq!(SalesManufactureClause::Manufacture.to_cwr_str(), "M");
    }

    #[test]
    fn test_default() {
        assert_eq!(SalesManufactureClause::default(), SalesManufactureClause::Sales);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SalesManufactureClause::Sales), "S");
        assert_eq!(format!("{}", SalesManufactureClause::Manufacture), "M");
    }
}
