use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::DescriptorHeader;
use crate::mpegts::psi::ProgramSpecificInformationHeader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalAccessTable {
    pub header: ProgramSpecificInformationHeader,
    pub descriptors: Vec<DescriptorHeader>,
}

