use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

const SB_LEAK_RATE_DATA_IN_UNIT: u32 = 400; // 400 bits/s

implement_descriptor! {
    pub struct SmoothingBufferDescriptor {
        pub sb_leak_rate: u32,
        pub sb_size: u32,
    }
    unmarshall_impl: |header, data| {
        if data.len() != 6 {
            return None;
        }

        let reader = BitReader::new(data);

        let sb_leak_rate_1 = reader.get_bits(0, 0x3F, 0)?;
        let sb_leak_rate_2 = reader.get_bits_u16(1, 0xFF, 0xFF)?;
        let sb_size_1 = reader.get_bits(3, 0x3F, 0)?;
        let sb_size_2 = reader.get_bits_u16(4, 0xFF, 0xFF)?;
        Some(SmoothingBufferDescriptor {
            header,
            sb_leak_rate: (sb_leak_rate_1 as u32) << 16 | sb_leak_rate_2 as u32,
            sb_size: (sb_size_1 as u32) << 16 | sb_size_2 as u32,
        })
    }
    ;
    custom_display: impl std::fmt::Display for SmoothingBufferDescriptor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Smoothing Buffer Descriptor\nSb Leak Rate: {}bits/s\nSb Size: {}\n", self.sb_leak_rate * SB_LEAK_RATE_DATA_IN_UNIT, self.sb_size )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_smoothing_buffer_descriptor_unmarshall() {
        let data = vec![0x3F, 0xFF, 0xFF, 0x3F, 0xFF, 0xFF];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x01),
            descriptor_length: 0x06,
        };
        let descriptor = SmoothingBufferDescriptor {
            header: header.clone(),
            sb_leak_rate: 0x3FFFFF,
            sb_size: 0x3FFFFF,
        };

        assert_eq!(
            SmoothingBufferDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_smoothing_buffer_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x01),
            descriptor_length: 0x06,
        };
        let descriptor = SmoothingBufferDescriptor {
            header: header.clone(),
            sb_leak_rate: 0x3FFFFF,
            sb_size: 0x3FFFFF,
        };

        assert_eq!(descriptor, descriptor);
    }

    #[test]
    fn test_smoothing_buffer_descriptor_neq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x01),
            descriptor_length: 0x06,
        };
        let descriptor = SmoothingBufferDescriptor {
            header: header.clone(),
            sb_leak_rate: 0x3FFFFF,
            sb_size: 0x3FFFFF,
        };

        let descriptor2 = SmoothingBufferDescriptor {
            header: header.clone(),
            sb_leak_rate: 0x3FFFFF,
            sb_size: 0x3FFFFE,
        };

        assert_ne!(descriptor, descriptor2);
    }

    #[test]
    fn test_should_display_smoothing_buffer_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x01),
            descriptor_length: 0x06,
        };
        let descriptor = SmoothingBufferDescriptor {
            header,
            sb_leak_rate: 0x3FFFFF,
            sb_size: 0x3FFFFF,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Smoothing Buffer Descriptor\nSb Leak Rate: 1677721200bits/s\nSb Size: 4194303\n"
        );
    }
}
