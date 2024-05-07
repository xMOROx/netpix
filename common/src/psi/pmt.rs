use serde::{Deserialize, Serialize};
use crate::psi::ProgramSpecificInformationHeader;
use crate::descriptor::Descriptor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramMapTable {
    pub header: ProgramSpecificInformationHeader,
    pub program_number: u16,
    pub pcr_pid: u16,
    pub program_info_length: u16,
    pub descriptors: Vec<Descriptor>,
    pub elementary_streams_info: Vec<ElementaryStreamInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementaryStreamInfo {
    pub stream_type: u8, // table is defined on page 55 of H.222.0 (03/2017)
    pub elementary_pid: u16,
    pub es_info_length: u16,
    pub descriptors: Vec<Descriptor>,
}

impl ProgramMapTable {
    pub fn build() -> Self {
        Self {
            header: ProgramSpecificInformationHeader {
                table_id: 0,
                section_syntax_indicator: false,
                section_length: 0,
                version_number: 0,
                current_next_indicator: false,
                section_number: 0,
                last_section_number: 0,
                crc_32: 0,
            },

            program_number: 0,
            pcr_pid: 0,
            program_info_length: 0,
            descriptors: vec!(),
            elementary_streams_info: vec!(),
        }
    }
}