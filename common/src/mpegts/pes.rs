use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PesPacketHeader {
    pub packet_start_code_prefix: u32,
    pub stream_id: u8,
    pub pes_packet_length: u16,
    pub optional_header: Option<OptionalPesHeader>,
    pub pes_packet_data_bytes: Vec<u8>,
    pub padding_bytes: Vec<u8>,
}

impl PesPacketHeader {
    pub fn build() -> Self {
        Self {
            packet_start_code_prefix: 0,
            stream_id: 0,
            pes_packet_length: 0,
            optional_header: Some(OptionalPesHeader::build()),
            pes_packet_data_bytes: vec!(),
            padding_bytes: vec!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalPesHeader {
    pub marker_bits: u8,
    pub pes_scrambling_control: u8,
    pub pes_priority: bool,
    pub data_alignment_indicator: bool,
    pub copyright: bool,
    pub original_or_copy: bool,
    pub pts_dts_flags: u8,
    pub escr_flag: bool,
    pub es_rate_flag: bool,
    pub dsm_trick_mode_flag: bool,
    pub additional_copy_info_flag: bool,
    pub pes_crc_flag: bool,
    pub pes_extension_flag: bool,
    pub pes_header_data_length: u8,
    pub optional_fields: Option<OptionalPesHeaderFields>,
    pub stuffing_bytes: Vec<u8>,
}

impl OptionalPesHeader {
    pub fn build() -> Self {
        Self {
            marker_bits: 0,
            pes_scrambling_control: 0,
            pes_priority: false,
            data_alignment_indicator: false,
            copyright: false,
            original_or_copy: false,
            pts_dts_flags: 0,
            escr_flag: false,
            es_rate_flag: false,
            dsm_trick_mode_flag: false,
            additional_copy_info_flag: false,
            pes_crc_flag: false,
            pes_extension_flag: false,
            pes_header_data_length: 0,
            optional_fields: Some(OptionalPesHeaderFields::build()),
            stuffing_bytes: vec!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalPesHeaderFields {
    pub pts: Option<u64>,
    pub dts: Option<u64>,
    pub escr: Option<u64>,
    pub es_rate: Option<u32>,
    pub trick_mode_control: Option<TrickModeControl>,
    pub additional_copy_info: Option<u8>,
    pub previous_pes_packet_crc: Option<u16>,
    pub pes_private_data_flag: Option<u8>,
    pub pack_header_field_flag: Option<u8>,
    pub program_packet_sequence_counter_flag: Option<u8>,
    pub p_std_buffer_flag: Option<u8>,
    pub pes_extension_flag_2: Option<u8>,
    pub pes_private_data: Option<u128>,
    pub pack_field_length: Option<u8>,
    // The pack_header() field of a program stream, or an ISO/IEC 11172-1 system stream, is carried in the transport stream in the header of the immediately following PES packet.
    pub program_packet_sequence_counter: Option<u8>,
    pub mpeg1_mpeg2_identifier: Option<u8>,
    pub original_stuff_length: Option<u8>,
    pub p_std_buffer_scale: Option<u8>,
    pub p_std_buffer_size: Option<u16>,
    pub pes_extension_field_length: Option<u8>,
    pub stream_id_extension_flag: Option<u8>,
    pub stream_id_extension: Option<u8>,
    pub tref_extension_flag: Option<u8>,
    pub tref: Option<u64>,
}

impl OptionalPesHeaderFields {
    pub fn build() -> Self {
        Self {
            pts: None,
            dts: None,
            escr: None,
            es_rate: None,
            trick_mode_control: Some(TrickModeControl::build()),
            additional_copy_info: None,
            previous_pes_packet_crc: None,
            pes_private_data_flag: None,
            pack_header_field_flag: None,
            program_packet_sequence_counter_flag: None,
            p_std_buffer_flag: None,
            pes_extension_flag_2: None,
            pes_private_data: None,
            pack_field_length: None,
            program_packet_sequence_counter: None,
            mpeg1_mpeg2_identifier: None,
            original_stuff_length: None,
            p_std_buffer_scale: None,
            p_std_buffer_size: None,
            pes_extension_field_length: None,
            stream_id_extension_flag: None,
            stream_id_extension: None,
            tref_extension_flag: None,
            tref: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrickModeControl {
    pub field_id: Option<u8>,
    pub intra_slice_refresh: Option<u8>,
    pub frequency_truncation: Option<u8>,
    pub rep_cntrl: Option<u8>,
}

impl TrickModeControl {
    fn build() -> Self {
        Self {
            field_id: None,
            intra_slice_refresh: None,
            frequency_truncation: None,
            rep_cntrl: None,
        }
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
