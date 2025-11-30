use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};

const STUFFING_BYTE: u8 = 0xFF;
const LTW_OFFSET_MASK: u8 = 0x7F;

#[derive(Default, Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct AdaptationField {
    pub adaptation_field_length: u8,
    pub discontinuity_indicator: bool,
    pub random_access_indicator: bool,
    pub elementary_stream_priority_indicator: bool,
    pub pcr_flag: bool,
    pub opcr_flag: bool,
    pub splicing_point_flag: bool,
    pub transport_private_data_flag: bool,
    pub adaptation_field_extension_flag: bool,
    pub program_clock_reference_base: Option<u64>,
    pub program_clock_reference_extension: Option<u16>,
    pub original_program_clock_reference_base: Option<u64>,
    pub original_program_clock_reference_extension: Option<u16>,
    pub splice_countdown: Option<u8>,
    pub transport_private_data_length: Option<u8>,
    pub transport_private_data: Option<Vec<u8>>,
    pub adaptation_field_extension: Option<AdaptationFieldExtension>,
    pub number_of_stuffing_bytes: Option<u8>,
}

#[derive(Default, Decode, Encode, Debug, Clone, Eq, PartialEq)]
pub struct AdaptationFieldExtension {
    pub adaptation_field_extension_length: u8,
    pub ltw_flag: bool,
    pub piecewise_rate_flag: bool,
    pub seamless_splice_flag: bool,
    pub af_descriptor_not_present_float: bool,
    pub ltw_valid_flag: Option<bool>,
    pub ltw_offset: Option<u16>,
    pub piecewise_rate: Option<u32>,
    pub splice_type: Option<u8>,
    pub dts_next_access_unit: Option<u32>,
    // pub af_descriptor: Option<AdaptationFieldExtensionDescriptor>,
    pub reserved: Option<u8>,
}

#[derive(Default, Clone, Copy)]
struct ControlFlags {
    pub discontinuity_indicator: bool,
    pub random_access_indicator: bool,
    pub elementary_stream_priority_indicator: bool,
    pub pcr_flag: bool,
    pub opcr_flag: bool,
    pub splicing_point_flag: bool,
    pub transport_private_data_flag: bool,
    pub adaptation_field_extension_flag: bool,
}

impl AdaptationField {
    pub fn unmarshall(buffer: &[u8]) -> Option<Self> {
        if buffer[0] == 0 || buffer[0] as usize > buffer.len() {
            return None;
        }

        let reader = BitReader::new(buffer);
        let adaptation_field_length = buffer[0];

        let flags = Self::read_control_flags(&reader)?;

        let mut index = 2;

        let mut field = AdaptationField {
            adaptation_field_length,
            ..Default::default()
        };

        field.merge_control_flags(flags);

        if flags.pcr_flag
            && let Some((base, ext)) = Self::read_pcr(&reader, index)
        {
            field.program_clock_reference_base = Some(base);
            field.program_clock_reference_extension = Some(ext);
            index += 6;
        }

        if flags.opcr_flag
            && let Some((base, ext)) = Self::read_pcr(&reader, index)
        {
            field.original_program_clock_reference_base = Some(base);
            field.original_program_clock_reference_extension = Some(ext);
            index += 6;
        }
        index = Self::parse_optional_fields(&reader, index, &mut field)?;

        let stuffing_count =
            Self::count_stuffing_bytes(&buffer[index..], adaptation_field_length as usize);
        field.number_of_stuffing_bytes = Some(stuffing_count);

        Some(field)
    }

    fn read_control_flags(reader: &BitReader) -> Option<ControlFlags> {
        Some(ControlFlags {
            discontinuity_indicator: reader.get_bit(1, 7)?,
            random_access_indicator: reader.get_bit(1, 6)?,
            elementary_stream_priority_indicator: reader.get_bit(1, 5)?,
            pcr_flag: reader.get_bit(1, 4)?,
            opcr_flag: reader.get_bit(1, 3)?,
            splicing_point_flag: reader.get_bit(1, 2)?,
            transport_private_data_flag: reader.get_bit(1, 1)?,
            adaptation_field_extension_flag: reader.get_bit(1, 0)?,
        })
    }

    fn read_pcr(reader: &BitReader, offset: usize) -> Option<(u64, u16)> {
        let base = ((reader.get_bits(offset, 0xFF, 0)? as u64) << 25)
            | ((reader.get_bits(offset + 1, 0xFF, 0)? as u64) << 17)
            | ((reader.get_bits(offset + 2, 0xFF, 0)? as u64) << 9)
            | ((reader.get_bits(offset + 3, 0xFF, 0)? as u64) << 1)
            | (reader.get_bits(offset + 4, 0x80, 7)? as u64);

        let ext = reader.get_bits_u16_with_shift(offset + 4, 0x01, 0xFF, 8)?;

        Some((base, ext))
    }

    fn parse_optional_fields(
        reader: &BitReader,
        mut index: usize,
        field: &mut AdaptationField,
    ) -> Option<usize> {
        if field.splicing_point_flag {
            field.splice_countdown = reader.get_bits(index, 0xFF, 0);
            index += 1;
        }

        if field.transport_private_data_flag {
            let length = reader.get_bits(index, 0xFF, 0)? as usize;
            index += 1;
            field.transport_private_data_length = Some(length as u8);
            field.transport_private_data = reader.get_bytes(index, length);
            index += length;
        }

        if field.adaptation_field_extension_flag {
            field.adaptation_field_extension =
                AdaptationFieldExtension::unmarshall(&reader.remaining_from(index)?);
        }

        Some(index)
    }

    fn count_stuffing_bytes(data: &[u8], max_length: usize) -> u8 {
        data.iter()
            .take(max_length)
            .take_while(|&&byte| byte == STUFFING_BYTE)
            .count() as u8
    }

    fn merge_control_flags(&mut self, flags: ControlFlags) {
        self.discontinuity_indicator = flags.discontinuity_indicator;
        self.random_access_indicator = flags.random_access_indicator;
        self.elementary_stream_priority_indicator = flags.elementary_stream_priority_indicator;
        self.pcr_flag = flags.pcr_flag;
        self.opcr_flag = flags.opcr_flag;
        self.splicing_point_flag = flags.splicing_point_flag;
        self.transport_private_data_flag = flags.transport_private_data_flag;
        self.adaptation_field_extension_flag = flags.adaptation_field_extension_flag;
    }
}

impl AdaptationFieldExtension {
    pub fn unmarshall(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 2 {
            return None;
        }

        let reader = BitReader::new(buffer);
        let adaptation_field_extension_length = buffer[0];

        let extension = Self::read_extension_flags(&reader)?;
        let mut index = 2;

        let mut result = Self {
            adaptation_field_extension_length,
            ..extension
        };

        if extension.ltw_flag
            && let Some((valid, offset)) = Self::read_ltw(&reader, index)
        {
            result.ltw_valid_flag = Some(valid);
            result.ltw_offset = Some(offset);
            index += 2;
        }

        if extension.piecewise_rate_flag {
            result.piecewise_rate = reader.get_bits_u24(index);
            index += 3;
        }

        if extension.seamless_splice_flag
            && let Some((splice_type, dts)) = Self::read_splice_info(&reader, index)
        {
            result.splice_type = Some(splice_type);
            result.dts_next_access_unit = Some(dts);
            index += 5;
        }

        if extension.af_descriptor_not_present_float {
            result.reserved = reader.get_bits(index, 0xFF, 0);
        }

        Some(result)
    }

    fn read_extension_flags(reader: &BitReader) -> Option<Self> {
        Some(Self {
            ltw_flag: reader.get_bit(1, 7)?,
            piecewise_rate_flag: reader.get_bit(1, 6)?,
            seamless_splice_flag: reader.get_bit(1, 5)?,
            af_descriptor_not_present_float: reader.get_bit(1, 4)?,
            ..Default::default()
        })
    }

    fn read_ltw(reader: &BitReader, offset: usize) -> Option<(bool, u16)> {
        let valid = reader.get_bit(offset, 7)?;
        let offset = reader.get_bits_u16_with_shift(offset, LTW_OFFSET_MASK, 0xFF, 8)?;
        Some((valid, offset))
    }

    fn read_splice_info(reader: &BitReader, offset: usize) -> Option<(u8, u32)> {
        let splice_type = reader.get_bits(offset, 0xF0, 4)?;
        let dts = ((reader.get_bits(offset, 0x0E, 1)? as u32) << 30)
            | (reader.get_bits(offset + 1, 0xFF, 0)? as u32) << 22
            | (reader.get_bits(offset + 2, 0xFF, 0)? as u32) << 14
            | (reader.get_bits(offset + 3, 0xFE, 1)? as u32) << 7
            | (reader.get_bits(offset + 4, 0xFE, 1)? as u32);

        Some((splice_type, dts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_length() {
        let buffer = vec![0];
        assert!(AdaptationField::unmarshall(&buffer).is_none());

        let buffer = vec![5, 0, 0]; // Length larger than buffer
        assert!(AdaptationField::unmarshall(&buffer).is_none());
    }

    #[test]
    fn test_minimal_adaptation_field() {
        let buffer = vec![1, 0]; // Length=1, no flags set
        let field = AdaptationField::unmarshall(&buffer).unwrap();

        assert_eq!(field.adaptation_field_length, 1);
        assert!(!field.discontinuity_indicator);
        assert!(!field.random_access_indicator);
        assert!(!field.elementary_stream_priority_indicator);
        assert!(!field.pcr_flag);
        assert!(!field.opcr_flag);
        assert!(!field.splicing_point_flag);
        assert!(!field.transport_private_data_flag);
        assert!(!field.adaptation_field_extension_flag);
    }

    #[test]
    fn test_splice_countdown() {
        // Length=2, splice flag set, countdown value
        let buffer = vec![2, 0x04, 0x42];
        let field = AdaptationField::unmarshall(&buffer).unwrap();

        assert!(field.splicing_point_flag);
        assert_eq!(field.splice_countdown.unwrap(), 0x42_u8);
    }

    #[test]
    fn test_multiple_flags() {
        // Length=10, PCR and private data flags set
        let buffer = vec![
            10, 0x12, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x02, 0xAA, 0xBB,
        ];
        let field = AdaptationField::unmarshall(&buffer).unwrap();

        assert!(field.pcr_flag);
        assert!(field.transport_private_data_flag);
        assert!(field.program_clock_reference_base.is_some());
        assert!(field.program_clock_reference_extension.is_some());
        assert_eq!(field.transport_private_data_length.unwrap(), 2);
        assert_eq!(
            field.transport_private_data.as_ref().unwrap(),
            &vec![0xAA, 0xBB]
        );
    }

    #[test]
    fn test_all_control_flags() {
        let mut buffer = vec![0x01, 0xFF]; // Start with length and flags

        // Add optional fields based on flags
        buffer.push(0xF7); // PCR base
        buffer.push(0x77);
        buffer.push(0x77);
        buffer.push(0x77);
        buffer.push(0x7F);
        buffer.push(0x7E); // PCR extension

        buffer.push(0xF7); // OPCR base
        buffer.push(0x77);
        buffer.push(0x77);
        buffer.push(0x77);
        buffer.push(0x7F);
        buffer.push(0x7E); // OPCR extension

        buffer.push(0x25); // Splice countdown

        buffer.push(0x02); // Transport private data length
        buffer.push(0xAA); // Transport private data
        buffer.push(0xBB);

        // Fill rest with stuffing bytes
        let rest = 255 - buffer.len();
        while buffer.len() < 255 {
            buffer.push(STUFFING_BYTE);
        }

        buffer[0] = (buffer.len() - 1) as u8; // Update length field
        let field = AdaptationField::unmarshall(&buffer).unwrap();

        assert!(field.discontinuity_indicator);
        assert!(field.random_access_indicator);
        assert!(field.elementary_stream_priority_indicator);
        assert!(field.pcr_flag);
        assert!(field.opcr_flag);
        assert!(field.splicing_point_flag);
        assert!(field.transport_private_data_flag);
        assert!(field.adaptation_field_extension_flag);
        assert_eq!(field.program_clock_reference_base.unwrap(), 0x1_EEEE_EEEE);
        assert_eq!(field.program_clock_reference_extension.unwrap(), 0x17E);
        assert_eq!(
            field.original_program_clock_reference_base.unwrap(),
            0x1_EEEE_EEEE
        );
        assert_eq!(
            field.original_program_clock_reference_extension.unwrap(),
            0x17E
        );
        assert_eq!(field.splice_countdown.unwrap(), 0x25);
        assert_eq!(field.transport_private_data_length.unwrap(), 2);
        assert_eq!(
            field.transport_private_data.as_ref().unwrap(),
            &vec![0xAA, 0xBB]
        );
        assert_eq!(field.number_of_stuffing_bytes.unwrap(), rest as u8);
    }

    #[test]
    fn test_incomplete_buffer() {
        // PCR flag set but not enough bytes
        let buffer = vec![7, 0x10, 0x00, 0x11];
        assert!(AdaptationField::unmarshall(&buffer).is_none());
    }

    #[test]
    fn test_stuffing_bytes() {
        // Length=5, no flags set, 4 stuffing bytes
        let buffer = vec![5, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        let field = AdaptationField::unmarshall(&buffer).unwrap();
        assert_eq!(field.number_of_stuffing_bytes.unwrap(), 4);

        // Length=3, no flags, 2 stuffing bytes
        let buffer = vec![3, 0x00, 0xFF, 0xFF];
        let field = AdaptationField::unmarshall(&buffer).unwrap();
        assert_eq!(field.number_of_stuffing_bytes.unwrap(), 2);

        // Length=2, no flags, 1 stuffing byte
        let buffer = vec![2, 0x00, 0xFF];
        let field = AdaptationField::unmarshall(&buffer).unwrap();
        assert_eq!(field.number_of_stuffing_bytes.unwrap(), 1);
    }
}
