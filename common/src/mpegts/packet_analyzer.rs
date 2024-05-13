use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::mpegts::{MpegtsFragment, PayloadType, header::PIDTable};
use crate::psi::pat::ProgramAssociationTable;

pub struct Analyzer;


impl Analyzer {
    pub fn collect_data(mut raw_data_packet: RawDataPacket,
                        raw_packet: &MpegtsFragment) -> Option<RawDataPacket> {
        if raw_packet.payload.is_none() {
            return None;
        }

        let payload = raw_packet.payload.as_ref().unwrap();

        if raw_data_packet.pid == PIDTable::ProgramAssociation {
            raw_data_packet.data = payload.data.clone();
        } else {
            raw_data_packet.data.extend_from_slice(&payload.data);
        }

        Some(raw_data_packet)
    }

    pub fn analyze(mpeg_ts_payloads: &HashMap<u16, RawDataPacket>) {
        let pat_pid: u16 = PIDTable::ProgramAssociation.into();
        if let Some(pat_payload) = mpeg_ts_payloads.get(&pat_pid) {
            let pat = ProgramAssociationTable::unmarshal(pat_payload.payload_unit_start_indicator, &pat_payload.data);
            println!("{:#?}", pat);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawDataPacket {
    pub payload_unit_start_indicator: bool,
    pub data: Vec<u8>,
    pid: PIDTable,
}


impl RawDataPacket {
    pub fn build(payload_unit_start_indicator: bool, pid: &PIDTable) -> Self {
        Self {
            data: vec!(),
            payload_unit_start_indicator,
            pid: pid.clone(),
        }
    }
}
