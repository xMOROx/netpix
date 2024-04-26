use crate::descriptor::Descriptor;
use crate::psi::ProgramSpecificInformationHeader;

pub struct ProgramSpecificInformation {
    pub header: ProgramSpecificInformationHeader,
    pub data: Vec<Descriptor>,
}