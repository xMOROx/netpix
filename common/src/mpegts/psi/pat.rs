pub mod constants;
pub mod fragmentary_pat;
pub mod pat_buffer;

use crate::utils::{BitReader, Crc32Reader};
use constants::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ProgramAssociationTable {
    pub transport_stream_id: u16,
    pub programs: Vec<ProgramAssociationItem>,
    pub crc_32: u32,
    pub fragment_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ProgramAssociationItem {
    pub program_number: u16,
    pub network_pid: Option<u16>,
    pub program_map_pid: Option<u16>,
}

impl PartialEq for ProgramAssociationTable {
    fn eq(&self, other: &Self) -> bool {
        let transport_stream_id = self.transport_stream_id == other.transport_stream_id;
        let programs = self.programs == other.programs;
        let crc_32 = self.crc_32 == other.crc_32;

        transport_stream_id && programs && crc_32
    }
}

impl PartialEq for ProgramAssociationItem {
    fn eq(&self, other: &Self) -> bool {
        let program_numbers = self.program_number == other.program_number;
        let network_pids = match (self.network_pid, other.network_pid) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };
        let program_map_pids = match (self.program_map_pid, other.program_map_pid) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        };

        program_numbers && network_pids && program_map_pids
    }
}

impl ProgramAssociationTable {
    pub fn build(transport_stream_id: u16, data: &[u8], fragment_count: usize) -> Option<Self> {
        let crc_reader = Crc32Reader::new(data);

        Some(ProgramAssociationTable {
            transport_stream_id,
            programs: Self::unmarshal_programs(crc_reader.data_without_crc())?,
            crc_32: crc_reader.read_crc32()?,
            fragment_count,
        })
    }

    fn unmarshal_programs(data: &[u8]) -> Option<Vec<ProgramAssociationItem>> {
        let reader = BitReader::new(data);
        let mut programs = Vec::new();

        let mut offset = 0;
        while offset < data.len() {
            let (program_number, pid) =
                reader.read_program_entry(offset, PROGRAM_PID_UPPER_MASK)?;

            programs.push(ProgramAssociationItem {
                program_number,
                network_pid: (program_number == 0).then_some(pid),
                program_map_pid: (program_number != 0).then_some(pid),
            });

            offset += PROGRAM_SECTION_SIZE;
        }

        Some(programs)
    }
}
