//! Non-Roman Alphabet text type
//!
//! This type behaves exactly like a String but serves as a marker
//! for fields that contain text in non-Roman alphabets (e.g., Cyrillic,
//! Arabic, Chinese, Japanese, Korean, etc.).

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, format_number, format_text};
use std::ops::Deref;

/// Non-Roman Alphabet text type
///
/// This type accepts any string value without validation.
/// It serves as a marker for fields containing text in non-Roman alphabets
/// such as writer names, performer names, and titles in languages that
/// use different writing systems.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct NonRomanAlphabet(pub String);

impl NonRomanAlphabet {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for NonRomanAlphabet {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CwrFieldWrite for NonRomanAlphabet {
    fn to_cwr_str(&self, _width: usize) -> String {
        format_text(self.as_str(), _width)
    }
}

impl CwrFieldParse for NonRomanAlphabet {
    fn parse_cwr_field(
        source: &str, _field_name: &'static str, _field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        // No validation - accepts any string for non-Roman alphabet text
        (NonRomanAlphabet(source.trim().to_string()), vec![])
    }
}

impl CwrFieldParse for Option<NonRomanAlphabet> {
    fn parse_cwr_field(
        source: &str, field_name: &'static str, field_title: &'static str,
    ) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (text, warnings) = NonRomanAlphabet::parse_cwr_field(source, field_name, field_title);
            (Some(text), warnings)
        }
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::ToSql for NonRomanAlphabet {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.as_str().to_sql()
    }
}

#[cfg(feature = "sqlite")]
impl rusqlite::types::FromSql for NonRomanAlphabet {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value).map(NonRomanAlphabet)
    }
}
