use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct PrivateDataIndicatorDescriptor {
    pub header: DescriptorHeader,
    pub private_data_indicator: u32,
}

impl ParsableDescriptor<PrivateDataIndicatorDescriptor> for PrivateDataIndicatorDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<PrivateDataIndicatorDescriptor> {
        if data.len() < 4 {
            return None;
        }
        Some(PrivateDataIndicatorDescriptor {
            header: header.clone(),
            private_data_indicator: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }
}

impl std::fmt::Display for PrivateDataIndicatorDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Private Data Indicator: {}", self.private_data_indicator)
    }
}

impl PartialEq for PrivateDataIndicatorDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.private_data_indicator == other.private_data_indicator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_private_data_indicator_descriptor_unmarshall() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0F),
            descriptor_length: data.len() as u8,
        };
        let descriptor = PrivateDataIndicatorDescriptor {
            header: header.clone(),
            private_data_indicator: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
        };

        assert_eq!(PrivateDataIndicatorDescriptor::unmarshall(header, &data), Some(descriptor));
    }
}