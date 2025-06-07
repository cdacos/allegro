use crate::{AckRecord, AgrRecord, AltRecord, AriRecord, ComRecord, EwtRecord, GrhRecord, GrtRecord, HdrRecord, IndRecord, InsRecord, IpaRecord, MsgRecord, NatRecord, NetRecord, NowRecord, NpaRecord, NpnRecord, NprRecord, NwnRecord, NwrRecord, OrnRecord, PerRecord, PwrRecord, RecRecord, SptRecord, SpuRecord, SwrRecord, SwtRecord, TerRecord, TrlRecord, VerRecord, XrfRecord};

/// Enum containing all possible parsed CWR record types
#[derive(Debug, Clone, serde::Serialize)]
pub enum CwrRecord {
    #[serde(rename = "hdr")]
    Hdr(HdrRecord),
    #[serde(rename = "grh")]
    Grh(GrhRecord),
    #[serde(rename = "grt")]
    Grt(GrtRecord),
    #[serde(rename = "trl")]
    Trl(TrlRecord),
    #[serde(rename = "agr")]
    Agr(AgrRecord),
    #[serde(rename = "nwr")]
    Nwr(NwrRecord),
    #[serde(rename = "ack")]
    Ack(AckRecord),
    #[serde(rename = "ter")]
    Ter(TerRecord),
    #[serde(rename = "ipa")]
    Ipa(IpaRecord),
    #[serde(rename = "npa")]
    Npa(NpaRecord),
    #[serde(rename = "spu")]
    Spu(SpuRecord),
    #[serde(rename = "npn")]
    Npn(NpnRecord),
    #[serde(rename = "spt")]
    Spt(SptRecord),
    #[serde(rename = "swr")]
    Swr(SwrRecord),
    #[serde(rename = "nwn")]
    Nwn(NwnRecord),
    #[serde(rename = "swt")]
    Swt(SwtRecord),
    #[serde(rename = "pwr")]
    Pwr(PwrRecord),
    #[serde(rename = "alt")]
    Alt(AltRecord),
    #[serde(rename = "nat")]
    Nat(NatRecord),
    #[serde(rename = "ewt")]
    Ewt(EwtRecord),
    #[serde(rename = "ver")]
    Ver(VerRecord),
    #[serde(rename = "per")]
    Per(PerRecord),
    #[serde(rename = "npr")]
    Npr(NprRecord),
    #[serde(rename = "rec")]
    Rec(RecRecord),
    #[serde(rename = "orn")]
    Orn(OrnRecord),
    #[serde(rename = "ins")]
    Ins(InsRecord),
    #[serde(rename = "ind")]
    Ind(IndRecord),
    #[serde(rename = "com")]
    Com(ComRecord),
    #[serde(rename = "msg")]
    Msg(MsgRecord),
    #[serde(rename = "net")]
    Net(NetRecord),
    #[serde(rename = "now")]
    Now(NowRecord),
    #[serde(rename = "ari")]
    Ari(AriRecord),
    #[serde(rename = "xrf")]
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