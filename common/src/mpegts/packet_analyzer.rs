use crate::mpegts::{MpegtsPacket, MpegtsFragment, PayloadType, header::PIDTable};
use crate::psi::PsiTypes;
use crate::pes::PesPacketHeader;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(incomplete_data: &mut IncompleteData,
                   packet: &MpegtsPacket) -> Option<PayloadType> {
        match incomplete_data.payload_type {
            PayloadType::PSI(table) => {
                match table {
                    PsiTypes::PAT => {
                        todo!("Implement the logic to analyze the payload")
                    }
                    PsiTypes::PMT => {
                        todo!("Implement the logic to analyze the payload")
                    }
                }
            }
            PayloadType::PES(packet) => {
                todo!("Implement the logic to analyze the payload")
            }
        }


        if incomplete_data.current_number_of_bytes < packet.payload.len() {
            None
        } else {
            todo!("Implement the logic to analyze the payload");
        }
    }
}

pub struct IncompleteData {
    data: Vec<u8>,
    payload_type: PayloadType,
    pid: PIDTable,
    number_of_analyzed_bytes: usize,
    current_number_of_bytes: usize,
}

impl IncompleteData {
    pub fn build(pid: PIDTable, payload_type: PayloadType) -> Self {
        Self {
            data: vec!(),
            payload_type,
            pid,
            number_of_analyzed_bytes: 0,
            current_number_of_bytes: 0,
        }
    }
}