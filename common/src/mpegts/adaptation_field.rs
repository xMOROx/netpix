use serde::{Deserialize, Serialize};

const STUFFING_BYTE: u8 = 0xFF;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

impl AdaptationField {
    pub fn unmarshall(buffer: &[u8]) -> Option<Self> {
        if buffer[0] == 0 || buffer[0] > buffer.len() as u8 {
            return None;
        }

        let adaptation_field_length = buffer[0];

        let mut index = 1;

        // Basic flags from control byte
        let discontinuity_indicator = (buffer[index] & 0x80) == 0x80;
        let random_access_indicator = (buffer[index] & 0x40) == 0x40;
        let elementary_stream_priority_indicator = (buffer[index] & 0x20) == 0x20;
        let pcr_flag = (buffer[index] & 0x10) == 0x10;
        let opcr_flag = (buffer[index] & 0x08) == 0x08;
        let splicing_point_flag = (buffer[index] & 0x04) == 0x04;
        let transport_private_data_flag = (buffer[index] & 0x02) == 0x02;
        let adaptation_field_extension_flag = (buffer[index] & 0x01) == 0x01;

        index += 1;

        let mut field = AdaptationField {
            adaptation_field_length,
            discontinuity_indicator,
            random_access_indicator,
            elementary_stream_priority_indicator,
            pcr_flag,
            opcr_flag,
            splicing_point_flag,
            transport_private_data_flag,
            adaptation_field_extension_flag,
            program_clock_reference_base: None,
            program_clock_reference_extension: None,
            original_program_clock_reference_base: None,
            original_program_clock_reference_extension: None,
            splice_countdown: None,
            transport_private_data_length: None,
            transport_private_data: None,
            adaptation_field_extension: None,
            number_of_stuffing_bytes: None,
        };

        if pcr_flag && index + 6 <= buffer.len() {
            field.program_clock_reference_base = Some(
                ((buffer[index] as u64) << 25)
                    | ((buffer[index + 1] as u64) << 17)
                    | ((buffer[index + 2] as u64) << 9)
                    | ((buffer[index + 3] as u64) << 1)
                    | ((buffer[index + 4] & 0x80) as u64 >> 7),
            );
            field.program_clock_reference_extension =
                Some(((buffer[index + 4] & 0x01) as u16) << 8 | buffer[index + 5] as u16);
            index += 6;
        }

        if opcr_flag && index + 6 <= buffer.len() {
            field.original_program_clock_reference_base = Some(
                ((buffer[index] as u64) << 25)
                    | ((buffer[index + 1] as u64) << 17)
                    | ((buffer[index + 2] as u64) << 9)
                    | ((buffer[index + 3] as u64) << 1)
                    | ((buffer[index + 4] & 0x80) as u64 >> 7),
            );
            field.original_program_clock_reference_extension =
                Some(((buffer[index + 4] & 0x01) as u16) << 8 | buffer[index + 5] as u16);
            index += 6;
        }

        if splicing_point_flag && index < buffer.len() {
            field.splice_countdown = Some(buffer[index]);
            index += 1;
        }

        if transport_private_data_flag && index < buffer.len() {
            let length = buffer[index] as usize;
            index += 1;
            if index + length <= buffer.len() {
                field.transport_private_data_length = Some(length as u8);
                field.transport_private_data = Some(buffer[index..index + length].to_vec());
                index += length;
            }
        }

        if adaptation_field_extension_flag && index < buffer.len() {
            field.adaptation_field_extension =
                AdaptationFieldExtension::unmarshall(&buffer[index..]);
        }

        let mut stuffing_count = 0;

        while index <= adaptation_field_length as usize && index < buffer.len() {
            if buffer[index] == STUFFING_BYTE {
                stuffing_count += 1;
                index += 1;
            } else {
                break;
            }
        }

        field.number_of_stuffing_bytes = Some(stuffing_count);
        Some(field)
    }
}

impl AdaptationFieldExtension {
    pub fn unmarshall(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 2 {
            return None;
        }

        let adaptation_field_extension_length = buffer[0];
        let mut index = 1;

        let ltw_flag = (buffer[index] & 0x80) == 0x80;
        let piecewise_rate_flag = (buffer[index] & 0x40) == 0x40;
        let seamless_splice_flag = (buffer[index] & 0x20) == 0x20;
        let af_descriptor_not_present_float = (buffer[index] & 0x10) == 0x10;
        index += 1;

        let mut extension = AdaptationFieldExtension {
            adaptation_field_extension_length,
            ltw_flag,
            piecewise_rate_flag,
            seamless_splice_flag,
            af_descriptor_not_present_float,
            ltw_valid_flag: None,
            ltw_offset: None,
            piecewise_rate: None,
            splice_type: None,
            dts_next_access_unit: None,
            reserved: None,
        };

        if ltw_flag && index + 2 <= buffer.len() {
            extension.ltw_valid_flag = Some((buffer[index] & 0x80) == 0x80);
            extension.ltw_offset =
                Some(((buffer[index] & 0x7F) as u16) << 8 | buffer[index + 1] as u16);
            index += 2;
        }

        if piecewise_rate_flag && index + 3 <= buffer.len() {
            extension.piecewise_rate = Some(
                ((buffer[index] as u32) << 16)
                    | ((buffer[index + 1] as u32) << 8)
                    | buffer[index + 2] as u32,
            );
            index += 3;
        }

        if seamless_splice_flag && index + 5 <= buffer.len() {
            extension.splice_type = Some(buffer[index] >> 4);
            extension.dts_next_access_unit = Some(
                ((buffer[index] as u32 & 0x0E) << 30)
                    | ((buffer[index + 1] as u32) << 22)
                    | ((buffer[index + 2] as u32) << 14)
                    | ((buffer[index + 3] as u32 & 0xFE) << 7)
                    | (buffer[index + 4] as u32 >> 1),
            );
            index += 5;
        }

        if af_descriptor_not_present_float && index < buffer.len() {
            extension.reserved = Some(buffer[index]);
        }

        Some(extension)
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
        assert_eq!(field.splice_countdown.unwrap(), 0x42 as u8);
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
        let buffer = vec![1, 0xFF]; // All flags set
        let field = AdaptationField::unmarshall(&buffer).unwrap();

        assert!(field.discontinuity_indicator);
        assert!(field.random_access_indicator);
        assert!(field.elementary_stream_priority_indicator);
        assert!(field.pcr_flag);
        assert!(field.opcr_flag);
        assert!(field.splicing_point_flag);
        assert!(field.transport_private_data_flag);
        assert!(field.adaptation_field_extension_flag);
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
