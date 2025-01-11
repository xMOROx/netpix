#[cfg(test)]
mod tests;

use super::{PacketizedElementaryStream, REQUIRED_FIELDS_SIZE};
use crate::mpegts::MpegtsFragment;
use crate::utils::BufferOperations;
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, Default)]
pub struct PesPacketPayload {
    data: Vec<u8>,
    is_completable: bool,
    packet_length: u16,
}

impl PesPacketPayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_packet_length(&mut self, length: u16) {
        self.packet_length = length;
    }

    pub fn set_completable(&mut self, completable: bool) {
        self.is_completable = completable;
    }

    pub fn get_packet_length(&self) -> u16 {
        self.packet_length
    }

    pub fn is_completable(&self) -> bool {
        self.is_completable
    }
}

impl BufferOperations for PesPacketPayload {
    type Item = u8;

    fn clear(&mut self) {
        self.data.clear();
        self.is_completable = false;
        self.packet_length = 0;
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn is_complete(&self) -> bool {
        self.is_completable
            && (self.data.len() - REQUIRED_FIELDS_SIZE) == self.packet_length as usize
    }

    fn append(&mut self, payload: &[u8]) {
        self.data.extend_from_slice(payload);
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Decode, Encode, Debug, Clone, Default)]
pub struct PesBuffer {
    payload: PesPacketPayload,
}

impl PesBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_fragment(&mut self, packet: &MpegtsFragment) {
        let Some(payload) = &packet.payload else {
            return;
        };

        if packet.header.payload_unit_start_indicator {
            if let Some(fields) =
                PacketizedElementaryStream::unmarshall_required_fields(&payload.data[..])
            {
                let mut pes_packet = PesPacketPayload::new();
                pes_packet.set_packet_length(fields.pes_packet_length);
                pes_packet.append(&payload.data[..]);
                pes_packet.set_completable(fields.pes_packet_length != 0);
                self.payload = pes_packet;
                return;
            }
        }

        self.payload.append(&payload.data[..]);
    }

    pub fn build(&mut self) -> Option<PacketizedElementaryStream> {
        if !self.payload.is_complete() {
            return None;
        }

        PacketizedElementaryStream::build(self.payload.get_data())
    }
}

impl BufferOperations for PesBuffer {
    type Item = PesPacketPayload;

    fn clear(&mut self) {
        self.payload.clear();
    }

    fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }

    fn is_complete(&self) -> bool {
        self.payload.is_complete()
    }

    fn append(&mut self, data: &[u8]) {
        self.payload.append(data);
    }

    fn get_data(&self) -> &[u8] {
        self.payload.get_data()
    }
}
