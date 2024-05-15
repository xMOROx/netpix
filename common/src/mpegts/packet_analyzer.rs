use std::collections::HashMap;
use std::hash::Hash;
use log::info;
use serde::{Deserialize, Serialize};
use crate::mpegts;
use crate::mpegts::{MpegtsFragment, PayloadType, header::PIDTable};
use crate::packet::SessionProtocol::Mpegts;
use crate::psi::pat::{ProgramAssociationTable, MAX_NUMBER_OF_SECTIONS};

pub struct Analyzer;


impl Analyzer {
    pub fn analyze(mpeg_ts_payloads: &mut HashMap<u16, Vec<MpegtsFragment>>) {
        let mut pat_packets: HashMap<u8, ProgramAssociationTable> = HashMap::new();

        let pat_pid: u16 = PIDTable::ProgramAssociation.into();

        if let Some(pat_payloads) = mpeg_ts_payloads.get(&pat_pid) {
            for pat_payload in pat_payloads {
                let pat = ProgramAssociationTable::unmarshal(pat_payload);
                pat_packets.insert(pat.clone().unwrap().header.section_number, pat.unwrap());
            }
        }
    }
}




