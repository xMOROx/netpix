use serde::{Deserialize, Serialize};
use crate::mpegts::{MpegtsPacket, MpegtsFragment, PayloadType, header::PIDTable};
use crate::psi::{PsiTypes, pat::ProgramAssociationTable};
use crate::pes::PesPacketHeader;

pub struct Analyzer;

impl Analyzer {
    pub fn collect_data(mut raw_data_packet: RawDataPacket,
                        raw_packet: &MpegtsFragment) -> Option<RawDataPacket> {
        if raw_packet.payload.is_none() {
            return None;
        }

        let payload = raw_packet.payload.as_ref().unwrap();


        raw_data_packet.data.extend_from_slice(&payload.data);

        Some(raw_data_packet)
    }

    pub fn analyze(mut raw_data_packet: RawDataPacket) -> PayloadType {
        todo!("Implement the logic to analyze the payload")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawDataPacket {
    pub data: Vec<u8>,
    pid: PIDTable,
}


impl RawDataPacket {
    pub fn build(pid: &PIDTable) -> Self {
        Self {
            data: vec!(),
            pid: pid.clone(),
        }
    }
}
