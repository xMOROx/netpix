use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const HORIZONTAL_SIZE: u8 = 0b11111100;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_SIZE_UP: u8 = 0b11000000;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_SIZE_MIDDLE_1: u8 = 0b11111100;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_SIZE_MIDDLE_2: u8 = 0b00000011;
#[cfg(not(target_arch = "wasm32"))]
const VERTICAL_SIZE_DOWN: u8 = 0b11110000;
#[cfg(not(target_arch = "wasm32"))]
const ASPECT_RATIO_INFORMATION: u8 = 0b00001111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct TargetBackgroundGridDescriptor {
    pub header: DescriptorHeader,
    pub horizontal_size: u16,
    pub vertical_size: u16,
    pub aspect_ratio_information: u8,
}

impl std::fmt::Display for TargetBackgroundGridDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Horizontal Size: {}, Vertical Size: {}, Aspect Ratio Information: {}", self.horizontal_size, self.vertical_size, self.aspect_ratio_information)
    }
}

impl PartialEq for TargetBackgroundGridDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.horizontal_size == other.horizontal_size &&
            self.vertical_size == other.vertical_size &&
            self.aspect_ratio_information == other.aspect_ratio_information
    }
}

impl ParsableDescriptor<TargetBackgroundGridDescriptor> for TargetBackgroundGridDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<TargetBackgroundGridDescriptor> {
        if data.len() != 4 {
            return None;
        }

        Some(TargetBackgroundGridDescriptor {
            header,
            horizontal_size: u16::from_be_bytes([data[0], data[1] & HORIZONTAL_SIZE ]) >> 2,
            vertical_size: ((data[1] & VERTICAL_SIZE_UP) as u16) << 6 | ((data[2] & VERTICAL_SIZE_MIDDLE_1) as u16) << 4 | ((data[2] & VERTICAL_SIZE_MIDDLE_2) as u16) << 4 | ((data[3] & VERTICAL_SIZE_DOWN) as u16) >> 4,
            aspect_ratio_information: data[3] & ASPECT_RATIO_INFORMATION,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
    use crate::mpegts::descriptors::tags::DescriptorTag;

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
            descriptor_length: 0x04
        };
        let descriptor = TargetBackgroundGridDescriptor {
            header,
            horizontal_size: 0x1234,
            vertical_size: 0x5678,
            aspect_ratio_information: 0x08
        };
        assert_eq!(descriptor.descriptor_tag(), 0x07);
    }

    #[test]
    fn test_descriptor_length() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::TargetBackgroundGridDescriptorTag,
            descriptor_length: 0x04
        };
        let descriptor = TargetBackgroundGridDescriptor {
            header,
            horizontal_size: 0x1234,
            vertical_size: 0x5678,
            aspect_ratio_information: 0x08
        };
        assert_eq!(descriptor.descriptor_length(), 0x04);
    }
}