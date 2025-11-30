use super::constants::*;
use super::optional_fields::OptionalFields;
use crate::utils::traits::{BitManipulation, DataParser, DataValidator};
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct PesHeader {
    pub size: usize,
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

impl BitManipulation for PesHeader {}

impl DataValidator for PesHeader {
    fn validate(&self) -> bool {
        if self.pes_header_data_length < 3 {
            return false;
        }

        matches!(self.pts_dts_flags, 0b00 | 0b10 | 0b11)
    }
}

impl DataParser for PesHeader {
    type Output = Self;

    fn parse(data: &[u8]) -> Option<Self::Output> {
        if data.len() < HEADER_REQUIRED_FIELDS_SIZE {
            return None;
        }

        if Self::get_bits(data[0], HEADER_MANDATORY_BITS_MASK, 6) != HEADER_MANDATORY_BITS_VALUE {
            return None;
        }

        let header = Self {
            size: HEADER_REQUIRED_FIELDS_SIZE,
            scrambling_control: Self::get_bits(data[0], SCRAMBLING_CONTROL_MASK, 4),
            priority: Self::get_bit(data[0], 3),
            data_alignment_indicator: Self::get_bit(data[0], 2),
            copyright: Self::get_bit(data[0], 1),
            original: Self::get_bit(data[0], 0),
            pts_dts_flags: Self::get_bits(data[1], PTS_DTS_FLAGS_MASK, 6),
            escr_flag: Self::get_bit(data[1], 5),
            es_rate_flag: Self::get_bit(data[1], 4),
            dsm_trick_mode_flag: Self::get_bit(data[1], 3),
            additional_copy_info_flag: Self::get_bit(data[1], 2),
            pes_crc_flag: Self::get_bit(data[1], 1),
            pes_extension_flag: Self::get_bit(data[1], 0),
            pes_header_data_length: data[2],
            optional_fields: None,
        };

        let mut header = header;
        if header.pes_extension_flag
            && let Some((fields, _)) = OptionalFields::parse(&data[1..])
        {
            header.optional_fields = Some(fields);
            header.size += header.pes_header_data_length as usize;
        }

        Some(header)
    }
}

impl PesHeader {
    pub fn build(data: &[u8]) -> Option<Self> {
        Self::parse(data)
    }
}
