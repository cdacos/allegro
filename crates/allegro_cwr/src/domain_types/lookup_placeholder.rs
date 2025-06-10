//! Placeholder for fields that need lookup validation
//!
//! This type behaves exactly like a String but serves as a marker
//! for fields that should use proper lookup domain types.

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, format_number, format_text};
use std::ops::Deref;

/// Placeholder for fields that need proper lookup validation
///
/// This type accepts any string value without validation.
/// It should be replaced with proper domain types that validate
/// against lookup tables.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct LookupPlaceholder(pub String);

impl LookupPlaceholder {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for LookupPlaceholder {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for LookupPlaceholder {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for LookupPlaceholder {
    fn parse_cwr_field(
        source: &str, _field_name: &'static str, _field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        // No validation - accepts any string
        (LookupPlaceholder(source.trim().to_string()), vec![])
    }
}

impl CwrFieldParse for Option<LookupPlaceholder> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (placeholder, warnings) = LookupPlaceholder::parse_cwr_field(source, field_name, field_title);
            (Some(placeholder), warnings)
        }
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::ToSql for LookupPlaceholder {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.as_str().to_sql()
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::FromSql for LookupPlaceholder {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value).map(LookupPlaceholder)
    }
}
