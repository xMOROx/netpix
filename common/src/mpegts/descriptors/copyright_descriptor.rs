use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};

implement_descriptor! {
    pub struct CopyrightDescriptor {
        pub copyright_identifier: u32,
        pub additional_copyright_info: Vec<u8>,
    }
    unmarshall_impl: |header, data| {
        if data.len() < 4 {
            return None;
        }

        let reader = BitReader::new(data);
        let copyright_identifier = reader.get_bits_u32(0)?;

        Some(CopyrightDescriptor {
            header,
            copyright_identifier,
            additional_copyright_info: reader.remaining_from(4)?,
        })
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

        assert_eq!(
            CopyrightDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
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

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0B),
            descriptor_length: 6,
        };
        let descriptor = CopyrightDescriptor {
            header: header.clone(),
            copyright_identifier: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
            additional_copyright_info: vec![0x05, 0x06],
        };

        assert_eq!(
            format!("{}", descriptor),
            "Copyright Descriptor\nCopyright Identifier: 16909060\nAdditional Copyright Info: [5, 6]\n"
        );
    }
}
