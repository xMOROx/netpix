use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const MAXIMUM_BITRATE: u8 = 0b00111111;
const BITRATE_PER_SECOND: usize = 50;


#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct MaximumBitrateDescriptor {
    pub header: DescriptorHeader,
    pub maximum_bitrate: u32,
}

impl ParsableDescriptor<MaximumBitrateDescriptor> for MaximumBitrateDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<MaximumBitrateDescriptor> {
        if data.len() != 3 {
            return None;
        }

        let maximum_bitrate = u32::from(data[0] & MAXIMUM_BITRATE) << 16 | u32::from(data[1]) << 8 | u32::from(data[2]);

        Some(MaximumBitrateDescriptor {
            header,
            maximum_bitrate,
        })
    }
}

impl PartialEq for MaximumBitrateDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.maximum_bitrate == other.maximum_bitrate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, DescriptorTag};

    #[test]
    fn test_maximum_bitrate_descriptor() {
        let data = vec![0b00111111, 0b11111111, 0b11111111];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };

        let descriptor = MaximumBitrateDescriptor::unmarshall(header, &data).unwrap();
        assert_eq!(descriptor.maximum_bitrate, 0b00111111_11111111_11111111);
    }
    #[test]
    fn test_maximum_bitrate_descriptor_2() {
        let data = vec![0x0e, 0x03, 0xc0, 0x00, 0x00];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };

        let descriptor = MaximumBitrateDescriptor::unmarshall(header, &data[2..]).unwrap();
        assert_eq!(descriptor.maximum_bitrate, 0);
    }
    #[test]
    fn test_maximum_bitrate_descriptor_3() {
        let data = vec![0x0e, 0x03, 0xc0, 0x17, 0x15];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };

        let descriptor = MaximumBitrateDescriptor::unmarshall(header, &data[2..]).unwrap();
        assert_eq!(descriptor.maximum_bitrate, 5909);
    }
}