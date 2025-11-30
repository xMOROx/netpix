#[cfg(test)]
mod tests;

use crate::utils::traits::{BitManipulation, DataParser, DataValidator};
use crate::utils::{BitReader, PesExtensionReader, TimestampReader};
use bincode::{Decode, Encode};

use super::constants::*;
use super::enums::PtsDtsFlags;
use super::trick_mode_control::TrickModeControl;

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
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
#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
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

#[derive(Decode, Encode, Debug, Clone, Eq, PartialEq)]
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

impl DataValidator for OptionalFields {
    fn validate(&self) -> bool {
        // Validate PTS/DTS consistency
        match (self.pts, self.dts) {
            (None, Some(_)) => false,                     // DTS without PTS is invalid
            (Some(pts), Some(dts)) if pts < dts => false, // PTS must be >= DTS
            _ => true,
        }
    }
}

impl BitManipulation for OptionalFields {}

impl DataParser for OptionalFields {
    type Output = (Self, usize); // Return parsed fields and consumed bytes

    fn parse(data: &[u8]) -> Option<Self::Output> {
        if data.len() < 2 {
            return None;
        }

        let context = ContextFlags::parse(data)?;
        Self::unmarshall(&data[2..], context)
    }
}

impl OptionalFields {
    pub fn build(data: &[u8], context_flags: ContextFlags) -> Option<Self> {
        Self::unmarshall(data, context_flags).map(|(fields, _)| fields)
    }

    pub(super) fn unmarshall(data: &[u8], context_flags: ContextFlags) -> Option<(Self, usize)> {
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

        Some((
            Self {
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
            },
            index,
        ))
    }

    fn unmarshall_pts_dts(data: &[u8]) -> Result<u64, ()> {
        if data.len() < 5 {
            return Err(());
        }

        if !Self::validate_marker_bits(data, &[0, 2, 4], 0x01) {
            return Err(());
        }

        Ok(Self::extract_timestamp(data))
    }

    fn extract_timestamp(data: &[u8]) -> u64 {
        let ts_1 = ((data[0] & 0x0E) as u64) << 29;
        let ts_2 = (data[1] as u64) << 22;
        let ts_3 = ((data[2] & 0xFE) as u64) << 14;
        let ts_4 = (data[3] as u64) << 7;
        let ts_5 = ((data[4] & 0xFE) as u64) >> 1;

        ts_1 | ts_2 | ts_3 | ts_4 | ts_5
    }

    fn validate_marker_bits(data: &[u8], positions: &[usize], mask: u8) -> bool {
        positions
            .iter()
            .all(|&pos| !is_invalid_marker_bit(data[pos], mask))
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
        let extension_reader = PesExtensionReader::new(data);
        let mut index = 0;

        let (
            pes_private_data_flag,
            pack_header_field_flag,
            program_packet_sequence_counter_flag,
            p_std_buffer_flag,
            pes_extension_flag_2,
        ) = extension_reader.read_flags()?;

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
            pes_private_data = Some(extension_reader.read_private_data(index)?);
            index += 16;
        }

        if pack_header_field_flag {
            let reader = BitReader::at_position(data, index);
            pack_field_length = reader.get_bits(0, 0xFF, 0);
            index += pack_field_length.unwrap_or(0) as usize + 1;
        }

        if program_packet_sequence_counter_flag {
            let (counter, identifier, stuff_length) =
                extension_reader.read_sequence_counter(index)?;
            program_packet_sequence_counter = Some(counter);
            mpeg1_mpeg2_identifier = Some(identifier);
            original_stuff_length = Some(stuff_length);
            index += 2;
        }

        if p_std_buffer_flag {
            let (scale, size) = extension_reader.read_buffer_info(index)?;
            p_std_buffer_scale = Some(scale);
            p_std_buffer_size = Some(size);
            index += 2;
        }

        if pes_extension_flag_2 {
            let reader = BitReader::at_position(data, index);
            if !reader.get_bit(0, 7)? {
                return None;
            }

            let mut bytes_after_pes_extension_field_length = 1;
            pes_extension_field_length = reader.get_bits(0, 0x7F, 0);
            index += 1;

            stream_id_extension_flag = reader.get_bit(0, 7);
            if let Some(false) = stream_id_extension_flag {
                stream_id_extension = reader.get_bits(0, 0x7F, 0);
                index += 1;
            } else if let Some(true) = stream_id_extension_flag {
                tref_extension_flag = reader.get_bit(0, 0);
                index += 1;
                if let Some(false) = tref_extension_flag {
                    tref = Some(TimestampReader::new(&data[index..]).read_tref().ok()?);
                    index += 5;
                    bytes_after_pes_extension_field_length += 5;
                }
            }

            if let Some(len) = pes_extension_field_length {
                index += len as usize - bytes_after_pes_extension_field_length;
            }
        }

        Some(Self {
            size: index as u8,
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

    // Remove unmarshall_tref as it's now handled by TimestampReader
}

impl DataParser for PesExtensionData {
    type Output = Self;

    fn parse(data: &[u8]) -> Option<Self::Output> {
        Self::unmarshall(data)
    }
}

impl BitManipulation for PesExtensionData {}

impl DataValidator for PesExtensionData {
    fn validate(&self) -> bool {
        if let Some(len) = self.pes_extension_field_length
            && len as usize > self.size as usize
        {
            return false;
        }
        true
    }
}

impl DataParser for ContextFlags {
    type Output = Self;

    fn parse(data: &[u8]) -> Option<Self::Output> {
        if data.is_empty() {
            return None;
        }

        Some(ContextFlags {
            pts_dts_flags: Self::get_bits(data[0], PTS_DTS_FLAGS_MASK, 6),
            escr_flag: Self::get_bit(data[0], 5),
            es_rate_flag: Self::get_bit(data[0], 4),
            dsm_trick_mode_flag: Self::get_bit(data[0], 3),
            additional_copy_info_flag: Self::get_bit(data[0], 2),
            pes_crc_flag: Self::get_bit(data[0], 1),
            pes_extension_flag: Self::get_bit(data[0], 0),
        })
    }
}

impl BitManipulation for ContextFlags {}

fn is_invalid_marker_bit(byte: u8, mask: u8) -> bool {
    byte & mask != mask
}
