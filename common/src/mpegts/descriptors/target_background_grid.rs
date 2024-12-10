use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

const HORIZONTAL_SIZE_MASK: u8 = 0b1111_1100;
const VERTICAL_SIZE_UP_MASK: u8 = 0b1100_0000;
const VERTICAL_SIZE_MIDDLE_1_MASK: u8 = 0b1111_1100;
const VERTICAL_SIZE_MIDDLE_2_MASK: u8 = 0b0000_0011;
const VERTICAL_SIZE_DOWN_MASK: u8 = 0b1111_0000;
const ASPECT_RATIO_MASK: u8 = 0b0000_1111;

implement_descriptor! {
    pub struct TargetBackgroundGridDescriptor {
        pub horizontal_size: u16,
        pub vertical_size: u16,
        pub aspect_ratio_information: u8
    }
    unmarshall_impl: |header, data| {
        if data.len() != 4 {
            return None;
        }

        let reader = BitReader::new(data);

        let horizontal_size = reader.get_bits_u16(0, 0xFF, HORIZONTAL_SIZE_MASK)? >> 2;

        let vertical_up = ((data[1] & VERTICAL_SIZE_UP_MASK) as u16) << 6;
        let vertical_middle = ((data[2] & VERTICAL_SIZE_MIDDLE_1_MASK) as u16) << 4;
        let vertical_middle_2 = ((data[2] & VERTICAL_SIZE_MIDDLE_2_MASK) as u16) << 4;
        let vertical_down = ((data[3] & VERTICAL_SIZE_DOWN_MASK) as u16) >> 4;
        let vertical_size = vertical_up | vertical_middle | vertical_middle_2 | vertical_down;

        let aspect_ratio_information = reader.get_bits(3, ASPECT_RATIO_MASK, 0)?;

        Some(TargetBackgroundGridDescriptor {
            header,
            horizontal_size,
            vertical_size,
            aspect_ratio_information,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

    #[test]
    fn test_unmarshall() {
        let data = vec![0xCB, 0xC7, 0x3D, 0x8D];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::TargetBackgroundGridDescriptorTag,
            descriptor_length: 0x04,
        };
        let descriptor = TargetBackgroundGridDescriptor::unmarshall(header.clone(), &data).unwrap();
        assert_eq!(descriptor.header, header);
        assert_eq!(descriptor.horizontal_size, 0x32F1);
        assert_eq!(descriptor.vertical_size, 0x33D8);
        assert_eq!(descriptor.aspect_ratio_information, 0x0D);
    }

    #[test]
    fn test_descriptor_tag() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::TargetBackgroundGridDescriptorTag,
            descriptor_length: 0x04,
        };
        let descriptor = TargetBackgroundGridDescriptor {
            header,
            horizontal_size: 0x1234,
            vertical_size: 0x5678,
            aspect_ratio_information: 0x08,
        };
        assert_eq!(descriptor.descriptor_tag(), 0x07);
    }

    #[test]
    fn test_descriptor_length() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::TargetBackgroundGridDescriptorTag,
            descriptor_length: 0x04,
        };
        let descriptor = TargetBackgroundGridDescriptor {
            header,
            horizontal_size: 0x1234,
            vertical_size: 0x5678,
            aspect_ratio_information: 0x08,
        };
        assert_eq!(descriptor.descriptor_length(), 0x04);
    }

    #[test]
    fn test_audio_stream_descriptor_unmarshall() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::TargetBackgroundGridDescriptorTag,
            descriptor_length: 0x04,
        };
        let descriptor = TargetBackgroundGridDescriptor {
            header: header.clone(),
            horizontal_size: 0x32F1,
            vertical_size: 0x33D8,
            aspect_ratio_information: 0x0D,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Target Background Grid Descriptor\nHorizontal Size: 13041\nVertical Size: 13272\nAspect Ratio Information: 13\n"
        );
    }
}
