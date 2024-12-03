use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const HORIZONTAL_OFFSET_MASK: u8 = 0b1111_1100;
const VERTICAL_OFFSET_UP_MASK: u8 = 0b1100_0000;
const VERTICAL_OFFSET_MIDDLE_1_MASK: u8 = 0b1111_1100;
const VERTICAL_OFFSET_MIDDLE_2_MASK: u8 = 0b0000_0011;
const VERTICAL_OFFSET_DOWN_MASK: u8 = 0b1111_0000;
const WINDOW_PRIORITY_MASK: u8 = 0b0000_1111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct VideoWindowDescriptor {
    pub header: DescriptorHeader,
    pub horizontal_offset: u16,
    pub vertical_offset: u16,
    pub window_priority: u8,
}

impl std::fmt::Display for VideoWindowDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Horizontal Offset: {}, Vertical Offset: {}, Window Priority: {}",
            self.horizontal_offset, self.vertical_offset, self.window_priority
        )
    }
}

impl PartialEq for VideoWindowDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.horizontal_offset == other.horizontal_offset
            && self.vertical_offset == other.vertical_offset
            && self.window_priority == other.window_priority
    }
}

impl ParsableDescriptor<VideoWindowDescriptor> for VideoWindowDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<VideoWindowDescriptor> {
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
}
