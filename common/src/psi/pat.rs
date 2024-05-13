use serde::{Deserialize, Serialize};
use crate::psi::ProgramSpecificInformationHeader;

const STUFFED_BYTE: u8 = 0xFF;

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
    pub fn unmarshal(pointer_field: bool, data: &[u8]) -> ProgramAssociationTable {
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

        let transport_stream_id = (data[3] as u16) << 8 | data[4] as u16;
        // 0b_1100_0000 - reserved
        let version_number = data[5] & 0x3E >> 1;
        let current_next_indicator = (data[5] & 0x01) != 0;
        let section_number = data[6];
        let last_section_number = data[7];

        let mut programs = Vec::new();
        let mut i: usize = 8;
        while i < section_length - 4 {
            if ProgramAssociationTable::check_if_packed_has_stuffing(&data[i..i + 4]) {
                break;
            }

            let program_number = (data[i] as u16) << 8 | data[i + 1] as u16;
            let mut network_pid = 0;
            let mut program_map_pid = 0;
            // 0b_1110_0000 - reserved
            let pid = ((data[i + 2] & 0x1F) as u16) << 8 | data[i + 3] as u16;

            if program_number == 0 {
                network_pid = pid;
            } else {
                program_map_pid = pid;
            }
            programs.push(ProgramAssociationItem {
                program_number,
                network_pid: if network_pid != 0 { Some(network_pid) } else { None },
                program_map_pid: if program_map_pid != 0 { Some(program_map_pid) } else { None },
            });
            i += 4;
        }
        let crc_data = &data[section_length as usize + 8..];
        let mut crc_32: u32 = 0;

        for i in 0..crc_data.len() {
            crc_32 |= (crc_data[i] as u32) << (24 - i * 8);
        }

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


        ProgramAssociationTable {
            header,
            transport_stream_id,
            programs,
        }
    }

    fn check_if_packed_has_stuffing(data: &[u8]) -> bool {
        for byte in data {
            if *byte != STUFFED_BYTE {
                return false;
            }
        }
        true
    }
}