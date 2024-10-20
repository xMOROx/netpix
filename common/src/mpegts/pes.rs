mod pes_buffer;

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use crate::mpegts::MpegtsFragment;
use crate::mpegts::pes::pes_buffer::PesPacketPayload;

const SCRAMBLING_MASK: u8 = 0x30;
const PRIORITY_MASK: u8 = 0x08;
const DATA_ALIGNMENT_MASK: u8 = 0x04;
const COPYRIGHT_MASK: u8 = 0x02;
const ORIGINAL_MASK: u8 = 0x01;
const PTS_DTS_FLAGS_MASK: u8 = 0xC0;
const ESCR_FLAG_MASK: u8 = 0x20;
const ES_RATE_FLAG_MASK: u8 = 0x10;
const DSM_TRICK_MODE_FLAG_MASK: u8 = 0x08;
const ADDITIONAL_COPY_INFO_FLAG_MASK: u8 = 0x04;
const PES_CRC_FLAG_MASK: u8 = 0x02;
const PES_EXTENSION_FLAG_MASK: u8 = 0x01;
const PTS_DTS_MASK: u8 = 0xf0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PacketizedElementaryStream {
    pub packet_start_code_prefix: u32,
    pub stream_id: u8,
    pub pes_packet_length: u16,
    pub header: Option<PesHeader>,
}

impl PacketizedElementaryStream {
    pub fn build(packet: &PesPacketPayload) -> Option<Self> {
        if packet.data.is_empty() {
            return None;
        }
        let Some(payload) = &packet.data.as_ref() else {
            return None;
        };

        let Some(pes) = Self::unmarshall(&payload) else {
            return None;
        };

        Some(pes)
    }

    fn unmarshall(data: &Vec<u8>) -> Option<Self> {
        let packet_start_code_prefix: u32 = (data[0] as u32) << 16 |
            (data[1] as u32) << 8 |
            data[2] as u32;
        if packet_start_code_prefix != 0x000001 {
            return None;
        }

        let stream_id: u8 = data[3];

        let pes_packet_length: u16 = (data[4] as u16) << 8 | data[5] as u16;

        let header = PesHeader::build(data, StreamType::from(stream_id));

        Some(Self {
            packet_start_code_prefix,
            stream_id,
            pes_packet_length,
            header
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PesHeader {
    pub scrambling_control: u8,
    pub priority: bool,
    pub data_alignment_indicator: bool,
    pub copyright: bool,
    pub original: bool,
    pub pts_dts_flags: u8,
    pub escr_flag: bool,
    pub es_rate_flag: bool,
    pub dsm_trick_mode_flag: bool,
    pub additional_copy_info_flag: bool,
    pub pes_crc_flag: bool,
    pub pes_extension_flag: bool,
    pub pes_header_data_length: u8,
    pub optional_fields: Option<OptionalFields>,
}

impl PesHeader {
    pub fn build(data: &Vec<u8>, stream_type: StreamType) -> Option<Self> {
        if stream_type != StreamType::ProgramStreamMap
            && stream_type != StreamType::PaddingStream
            && stream_type != StreamType::PrivateStream2
            && stream_type != StreamType::ECMStream
            && stream_type != StreamType::EMMStream
            && stream_type != StreamType::ProgramStreamDirectory
            && stream_type != StreamType::DSMCCStream
            && stream_type != StreamType::H2221TypeE {
            let Some(header) = Self::unmarshall(data) else {
                return None;
            };
            return Some(header);
        } else {
            return None;
        }
    }

    fn unmarshall(data: &Vec<u8>) -> Option<Self> {
        if (data[6] & 0xc0) != 0x80 {
            return None;
        }
        let scrambling_control = data[6] & SCRAMBLING_MASK;
        let priority = (data[6] & PRIORITY_MASK) >> 3 == 1;
        let data_alignment_indicator = (data[6] & DATA_ALIGNMENT_MASK) >> 2 == 1;
        let copyright = (data[6] & COPYRIGHT_MASK) >> 1 == 1;
        let original = data[6] & ORIGINAL_MASK == 1;
        let pts_dts_flags = data[7] & PTS_DTS_FLAGS_MASK >> 6;
        let escr_flag = (data[7] & ESCR_FLAG_MASK) >> 5 == 1;
        let es_rate_flag = (data[7] & ES_RATE_FLAG_MASK) >> 4 == 1;
        let dsm_trick_mode_flag = (data[7] & DSM_TRICK_MODE_FLAG_MASK) >> 3 == 1;
        let additional_copy_info_flag = (data[7] & ADDITIONAL_COPY_INFO_FLAG_MASK) >> 2 == 1;
        let pes_crc_flag = (data[7] & PES_CRC_FLAG_MASK) >> 1 == 1;
        let pes_extension_flag = data[7] & PES_EXTENSION_FLAG_MASK == 1;
        let pes_header_data_length = data[8];
        let optional_fields = if pes_extension_flag {
            OptionalFields::build(data, pts_dts_flags, escr_flag, es_rate_flag, dsm_trick_mode_flag,
                                  additional_copy_info_flag, pes_crc_flag,
                                  pes_extension_flag)
        } else {
            None
        };

        Some(Self {
            scrambling_control,
            priority,
            data_alignment_indicator,
            copyright,
            original,
            pts_dts_flags,
            escr_flag,
            es_rate_flag,
            dsm_trick_mode_flag,
            additional_copy_info_flag,
            pes_crc_flag,
            pes_extension_flag,
            pes_header_data_length,
            optional_fields
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalFields {
    pub pts: Option<u64>,
    pub dts: Option<u64>,
    pub escr_base: Option<u64>,
    pub escr_extension: Option<u16>,
    pub es_rate: Option<u32>,
    pub trick_mode_control: Option<TrickModeControl>,
    pub additional_copy_info: Option<u8>,
    pub previous_pes_packet_crc: Option<u16>,
    pub pes_private_data_flag: Option<bool>,
    pub pack_header_field_flag: Option<bool>,
    pub program_packet_sequence_counter_flag: Option<bool>,
    pub p_std_buffer_flag: Option<bool>,
    pub pes_extension_flag_2: Option<bool>,
    pub pes_private_data: Option<u128>,
    pub pack_field_length: Option<u8>,
    // The pack_header() field of a program stream, or an ISO/IEC 11172-1 system stream, is carried in the transport stream in the header of the immediately following PES packet.
    pub program_packet_sequence_counter: Option<u8>,
    pub mpeg1_mpeg2_identifier: Option<u8>,
    pub original_stuff_length: Option<u8>,
    pub p_std_buffer_scale: Option<u8>,
    pub p_std_buffer_size: Option<u16>,
    pub pes_extension_field_length: Option<u8>,
    pub stream_id_extension_flag: Option<bool>,
    pub stream_id_extension: Option<u8>,
    pub tref_extension_flag: Option<bool>,
    pub tref: Option<u64>,
}

impl OptionalFields {
    pub fn build(data: &Vec<u8>, pts_dts_flags: u8, escr_flag: bool, es_rate_flag: bool,
                 dsm_trick_mode_flag: bool, additional_copy_info_flag: bool, pes_crc_flag: bool,
                 pes_extension_flag: bool) -> Option<Self> {
        let Some(optional_fields) = Self::unmarshall(data, pts_dts_flags, escr_flag,
                                                     es_rate_flag, dsm_trick_mode_flag,
                                                     additional_copy_info_flag, pes_crc_flag,
                                                     pes_extension_flag) else {
            return None;
        };
        Some(optional_fields)
    }

    fn unmarshall(data: &Vec<u8>, pts_dts_flags: u8, escr_flag: bool, es_rate_flag: bool,
                  dsm_trick_mode_flag: bool, additional_copy_info_flag: bool, pes_crc_flag: bool,
                  pes_extension_flag: bool) -> Option<Self> {
        let mut index = 9;
        let mut pts = None;
        let mut dts = None;
        let mut escr_base = None;
        let mut escr_extension = None;
        let mut es_rate = None;
        let mut trick_mode_control = None;
        let mut additional_copy_info = None;
        let mut previous_pes_packet_crc = None;
        let mut pes_private_data_flag = None;
        let mut pack_header_field_flag = None;
        let mut program_packet_sequence_counter_flag = None;
        let mut p_std_buffer_flag = None;
        let mut pes_extension_flag_2 = None;
        let mut pes_private_data = None;
        let mut pack_field_length = None;
        let mut program_packet_sequence_counter = None;
        let mut mpeg1_mpeg2_identifier = None;
        let mut original_stuff_length = None;
        let mut p_std_buffer_scale = None;
        let mut p_std_buffer_size = None;
        let mut pes_extension_field_length = None;
        let mut stream_id_extension_flag = None;
        let mut stream_id_extension = None;
        let mut tref_extension_flag = None;
        let mut tref = None;
        if pts_dts_flags == 0b10 {
            if data[index] & PTS_DTS_MASK != 0x20 {
                // packet invalid
            } else {
                pts = Some(Self::unmarshall_pts_dts(&data[index..]));
                index += 5;
            }
        } else if pts_dts_flags == 0b11 {
            if data[index] & PTS_DTS_MASK != 0x30 {
                // packet invalid
            } else {
                pts = Some(Self::unmarshall_pts_dts(&data[index..]));
                index += 5;
                dts = Some(Self::unmarshall_pts_dts(&data[index..]));
                index += 5;
            }
        }
        if escr_flag {
            escr_base = Some((((data[0] & 0b00111000) as u64) << 27) |
                (((data[0] & 0b00000011) as u64) << 28) |
                ((data[index + 1] as u64) << 20) |
                (((data[index + 2] & 0b11111000) as u64) << 12) |
                (((data[index + 2] & 0b00000011) as u64) << 13) |
                ((data[index + 3] as u64) << 5) |
                (((data[index + 4] & 0b11111000) as u64) >> 3));
            index += 4;
            escr_extension = Some((((data[index] & 0b00000011) as u16) << 7) |
                ((data[index + 1] as u16)>> 1));
            index +=2;
        }
        if es_rate_flag {
            es_rate = Some((((data[index] as u32) & 0b01111111) << 15) |
                (data[index + 1] as u32) << 7 |
                ((data[index + 2] as u32) >> 1));
            index += 3;
        }
        if dsm_trick_mode_flag {
            trick_mode_control = TrickModeControl::build(data, index);
            index += 1;
        }
        if additional_copy_info_flag {
            additional_copy_info = Some(data[index] & 0b01111111);
            index += 1;
        }
        if pes_crc_flag {
            previous_pes_packet_crc = Some(((data[index] as u16) << 8) |
                data[index + 1] as u16);
            index += 2;
        }
        if pes_extension_flag {
            pes_private_data_flag = Some((data[index] & 0b10000000 >> 7) == 1);
            pack_header_field_flag = Some((data[index] & 0b01000000 >> 6) == 1);
            program_packet_sequence_counter_flag = Some((data[index] & 0b00100000 >> 5) == 1);
            p_std_buffer_flag = Some((data[index] & 0b00010000 >> 4) == 1);
            pes_extension_flag_2 = Some((data[index] & 0b00000001 == 1));
            index += 1;
            if pes_private_data_flag.unwrap() {
                pes_private_data = Some(u128::from_be_bytes([
                    data[index], data[index + 1], data[index + 2], data[index + 3],
                    data[index + 4], data[index + 5], data[index + 6], data[index + 7],
                    data[index + 8], data[index + 9], data[index + 10], data[index + 11],
                    data[index + 12], data[index + 13], data[index + 14], data[index + 15],
                ]));
                index += 16;
            }
            if pack_header_field_flag.unwrap() {
                pack_field_length = Some(data[index]);
                index += 1;
            }
            if program_packet_sequence_counter_flag.unwrap() {
                program_packet_sequence_counter = Some(data[index] & 0b01111111);
                index += 1;
                mpeg1_mpeg2_identifier = Some((data[index] & 0b01000000) >> 6);
                original_stuff_length = Some(data[index] & 0b00111111);
                index += 1;
            }
            if p_std_buffer_flag.unwrap() {
                if data[index] & 0b11000000 != 0b01000000 {
                    // packet invalid
                }
                p_std_buffer_scale = Some((data[index] & 0b00100000) >> 5);
                p_std_buffer_size = Some((((data[index] & 0b00011111) as u16 ) << 8) |
                    data[index + 1] as u16);
                index += 2;
            }
            if pes_extension_flag_2.unwrap() {
                pes_extension_field_length = Some(data[index] & 0b01111111);
                index += 1;
                stream_id_extension_flag = Some((data[index] & 0b10000000) >> 7 == 1);
                if stream_id_extension_flag.unwrap() == false {
                    stream_id_extension = Some(data[index] & 0b01111111);
                    index += 1;
                } else {
                    tref_extension_flag = Some(data[index] & 0b00000001 == 1);
                    index += 1;
                    if tref_extension_flag.unwrap() == false {
                        tref = Some((((data[index] & 0b00001110) as u64) << 28) |
                            ((data[index + 1] as u64) << 22) |
                            (((data[index + 2] & 0b11111110) as u64) << 14) |
                            ((data[index + 3] as u64) << 7) |
                            (((data[index + 4] & 0b11111110) >> 1) as u64));
                        index += 5;
                    }
                }
            }
        }

        Some(Self {
            pts,
            dts,
            escr_base,
            escr_extension,
            es_rate,
            trick_mode_control,
            additional_copy_info,
            previous_pes_packet_crc,
            pes_private_data_flag,
            pack_header_field_flag,
            program_packet_sequence_counter_flag,
            p_std_buffer_flag,
            pes_extension_flag_2,
            pes_private_data,
            pack_field_length,
            program_packet_sequence_counter,
            mpeg1_mpeg2_identifier,
            original_stuff_length,
            p_std_buffer_scale,
            p_std_buffer_size,
            pes_extension_field_length,
            stream_id_extension_flag,
            stream_id_extension,
            tref_extension_flag,
            tref,
        })
    }

    fn unmarshall_pts_dts(data: &[u8]) -> u64 {
        let ts = (((data[0] & 0b00001111) as u64) << 30) |
            ((data[1] as u64) << 22) |
            ((data[2] as u64) << 15) |
            ((data[3] as u64) << 7) |
            ((data[0] & 0b11111110) as u64);
        ts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TrickModeControlValues {
    FastForward,
    SlowMotion,
    FreezeFrame,
    FastReverse,
    SlowReverse,
    Reserved,
}

impl From<u8> for TrickModeControlValues {
    fn from(value: u8) -> Self {
        match value {
            0b000 => TrickModeControlValues::FastForward,
            0b001 => TrickModeControlValues::SlowMotion,
            0b010 => TrickModeControlValues::FreezeFrame,
            0b011 => TrickModeControlValues::FastReverse,
            0b100 => TrickModeControlValues::SlowReverse,
            _ => TrickModeControlValues::Reserved,
        }
    }
}

impl Into<u8> for TrickModeControlValues {
    fn into(self) -> u8 {
        match self {
            TrickModeControlValues::FastForward => 0b000,
            TrickModeControlValues::SlowMotion => 0b001,
            TrickModeControlValues::FreezeFrame => 0b010,
            TrickModeControlValues::FastReverse => 0b011,
            TrickModeControlValues::SlowReverse => 0b100,
            TrickModeControlValues::Reserved => 0b111,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrickModeControl {
    pub trick_mode_control: TrickModeControlValues,
    pub field_id: u8,
    pub intra_slice_refresh: u8,
    pub frequency_truncation: u8,
    pub rep_cntrl: u8,
}

impl TrickModeControl {
    fn build(data: &Vec<u8>, index: usize) -> Option<Self> {
        let Some(trick_mode_control) = Self::unmarshall(data, index) else {
            return None
        };
        Some(trick_mode_control)
    }

    fn unmarshall(data: &Vec<u8>, index: usize) -> Option<Self> {
        let trick_mode_control = TrickModeControlValues::from(data[index] & 0b11100000);
        let mut field_id = None;
        let mut intra_slice_refresh = None;
        let mut frequency_truncation = None;
        let mut rep_cntrl = None;
        if trick_mode_control == TrickModeControlValues::FastForward || trick_mode_control == TrickModeControlValues::FastReverse {
            field_id = Some((data[index] & 0b00011000) >> 3);
            intra_slice_refresh = Some((data[index] & 0b00000100) >> 2);
            frequency_truncation = Some(data[index] & 0b00000011);
        }
        if trick_mode_control == TrickModeControlValues::SlowMotion || trick_mode_control == TrickModeControlValues::SlowReverse {
            rep_cntrl = Some(data[index] & 0b00011111);
        }
        if trick_mode_control == TrickModeControlValues::FreezeFrame {
            field_id = Some((data[index] & 0b00011000) >> 3);
        }

        Some(Self {
            trick_mode_control,
            field_id: field_id.unwrap_or(0),
            intra_slice_refresh: intra_slice_refresh.unwrap_or(0),
            frequency_truncation: frequency_truncation.unwrap_or(0),
            rep_cntrl: rep_cntrl.unwrap_or(0),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum StreamType {
    ProgramStreamMap,
    PrivateStream1,
    PaddingStream,
    PrivateStream2,
    AudioStream(u8),
    VideoStream(u8),
    ECMStream,
    EMMStream,
    DSMCCStream,
    ISOIEC13522Stream,
    H2221TypeA,
    H2221TypeB,
    H2221TypeC,
    H2221TypeD,
    H2221TypeE,
    AncillaryStream,
    SLPacketizedStream,
    FlexMuxStream,
    MetadataStream,
    ExtendedStreamId,
    ReservedDataStream,
    ProgramStreamDirectory,
    Unknown,
}

impl From<u8> for StreamType {
    fn from(stream_id: u8) -> Self {
        match stream_id {
            0xBC => StreamType::ProgramStreamMap,
            0xBD => StreamType::PrivateStream1,
            0xBE => StreamType::PaddingStream,
            0xBF => StreamType::PrivateStream2,
            0xF0 => StreamType::ECMStream,
            0xF1 => StreamType::EMMStream,
            0xF2 => StreamType::DSMCCStream,
            0xF3 => StreamType::ISOIEC13522Stream,
            0xF4 => StreamType::H2221TypeA,
            0xF5 => StreamType::H2221TypeB,
            0xF6 => StreamType::H2221TypeC,
            0xF7 => StreamType::H2221TypeD,
            0xF8 => StreamType::H2221TypeE,
            0xF9 => StreamType::AncillaryStream,
            0xFA => StreamType::SLPacketizedStream,
            0xFB => StreamType::FlexMuxStream,
            0xFC => StreamType::MetadataStream,
            0xFD => StreamType::ExtendedStreamId,
            0xFE => StreamType::ReservedDataStream,
            0xFF => StreamType::ProgramStreamDirectory,
            id @ 0xC0..=0xDF => StreamType::AudioStream(id & 0x1F),
            id @ 0xE0..=0xEF => StreamType::VideoStream(id & 0x0F),
            _ => StreamType::Unknown,
        }
    }
}



