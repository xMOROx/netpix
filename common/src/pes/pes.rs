use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct PesPacketHeader {
    packet_start_code_prefix: u32,
    stream_id: u8,
    pes_packet_length: u16,
    header: Option<OptionalPesHeader>,  
    pes_packet_data_bytes: Vec<u8>,
    padding_bytes: Vec<u8>,              
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct OptionalPesHeader {
    marker_bits: u8,
    pes_scrambling_control: u8,
    pes_priority: bool,
    data_alignment_indicator: bool,
    copyright: bool,
    original_or_copy: bool,
    pts_dts_flags: u8,
    escr_flag: bool,
    es_rate_flag: bool,
    dsm_trick_mode_flag: bool,
    additional_copy_info_flag: bool,
    pes_crc_flag: bool,
    pes_extension_flag: bool,
    pes_header_data_length: u8,
    optional_fields: Option<OptionalPesHeaderFields>,
    stuffing_bytes: Vec<u8>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct OptionalPesHeaderFields {
    pts: Option<u64>,
    dts: Option<u64>,
    escr: Option<u64>,
    es_rate: Option<u32>,
    trick_mode_control: Option<TrickModeControl>,
    additional_copy_info: Option<u8>,
    previous_pes_packet_crc: Option<u16>,
    pes_private_data_flag: Option<u8>,
    pack_header_field_flag: Option<u8>,
    program_packet_sequence_counter_flag: Option<u8>,
    p_std_buffer_flag: Option<u8>,
    pes_extension_flag_2: Option<u8>,
    pes_private_data: Option<u128>,
    pack_field_length: Option<u8>,
    // The pack_header() field of a program stream, or an ISO/IEC 11172-1 system stream, is carried in the transport stream in the header of the immediately following PES packet.
    program_packet_sequence_counter: Option<u8>,
    mpeg1_mpeg2_identifier: Option<u8>,
    original_stuff_length: Option<u8>,
    p_std_buffer_scale: Option<u8>,
    p_std_buffer_size : Option<u16>,
    pes_extension_field_length: Option<u8>,
    stream_id_extension_flag: Option<u8>,
    stream_id_extension: Option<u8>,
    tref_extension_flag: Option<u8>,
    tref: Option<u64>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct TrickModeControl {
    field_id: Option<u8>,
    intra_slice_refresh: Option<u8>,
    frequency_truncation: Option<u8>,
    rep_cntrl: Option<u8>,
}