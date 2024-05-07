pub mod header;
pub mod adaptation_field;
pub mod payload;
pub mod packet_analyzer;

use serde::{Deserialize, Serialize};
use crate::mpegts::adaptation_field::AdaptationField;
use crate::mpegts::header::Header;
use crate::mpegts::payload::{PayloadType, RawPayload};
#[cfg(not(target_arch = "wasm32"))]
use crate::mpegts::header::{PIDTable, TransportScramblingControl, AdaptationFieldControl};
use crate::pes::PesPacketHeader;
use crate::psi::PsiTypes;

#[cfg(not(target_arch = "wasm32"))]
const PAYLOAD_LENGTH: usize = 1316;
#[cfg(not(target_arch = "wasm32"))]
const FRAGMENT_SIZE: usize = 188;
#[cfg(not(target_arch = "wasm32"))]
const HEADER_SIZE: usize = 4;
#[cfg(not(target_arch = "wasm32"))]
const SYNC_BYTE: u8 = 0x47;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MpegtsPacket {
    pub number_of_fragments: usize,
    pub fragments: Vec<MpegtsFragment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        if buffer.len() != PAYLOAD_LENGTH {
            return None;
        }
        let mut number_of_fragments: usize = 0;
        let mut fragments: Vec<MpegtsFragment> = vec!();
        // sync byte should be 0x47
        while (number_of_fragments * FRAGMENT_SIZE) < PAYLOAD_LENGTH && buffer[number_of_fragments * FRAGMENT_SIZE] == SYNC_BYTE {
            let Some(fragment) = Self::get_frgment(buffer, number_of_fragments * FRAGMENT_SIZE) else {
                break;
            };
            fragments.push(fragment);
            number_of_fragments += 1;
        }
        match fragments.len() {
            0 => None,
            _ => Some(Self { number_of_fragments, fragments })
        }
    }

    fn get_frgment(buffer: &Vec<u8>, mut start_index: usize) -> Option<MpegtsFragment> {
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
                // I'm 99% sure that there should be + 1 but if something doesn't work with payload try removing it xD
                start_index += adaptation_field.adaptation_field_length as usize + 1;
                Some(adaptation_field)
            }
            _ => None
        };

        let payload = match header.adaptation_field_control {
            AdaptationFieldControl::PayloadOnly
            | AdaptationFieldControl::AdaptationFieldAndPaylod => {
                let Some(payload) = Self::get_payload(buffer, start_index) else {
                    return None;
                };
                Some(payload)
            }
            _ => None
        };
        Some(MpegtsFragment { header, adaptation_field, payload })
    }

    fn get_header(buffer: &Vec<u8>, start_index: usize) -> Option<Header> {
        let transport_error_indicator = buffer[start_index + 1] & 128 == 1;
        let payload_unit_start_indicator = buffer[start_index + 1] & 64 == 1;
        let transport_priority = buffer[start_index + 1] & 32 == 1;
        let pid = (((buffer[start_index + 1] & 31) as u16) << 8) + buffer[start_index + 2] as u16;
        let pid: PIDTable = PIDTable::from(pid);

        let transport_scrambling_control = match (buffer[start_index + 3] & 192) >> 6 {
            0 => TransportScramblingControl::NotScrambled,
            val => TransportScramblingControl::UserDefined(val),
        };
        let adaptation_field_control = match (buffer[start_index + 3] & 48) >> 4 {
            1 => AdaptationFieldControl::PayloadOnly,
            2 => AdaptationFieldControl::AdaptationFieldOnly,
            3 => AdaptationFieldControl::AdaptationFieldAndPaylod,
            _ => return None
        };
        let continuity_counter = buffer[start_index + 3] & 15;
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
        let adaptation_field_length = buffer[start_index];
        Some(AdaptationField { adaptation_field_length })
    }

    fn get_payload(buffer: &Vec<u8>, start_index: usize) -> Option<RawPayload> {
        let length = if start_index + FRAGMENT_SIZE < buffer.len() { FRAGMENT_SIZE } else { buffer.len() - start_index };
        let data = buffer[start_index..start_index + length].to_vec();
        Some(RawPayload { data })
    }
}