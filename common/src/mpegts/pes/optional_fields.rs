use serde::{Deserialize, Serialize};

use super::constants::*;
use super::enums::PtsDtsFlags;
use super::trick_mode_control::TrickModeControl;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct OptionalFields {
    pub size: u8,
    pub pts: Option<u64>,
    pub dts: Option<u64>,
    pub escr_base: Option<u64>,
    pub escr_extension: Option<u16>,
    pub es_rate: Option<u32>,
    pub trick_mode_control: Option<TrickModeControl>,
    pub additional_copy_info: Option<u8>,
    pub previous_pes_packet_crc: Option<u16>,
    pub pes_extension_data: Option<PesExtensionData>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PesExtensionData {
    pub size: u8,
    pub pes_private_data_flag: bool,
    pub pack_header_field_flag: bool,
    pub program_packet_sequence_counter_flag: bool,
    pub p_std_buffer_flag: bool,
    pub pes_extension_flag_2: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ContextFlags {
    pub pts_dts_flags: u8,
    pub escr_flag: bool,
    pub es_rate_flag: bool,
    pub dsm_trick_mode_flag: bool,
    pub additional_copy_info_flag: bool,
    pub pes_crc_flag: bool,
    pub pes_extension_flag: bool,
}

pub struct ContextFlagsBuilder {
    pts_dts_flags: u8,
    escr_flag: bool,
    es_rate_flag: bool,
    dsm_trick_mode_flag: bool,
    additional_copy_info_flag: bool,
    pes_crc_flag: bool,
    pes_extension_flag: bool,
}

impl ContextFlagsBuilder {
    pub fn new() -> Self {
        Self {
            pts_dts_flags: 0,
            escr_flag: false,
            es_rate_flag: false,
            dsm_trick_mode_flag: false,
            additional_copy_info_flag: false,
            pes_crc_flag: false,
            pes_extension_flag: false,
        }
    }

    pub fn with_pts_dts_flags(mut self, pts_dts_flags: u8) -> Self {
        self.pts_dts_flags = pts_dts_flags;
        self
    }

    pub fn with_escr_flag(mut self, escr_flag: bool) -> Self {
        self.escr_flag = escr_flag;
        self
    }

    pub fn with_es_rate_flag(mut self, es_rate_flag: bool) -> Self {
        self.es_rate_flag = es_rate_flag;
        self
    }

    pub fn with_dsm_trick_mode_flag(mut self, dsm_trick_mode_flag: bool) -> Self {
        self.dsm_trick_mode_flag = dsm_trick_mode_flag;
        self
    }

    pub fn with_additional_copy_info_flag(mut self, additional_copy_info_flag: bool) -> Self {
        self.additional_copy_info_flag = additional_copy_info_flag;
        self
    }

    pub fn with_pes_crc_flag(mut self, pes_crc_flag: bool) -> Self {
        self.pes_crc_flag = pes_crc_flag;
        self
    }

    pub fn with_pes_extension_flag(mut self, pes_extension_flag: bool) -> Self {
        self.pes_extension_flag = pes_extension_flag;
        self
    }

    pub fn build(self) -> ContextFlags {
        ContextFlags {
            pts_dts_flags: self.pts_dts_flags,
            escr_flag: self.escr_flag,
            es_rate_flag: self.es_rate_flag,
            dsm_trick_mode_flag: self.dsm_trick_mode_flag,
            additional_copy_info_flag: self.additional_copy_info_flag,
            pes_crc_flag: self.pes_crc_flag,
            pes_extension_flag: self.pes_extension_flag,
        }
    }
}

impl OptionalFields {
    pub fn build(data: &[u8], context_flags: ContextFlags) -> Option<Self> {
        let Some(optional_fields) = Self::unmarshall(data, context_flags) else {
            return None;
        };
        Some(optional_fields)
    }

    pub(super) fn unmarshall(data: &[u8], context_flags: ContextFlags) -> Option<Self> {
        let ContextFlags {
            pts_dts_flags,
            escr_flag,
            es_rate_flag,
            dsm_trick_mode_flag,
            additional_copy_info_flag,
            pes_crc_flag,
            pes_extension_flag,
        } = context_flags;

        let mut index = 0;
        let mut pts = None;
        let mut dts = None;
        let mut escr_base = None;
        let mut escr_extension = None;
        let mut es_rate = None;
        let mut trick_mode_control = None;
        let mut additional_copy_info = None;
        let mut previous_pes_packet_crc = None;
        let mut pes_extension_data = None;

        match PtsDtsFlags::from(pts_dts_flags) {
            PtsDtsFlags::Forbidden => { /*//todo: signal error*/ }
            PtsDtsFlags::No => {}
            PtsDtsFlags::PresentPts => {
                if ((data[index] & PTS_DTS_REQUIRED_BITS_MASK) >> 4) != ONLY_PTS_REQUIRED_BITS_VALUE
                {
                    return None;
                } else {
                    if let Ok(pts_value) = Self::unmarshall_pts_dts(&data[index..]) {
                        pts = Some(pts_value);
                    } else {
                        return None;
                    }

                    index += 5;
                }
            }
            PtsDtsFlags::PresentPtsAndDts => {
                if ((data[index] & PTS_DTS_REQUIRED_BITS_MASK) >> 4)
                    != PTS_AND_DTS_REQUIRED_BITS_FIRST_VALUE
                {
                    return None;
                } else {
                    if let Ok(pts_value) = Self::unmarshall_pts_dts(&data[index..]) {
                        pts = Some(pts_value);
                    } else {
                        return None;
                    }

                    index += 5;
                    if ((data[index] & PTS_DTS_REQUIRED_BITS_MASK) >> 4)
                        != PTS_AND_DTS_REQUIRED_BITS_SECOND_VALUE
                    {
                        return None;
                    } else {
                        if let Ok(dts_value) = Self::unmarshall_pts_dts(&data[index..]) {
                            dts = Some(dts_value);
                        } else {
                            return None;
                        }
                        index += 5;
                    }
                }
            }
        }

        if escr_flag {
            if let Ok((_escr_base, _escr_extension)) = Self::unmarshall_escr(&data[index..]) {
                escr_base = Some(_escr_base);
                escr_extension = Some(_escr_extension);
            } else {
                return None;
            }
            index += 6;
        }
        if es_rate_flag {
            if let Ok(_es_rate) = Self::unmarshall_es_rate(&data[index..]) {
                es_rate = Some(_es_rate);
            } else {
                return None;
            }
            index += 3;
        }
        if dsm_trick_mode_flag {
            trick_mode_control = TrickModeControl::build(&data[index..]);
            index += 1;
        }
        if additional_copy_info_flag {
            if is_invalid_marker_bit(data[index], 0b10000000) {
                return None;
            }

            additional_copy_info = Some((data[index] & 0b01111111) >> 1);
            index += 1;
        }
        if pes_crc_flag {
            previous_pes_packet_crc = Some(((data[index] as u16) << 8) | data[index + 1] as u16);
            index += 2;
        }
        if pes_extension_flag {
            pes_extension_data = PesExtensionData::build(&data[index..]);
            index += pes_extension_data.as_ref().map_or(0, |data| data.size) as usize;
        }

        Some(Self {
            size: index as u8,
            pts,
            dts,
            escr_base,
            escr_extension,
            es_rate,
            trick_mode_control,
            additional_copy_info,
            previous_pes_packet_crc,
            pes_extension_data,
        })
    }

    fn unmarshall_pts_dts(data: &[u8]) -> Result<u64, ()> {
        //todo: add better error handling
        let marker_bit_mask = 0x01;
        if data.len() < 5 {
            return Err(());
        }

        if is_invalid_marker_bit(data[0], marker_bit_mask)
            | is_invalid_marker_bit(data[2], marker_bit_mask)
            | is_invalid_marker_bit(data[4], marker_bit_mask)
        {
            return Err(());
        }

        let ts_1 = ((data[0] & 0b00001110) as u64) << 29;
        let ts_2 = (data[1] as u64) << 22;
        let ts_3 = ((data[2] & 0b11111110) as u64) << 14;
        let ts_4 = (data[3] as u64) << 7;
        let ts_5 = ((data[4] & 0b11111110) as u64) >> 1;

        Ok(ts_1 | ts_2 | ts_3 | ts_4 | ts_5)
    }

    fn unmarshall_escr(data: &[u8]) -> Result<(u64, u16), ()> {
        if data.len() < 6 {
            return Err(());
        }

        let base_1 = ((data[0] & 0b00111000) as u64) << 27;
        if is_invalid_marker_bit(data[0], 0b00000100)
            | is_invalid_marker_bit(data[2], 0b00000100)
            | is_invalid_marker_bit(data[4], 0b00000100)
            | is_invalid_marker_bit(data[5], 0b00000001)
        {
            return Err(());
        }
        let base_2 = ((data[0] & 0b00000011) as u64) << 28;
        let base_3 = (data[1] as u64) << 20;
        let base_4 = ((data[2] & 0b11111000) as u64) << 12;
        let base_5 = ((data[2] & 0b00000011) as u64) << 13;
        let base_6 = (data[3] as u64) << 5;
        let base_7 = ((data[4] & 0b11111000) >> 3) as u64;
        let extension_1 = ((data[4] & 0b00000011) as u16) << 7;
        let extension_2 = ((data[5]) as u16) >> 1;

        Ok((
            base_1 | base_2 | base_3 | base_4 | base_5 | base_6 | base_7,
            extension_1 | extension_2,
        ))
    }

    fn unmarshall_es_rate(data: &[u8]) -> Result<u32, ()> {
        if data.len() < 3 {
            return Err(());
        }

        if is_invalid_marker_bit(data[0], 0b10000000) || is_invalid_marker_bit(data[2], 0b00000001)
        {
            return Err(());
        }

        Ok((((data[0] as u32) & 0b01111111) << 15)
            | (data[1] as u32) << 7
            | ((data[2] as u32) >> 1))
    }
}

impl PesExtensionData {
    pub fn build(data: &[u8]) -> Option<Self> {
        Self::unmarshall(data)
    }

    fn unmarshall(data: &[u8]) -> Option<Self> {
        let mut index = 0;

        let pes_private_data_flag = (data[index] & 0b10000000 >> 7) == 1;
        let pack_header_field_flag = (data[index] & 0b01000000 >> 6) == 1;
        let program_packet_sequence_counter_flag = (data[index] & 0b00100000 >> 5) == 1;
        let p_std_buffer_flag = (data[index] & 0b00010000 >> 4) == 1;
        let pes_extension_flag_2 = data[index] & 0b00000001 == 1;

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

        index += 1;
        if pes_private_data_flag {
            pes_private_data = Some(u128::from_be_bytes([
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
                data[index + 4],
                data[index + 5],
                data[index + 6],
                data[index + 7],
                data[index + 8],
                data[index + 9],
                data[index + 10],
                data[index + 11],
                data[index + 12],
                data[index + 13],
                data[index + 14],
                data[index + 15],
            ]));
            index += 16;
        }
        if pack_header_field_flag {
            pack_field_length = Some(data[index]);
            index += data[index] as usize;
        }
        if program_packet_sequence_counter_flag {
            if is_invalid_marker_bit(data[index], 0b10000000) {
                return None;
            }

            program_packet_sequence_counter = Some(data[index] & 0b01111111);
            index += 1;

            if is_invalid_marker_bit(data[index], 0b10000000) {
                return None;
            }

            mpeg1_mpeg2_identifier = Some((data[index] & 0b01000000) >> 6);
            original_stuff_length = Some(data[index] & 0b00111111);
            index += 1;
        }
        if p_std_buffer_flag {
            if ((data[index] & P_STD_BUFFER_REQUIRED_BITS_MASK) >> 6)
                != P_STD_BUFFER_REQUIRED_BITS_VALUE
            {
                return None;
            }
            p_std_buffer_scale = Some((data[index] & 0b00100000) >> 5);
            p_std_buffer_size =
                Some((((data[index] & 0b00011111) as u16) << 8) | data[index + 1] as u16);
            index += 2;
        }
        if pes_extension_flag_2 {
            if is_invalid_marker_bit(data[index], 0b10000000) {
                return None;
            }

            let mut bytes_after_pes_extension_field_length = 1;

            pes_extension_field_length = Some(data[index] & 0b01111111);
            index += 1;
            stream_id_extension_flag = Some((data[index] & 0b10000000) >> 7 == 1);
            if !stream_id_extension_flag.unwrap() {
                stream_id_extension = Some(data[index] & 0b01111111);
                index += 1;
            } else {
                tref_extension_flag = Some((data[index] & 0b00000001) == 1);
                index += 1;
                if !tref_extension_flag.unwrap() {
                    if let Ok(tref_value) = Self::unmarshall_tref(&data[index..]) {
                        tref = Some(tref_value);
                    } else {
                        return None;
                    }
                    index += 5;
                    bytes_after_pes_extension_field_length += 5;
                }
            }

            index += pes_extension_field_length.unwrap() as usize
                - bytes_after_pes_extension_field_length;
        }

        Some(Self {
            size: index as u8,
            pes_private_data_flag: pes_private_data_flag,
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

    fn unmarshall_tref(data: &[u8]) -> Result<u64, ()> {
        if data.len() < 5 {
            return Err(());
        }

        if is_invalid_marker_bit(data[0], 0b00000001)
            | is_invalid_marker_bit(data[2], 0b00000001)
            | is_invalid_marker_bit(data[4], 0b00000001)
        {
            return Err(());
        }

        let tref_1 = ((data[0] & 0b00001110) as u64) << 29;
        let tref_2 = (data[1] as u64) << 22;
        let tref_3 = ((data[2] & 0b11111110) as u64) << 14;
        let tref_4 = (data[3] as u64) << 7;
        let tref_5 = ((data[4] & 0b11111110) as u64) >> 1;

        Ok(tref_1 | tref_2 | tref_3 | tref_4 | tref_5)
    }
}

fn is_invalid_marker_bit(byte: u8, mask: u8) -> bool {
    byte & mask != mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_fields_unmarshall_pts_dts() {
        let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011, 0b10000011];
        let result = OptionalFields::unmarshall_pts_dts(&data);
        assert_eq!(result, Ok(6_123_313_601));
    }

    #[test]
    fn test_optional_fields_unmarshall_pts_dts_invalid() {
        let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011];
        let result = OptionalFields::unmarshall_pts_dts(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_pts_dts_invalid_first_marker_bit() {
        let data = [0b00001010, 0b10110011, 0b11101001, 0b10110011, 0b10000011];
        let result = OptionalFields::unmarshall_pts_dts(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_pts_dts_invalid_second_marker_bit() {
        let data = [0b00001011, 0b10110011, 0b11101000, 0b10110011, 0b10000011];
        let result = OptionalFields::unmarshall_pts_dts(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_pts_dts_invalid_third_marker_bit() {
        let data = [0b00001011, 0b10110011, 0b11101001, 0b10110011, 0b10000010];
        let result = OptionalFields::unmarshall_pts_dts(&data);
        assert_eq!(result, Err(()));
    }
    #[test]
    fn test_optional_fields_unmarshall_escr() {
        let data = [
            0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        ];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Ok((8_589_934_591, 511)));
    }

    #[test]
    fn test_optional_fields_unmarshall_escr_invalid() {
        let data = [0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_escr_invalid_first_marker_bit() {
        let data = [
            0b00111011, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        ];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Err(()));
    }
    #[test]
    fn test_optional_fields_unmarshall_escr_invalid_second_marker_bit() {
        let data = [
            0b00111111, 0b11111111, 0b11111011, 0b11111111, 0b11111111, 0b11111111,
        ];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Err(()));
    }
    #[test]
    fn test_optional_fields_unmarshall_escr_invalid_third_marker_bit() {
        let data = [
            0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111011, 0b11111111,
        ];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Err(()));
    }
    #[test]
    fn test_optional_fields_unmarshall_escr_invalid_fourth_marker_bit() {
        let data = [
            0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111110,
        ];
        let result = OptionalFields::unmarshall_escr(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_es_rate() {
        let data = [0b11111111, 0b11111111, 0b11111111];
        let result = OptionalFields::unmarshall_es_rate(&data);
        assert_eq!(result, Ok(4_194_303));
    }

    #[test]
    fn test_optional_fields_unmarshall_es_rate_invalid() {
        let data = [0b11111111, 0b11111111];
        let result = OptionalFields::unmarshall_es_rate(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_es_rate_invalid_first_marker_bit() {
        let data = [0b01111111, 0b11111111, 0b11111111];
        let result = OptionalFields::unmarshall_es_rate(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_optional_fields_unmarshall_es_rate_invalid_second_marker_bit() {
        let data = [0b11111111, 0b11111111, 0b11111110];
        let result = OptionalFields::unmarshall_es_rate(&data);
        assert_eq!(result, Err(()));
    }
}

#[cfg(test)]
mod pes_extension_data_tests {
    use super::*;

    #[test]
    fn test_unmarshall_tref() {
        let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let result = PesExtensionData::unmarshall_tref(&data);
        assert_eq!(result, Ok(8_589_934_591));
    }

    #[test]
    fn test_unmarshall_tref_invalid() {
        let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let result = PesExtensionData::unmarshall_tref(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_unmarshall_tref_invalid_first_marker_bit() {
        let data = [0b11111110, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let result = PesExtensionData::unmarshall_tref(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_unmarshall_tref_invalid_second_marker_bit() {
        let data = [0b11111111, 0b11111111, 0b11111110, 0b11111111, 0b11111111];
        let result = PesExtensionData::unmarshall_tref(&data);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_unmarshall_tref_invalid_third_marker_bit() {
        let data = [0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111110];
        let result = PesExtensionData::unmarshall_tref(&data);
        assert_eq!(result, Err(()));
    }
}
