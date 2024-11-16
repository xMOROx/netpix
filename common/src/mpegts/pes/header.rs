use serde::{Deserialize, Serialize};

use super::constants::*;
use super::optional_fields::ContextFlagsBuilder;
use super::optional_fields::OptionalFields;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

impl PesHeader {
    pub fn build(data: &[u8]) -> Option<Self> {
        Self::unmarshall(data)
    }

    fn unmarshall(data: &[u8]) -> Option<Self> {
        if data.is_empty() || data.len() < HEADER_REQUIRED_FIELDS_SIZE {
            return None;
        }

        if ((data[0] & HEADER_MANDATORY_BITS_MASK) >> 6) != HEADER_MANDATORY_BITS_VALUE {
            return None;
        }

        let scrambling_control = (data[0] & SCRAMBLING_CONTROL_MASK) >> 4;
        let priority = (data[0] & PRIORITY_MASK) >> 3 == 1;
        let data_alignment_indicator = (data[0] & DATA_ALIGNMENT_MASK) >> 2 == 1;
        let copyright = (data[0] & COPYRIGHT_MASK) >> 1 == 1;
        let original = data[0] & ORIGINAL_MASK == 1;
        let pts_dts_flags = (data[1] & PTS_DTS_FLAGS_MASK) >> 6;
        let escr_flag = (data[1] & ESCR_FLAG_MASK) >> 5 == 1;
        let es_rate_flag = (data[1] & ES_RATE_FLAG_MASK) >> 4 == 1;
        let dsm_trick_mode_flag = (data[1] & DSM_TRICK_MODE_FLAG_MASK) >> 3 == 1;
        let additional_copy_info_flag = (data[1] & ADDITIONAL_COPY_INFO_FLAG_MASK) >> 2 == 1;
        let pes_crc_flag = (data[1] & PES_CRC_FLAG_MASK) >> 1 == 1;
        let pes_extension_flag = data[1] & PES_EXTENSION_FLAG_MASK == 1;
        let pes_header_data_length = data[2];

        let context = ContextFlagsBuilder::new()
            .with_pts_dts_flags(pts_dts_flags)
            .with_escr_flag(escr_flag)
            .with_es_rate_flag(es_rate_flag)
            .with_dsm_trick_mode_flag(dsm_trick_mode_flag)
            .with_additional_copy_info_flag(additional_copy_info_flag)
            .with_pes_crc_flag(pes_crc_flag)
            .with_pes_extension_flag(pes_extension_flag)
            .build();

        let optional_fields = if pes_extension_flag {
            OptionalFields::build(&data[3..], context)
        } else {
            None
        };

        Some(Self {
            size: REQUIRED_FIELDS_SIZE + pes_header_data_length as usize,
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
            optional_fields,
        })
    }
}
