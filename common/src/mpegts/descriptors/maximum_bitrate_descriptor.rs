use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const MAXIMUM_BITRATE: u8 = 0b00111111;
const BITRATE_PER_SECOND: u32 = 50;

implement_descriptor! {
    pub struct MaximumBitrateDescriptor {
        pub maximum_bitrate: u32
    }
    unmarshall_impl: |header, data| {
        if data.len() != 3 {
            return None;
        }

        let reader = BitReader::new(data);
        let bitrate_high = reader.get_bits(0, MAXIMUM_BITRATE, 0)? as u32;
        let bitrate_mid = data[1] as u32;
        let bitrate_low = data[2] as u32;

        let maximum_bitrate = (bitrate_high << 16) | (bitrate_mid << 8) | bitrate_low;

        Some(MaximumBitrateDescriptor {
            header,
            maximum_bitrate,
        })
    }
    ;
    custom_display: impl std::fmt::Display for MaximumBitrateDescriptor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Maximum bitrate: {} kbps", self.maximum_bitrate * BITRATE_PER_SECOND)
        }
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
        let data = [0x0e, 0x03, 0xc0, 0x00, 0x00];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };

        let descriptor = MaximumBitrateDescriptor::unmarshall(header, &data[2..]).unwrap();
        assert_eq!(descriptor.maximum_bitrate, 0);
    }
    #[test]
    fn test_maximum_bitrate_descriptor_3() {
        let data = [0x0e, 0x03, 0xc0, 0x17, 0x15];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };

        let descriptor = MaximumBitrateDescriptor::unmarshall(header, &data[2..]).unwrap();
        assert_eq!(descriptor.maximum_bitrate, 5909);
    }

    #[test]
    fn test_should_display_maximum_bitrate_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::MaximumBitrateDescriptorTag,
            descriptor_length: 3,
        };
        let descriptor = MaximumBitrateDescriptor {
            header,
            maximum_bitrate: 0b00111111_11111111_11111111,
        };

        assert_eq!(format!("{}", descriptor), "Maximum bitrate: 209715150 kbps");
    }
}
