use serde::{Deserialize, Serialize};
use crate::mpegts::pes::PacketizedElementaryStream;
use crate::mpegts::psi::PsiTypes;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PayloadType {
    PSI(PsiTypes),
    PES(PacketizedElementaryStream),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawPayload {
    pub data: Vec<u8>,
}