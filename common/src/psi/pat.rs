use crate::psi::ProgramSpecificInformationHeader;

pub struct ProgramAssociationTable {
    pub header: ProgramSpecificInformationHeader,
    pub transport_stream_id: u16,
    pub programs: Vec<ProgramAssociationItem>,
}

pub struct ProgramAssociationItem {
    pub program_number: u16,
    pub network_pid: Option<u16>,
    pub program_map_pid: Option<u16>,
}