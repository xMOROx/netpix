pub mod constants;
pub mod enums;
pub mod header;
pub mod optional_fields;
pub mod pes_buffer;
pub mod trick_mode_control;

use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};
use constants::*;
use enums::StreamType;
use header::PesHeader;
use std::cmp::PartialEq;

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct PacketizedElementaryStream {
    pub required_fields: RequiredFields,
    pub header: Option<PesHeader>,
    pub packet_data: Option<Vec<u8>>,
    pub padding_bytes: Option<Vec<u8>>,
}

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct RequiredFields {
    pub packet_start_code_prefix: u32,
    pub stream_id: u8,
    pub pes_packet_length: u16,
}

impl PacketizedElementaryStream {
    pub fn build(data: &[u8]) -> Option<Self> {
        if data.len() < REQUIRED_FIELDS_SIZE {
            return None;
        }
        Self::unmarshall(data)
    }

    pub fn unmarshall_required_fields(data: &[u8]) -> Option<RequiredFields> {
        let reader = BitReader::new(data);

        let packet_start_code_prefix = reader.get_bits_u24(0)?;
        if packet_start_code_prefix != PACKET_START_CODE_PREFIX {
            return None;
        }

        let stream_id = *reader.get_bytes(3, 1)?.first()?;
        let pes_packet_length = reader.get_bits_u16(4, 0xFF, 0xFF)?;

        Some(RequiredFields {
            packet_start_code_prefix,
            stream_id,
            pes_packet_length,
        })
    }

    fn unmarshall(data: &[u8]) -> Option<Self> {
        let required_fields = Self::unmarshall_required_fields(data)?;
        let reader = BitReader::new(data);

        let (header, packet_data, padding_bytes) = match StreamType::from(required_fields.stream_id)
        {
            StreamType::PaddingStream => (None, None, reader.remaining_from(REQUIRED_FIELDS_SIZE)),
            StreamType::ProgramStreamMap
            | StreamType::PrivateStream2
            | StreamType::ECMStream
            | StreamType::EMMStream
            | StreamType::ProgramStreamDirectory
            | StreamType::DSMCCStream
            | StreamType::H2221TypeE => (None, reader.remaining_from(REQUIRED_FIELDS_SIZE), None),
            _ => {
                let header = PesHeader::build(&data[REQUIRED_FIELDS_SIZE..])?;
                let header_size = header.size;
                let data_start = REQUIRED_FIELDS_SIZE
                    + header_size
                    + Self::number_of_stuffing_bytes(&data[REQUIRED_FIELDS_SIZE + header_size..]);

                (Some(header), reader.remaining_from(data_start), None)
            }
        };

        Some(Self {
            required_fields,
            header,
            packet_data,
            padding_bytes,
        })
    }

    fn number_of_stuffing_bytes(data: &[u8]) -> usize {
        data.iter()
            .take(MAXIMUM_NO_OF_STUFFING_BYTES)
            .take_while(|&&byte| byte == STUFFING_BYTE)
            .count()
    }
}
