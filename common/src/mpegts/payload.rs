use serde::{Deserialize, Serialize};
use crate::{pes::PesPacketHeader, psi::PsiTypes};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Payload {
    PSI(PsiTypes),
    PES(PesPacketHeader),
}