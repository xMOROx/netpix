use serde::{Deserialize, Serialize};
use crate::mpegts::MpegtsFragment;
use crate::psi::ProgramSpecificInformationHeader;

const STUFFED_BYTE: u8 = 0xFF;
const HEADER_SIZE: usize = 3;
const HEADER_AFTER_SECTION_LENGTH_SIZE: usize = 5;
const CRC_SIZE: usize = 4;
const PROGRAM_SECTION_SIZE: usize = 4;
const MAX_SECTION_SIZE: usize = 1024;
pub const MAX_NUMBER_OF_SECTIONS: usize = 255;

const SECTION_SYNTAX_INDICATOR_MASK: u8 = 0x80;
const SECTION_LENGTH_UPPER_MASK: u8 = 0x0F;
const SECTION_LENGTH_LOWER_MASK: u8 = 0xFF;
const VERSION_NUMBER_MASK: u8 = 0x3E;
const CURRENT_NEXT_INDICATOR_MASK: u8 = 0x01;
const PROGRAM_PID_UPPER_MASK: u8 = 0x1F;


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
    pub fn unmarshal(packet: &MpegtsFragment) -> Option<ProgramAssociationTable> {
        if packet.payload.is_none() {
            return None;
        }

        let data = &packet.payload.as_ref().unwrap().data;


        if data.len() < HEADER_SIZE {
            return None;
        }

        if packet.header.payload_unit_start_indicator {
            let pointer_field = data[0];
            let data = &data[pointer_field as usize + 1..];
        }

        let start = 1;

        let header = Self::parse_header(data, start);

        let transport_stream_id = (data[start + 3] as u16) << 8 | data[start + 4] as u16;

        let programs = Self::parse_programs(data, header.section_length as usize, start);

        Some(ProgramAssociationTable {
            header,
            transport_stream_id,
            programs,
        })
    }


    fn parse_header(data: &Vec<u8>, start: usize) -> ProgramSpecificInformationHeader {
        let table_id = data[start];
        let section_syntax_indicator = ((data[start + 1] & SECTION_SYNTAX_INDICATOR_MASK) >> 7) == 1;
        // 0b_0100_0000 - 0 bit
        // 0b_0011_0000 - reserved
        // 0b_0000_1100 - section_length unused bits
        let section_length: usize =
            (((data[start + 1] & SECTION_LENGTH_UPPER_MASK) as u16) << 8 | (data[start + 2] & SECTION_LENGTH_LOWER_MASK) as u16).into();

        // 0b_1100_0000 - reserved
        let version_number = (data[start + 5] & VERSION_NUMBER_MASK) >> 1;
        let current_next_indicator = (data[start + 5] & CURRENT_NEXT_INDICATOR_MASK) == 1;
        let section_number = data[start + 6];
        let last_section_number = data[start + 7];
        let crc_start = HEADER_SIZE + section_length - CRC_SIZE + 1;
        let crc_32 = Self::parse_crc32(&data[crc_start..crc_start + 4]);

        ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length: section_length as u16,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            crc_32,
        }
    }

    fn parse_programs(data: &[u8], section_length: usize, start: usize) -> Vec<ProgramAssociationItem> {
        let mut programs: Vec<ProgramAssociationItem> = vec!();

        let mut i: usize = HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE + start;
        while i < HEADER_SIZE + section_length - CRC_SIZE {
            let program_number = ((data[i] as u16) << 8) | data[i + 1] as u16;
            let mut network_pid = 0;
            let mut program_map_pid = 0;
            // 0b_1110_0000 - reserved
            let pid = (((data[i + 2] & PROGRAM_PID_UPPER_MASK) as u16) << 8) | data[i + 3] as u16;

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
            i += PROGRAM_SECTION_SIZE;
        }
        programs
    }

    fn parse_crc32(data: &[u8]) -> u32 {
        let mut crc_32: u32 = 0;
        for i in 0..CRC_SIZE {
            crc_32 |= ((data[i] as u32) << (24 - i * 8));
        }
        crc_32
    }
}