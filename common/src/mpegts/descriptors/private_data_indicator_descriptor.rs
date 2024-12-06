use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

implement_descriptor! {
    pub struct PrivateDataIndicatorDescriptor {
        pub private_data_indicator: u32
    }
    unmarshall_impl: |header, data| {
        if data.len() < 4 {
            return None;
        }

        let reader = BitReader::new(data);
        let private_data_indicator = reader.get_bits_u32(0)?;

        Some(PrivateDataIndicatorDescriptor {
            header,
            private_data_indicator,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

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

        assert_eq!(
            PrivateDataIndicatorDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x0F),
            descriptor_length: 4,
        };
        let descriptor = PrivateDataIndicatorDescriptor {
            header: header.clone(),
            private_data_indicator: u32::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
        };

        assert_eq!(
            format!("{}", descriptor),
            "Private Data Indicator Descriptor\nPrivate Data Indicator: 16909060\n"
        );
    }
}
