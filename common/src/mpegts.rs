pub mod adaptation_field;
pub mod aggregator;
pub mod descriptors;
pub mod header;
pub mod payload;
pub mod pes;
pub mod psi;
#[cfg(test)]
mod tests;

use crate::mpegts::adaptation_field::AdaptationField;
use crate::mpegts::header::Header;
#[cfg(not(target_arch = "wasm32"))]
use crate::mpegts::header::{AdaptationFieldControl, PIDTable, TransportScramblingControl};
use crate::mpegts::payload::RawPayload;
use serde::{Deserialize, Serialize};

pub const FRAGMENT_SIZE: usize = 188;
pub const HEADER_SIZE: usize = 4;
pub const MAX_FRAGMENTS: usize = 7;
pub const SYNC_BYTE: u8 = 0x47;
pub const SYNC_BYTE_MASK: u8 = 0xFF;
pub const TEI_MASK: u8 = 0x80;
pub const PUSI_MASK: u8 = 0x40;
pub const TP_MASK: u8 = 0x20;
pub const PID_MASK_UPPER: u8 = 0x1F;
pub const TSC_MASK: u8 = 0xC0;
pub const AFC_MASK: u8 = 0x30;
pub const CC_MASK: u8 = 0x0F;
pub const PADDING_BYTE: u8 = 0xFF;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MpegtsPacket {
    pub number_of_fragments: usize,
    pub transport_stream_id: u32,
    pub fragments: Vec<MpegtsFragment>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MpegtsFragment {
    pub header: Header,
    pub adaptation_field: Option<AdaptationField>,
    pub payload: Option<RawPayload>,
}

#[cfg(not(target_arch = "wasm32"))]
impl MpegtsPacket {
    pub fn build(packet: &super::Packet) -> Option<Self> {
        let Some(payload) = packet.payload.as_ref() else {
            return None;
        };
        let Some(packet) = Self::unmarshall(payload) else {
            return None;
        };
        Some(packet)
    }

    fn unmarshall(buffer: &Vec<u8>) -> Option<Self> {
        if buffer.len() % FRAGMENT_SIZE != 0 || buffer.len() > FRAGMENT_SIZE * MAX_FRAGMENTS {
            return None;
        }

        let expected_fragments = buffer.len() / FRAGMENT_SIZE;
        let mut fragments: Vec<MpegtsFragment> = Vec::with_capacity(expected_fragments);

        for fragment_index in 0..expected_fragments {
            let start_index = fragment_index * FRAGMENT_SIZE;

            if (buffer[start_index] & SYNC_BYTE_MASK) != SYNC_BYTE {
                return None;
            }

            let Some(fragment) = Self::get_fragment(buffer, start_index, fragment_index) else {
                return None;
            };
            fragments.push(fragment);
        }

        (!fragments.is_empty()).then_some(Self {
            number_of_fragments: fragments.len(),
            fragments,
            transport_stream_id: 0,
        })
    }

    fn get_fragment(
        buffer: &Vec<u8>,
        mut start_index: usize,
        fragment_number: usize,
    ) -> Option<MpegtsFragment> {
        let Some(header) = Self::get_header(buffer, start_index) else {
            return None;
        };
        start_index += HEADER_SIZE;

        let adaptation_field = match header.adaptation_field_control {
            AdaptationFieldControl::AdaptationFieldOnly
            | AdaptationFieldControl::AdaptationFieldAndPaylod => {
                let Some(adaptation_field) = Self::get_adaptation_field(buffer, start_index) else {
                    return None;
                };
                start_index += adaptation_field.adaptation_field_length as usize + 1;
                Some(adaptation_field)
            }
            _ => None,
        };

        let payload = match header.adaptation_field_control {
            AdaptationFieldControl::PayloadOnly
            | AdaptationFieldControl::AdaptationFieldAndPaylod => {
                let Some(payload) = Self::get_payload(buffer, start_index, fragment_number) else {
                    return None;
                };
                Some(payload)
            }
            _ => None,
        };

        Some(MpegtsFragment {
            header,
            adaptation_field,
            payload,
        })
    }

    fn get_header(buffer: &Vec<u8>, start_index: usize) -> Option<Header> {
        let transport_error_indicator = ((buffer[start_index + 1] & TEI_MASK) >> 7) == 1;
        let payload_unit_start_indicator = ((buffer[start_index + 1] & PUSI_MASK) >> 6) == 1;
        let transport_priority = ((buffer[start_index + 1] & TP_MASK) >> 5) == 1;
        let pid = ((buffer[start_index + 1] & PID_MASK_UPPER) as u16) << 8
            | buffer[start_index + 2] as u16;
        let pid: PIDTable = PIDTable::from(pid);

        let transport_scrambling_control = match (buffer[start_index + 3] & TSC_MASK) >> 6 {
            0 => TransportScramblingControl::NotScrambled,
            val => TransportScramblingControl::UserDefined(val),
        };
        let adaptation_field_control = match (buffer[start_index + 3] & AFC_MASK) >> 4 {
            1 => AdaptationFieldControl::PayloadOnly,
            2 => AdaptationFieldControl::AdaptationFieldOnly,
            3 => AdaptationFieldControl::AdaptationFieldAndPaylod,
            _ => return None,
        };
        let continuity_counter = buffer[start_index + 3] & CC_MASK;
        Some(Header {
            transport_error_indicator,
            payload_unit_start_indicator,
            transport_priority,
            pid,
            transport_scrambling_control,
            adaptation_field_control,
            continuity_counter,
        })
    }

    fn get_adaptation_field(buffer: &Vec<u8>, start_index: usize) -> Option<AdaptationField> {
        AdaptationField::unmarshall(&buffer[start_index..])
    }

    fn get_payload(
        buffer: &Vec<u8>,
        start_index: usize,
        fragment_number: usize,
    ) -> Option<RawPayload> {
        let end_index = (fragment_number + 1) * FRAGMENT_SIZE;
        let length = end_index.saturating_sub(start_index);

        if length == 0 {
            return None;
        }

        let data = buffer[start_index..end_index.min(buffer.len())].to_vec();
        Some(RawPayload { data })
    }
}
