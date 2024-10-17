use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const HORIZONTAL_OFFSET: u8 = 0b11111100;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_OFFSET_UP: u8 = 0b11000000;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_OFFSET_MIDDLE_1: u8 = 0b11111100;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_OFFSET_MIDDLE_2: u8 = 0b00000011;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_OFFSET_DOWN: u8 = 0b11110000;
#[cfg(not(target_arch = "wasm32"))]
const WINDOW_PRIORITY: u8 = 0b00001111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct VideoWindowDescriptor {
    pub header: DescriptorHeader,
    pub horizontal_offset: u16,
    pub vertical_offset: u16,
    pub window_priority: u8,
}

impl PartialEq for VideoWindowDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.horizontal_offset == other.horizontal_offset &&
            self.vertical_offset == other.vertical_offset &&
            self.window_priority == other.window_priority
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

        Some(VideoWindowDescriptor {
            header,
            horizontal_offset: u16::from_be_bytes([data[0], data[1] & HORIZONTAL_OFFSET ]) >> 2,
            vertical_offset: ((data[1] & VERTICAL_OFFSET_UP) as u16) << 6 | ((data[2] & VERTICAL_OFFSET_MIDDLE_1) as u16) << 4 | ((data[2] & VERTICAL_OFFSET_MIDDLE_2) as u16) << 4 | ((data[3] & VERTICAL_OFFSET_DOWN) as u16) >> 4,
            window_priority: data[3] & WINDOW_PRIORITY,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
    use crate::mpegts::descriptors::tags::DescriptorTag;

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