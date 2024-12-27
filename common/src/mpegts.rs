#![allow(unused_imports)]
pub mod adaptation_field;
pub mod aggregator;
pub mod constants;
pub mod descriptors;
pub mod header;
pub mod payload;
pub mod pes;
pub mod psi;
#[cfg(test)]
mod tests;

use crate::mpegts::adaptation_field::AdaptationField;
use crate::mpegts::header::Header;
use crate::mpegts::header::{AdaptationFieldControl, PIDTable, TransportScramblingControl};
use crate::mpegts::payload::RawPayload;
use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};
use constants::*;

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct MpegtsPacket {
    pub number_of_fragments: usize,
    pub fragments: Vec<MpegtsFragment>,
}

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct MpegtsFragment {
    pub header: Header,
    pub adaptation_field: Option<AdaptationField>,
    pub payload: Option<RawPayload>,
    pub size: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl MpegtsPacket {
    pub fn build(packet: &super::Packet) -> Option<Self> {
        packet
            .payload
            .as_ref()
            .and_then(|payload| Self::unmarshall(payload))
    }

    fn unmarshall(buffer: &[u8]) -> Option<Self> {
        if buffer.len() % FRAGMENT_SIZE != 0 || buffer.len() > FRAGMENT_SIZE * MAX_FRAGMENTS {
            return None;
        }

        let fragments: Vec<_> = (0..buffer.len())
            .step_by(FRAGMENT_SIZE)
            .filter_map(|start_index| {
                ((buffer[start_index] & SYNC_BYTE_MASK) == SYNC_BYTE)
                    .then(|| Self::get_fragment(buffer, start_index, start_index / FRAGMENT_SIZE))
                    .flatten()
            })
            .collect();

        (!fragments.is_empty()).then_some(Self {
            number_of_fragments: fragments.len(),
            fragments,
        })
    }

    fn get_fragment(
        buffer: &[u8],
        start_index: usize,
        fragment_number: usize,
    ) -> Option<MpegtsFragment> {
        let header = Self::get_header(buffer, start_index)?;
        let current_index = start_index + HEADER_SIZE;

        let (adaptation_field, payload_start) =
            Self::process_adaptation_field(&header, buffer, current_index)?;

        let payload = Self::process_payload(&header, buffer, payload_start, fragment_number);

        Some(MpegtsFragment {
            header,
            adaptation_field: adaptation_field.clone(),
            payload: payload.clone(),
            size: Self::calculate_fragment_size(&adaptation_field, &payload),
        })
    }

    fn process_adaptation_field(
        header: &Header,
        buffer: &[u8],
        start_index: usize,
    ) -> Option<(Option<AdaptationField>, usize)> {
        match header.adaptation_field_control {
            AdaptationFieldControl::AdaptationFieldOnly
            | AdaptationFieldControl::AdaptationFieldAndPayload => {
                let field = AdaptationField::unmarshall(&buffer[start_index..])?;
                let next_index = start_index + field.adaptation_field_length as usize + 1;
                Some((Some(field), next_index))
            }
            _ => Some((None, start_index)),
        }
    }

    fn process_payload(
        header: &Header,
        buffer: &[u8],
        start_index: usize,
        fragment_number: usize,
    ) -> Option<RawPayload> {
        match header.adaptation_field_control {
            AdaptationFieldControl::PayloadOnly
            | AdaptationFieldControl::AdaptationFieldAndPayload => {
                Self::get_payload(buffer, start_index, fragment_number)
            }
            _ => None,
        }
    }

    fn get_header(buffer: &[u8], start_index: usize) -> Option<Header> {
        let reader = BitReader::at_position(buffer, start_index);
        Some(Header {
            transport_error_indicator: reader.get_bit(1, 7)?,
            payload_unit_start_indicator: reader.get_bit(1, 6)?,
            transport_priority: reader.get_bit(1, 5)?,
            pid: PIDTable::from(reader.get_bits_u16(1, PID_MASK_UPPER, 0xFF)?),
            transport_scrambling_control: match reader.get_bits(3, TSC_MASK, 6)? {
                0 => TransportScramblingControl::NotScrambled,
                val => TransportScramblingControl::UserDefined(val),
            },
            adaptation_field_control: match reader.get_bits(3, AFC_MASK, 4)? {
                1 => AdaptationFieldControl::PayloadOnly,
                2 => AdaptationFieldControl::AdaptationFieldOnly,
                3 => AdaptationFieldControl::AdaptationFieldAndPayload,
                _ => return None,
            },
            continuity_counter: reader.get_bits(3, CC_MASK, 0)?,
        })
    }

    fn calculate_fragment_size(
        adaptation_field: &Option<AdaptationField>,
        payload: &Option<RawPayload>,
    ) -> usize {
        HEADER_SIZE
            + adaptation_field
                .as_ref()
                .map_or(0, |af| af.adaptation_field_length as usize + 1)
            + payload.as_ref().map_or(0, |p| p.data.len())
    }

    fn get_payload(
        buffer: &[u8],
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
