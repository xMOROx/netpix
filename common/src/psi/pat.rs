use serde::{Deserialize, Serialize};
use crate::psi::ProgramSpecificInformationHeader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramAssociationTable {
    pub header: ProgramSpecificInformationHeader,
    pub transport_stream_id: u16,
    pub programs: Vec<ProgramAssociationItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramAssociationItem {
    pub program_number: u16,
    pub network_pid: Option<u16>,
    pub program_map_pid: Option<u16>,
}

impl ProgramAssociationTable {
    pub fn unmarshal_first_packet(data: &[u8]) -> ProgramAssociationTable {
        let pointer_field = data[0];
        let number_of_bytes = pointer_field as usize;
        let table_id = data[number_of_bytes + 1];
        let section_syntax_indicator = (data[number_of_bytes + 2] & 0x80) != 0;
        let section_length:usize = (((data[number_of_bytes + 2] & 0x0F) as u16) << 8 | data[number_of_bytes + 3] as u16).into();
        let transport_stream_id = (data[number_of_bytes + 4] as u16) << 8 | data[number_of_bytes + 5] as u16;
        let version_number = (data[number_of_bytes + 6] & 0x3E) >> 1;
        let current_next_indicator = (data[number_of_bytes + 6] & 0x01) != 0;
        let section_number = data[number_of_bytes + 7];
        let last_section_number = data[number_of_bytes + 8];
        // let crc_32 = ((data[section_length as usize - 4] as u32) << 24) |
        //     ((data[section_length as usize - 3] as u32) << 16) |
        //     ((data[section_length as usize - 2] as u32) << 8) |
        //     data[section_length as usize - 1] as u32;
        let header = ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length: section_length as u16,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            crc_32: 0
        };

        let mut programs = Vec::new();

        ProgramAssociationTable {
            header,
            transport_stream_id,
            programs,
        }
    }

    pub fn unmarshal(data: &[u8]) -> ProgramAssociationTable {
        let table_id = data[0];
        let section_syntax_indicator = (data[1] & 0x80) != 0;
        let section_length:usize = (((data[1] & 0x0F) as u16) << 8 | data[2] as u16).into();
        let transport_stream_id = (data[3] as u16) << 8 | data[4] as u16;
        let version_number = (data[5] & 0x3E) >> 1;
        let current_next_indicator = (data[5] & 0x01) != 0;
        let section_number = data[6];
        let last_section_number = data[7];
        // let crc_32 = ((data[section_length as usize - 4] as u32) << 24) |
        //     ((data[section_length as usize - 3] as u32) << 16) |
        //     ((data[section_length as usize - 2] as u32) << 8) |
        //     data[section_length as usize - 1] as u32;
        let header = ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length: section_length as u16,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            crc_32: 0
        };

        let mut programs = Vec::new();
        let mut i:usize = 8;
        while i < section_length - 4 {
            let program_number = (data[i] as u16) << 8 | data[i + 1] as u16;
            let mut network_pid = 0;
            let mut program_map_pid = 0;
            if program_number == 0 {
                network_pid = (data[i + 2] as u16) << 8 | data[i + 3] as u16;
            } else {
                program_map_pid = (data[i + 2] as u16) << 8 | data[i + 3] as u16;
            }
            programs.push(ProgramAssociationItem {
                program_number,
                network_pid: if network_pid != 0 { Some(network_pid) } else { None },
                program_map_pid: if program_map_pid != 0 { Some(program_map_pid) } else { None },
            });
            i += 4;
        }

        ProgramAssociationTable {
            header,
            transport_stream_id,
            programs,
        }
    }
}