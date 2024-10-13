pub mod fragmentary_pmt;
pub mod pmt_buffer;

use serde::{Deserialize, Serialize};
use crate::descriptor::Descriptor;

#[cfg(not(target_arch = "wasm32"))]
pub const HEADER_AFTER_SECTION_LENGTH_SIZE: usize = 9;
#[cfg(not(target_arch = "wasm32"))]
pub const HEADER_SIZE: usize = 3;
#[cfg(not(target_arch = "wasm32"))]
pub const PCR_PID_UPPER_MASK: usize = 0x1F;
#[cfg(not(target_arch = "wasm32"))]
pub const PCR_PID_LOWER_MASK: usize = 0xFF;
#[cfg(not(target_arch = "wasm32"))]
pub const PROGRAM_INFO_LENGTH_UPPER_MASK: usize = 0x0F;
#[cfg(not(target_arch = "wasm32"))]
pub const PROGRAM_INFO_LENGTH_LOWER_MASK: usize = 0xFF;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ProgramMapTable {
    pub program_number: u16,
    pub pcr_pid: u16,
    pub program_info_length: u16,
    pub descriptors: Vec<Descriptor>,
    pub elementary_streams_info: Vec<ElementaryStreamInfo>,
    pub crc_32: u32,
}

impl PartialEq for ProgramMapTable {
    fn eq(&self, other: &Self) -> bool {
        let program_number = self.program_number == other.program_number;
        let pcr_pid = self.pcr_pid == other.pcr_pid;
        let program_info_length = self.program_info_length == other.program_info_length;
        let descriptors = self.descriptors == other.descriptors;
        let elementary_streams_info = self.elementary_streams_info == other.elementary_streams_info;
        let crc_32 = self.crc_32 == other.crc_32;

        program_number && pcr_pid && program_info_length && descriptors && elementary_streams_info && crc_32
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct ElementaryStreamInfo {
    pub stream_type: u8, // table is defined on page 55 of H.222.0 (03/2017)
    pub elementary_pid: u16,
    pub es_info_length: u16,
    pub descriptors: Vec<Descriptor>,
}

impl PartialEq for ElementaryStreamInfo {
    fn eq(&self, other: &Self) -> bool {
        let stream_type = self.stream_type == other.stream_type;
        let elementary_pid = self.elementary_pid == other.elementary_pid;
        let es_info_length = self.es_info_length == other.es_info_length;
        let descriptors = self.descriptors == other.descriptors;

        stream_type && elementary_pid && es_info_length && descriptors
    }
}
