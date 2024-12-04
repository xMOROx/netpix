use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const HORIZONTAL_OFFSET_MASK: u8 = 0b1111_1100;
const VERTICAL_OFFSET_UP_MASK: u8 = 0b1100_0000;
const VERTICAL_OFFSET_MIDDLE_1_MASK: u8 = 0b1111_1100;
const VERTICAL_OFFSET_MIDDLE_2_MASK: u8 = 0b0000_0011;
const VERTICAL_OFFSET_DOWN_MASK: u8 = 0b1111_0000;
const WINDOW_PRIORITY_MASK: u8 = 0b0000_1111;

implement_descriptor! {
    pub struct VideoWindowDescriptor {
        pub horizontal_offset: u16,
        pub vertical_offset: u16,
        pub window_priority: u8,
    }
    unmarshall_impl: |header, data| {
        if data.len() != 4 {
            return None;
        }

        let reader = BitReader::new(data);

        let horizontal_offset = reader.get_bits_u16(0, 0xFF, HORIZONTAL_OFFSET_MASK)? >> 2;

        let vertical_up = ((data[1] & VERTICAL_OFFSET_UP_MASK) as u16) << 6;
        let vertical_middle = ((data[2] & VERTICAL_OFFSET_MIDDLE_1_MASK) as u16) << 4;
        let vertical_middle_2 = ((data[2] & VERTICAL_OFFSET_MIDDLE_2_MASK) as u16) << 4;
        let vertical_down = ((data[3] & VERTICAL_OFFSET_DOWN_MASK) as u16) >> 4;
        let vertical_offset = vertical_up | vertical_middle | vertical_middle_2 | vertical_down;

        let window_priority = reader.get_bits(3, WINDOW_PRIORITY_MASK, 0)?;

        Some(VideoWindowDescriptor {
            header,
            horizontal_offset,
            vertical_offset,
            window_priority,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

    #[test]
    fn test_video_window_descriptor() {
        let bytes = vec![0xCB, 0xC7, 0x3D, 0x8D];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::VideoWindowDescriptorTag,
            descriptor_length: 0x04,
        };
        let descriptor = VideoWindowDescriptor::unmarshall(header.clone(), &bytes).unwrap();
        assert_eq!(descriptor.header, header);
        assert_eq!(descriptor.horizontal_offset, 0x32F1);
        assert_eq!(descriptor.vertical_offset, 0x33D8);
        assert_eq!(descriptor.window_priority, 0x0D);
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let descriptor = VideoWindowDescriptor {
            header: DescriptorHeader {
                descriptor_tag: DescriptorTag::VideoWindowDescriptorTag,
                descriptor_length: 0x04,
            },
            horizontal_offset: 0x32F1,
            vertical_offset: 0x33D8,
            window_priority: 0x0D,
        };
        assert_eq!(
            descriptor.to_string(),
            "Video Window Descriptor\nHorizontal Offset: 13041\nVertical Offset: 13272\nWindow Priority: 13\n"
        );
    }
}
