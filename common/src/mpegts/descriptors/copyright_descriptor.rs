use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct CopyrightDescriptor {
    pub header: DescriptorHeader,
    pub copyright_identifier: u32,
    pub additional_copyright_info: Vec<u8>,
}

impl ParsableDescriptor<CopyrightDescriptor> for CopyrightDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<CopyrightDescriptor> {
        if data.len() < 4 {
            return None;
        }
        Some(CopyrightDescriptor {
            header: header.clone(),
            copyright_identifier: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            additional_copyright_info: data[4..header.descriptor_length.clone() as usize].to_vec(),
        })
    }
}

impl PartialEq for CopyrightDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.copyright_identifier == other.copyright_identifier &&
            self.additional_copyright_info == other.additional_copyright_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_copyright_descriptor_unmarshall() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0B),
            descriptor_length: data.len() as u8,
        };
        let descriptor = CopyrightDescriptor {
            header: header.clone(),
            copyright_identifier: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
            additional_copyright_info: vec![0x05, 0x06],
        };

        assert_eq!(CopyrightDescriptor::unmarshall(header, &data), Some(descriptor));
    }

    #[test]
    fn test_copyright_descriptor_unmarshall_invalid_length() {
        let data = vec![0x01, 0x02, 0x03]; // Invalid length
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0B),
            descriptor_length: data.len() as u8,
        };

        assert_eq!(CopyrightDescriptor::unmarshall(header, &data), None);
    }

    #[test]
    fn test_copyright_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0B),
            descriptor_length: 6,
        };
        let descriptor1 = CopyrightDescriptor {
            header: header.clone(),
            copyright_identifier: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
            additional_copyright_info: vec![0x05, 0x06],
        };
        let descriptor2 = CopyrightDescriptor {
            header,
            copyright_identifier: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
            additional_copyright_info: vec![0x05, 0x06],
        };

        assert_eq!(descriptor1, descriptor2);
    }
}