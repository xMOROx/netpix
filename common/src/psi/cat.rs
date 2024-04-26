use serde::{Deserialize, Serialize};
use crate::psi::ProgramSpecificInformationHeader;
use crate::descriptor::Descriptor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalAccessTable {
    pub header: ProgramSpecificInformationHeader,
    pub descriptors: Vec<Descriptor>,
}

