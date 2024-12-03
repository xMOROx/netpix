use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

implement_descriptor! {
    pub struct RegistrationDescriptor {
        pub format_identifier: u32,
        pub additional_identification_info: Vec<u8>
    }
    unmarshall_impl: |header, data| {
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

    #[test]
    fn test_should_display_audio_stream_descriptor() {
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
            format!("{}", descriptor),
            "Registration Descriptor\nFormat Identifier: 1161904947\nAdditional Identification Info: []\n"
        );
    }
}
