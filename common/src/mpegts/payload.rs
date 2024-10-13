use serde::{Deserialize, Serialize};
use crate::mpegts::pes::PesPacketHeader;
use crate::mpegts::psi::PsiTypes;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PayloadType {
    PSI(PsiTypes),
    PES(PesPacketHeader),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawPayload {
    pub data: Vec<u8>,
}