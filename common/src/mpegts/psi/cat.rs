use serde::{Deserialize, Serialize};
use crate::mpegts::psi::pmt::descriptor::Descriptor;
use crate::mpegts::psi::ProgramSpecificInformationHeader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalAccessTable {
    pub header: ProgramSpecificInformationHeader,
    pub descriptors: Vec<Descriptor>,
}

