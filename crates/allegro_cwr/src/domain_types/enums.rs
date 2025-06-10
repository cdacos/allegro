//! Enum types for CWR parsing

use crate::parsing::{CwrFieldParse, CwrFieldWrite, CwrWarning, WarningLevel};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum YesNo {
    Yes,
    #[default]
    No,
}

impl YesNo {
    pub fn as_str(&self) -> &str {
        match self {
            YesNo::Yes => "Y",
            YesNo::No => "N",
        }
    }
}

impl CwrFieldWrite for YesNo {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for YesNo {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (YesNo::Yes, vec![]),
            "N" => (YesNo::No, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid Yes/No value '{}', defaulting to No", trimmed) }];
                (YesNo::No, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<YesNo> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (yes_no, warnings) = YesNo::parse_cwr_field(source, field_name, field_title);
            (Some(yes_no), warnings)
        }
    }
}

/// Sender type for HDR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum SenderType {
    #[default]
    Publisher,
    Society,
    Writer,
    AdministrativeAgency,
    /// For IPNN > 9 digits, this contains the leading digits
    NumericPrefix(String),
}

impl SenderType {
    pub fn as_str(&self) -> &str {
        match self {
            SenderType::Publisher => "PB",
            SenderType::Society => "SO",
            SenderType::Writer => "WR",
            SenderType::AdministrativeAgency => "AA",
            SenderType::NumericPrefix(s) => s,
        }
    }
}

impl CwrFieldWrite for SenderType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for SenderType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "PB" => (SenderType::Publisher, vec![]),
            "SO" => (SenderType::Society, vec![]),
            "WR" => (SenderType::Writer, vec![]),
            "AA" => (SenderType::AdministrativeAgency, vec![]),
            s if s.chars().all(|c| c.is_ascii_digit()) && s.len() <= 2 => (SenderType::NumericPrefix(s.to_string()), vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid sender type '{}', must be PB, SO, WR, AA, or 2-digit numeric prefix", trimmed) }];
                (SenderType::Publisher, warnings)
            }
        }
    }
}

/// Character set indicator for HDR record (v2.1+)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum CharacterSet {
    #[default]
    ASCII,
    TraditionalBig5,
    SimplifiedGb,
    UTF8,
    Unicode,
    Unknown(String),
}

impl CharacterSet {
    pub fn as_str(&self) -> &str {
        match self {
            CharacterSet::ASCII => "ASCII",
            CharacterSet::TraditionalBig5 => "Traditional Big5",
            CharacterSet::SimplifiedGb => "Simplified GB",
            CharacterSet::UTF8 => "UTF-8",
            CharacterSet::Unicode => "Unicode",
            CharacterSet::Unknown(s) => s,
        }
    }
}

impl CwrFieldWrite for CharacterSet {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for CharacterSet {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "ASCII" => (CharacterSet::ASCII, vec![]),
            "Traditional Big5" => (CharacterSet::TraditionalBig5, vec![]),
            "Simplified GB" => (CharacterSet::SimplifiedGb, vec![]),
            "UTF-8" => (CharacterSet::UTF8, vec![]),
            "Unicode" => (CharacterSet::Unicode, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Info, description: format!("Unknown character set '{}', treating as custom", trimmed) }];
                (CharacterSet::Unknown(trimmed.to_string()), warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<CharacterSet> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (charset, warnings) = CharacterSet::parse_cwr_field(source, field_name, field_title);
            (Some(charset), warnings)
        }
    }
}

/// Transaction type for GRH record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TransactionType {
    #[default]
    NWR,
    REV,
    AGR,
    ACK,
    ISW,
    EXC,
}

impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            TransactionType::NWR => "NWR",
            TransactionType::REV => "REV",
            TransactionType::AGR => "AGR",
            TransactionType::ACK => "ACK",
            TransactionType::ISW => "ISW",
            TransactionType::EXC => "EXC",
        }
    }
}

impl CwrFieldWrite for TransactionType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for TransactionType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "NWR" => (TransactionType::NWR, vec![]),
            "REV" => (TransactionType::REV, vec![]),
            "AGR" => (TransactionType::AGR, vec![]),
            "ACK" => (TransactionType::ACK, vec![]),
            "ISW" => (TransactionType::ISW, vec![]),
            "EXC" => (TransactionType::EXC, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid transaction type '{}', must be NWR, REV, AGR, ACK, ISW, or EXC", trimmed) }];
                (TransactionType::NWR, warnings)
            }
        }
    }
}

/// Prior royalty status for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PriorRoyaltyStatus {
    #[default]
    None,
    Acquired,
    Designated,
}

impl PriorRoyaltyStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PriorRoyaltyStatus::None => "N",
            PriorRoyaltyStatus::Acquired => "A",
            PriorRoyaltyStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PriorRoyaltyStatus {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for PriorRoyaltyStatus {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PriorRoyaltyStatus::None, vec![]),
            "A" => (PriorRoyaltyStatus::Acquired, vec![]),
            "D" => (PriorRoyaltyStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid prior royalty status '{}', must be N, A, or D", trimmed) }];
                (PriorRoyaltyStatus::None, warnings)
            }
        }
    }
}

/// Post-term collection status for AGR record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PostTermCollectionStatus {
    #[default]
    None,
    Original,
    Designated,
}

impl PostTermCollectionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            PostTermCollectionStatus::None => "N",
            PostTermCollectionStatus::Original => "O",
            PostTermCollectionStatus::Designated => "D",
        }
    }
}

impl CwrFieldWrite for PostTermCollectionStatus {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for PostTermCollectionStatus {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "N" => (PostTermCollectionStatus::None, vec![]),
            "O" => (PostTermCollectionStatus::Original, vec![]),
            "D" => (PostTermCollectionStatus::Designated, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid post-term collection status '{}', must be N, O, or D", trimmed) }];
                (PostTermCollectionStatus::None, warnings)
            }
        }
    }
}

/// Flag with Yes/No/Unknown values
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum FlagYNU {
    Yes,
    #[default]
    No,
    Unknown,
}

impl FlagYNU {
    pub fn as_str(&self) -> &str {
        match self {
            FlagYNU::Yes => "Y",
            FlagYNU::No => "N",
            FlagYNU::Unknown => "U",
        }
    }
}

impl CwrFieldWrite for FlagYNU {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for FlagYNU {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "Y" => (FlagYNU::Yes, vec![]),
            "N" => (FlagYNU::No, vec![]),
            "U" => (FlagYNU::Unknown, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid Y/N/U flag value '{}', defaulting to No", trimmed) }];
                (FlagYNU::No, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<FlagYNU> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (flag, warnings) = FlagYNU::parse_cwr_field(source, field_name, field_title);
            (Some(flag), warnings)
        }
    }
}

/// Inclusion/Exclusion indicator for territory records
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum InclusionExclusionIndicator {
    #[default]
    Included,
    Excluded,
}

impl InclusionExclusionIndicator {
    pub fn as_str(&self) -> &str {
        match self {
            InclusionExclusionIndicator::Included => "I",
            InclusionExclusionIndicator::Excluded => "E",
        }
    }
}

impl CwrFieldWrite for InclusionExclusionIndicator {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for InclusionExclusionIndicator {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "I" => (InclusionExclusionIndicator::Included, vec![]),
            "E" => (InclusionExclusionIndicator::Excluded, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid inclusion/exclusion indicator '{}', must be I or E", trimmed) }];
                (InclusionExclusionIndicator::Included, warnings)
            }
        }
    }
}

/// Agreement role code for IPA record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AgreementRoleCode {
    #[default]
    Assignor,
    Acquirer,
}

impl AgreementRoleCode {
    pub fn as_str(&self) -> &str {
        match self {
            AgreementRoleCode::Assignor => "AS",
            AgreementRoleCode::Acquirer => "AC",
        }
    }
}

impl CwrFieldWrite for AgreementRoleCode {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for AgreementRoleCode {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::agreement_role_codes::is_valid_agreement_role_code;

        let trimmed = source.trim();
        match trimmed {
            "AS" => (AgreementRoleCode::Assignor, vec![]),
            "AC" => (AgreementRoleCode::Acquirer, vec![]),
            _ => {
                let mut warnings = vec![];
                if !is_valid_agreement_role_code(trimmed) {
                    warnings.push(CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Critical, description: format!("Invalid agreement role code '{}', must be AS or AC", trimmed) });
                }
                (AgreementRoleCode::Assignor, warnings)
            }
        }
    }
}

/// Title type for ALT record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum TitleType {
    #[default]
    AlternateTitle,
    FormalTitle,
    OriginalTitle,
    TransliteratedTitle,
    AbbreviatedTitle,
    SearchTitle,
    TranslatedTitle,
    TransliterationTrans,
    TransliterationAlt,
    // Add more as needed
}

impl TitleType {
    pub fn as_str(&self) -> &str {
        match self {
            TitleType::AlternateTitle => "AT",
            TitleType::FormalTitle => "FT",
            TitleType::OriginalTitle => "OT",
            TitleType::TransliteratedTitle => "TR",
            TitleType::AbbreviatedTitle => "AB",
            TitleType::SearchTitle => "ST",
            TitleType::TranslatedTitle => "TT",
            TitleType::TransliterationTrans => "LT",
            TitleType::TransliterationAlt => "LA",
        }
    }
}

impl CwrFieldWrite for TitleType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for TitleType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "AT" => (TitleType::AlternateTitle, vec![]),
            "FT" => (TitleType::FormalTitle, vec![]),
            "OT" => (TitleType::OriginalTitle, vec![]),
            "TR" => (TitleType::TransliteratedTitle, vec![]),
            "LT" => (TitleType::TransliterationTrans, vec![]),
            "LA" => (TitleType::TransliterationAlt, vec![]),
            "AB" => (TitleType::AbbreviatedTitle, vec![]),
            "ST" => (TitleType::SearchTitle, vec![]),
            "TT" => (TitleType::TranslatedTitle, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Unknown title type '{}', defaulting to AT", trimmed) }];
                (TitleType::AlternateTitle, warnings)
            }
        }
    }
}

/// Recording format for REC record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingFormat {
    #[default]
    Unknown,
    Stereo,
    Mono,
    Quadrophonic,
}

impl RecordingFormat {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingFormat::Unknown => "U",
            RecordingFormat::Stereo => "S",
            RecordingFormat::Mono => "M",
            RecordingFormat::Quadrophonic => "Q",
        }
    }
}

impl CwrFieldWrite for RecordingFormat {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for RecordingFormat {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "U" => (RecordingFormat::Unknown, vec![]),
            "S" => (RecordingFormat::Stereo, vec![]),
            "M" => (RecordingFormat::Mono, vec![]),
            "Q" => (RecordingFormat::Quadrophonic, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid recording format '{}', defaulting to Unknown", trimmed) }];
                (RecordingFormat::Unknown, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<RecordingFormat> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (format, warnings) = RecordingFormat::parse_cwr_field(source, field_name, field_title);
            (Some(format), warnings)
        }
    }
}

/// Recording technique for REC record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum RecordingTechnique {
    #[default]
    Unknown,
    Analog,
    Digital,
}

impl RecordingTechnique {
    pub fn as_str(&self) -> &str {
        match self {
            RecordingTechnique::Unknown => "U",
            RecordingTechnique::Analog => "A",
            RecordingTechnique::Digital => "D",
        }
    }
}

impl CwrFieldWrite for RecordingTechnique {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for RecordingTechnique {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        match trimmed {
            "U" => (RecordingTechnique::Unknown, vec![]),
            "A" => (RecordingTechnique::Analog, vec![]),
            "D" => (RecordingTechnique::Digital, vec![]),
            _ => {
                let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid recording technique '{}', defaulting to Unknown", trimmed) }];
                (RecordingTechnique::Unknown, warnings)
            }
        }
    }
}

impl CwrFieldParse for Option<RecordingTechnique> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (technique, warnings) = RecordingTechnique::parse_cwr_field(source, field_name, field_title);
            (Some(technique), warnings)
        }
    }
}

/// Publisher type for SPU record
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum PublisherType {
    #[default]
    Acquirer,
    Administrator,
    AssignorAgent,
    SubPublisher,
    OriginalPublisher,
    IncomeParticipant,
    // Add more as needed
}

impl PublisherType {
    pub fn as_str(&self) -> &str {
        match self {
            PublisherType::Acquirer => "AQ",
            PublisherType::Administrator => "AM",
            PublisherType::AssignorAgent => "AS",
            PublisherType::SubPublisher => "SP",
            PublisherType::OriginalPublisher => "OP",
            PublisherType::IncomeParticipant => "IP",
        }
    }
}

impl CwrFieldWrite for PublisherType {
    fn to_cwr_str(&self) -> String {
        self.as_str().to_string()
    }
}

impl CwrFieldParse for PublisherType {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        use crate::lookups::publisher_types::is_valid_publisher_type;

        let trimmed = source.trim();
        let default_type = PublisherType::Acquirer;

        if !is_valid_publisher_type(trimmed) {
            let warnings = vec![CwrWarning { field_name, field_title, source_str: Cow::Owned(source.to_string()), level: WarningLevel::Warning, description: format!("Invalid publisher type '{}', defaulting to AQ", trimmed) }];
            return (default_type, warnings);
        }

        match trimmed {
            "AQ" => (PublisherType::Acquirer, vec![]),
            "AM" => (PublisherType::Administrator, vec![]),
            "AS" => (PublisherType::AssignorAgent, vec![]),
            "SP" => (PublisherType::SubPublisher, vec![]),
            "OP" => (PublisherType::OriginalPublisher, vec![]),
            "IP" => (PublisherType::IncomeParticipant, vec![]),
            _ => (default_type, vec![]),
        }
    }
}

impl CwrFieldParse for Option<PublisherType> {
    fn parse_cwr_field(source: &str, field_name: &'static str, field_title: &'static str) -> (Self, Vec<CwrWarning<'static>>) {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            (None, vec![])
        } else {
            let (publisher_type, warnings) = PublisherType::parse_cwr_field(source, field_name, field_title);
            (Some(publisher_type), warnings)
        }
    }
}
