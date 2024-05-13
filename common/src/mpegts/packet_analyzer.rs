use std::collections::HashMap;
use std::hash::Hash;
use log::info;
use serde::{Deserialize, Serialize};
use crate::mpegts;
use crate::mpegts::{MpegtsFragment, PayloadType, header::PIDTable};
use crate::packet::SessionProtocol::Mpegts;
use crate::psi::pat::{ProgramAssociationTable, MAX_NUMBER_OF_SECTIONS, MAX_SIZE_OF_PAT, ProgramAssociationTableWithRawData};

pub struct Analyzer;


impl Analyzer {
    pub fn analyze(mpeg_ts_payloads: &mut HashMap<u16, Vec<MpegtsFragment>>) {
        let mut pat_packets: HashMap<u8, ProgramAssociationTableWithRawData> = HashMap::new();

        let pat_pid: u16 = PIDTable::ProgramAssociation.into();

        if let Some(pat_payloads) = mpeg_ts_payloads.get(&pat_pid) {
            for pat_payload in pat_payloads {
                let pat = ProgramAssociationTable::unmarshal(pat_payload);

                if let Some(raw_pat) = pat {
                    let mut_pat = pat_packets.entry(raw_pat.header.section_number).or_insert(raw_pat);

                    mut_pat.raw_data.extend(pat_payload.payload.as_ref().unwrap().data.clone());
                }
            }
        }

        for (key, mut value) in pat_packets {
            if value.header.section_length <= value.raw_data.len() as u16 && value.raw_data.len() >= mpegts::FRAGMENT_SIZE {
                let pat = ProgramAssociationTable::unmarshal_collected(&mut value);
            }
        }
    }
}




