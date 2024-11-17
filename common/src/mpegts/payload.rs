use crate::mpegts::pes::PacketizedElementaryStream;
use crate::mpegts::psi::PsiTypes;
use serde::{Deserialize, Serialize};

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
