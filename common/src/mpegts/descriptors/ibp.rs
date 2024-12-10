use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

implement_descriptor! {
    pub struct IbpDescriptor {
        pub closed_gop: bool,
        pub identical_gop: bool,
        pub max_gop_length: u16,

    }
    unmarshall_impl: |header, data| {
        if data.len() != 2 {
            return None;
        }

        let reader = BitReader::new(data);

        Some(IbpDescriptor {
            header,
            closed_gop: reader.get_bit(0, 7)?,
            identical_gop: reader.get_bit(0, 6)?,
            max_gop_length: reader.get_bits_u16(0, 0x3F, 0xFF)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_ibp_descriptor_unmarshall() {
        let data = vec![0xFF, 0xFF];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x05),
            descriptor_length: 0x03,
        };
        let descriptor = IbpDescriptor {
            header: header.clone(),
            closed_gop: true,
            identical_gop: true,
            max_gop_length: 0x3FFF,
        };

        assert_eq!(IbpDescriptor::unmarshall(header, &data), Some(descriptor));
    }

    #[test]
    fn test_ibp_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x05),
            descriptor_length: 0x03,
        };
        let descriptor = IbpDescriptor {
            header: header.clone(),
            closed_gop: true,
            identical_gop: true,
            max_gop_length: 0x3FFF,
        };

        assert_eq!(descriptor, descriptor);
    }

    #[test]
    fn test_ibp_descriptor_ne() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x05),
            descriptor_length: 0x03,
        };
        let descriptor = IbpDescriptor {
            header: header.clone(),
            closed_gop: true,
            identical_gop: true,
            max_gop_length: 0x3FFF,
        };

        let descriptor2 = IbpDescriptor {
            header: header.clone(),
            closed_gop: true,
            identical_gop: true,
            max_gop_length: 0x3FFE,
        };

        assert_ne!(descriptor, descriptor2);
    }

    #[test]
    fn test_should_display_ibp_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x05),
            descriptor_length: 0x03,
        };
        let descriptor = IbpDescriptor {
            header,
            closed_gop: true,
            identical_gop: true,
            max_gop_length: 0x3FFF,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Ibp Descriptor\nClosed Gop: true\nIdentical Gop: true\nMax Gop Length: 16383\n"
        );
    }
}
