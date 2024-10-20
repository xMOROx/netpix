use std::collections::HashMap;

use crate::mpegts::MpegtsFragment;


pub struct PesPacketPayload {
    pub data: Vec<u8>,
    pub is_complete: bool,
}

impl PesPacketPayload {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            is_complete: false,
        }
    }

    pub fn append(&mut self, payload: &[u8]) {
        self.data.extend_from_slice(payload);
    }

}
pub struct PesBuffer {
    packets: HashMap<u16, PesPacketPayload>
}

impl PesBuffer {
    pub fn new() -> Self {
        Self {
            packets: HashMap::new(),
        }
    }

    pub fn add_fragment(&mut self, packet: &MpegtsFragment) {
        let pid = packet.clone().header.pid.into();
        if packet.header.payload_unit_start_indicator {
            let new_pes = PesPacketPayload::new();
            self.packets.insert(pid, new_pes);
        }
        // naive append, probably needs to be sorted by continuity counter
        if let Some(pes) = self.packets.get_mut(&pid) {
            pes.append(&packet.payload.as_ref().unwrap().data);
        } else {
            println!("Warning: packet with pid {} not found", pid);
        }
    }

    pub fn remove_payload(&mut self, pid: u16) {
        self.packets.remove(&pid);
    }
}