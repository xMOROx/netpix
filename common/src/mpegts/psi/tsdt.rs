use crate::mpegts::descriptors::DescriptorHeader;
use crate::mpegts::psi::ProgramSpecificInformationHeader;

pub struct ProgramSpecificInformation {
    pub header: ProgramSpecificInformationHeader,
    pub data: Vec<DescriptorHeader>,
}