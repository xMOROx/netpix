pub mod constants;
pub mod enums;
pub mod header;
pub mod optional_fields;
pub mod pes_buffer;
pub mod trick_mode_control;

use constants::*;
use enums::StreamType;
use header::PesHeader;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PacketizedElementaryStream {
    pub required_fields: RequiredFields,
    pub header: Option<PesHeader>,
    pub packet_data: Option<Vec<u8>>,
    pub padding_bytes: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RequiredFields {
    pub packet_start_code_prefix: u32,
    pub stream_id: u8,
    pub pes_packet_length: u16,
}

impl PacketizedElementaryStream {
    pub fn build(data: &[u8]) -> Option<Self> {
        if data.is_empty() || data.len() < REQUIRED_FIELDS_SIZE {
            return None;
        }

        let Some(pes) = Self::unmarshall(data) else {
            return None;
        };

        Some(pes)
    }

    pub fn unmarshall_required_fields(data: &[u8]) -> Option<RequiredFields> {
        if data.len() < REQUIRED_FIELDS_SIZE {
            return None;
        }

        let packet_start_code_prefix: u32 =
            (data[0] as u32) << 16 | (data[1] as u32) << 8 | data[2] as u32;
        if packet_start_code_prefix != PACKET_START_CODE_PREFIX {
            return None;
        }

        let stream_id: u8 = data[3];

        let pes_packet_length: u16 = (data[4] as u16) << 8 | data[5] as u16;

        Some(RequiredFields {
            packet_start_code_prefix,
            stream_id,
            pes_packet_length,
        })
    }

    // TODO: implement Result instead of Option
    fn unmarshall(data: &[u8]) -> Option<Self> {
        let Some(required_fields) = Self::unmarshall_required_fields(data) else {
            return None;
        };

        let mut header = None;
        let mut packet_data = None;
        let mut padding_bytes = None;

        match StreamType::from(required_fields.stream_id) {
            StreamType::PaddingStream => {
                padding_bytes = Some(data[6..].to_vec());
            }
            StreamType::ProgramStreamMap
            | StreamType::PrivateStream2
            | StreamType::ECMStream
            | StreamType::EMMStream
            | StreamType::ProgramStreamDirectory
            | StreamType::DSMCCStream
            | StreamType::H2221TypeE => {
                packet_data = Some(data[6..].to_vec());
            }
            _ => {
                let header_data = &data[6..];
                let mut header_size: usize = 0;
                header = PesHeader::build(header_data);
                if header.is_some() {
                    header_size = header.as_ref().unwrap().size;
                }

                let packet_data_start =
                    6 + header_size + Self::number_of_stuffing_bytes(&data[6 + header_size..]);
                packet_data = Some(data[packet_data_start..].to_vec());
            }
        }

        Some(Self {
            required_fields,
            header,
            packet_data,
            padding_bytes,
        })
    }

    fn number_of_stuffing_bytes(data: &[u8]) -> usize {
        let mut stuffing_bytes = 0;
        for (i, byte) in data.iter().enumerate() {
            if i >= MAXIMUM_NO_OF_STUFFING_BYTES {
                break;
            }
            if *byte == STUFFING_BYTE {
                stuffing_bytes += 1;
            } else {
                break;
            }
        }

        stuffing_bytes
    }
}
