use serde::{Deserialize, Serialize};
use crate::mpegts::pes::PacketizedElementaryStream;
use crate::mpegts::psi::PsiTypes;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum PayloadType {
    PSI(PsiTypes),
    PES(PacketizedElementaryStream),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RawPayload {
    pub data: Vec<u8>,
}