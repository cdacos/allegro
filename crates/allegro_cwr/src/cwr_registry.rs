use crate::error::CwrParseError;
use crate::records::*;
use std::collections::HashMap;
use std::sync::LazyLock;

type ParseResult = Result<(CwrRegistry, Vec<String>), CwrParseError>;
type ParseFunction = fn(&str) -> ParseResult;
type ParserMap = HashMap<&'static str, ParseFunction>;

/// Enum containing all possible parsed CWR record types.
/// Note: This represents the record types we parse INTO, not the input codes.
/// For example, REV/ISW/EXC codes all parse into CwrRegistry::Nwr variants.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CwrRegistry {
    Hdr(HdrRecord),
    Grh(GrhRecord),
    Grt(GrtRecord),
    Trl(TrlRecord),
    Agr(AgrRecord),
    Nwr(NwrRecord),
    Ack(AckRecord),
    Ter(TerRecord),
    Ipa(IpaRecord),
    Npa(NpaRecord),
    Spu(SpuRecord),
    Npn(NpnRecord),
    Spt(SptRecord),
    Swr(SwrRecord),
    Nwn(NwnRecord),
    Swt(SwtRecord),
    Pwr(PwrRecord),
    Alt(AltRecord),
    Nat(NatRecord),
    Ewt(EwtRecord),
    Ver(VerRecord),
    Per(PerRecord),
    Npr(NprRecord),
    Rec(RecRecord),
    Orn(OrnRecord),
    Ins(InsRecord),
    Ind(IndRecord),
    Com(ComRecord),
    Msg(MsgRecord),
    Net(NetRecord),
    Now(NowRecord),
    Ari(AriRecord),
    Xrf(XrfRecord),
}

impl CwrRegistry {
    pub fn record_type(&self) -> &str {
        use crate::records::RecordType;
        match self {
            CwrRegistry::Hdr(record) => record.record_type(),
            CwrRegistry::Grh(record) => record.record_type(),
            CwrRegistry::Grt(record) => record.record_type(),
            CwrRegistry::Trl(record) => record.record_type(),
            CwrRegistry::Agr(record) => record.record_type(),
            CwrRegistry::Nwr(record) => record.record_type(),
            CwrRegistry::Ack(record) => record.record_type(),
            CwrRegistry::Ter(record) => record.record_type(),
            CwrRegistry::Ipa(record) => record.record_type(),
            CwrRegistry::Npa(record) => record.record_type(),
            CwrRegistry::Spu(record) => record.record_type(),
            CwrRegistry::Npn(record) => record.record_type(),
            CwrRegistry::Spt(record) => record.record_type(),
            CwrRegistry::Swr(record) => record.record_type(),
            CwrRegistry::Nwn(record) => record.record_type(),
            CwrRegistry::Swt(record) => record.record_type(),
            CwrRegistry::Pwr(record) => record.record_type(),
            CwrRegistry::Alt(record) => record.record_type(),
            CwrRegistry::Nat(record) => record.record_type(),
            CwrRegistry::Ewt(record) => record.record_type(),
            CwrRegistry::Ver(record) => record.record_type(),
            CwrRegistry::Per(record) => record.record_type(),
            CwrRegistry::Npr(record) => record.record_type(),
            CwrRegistry::Rec(record) => record.record_type(),
            CwrRegistry::Orn(record) => record.record_type(),
            CwrRegistry::Ins(record) => record.record_type(),
            CwrRegistry::Ind(record) => record.record_type(),
            CwrRegistry::Com(record) => record.record_type(),
            CwrRegistry::Msg(record) => record.record_type(),
            CwrRegistry::Net(record) => record.record_type(),
            CwrRegistry::Now(record) => record.record_type(),
            CwrRegistry::Ari(record) => record.record_type(),
            CwrRegistry::Xrf(record) => record.record_type(),
        }
    }

    /// Convert this registry record to bytes with proper character set encoding
    pub fn to_cwr_record_bytes(
        &self, cwr_version: &crate::domain_types::CwrVersion, character_set: &crate::domain_types::CharacterSet,
    ) -> Vec<u8> {
        match self {
            CwrRegistry::Hdr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Grh(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Grt(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Trl(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Agr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Nwr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ack(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ter(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ipa(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Npa(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Spu(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Npn(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Spt(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Swr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Nwn(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Swt(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Pwr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Alt(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Nat(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ewt(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ver(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Per(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Npr(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Rec(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Orn(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ins(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ind(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Com(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Msg(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Net(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Now(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Ari(record) => record.to_cwr_record_bytes(cwr_version, character_set),
            CwrRegistry::Xrf(record) => record.to_cwr_record_bytes(cwr_version, character_set),
        }
    }
}

use crate::records::CwrRecord;

fn register_record<T: CwrRecord + 'static>(map: &mut ParserMap) {
    let parser_fn = |line: &str| -> ParseResult {
        let result = T::from_cwr_line(line)?;
        Ok((result.record.into_registry(), result.warnings))
    };

    for &code in T::record_codes() {
        map.insert(code, parser_fn);
    }
}

static RECORD_PARSERS: LazyLock<ParserMap> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Register all record types - each one declares its own codes
    register_record::<HdrRecord>(&mut map);
    register_record::<GrhRecord>(&mut map);
    register_record::<GrtRecord>(&mut map);
    register_record::<TrlRecord>(&mut map);
    register_record::<AgrRecord>(&mut map);
    register_record::<NwrRecord>(&mut map);
    register_record::<AckRecord>(&mut map);
    register_record::<TerRecord>(&mut map);
    register_record::<IpaRecord>(&mut map);
    register_record::<NpaRecord>(&mut map);
    register_record::<SpuRecord>(&mut map);
    register_record::<NpnRecord>(&mut map);
    register_record::<SptRecord>(&mut map);
    register_record::<SwrRecord>(&mut map);
    register_record::<NwnRecord>(&mut map);
    register_record::<SwtRecord>(&mut map);
    register_record::<PwrRecord>(&mut map);
    register_record::<AltRecord>(&mut map);
    register_record::<NatRecord>(&mut map);
    register_record::<EwtRecord>(&mut map);
    register_record::<VerRecord>(&mut map);
    register_record::<PerRecord>(&mut map);
    register_record::<NprRecord>(&mut map);
    register_record::<RecRecord>(&mut map);
    register_record::<OrnRecord>(&mut map);
    register_record::<InsRecord>(&mut map);
    register_record::<IndRecord>(&mut map);
    register_record::<ComRecord>(&mut map);
    register_record::<MsgRecord>(&mut map);
    register_record::<NetRecord>(&mut map);
    register_record::<NowRecord>(&mut map);
    register_record::<AriRecord>(&mut map);
    register_record::<XrfRecord>(&mut map);

    map
});

pub fn parse_by_record_type(record_type: &str, line: &str) -> ParseResult {
    let parser_fn = RECORD_PARSERS
        .get(record_type)
        .ok_or_else(|| CwrParseError::BadFormat(format!("Unrecognized record type '{}'", record_type)))?;

    parser_fn(line)
}

pub fn get_all_record_type_codes() -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = RECORD_PARSERS.keys().copied().collect();
    codes.sort();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_record_type_codes() {
        let codes = get_all_record_type_codes();

        assert!(!codes.is_empty());
        assert!(codes.contains(&"HDR"));
        assert!(codes.contains(&"TRL"));
        assert!(codes.contains(&"NWR"));
        assert!(codes.contains(&"REV"));
        assert!(codes.contains(&"SPU"));
        assert!(codes.contains(&"OPU"));

        assert_eq!(codes.len(), 42);

        assert!(codes.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn test_parse_by_record_type_hdr() {
        let line = "HDRPB285606836WARNER CHAPPELL MUSIC PUBLISHING LTD         01.102022122112541120221221";
        let result = parse_by_record_type("HDR", line);
        assert!(result.is_ok());
        let (record, _warnings) = result.unwrap();
        assert_eq!(record.record_type(), "HDR");
    }

    #[test]
    fn test_parse_by_record_type_rev() {
        let line = "REV0000000100000001Test Song                                               SW0000000001        SER        Y       ORI                                                                                                                                               ";
        let result = parse_by_record_type("REV", line);
        assert!(result.is_ok());
        let (record, _warnings) = result.unwrap();
        assert_eq!(record.record_type(), "REV");
    }

    #[test]
    fn test_parse_by_record_type_unknown() {
        let line = "UNKSOME_UNKNOWN_RECORD_TYPE";
        let result = parse_by_record_type("UNK", line);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unrecognized record type 'UNK'"));
    }
}
