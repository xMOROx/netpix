use serde::{Deserialize, Serialize};
use crate::mpegts::MpegtsFragment;
use crate::psi::ProgramSpecificInformationHeader;

const STUFFED_BYTE: u8 = 0xFF;
const HEADER_SIZE: usize = 8;
const MAX_SECTION_LENGTH: usize = 1024;
pub const MAX_NUMBER_OF_SECTIONS: usize = 255;

pub const MAX_SIZE_OF_PAT: usize = HEADER_SIZE + MAX_SECTION_LENGTH + 4;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramAssociationTableWithRawData {
    pub header: ProgramSpecificInformationHeader,
    pub transport_stream_id: u16,
    pub raw_data: Vec<u8>,
}

impl ProgramAssociationTable {
    pub fn unmarshal(packet: &MpegtsFragment) -> Option<ProgramAssociationTableWithRawData> {
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

        let start = 0;

        let header = Self::parse_header(data, start);

        let transport_stream_id = (data[start + 3] as u16) << 8 | data[start + 4] as u16;

        Some(
            ProgramAssociationTableWithRawData {
                header,
                transport_stream_id,
                raw_data: data[HEADER_SIZE..].to_vec(),
            }
        )
    }

    pub fn unmarshal_collected(packet: &mut ProgramAssociationTableWithRawData) -> ProgramAssociationTable {
        let programs = Self::parse_programs(&packet.raw_data, packet.header.section_length as usize);
        let start_idx = (packet.header.section_length + HEADER_SIZE as u16) as usize;
        let crc_32 = Self::parse_crc32(&packet.raw_data[start_idx..start_idx + 4]);

        packet.header.crc_32 = crc_32;

        ProgramAssociationTable {
            header: packet.header.clone(),
            transport_stream_id: packet.transport_stream_id,
            programs,
        }
    }

    fn parse_header(data: &[u8], start: usize) -> ProgramSpecificInformationHeader {
        let table_id = data[start];
        let section_syntax_indicator = (data[start + 1] & 0x80) != 0;
        // 0b_0100_0000 - 0 bit
        // 0b_0011_0000 - reserved
        // 0b_0000_1100 - section_length unused bits
        let section_length: usize = (((data[start + 1] & 0x03) as u16) << 8 | (data[start + 2] & 0xFF) as u16).into();

        // 0b_1100_0000 - reserved
        let version_number = data[start + 5] & 0x3E >> 1;
        let current_next_indicator = (data[start + 5] & 0x01) != 0;
        let section_number = data[start + 6];
        let last_section_number = data[start + 7];


        ProgramSpecificInformationHeader {
            table_id,
            section_syntax_indicator,
            section_length: section_length as u16,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
            crc_32: 0,
        }
    }

    fn parse_programs(data: &[u8], section_length: usize) -> Vec<ProgramAssociationItem> {
        let mut programs: Vec<ProgramAssociationItem> = vec!();

        let mut i: usize = HEADER_SIZE;
        while i < section_length - 4 {
            if Self::check_if_packed_has_stuffing(&data[i..i + 4]) {
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
        programs
    }

    fn parse_crc32(data: &[u8]) -> u32 {
        let mut crc_32: u32 = 0;
        for i in 0..4 {
            crc_32 |= (data[i] as u32) << (24 - i * 8);
        }
        crc_32
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