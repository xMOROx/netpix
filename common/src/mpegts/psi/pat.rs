pub mod fragmentary_pat;
mod pat_buffer;

use serde::{Deserialize, Serialize};
use crate::mpegts::psi::ProgramSpecificInformationHeader;
use crate::MpegtsPacket;

const HEADER_SIZE: usize = 3;
const HEADER_AFTER_SECTION_LENGTH_SIZE: usize = 5;
const CRC_SIZE: usize = 4;
const PROGRAM_SECTION_SIZE: usize = 4;

const SECTION_SYNTAX_INDICATOR_MASK: u8 = 0x80;
const SECTION_LENGTH_UPPER_MASK: u8 = 0x0F;
const SECTION_LENGTH_LOWER_MASK: u8 = 0xFF;
const VERSION_NUMBER_MASK: u8 = 0x3E;
const CURRENT_NEXT_INDICATOR_MASK: u8 = 0x01;
const PROGRAM_PID_UPPER_MASK: u8 = 0x1F;
const PADDING_BYTE: u8 = 0xFF;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramAssociationTable {
    pub transport_stream_id: u16,
    pub programs: Vec<ProgramAssociationItem>,
    pub crc_32: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramAssociationItem {
    pub program_number: u16,
    pub network_pid: Option<u16>,
    pub program_map_pid: Option<u16>,
}


impl ProgramAssociationTable {
    pub fn build(transport_stream_id: u16, data: &[u8]) -> Option<Self> {
        Some(ProgramAssociationTable {
            transport_stream_id,
            programs: ProgramAssociationTable::unmarshal_programs(data),
            crc_32: ProgramAssociationTable::unmarshal_crc_32(data),
        })
    }

    fn unmarshal_programs(data: &[u8]) -> Vec<ProgramAssociationItem> {
        let mut programs = Vec::new();
        let mut index = 0;
        while index < data.len() {
            let program_number = ((data[index] as u16) << 8) | data[index + 1] as u16;
            if program_number == 0 {
                // 0xrrrnnnnn nnnnnnnn; r = reserved, n = network_pid
                let network_pid = ((data[index + 2] & PROGRAM_PID_UPPER_MASK) as u16) << 8 | data[index + 3] as u16;
                programs.push(ProgramAssociationItem {
                    program_number,
                    network_pid: Some(network_pid),
                    program_map_pid: None,
                });
                index += PROGRAM_SECTION_SIZE;
                continue;
            }
            // 0xrrrppppp pppppppp; r - reserved, p = program_map_pid

            let program_map_pid = (((data[index + 2] & PROGRAM_PID_UPPER_MASK) as u16) << 8) | data[index + 3] as u16;

            programs.push(ProgramAssociationItem {
                program_number,
                network_pid: None,
                program_map_pid: Some(program_map_pid),
            });
            index += PROGRAM_SECTION_SIZE;
        }
        programs
    }

    fn unmarshal_crc_32(data: &[u8]) -> u32 {
        let crc_32 = ((data[data.len() - 4] as u32) << 24) | ((data[data.len() - 3] as u32) << 16) | ((data[data.len() - 2] as u32) << 8) | data[data.len() - 1] as u32;
        crc_32
    }
}


