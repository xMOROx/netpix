use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

implement_descriptor! {
    pub struct Mpeg4VideoDescriptor {
        pub visual_profile_and_level: u8,
    }
    unmarshall_impl: |header, data| {
        if data.len() != 1 {
            return None;
        }

        let reader = BitReader::new(data);

        Some(Mpeg4VideoDescriptor {
            header,
            visual_profile_and_level: reader.get_byte(0)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_mpeg4_video_descriptor_unmarshall() {
        let data = vec![0b1110_1000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x20),
            descriptor_length: 0x01,
        };
        let descriptor = Mpeg4VideoDescriptor {
            header: header.clone(),
            visual_profile_and_level: 0b1110_1000,
        };

        assert_eq!(
            Mpeg4VideoDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_mpeg4_video_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x20),
            descriptor_length: 0x01,
        };
        let descriptor = Mpeg4VideoDescriptor {
            header: header.clone(),
            visual_profile_and_level: 0b1110_1000,
        };

        assert_eq!(descriptor, descriptor);
    }

    #[test]
    fn test_mpeg4_video_descriptor_ne() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x20),
            descriptor_length: 0x01,
        };
        let descriptor = Mpeg4VideoDescriptor {
            header: header.clone(),
            visual_profile_and_level: 0b1110_1000,
        };

        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x20),
            descriptor_length: 0x01,
        };
        let descriptor2 = Mpeg4VideoDescriptor {
            header: header.clone(),
            visual_profile_and_level: 0b1110_1001,
        };

        assert_ne!(descriptor, descriptor2);
    }

    #[test]
    fn test_should_display_mpeg4_video_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x20),
            descriptor_length: 0x01,
        };
        let descriptor = Mpeg4VideoDescriptor {
            header,
            visual_profile_and_level: 0b1110_1000,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Mpeg4 Video Descriptor\nVisual Profile And Level: 232\n"
        );
    }
}
