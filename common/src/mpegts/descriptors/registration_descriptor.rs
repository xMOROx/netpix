use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct RegistrationDescriptor {
    pub header: DescriptorHeader,
    pub format_identifier: u32,
    pub additional_identification_info: Vec<u8>,
}

impl std::fmt::Display for RegistrationDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Format Identifier: {}\nAdditional Identification Info: {:?}",
            self.format_identifier, self.additional_identification_info
        )
    }
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

        let reader = BitReader::new(data);
        let format_identifier = reader.get_bits_u32(0)?;
        let additional_identification_info = if let Some(remaining) = reader.remaining_from(4) {
            remaining.to_vec()
        } else {
            vec![]
        };

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
        let bytes = vec![0x45, 0x41, 0x43, 0x33];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::RegistrationDescriptorTag,
            descriptor_length: 4,
        };

        let descriptor = RegistrationDescriptor {
            header: header.clone(),
            format_identifier: 0x45414333,
            additional_identification_info: vec![],
        };

        assert_eq!(
            descriptor,
            RegistrationDescriptor::unmarshall(header, &bytes).unwrap()
        );
    }
}
