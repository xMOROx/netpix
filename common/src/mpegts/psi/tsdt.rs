use crate::mpegts::psi::pmt::descriptor::Descriptor;
use crate::mpegts::psi::ProgramSpecificInformationHeader;

pub struct ProgramSpecificInformation {
    pub header: ProgramSpecificInformationHeader,
    pub data: Vec<Descriptor>,
}