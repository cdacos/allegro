use crate::error::CwrParseError;
use crate::{
    AckRecord, AgrRecord, AltRecord, AriRecord, ComRecord, EwtRecord, GrhRecord, GrtRecord, HdrRecord, IndRecord, InsRecord, IpaRecord, MsgRecord, NatRecord, NetRecord, NowRecord, NpaRecord, NpnRecord, NprRecord, NwnRecord, NwrRecord, OrnRecord, PerRecord, PwrRecord, RecRecord, SptRecord, SpuRecord, SwrRecord,
    SwtRecord, TerRecord, TrlRecord, VerRecord, XrfRecord,
};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Enum containing all possible parsed CWR record types
#[derive(Debug, Clone, serde::Serialize)]
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
        match self {
            CwrRegistry::Hdr(_) => "HDR",
            CwrRegistry::Grh(_) => "GRH",
            CwrRegistry::Grt(_) => "GRT",
            CwrRegistry::Trl(_) => "TRL",
            CwrRegistry::Agr(_) => "AGR",
            CwrRegistry::Nwr(_) => "NWR",
            CwrRegistry::Ack(_) => "ACK",
            CwrRegistry::Ter(_) => "TER",
            CwrRegistry::Ipa(_) => "IPA",
            CwrRegistry::Npa(_) => "NPA",
            CwrRegistry::Spu(_) => "SPU",
            CwrRegistry::Npn(_) => "NPN",
            CwrRegistry::Spt(_) => "SPT",
            CwrRegistry::Swr(_) => "SWR",
            CwrRegistry::Nwn(_) => "NWN",
            CwrRegistry::Swt(_) => "SWT",
            CwrRegistry::Pwr(_) => "PWR",
            CwrRegistry::Alt(_) => "ALT",
            CwrRegistry::Nat(_) => "NAT",
            CwrRegistry::Ewt(_) => "EWT",
            CwrRegistry::Ver(_) => "VER",
            CwrRegistry::Per(_) => "PER",
            CwrRegistry::Npr(_) => "NPR",
            CwrRegistry::Rec(_) => "REC",
            CwrRegistry::Orn(_) => "ORN",
            CwrRegistry::Ins(_) => "INS",
            CwrRegistry::Ind(_) => "IND",
            CwrRegistry::Com(_) => "COM",
            CwrRegistry::Msg(_) => "MSG",
            CwrRegistry::Net(_) => "NET",
            CwrRegistry::Now(_) => "NOW",
            CwrRegistry::Ari(_) => "ARI",
            CwrRegistry::Xrf(_) => "XRF",
        }
    }
}

fn parse_hdr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = HdrRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Hdr(result.record), result.warnings))
}

fn parse_grh(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = GrhRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Grh(result.record), result.warnings))
}

fn parse_grt(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = GrtRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Grt(result.record), result.warnings))
}

fn parse_trl(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = TrlRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Trl(result.record), result.warnings))
}

fn parse_agr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = AgrRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Agr(result.record), result.warnings))
}

fn parse_nwr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NwrRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Nwr(result.record), result.warnings))
}

fn parse_ack(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = AckRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ack(result.record), result.warnings))
}

fn parse_ter(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = TerRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ter(result.record), result.warnings))
}

fn parse_ipa(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = IpaRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ipa(result.record), result.warnings))
}

fn parse_npa(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NpaRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Npa(result.record), result.warnings))
}

fn parse_spu(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = SpuRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Spu(result.record), result.warnings))
}

fn parse_npn(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NpnRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Npn(result.record), result.warnings))
}

fn parse_spt(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = SptRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Spt(result.record), result.warnings))
}

fn parse_swr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = SwrRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Swr(result.record), result.warnings))
}

fn parse_nwn(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NwnRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Nwn(result.record), result.warnings))
}

fn parse_swt(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = SwtRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Swt(result.record), result.warnings))
}

fn parse_pwr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = PwrRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Pwr(result.record), result.warnings))
}

fn parse_alt(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = AltRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Alt(result.record), result.warnings))
}

fn parse_nat(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NatRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Nat(result.record), result.warnings))
}

fn parse_ewt(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = EwtRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ewt(result.record), result.warnings))
}

fn parse_ver(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = VerRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ver(result.record), result.warnings))
}

fn parse_per(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = PerRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Per(result.record), result.warnings))
}

fn parse_npr(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NprRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Npr(result.record), result.warnings))
}

fn parse_rec(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = RecRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Rec(result.record), result.warnings))
}

fn parse_orn(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = OrnRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Orn(result.record), result.warnings))
}

fn parse_ins(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = InsRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ins(result.record), result.warnings))
}

fn parse_ind(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = IndRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ind(result.record), result.warnings))
}

fn parse_com(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = ComRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Com(result.record), result.warnings))
}

fn parse_msg(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = MsgRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Msg(result.record), result.warnings))
}

fn parse_net(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NetRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Net(result.record), result.warnings))
}

fn parse_now(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = NowRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Now(result.record), result.warnings))
}

fn parse_ari(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = AriRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Ari(result.record), result.warnings))
}

fn parse_xrf(line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let result = XrfRecord::from_cwr_line(line)?;
    Ok((CwrRegistry::Xrf(result.record), result.warnings))
}

static RECORD_PARSERS: LazyLock<HashMap<&'static str, fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("HDR", parse_hdr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("GRH", parse_grh as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("GRT", parse_grt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("TRL", parse_trl as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("AGR", parse_agr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NWR", parse_nwr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("REV", parse_nwr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("ISW", parse_nwr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("EXC", parse_nwr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("ACK", parse_ack as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("TER", parse_ter as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("IPA", parse_ipa as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NPA", parse_npa as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("SPU", parse_spu as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("OPU", parse_spu as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NPN", parse_npn as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("SPT", parse_spt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("OPT", parse_spt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("SWR", parse_swr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("OWR", parse_swr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NWN", parse_nwn as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("SWT", parse_swt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("OWT", parse_swt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("PWR", parse_pwr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("ALT", parse_alt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NAT", parse_nat as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("EWT", parse_ewt as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("VER", parse_ver as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("PER", parse_per as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NPR", parse_npr as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("REC", parse_rec as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("ORN", parse_orn as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("INS", parse_ins as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("IND", parse_ind as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("COM", parse_com as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("MSG", parse_msg as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NET", parse_net as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NCT", parse_net as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NVT", parse_net as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("NOW", parse_now as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("ARI", parse_ari as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map.insert("XRF", parse_xrf as fn(&str) -> Result<(CwrRegistry, Vec<String>), CwrParseError>);
    map
});

pub fn parse_by_record_type(record_type: &str, line: &str) -> Result<(CwrRegistry, Vec<String>), CwrParseError> {
    let parser_fn = RECORD_PARSERS.get(record_type).ok_or_else(|| CwrParseError::BadFormat(format!("Unrecognized record type '{}'", record_type)))?;

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
    fn test_parse_by_record_type_unknown() {
        let line = "UNKSOME_UNKNOWN_RECORD_TYPE";
        let result = parse_by_record_type("UNK", line);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unrecognized record type 'UNK'"));
    }
}
