use serde::{Deserialize, Serialize};
use crate::psi::ProgramSpecificInformationHeader;
use crate::descriptor::Descriptor;
use crate::psi::pat::{ProgramAssociationItem, ProgramAssociationTable};

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
    pub fn unmarshal(pointer_field: bool, data: &[u8]) -> ProgramMapTable {
        if pointer_field {
            let pointer_field = data[0];
            let data = &data[pointer_field as usize + 1..];
        }


        let table_id = data[0];
        let section_syntax_indicator = (data[1] & 0x80) != 0;
        // 0b_0100_0000 - 0 bit
        // 0b_0011_0000 - reserved
        // 0b_0000_1100 - section_length unused bits
        let section_length: usize = (((data[1] & 0x03) as u16) << 8 | (data[2] & 0xFF) as u16).into();

        let program_number = (data[3] as u16) << 8 | data[4] as u16;
        // 0b_1100_0000 - reserved
        let version_number = data[5] & 0x3E >> 1;
        let current_next_indicator = (data[5] & 0x01) != 0;
        let section_number = data[6];
        let last_section_number = data[7];
        let crc_32 = ((data[section_length as usize - 4] as u32) << 24) |
            ((data[section_length as usize - 3] as u32) << 16) |
            ((data[section_length as usize - 2] as u32) << 8) |
            data[section_length as usize - 1] as u32;

        // 0b_1110_0000 - reserved
        let pcr_pid = ((data[8] as u16) << 8 | data[9] as u16) & 0x1FFF;
        // 0b_1111_0000 - reserved
        let program_info_length = ((data[10] as u16) << 8 | data[11] as u16) & 0x0FFF;

        let header = ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length: section_length as u16,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            crc_32,
        };
        todo!("Implement parsing of descriptors and elementary streams info");
    }
}