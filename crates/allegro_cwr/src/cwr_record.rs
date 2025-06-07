use crate::error::CwrParseError;
use crate::{
    AckRecord, AgrRecord, AltRecord, AriRecord, ComRecord, EwtRecord, GrhRecord, GrtRecord, HdrRecord, IndRecord, InsRecord, IpaRecord, MsgRecord, NatRecord, NetRecord, NowRecord, NpaRecord, NpnRecord, NprRecord, NwnRecord, NwrRecord, OrnRecord, PerRecord, PwrRecord, RecRecord, SptRecord, SpuRecord, SwrRecord,
    SwtRecord, TerRecord, TrlRecord, VerRecord, XrfRecord,
};

/// Enum containing all possible parsed CWR record types
#[derive(Debug, Clone, serde::Serialize)]
pub enum CwrRecord {
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

impl CwrRecord {
    pub fn record_type(&self) -> &str {
        match self {
            CwrRecord::Hdr(_) => "HDR",
            CwrRecord::Grh(_) => "GRH",
            CwrRecord::Grt(_) => "GRT",
            CwrRecord::Trl(_) => "TRL",
            CwrRecord::Agr(_) => "AGR",
            CwrRecord::Nwr(_) => "NWR",
            CwrRecord::Ack(_) => "ACK",
            CwrRecord::Ter(_) => "TER",
            CwrRecord::Ipa(_) => "IPA",
            CwrRecord::Npa(_) => "NPA",
            CwrRecord::Spu(_) => "SPU",
            CwrRecord::Npn(_) => "NPN",
            CwrRecord::Spt(_) => "SPT",
            CwrRecord::Swr(_) => "SWR",
            CwrRecord::Nwn(_) => "NWN",
            CwrRecord::Swt(_) => "SWT",
            CwrRecord::Pwr(_) => "PWR",
            CwrRecord::Alt(_) => "ALT",
            CwrRecord::Nat(_) => "NAT",
            CwrRecord::Ewt(_) => "EWT",
            CwrRecord::Ver(_) => "VER",
            CwrRecord::Per(_) => "PER",
            CwrRecord::Npr(_) => "NPR",
            CwrRecord::Rec(_) => "REC",
            CwrRecord::Orn(_) => "ORN",
            CwrRecord::Ins(_) => "INS",
            CwrRecord::Ind(_) => "IND",
            CwrRecord::Com(_) => "COM",
            CwrRecord::Msg(_) => "MSG",
            CwrRecord::Net(_) => "NET",
            CwrRecord::Now(_) => "NOW",
            CwrRecord::Ari(_) => "ARI",
            CwrRecord::Xrf(_) => "XRF",
        }
    }
}

pub fn parse_by_record_type(record_type: &str, line: &str) -> Result<(CwrRecord, Vec<String>), CwrParseError> {
    match record_type {
        "HDR" => {
            let result = HdrRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Hdr(result.record), result.warnings))
        }
        "GRH" => {
            let result = GrhRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Grh(result.record), result.warnings))
        }
        "GRT" => {
            let result = GrtRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Grt(result.record), result.warnings))
        }
        "TRL" => {
            let result = TrlRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Trl(result.record), result.warnings))
        }
        "AGR" => {
            let result = AgrRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Agr(result.record), result.warnings))
        }
        "NWR" | "REV" | "ISW" | "EXC" => {
            let result = NwrRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Nwr(result.record), result.warnings))
        }
        "ACK" => {
            let result = AckRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ack(result.record), result.warnings))
        }
        "TER" => {
            let result = TerRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ter(result.record), result.warnings))
        }
        "IPA" => {
            let result = IpaRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ipa(result.record), result.warnings))
        }
        "NPA" => {
            let result = NpaRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Npa(result.record), result.warnings))
        }
        "SPU" | "OPU" => {
            let result = SpuRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Spu(result.record), result.warnings))
        }
        "NPN" => {
            let result = NpnRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Npn(result.record), result.warnings))
        }
        "SPT" | "OPT" => {
            let result = SptRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Spt(result.record), result.warnings))
        }
        "SWR" | "OWR" => {
            let result = SwrRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Swr(result.record), result.warnings))
        }
        "NWN" => {
            let result = NwnRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Nwn(result.record), result.warnings))
        }
        "SWT" | "OWT" => {
            let result = SwtRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Swt(result.record), result.warnings))
        }
        "PWR" => {
            let result = PwrRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Pwr(result.record), result.warnings))
        }
        "ALT" => {
            let result = AltRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Alt(result.record), result.warnings))
        }
        "NAT" => {
            let result = NatRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Nat(result.record), result.warnings))
        }
        "EWT" => {
            let result = EwtRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ewt(result.record), result.warnings))
        }
        "VER" => {
            let result = VerRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ver(result.record), result.warnings))
        }
        "PER" => {
            let result = PerRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Per(result.record), result.warnings))
        }
        "NPR" => {
            let result = NprRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Npr(result.record), result.warnings))
        }
        "REC" => {
            let result = RecRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Rec(result.record), result.warnings))
        }
        "ORN" => {
            let result = OrnRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Orn(result.record), result.warnings))
        }
        "INS" => {
            let result = InsRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ins(result.record), result.warnings))
        }
        "IND" => {
            let result = IndRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ind(result.record), result.warnings))
        }
        "COM" => {
            let result = ComRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Com(result.record), result.warnings))
        }
        "MSG" => {
            let result = MsgRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Msg(result.record), result.warnings))
        }
        "NET" | "NCT" | "NVT" => {
            let result = NetRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Net(result.record), result.warnings))
        }
        "NOW" => {
            let result = NowRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Now(result.record), result.warnings))
        }
        "ARI" => {
            let result = AriRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Ari(result.record), result.warnings))
        }
        "XRF" => {
            let result = XrfRecord::from_cwr_line(line)?;
            Ok((CwrRecord::Xrf(result.record), result.warnings))
        }
        _ => Err(CwrParseError::BadFormat(format!("Unrecognized record type '{}'", record_type))),
    }
}
