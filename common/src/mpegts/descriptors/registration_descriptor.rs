use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct RegistrationDescriptor {
    pub header: DescriptorHeader,
    pub format_identifier: u32,
    pub additional_identification_info: Vec<u8>,
}

impl PartialEq for RegistrationDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.format_identifier == other.format_identifier
            && self.additional_identification_info == other.additional_identification_info
    }
}

impl ParsableDescriptor<RegistrationDescriptor> for RegistrationDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<RegistrationDescriptor> {
        if data.len() < 4 {
            return None;
        }

        let format_identifier = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let additional_identification_info = data[4..].to_vec();

        Some(RegistrationDescriptor {
            header,
            format_identifier,
            additional_identification_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, DescriptorTag};

    #[test]
    fn test_registration_descriptor() {
        let bytes = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::RegistrationDescriptorTag,
            descriptor_length: 5,
        };

        let descriptor = RegistrationDescriptor {
            header: header.clone(),
            format_identifier: 0,
            additional_identification_info: vec![0, 0],
        };

        assert_eq!(descriptor, RegistrationDescriptor::unmarshall(header, &bytes).unwrap());
    }

}