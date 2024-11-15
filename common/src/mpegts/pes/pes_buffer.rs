use log::warn;
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
    packets: HashMap<u16, PesPacketPayload>,
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
            warn!("Warning: packet with pid {} not found", pid);
        }
    }

    pub fn remove_payload(&mut self, pid: u16) {
        self.packets.remove(&pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::payload::RawPayload;
    use crate::mpegts::{
        AdaptationFieldControl, Header, MpegtsFragment, PIDTable, TransportScramblingControl,
    };

    fn create_test_fragment(
        pid: u16,
        payload_unit_start_indicator: bool,
        payload: Vec<u8>,
    ) -> MpegtsFragment {
        MpegtsFragment {
            header: Header {
                transport_error_indicator: false,
                payload_unit_start_indicator,
                transport_priority: false,
                pid: PIDTable::from(pid),
                transport_scrambling_control: TransportScramblingControl::NotScrambled,
                adaptation_field_control: AdaptationFieldControl::PayloadOnly,
                continuity_counter: 0,
            },
            adaptation_field: None,
            payload: Some(RawPayload { data: payload }),
        }
    }

    #[test]
    fn test_pes_packet_payload_initialization() {
        let payload = PesPacketPayload::new();
        assert!(payload.data.is_empty());
        assert!(!payload.is_complete);
    }

    #[test]
    fn test_pes_packet_payload_append() {
        let mut payload = PesPacketPayload::new();
        payload.append(&[1, 2, 3]);
        assert_eq!(payload.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_pes_buffer_initialization() {
        let buffer = PesBuffer::new();
        assert!(buffer.packets.is_empty());
    }

    #[test]
    fn test_pes_buffer_add_fragment_new_packet() {
        let mut buffer = PesBuffer::new();
        let fragment = create_test_fragment(0x100, true, vec![1, 2, 3]);
        buffer.add_fragment(&fragment);
        assert!(buffer.packets.contains_key(&0x100));
        assert_eq!(buffer.packets.get(&0x100).unwrap().data, vec![1, 2, 3]);
    }

    #[test]
    fn test_pes_buffer_add_fragment_existing_packet() {
        let mut buffer = PesBuffer::new();
        let fragment1 = create_test_fragment(0x100, true, vec![1, 2, 3]);
        let fragment2 = create_test_fragment(0x100, false, vec![4, 5, 6]);
        buffer.add_fragment(&fragment1);
        buffer.add_fragment(&fragment2);
        assert_eq!(
            buffer.packets.get(&0x100).unwrap().data,
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_pes_buffer_remove_payload() {
        let mut buffer = PesBuffer::new();
        let fragment = create_test_fragment(0x100, true, vec![1, 2, 3]);
        buffer.add_fragment(&fragment);
        buffer.remove_payload(0x100);
        assert!(!buffer.packets.contains_key(&0x100));
    }

    #[test]
    fn test_pes_buffer_add_fragment_no_payload() {
        let mut buffer = PesBuffer::new();
        let fragment = create_test_fragment(0x100, true, vec![]);
        buffer.add_fragment(&fragment);
        assert!(buffer.packets.contains_key(&0x100));
        assert!(buffer.packets.get(&0x100).unwrap().data.is_empty());
    }

    #[test]
    fn test_pes_buffer_add_fragment_same_pid() {
        let mut buffer = PesBuffer::new();
        let fragment1 = create_test_fragment(0x100, true, vec![1, 2, 3]);
        let fragment2 = create_test_fragment(0x100, true, vec![4, 5, 6]);
        buffer.add_fragment(&fragment1);
        buffer.add_fragment(&fragment2);
        assert_eq!(buffer.packets.get(&0x100).unwrap().data, vec![4, 5, 6]);
    }
}
