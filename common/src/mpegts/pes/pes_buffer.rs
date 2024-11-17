#[cfg(test)]
mod tests;

use super::PacketizedElementaryStream;
use crate::mpegts::MpegtsFragment;

pub struct PesPacketPayload {
    pub data: Vec<u8>,
    pub is_completable: bool,
    pub packet_length: u16,
}

impl PesPacketPayload {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            is_completable: false,
            packet_length: 0,
        }
    }

    pub fn set_packet_length(&mut self, length: u16) {
        self.packet_length = length;
    }

    pub fn get_packet_length(&self) -> u16 {
        self.packet_length
    }

    pub fn is_completable(&self) -> bool {
        self.is_completable
    }

    pub fn set_completable(&mut self, completable: bool) {
        self.is_completable = completable;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_complete(&self) -> bool {
        self.is_completable && self.data.len() == self.packet_length as usize
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.is_completable = false;
        self.packet_length = 0;
    }

    pub fn append(&mut self, payload: &[u8]) {
        self.data.extend_from_slice(payload);
    }
}
pub struct PesBuffer {
    payload: PesPacketPayload,
}

impl PesBuffer {
    pub fn new() -> Self {
        Self {
            payload: PesPacketPayload::new(),
        }
    }

    pub fn add_fragment(&mut self, packet: &MpegtsFragment) {
        let payload = match packet.payload.as_ref() {
            Some(data) => data,
            None => return,
        };

        let mut required_fields = None;

        if packet.header.payload_unit_start_indicator {
            required_fields =
                PacketizedElementaryStream::unmarshall_required_fields(&payload.data[..]);
        }

        if let Some(fields) = required_fields {
            let mut pes_packet = PesPacketPayload::new();
            let packet_length = fields.pes_packet_length;
            pes_packet.set_packet_length(packet_length);
            pes_packet.append(&payload.data[..]);
            pes_packet.set_completable(packet_length != 0);
            self.payload = pes_packet;
        } else {
            self.payload.append(&payload.data[..]);
        }
    }

    pub fn build(&mut self) -> Option<PacketizedElementaryStream> {
        if !self.payload.is_completable() {
            return None;
        }

        let data = self.payload.get_data();
        let pes = PacketizedElementaryStream::build(data);
        pes
    }
}
