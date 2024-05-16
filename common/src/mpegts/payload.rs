use serde::{Deserialize, Serialize};
use crate::{pes::PesPacketHeader, psi::PsiTypes};

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