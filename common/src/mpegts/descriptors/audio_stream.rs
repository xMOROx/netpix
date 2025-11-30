use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

const ID: u8 = 0b0100_0000;
const LAYER: u8 = 0b0011_0000;

implement_descriptor! {
    pub struct AudioStreamDescriptor {
        pub free_format_flag: bool,
        pub id: u8,
        pub layer: u8,
        pub variable_rate_audio_indicator: bool,
    }
    unmarshall_impl: |header, data| {
        if data.len() != 1 {
            return None;
        }

        let reader = BitReader::new(data);

        Some(AudioStreamDescriptor {
            header,
            free_format_flag: reader.get_bit(0, 7)?,
            id: reader.get_bits(0, ID, 6)?,
            layer: reader.get_bits(0, LAYER, 4)?,
            variable_rate_audio_indicator: reader.get_bit(0, 3)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_audio_stream_descriptor_unmarshall() {
        let data = vec![0b1110_1000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x03),
            descriptor_length: 0x01,
        };
        let descriptor = AudioStreamDescriptor {
            header: header.clone(),
            free_format_flag: true,
            id: 0x01,
            layer: 0x02,
            variable_rate_audio_indicator: true,
        };

        assert_eq!(
            AudioStreamDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_audio_stream_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x03),
            descriptor_length: 0x01,
        };
        let descriptor = AudioStreamDescriptor {
            header,
            free_format_flag: true,
            id: 0x01,
            layer: 0x02,
            variable_rate_audio_indicator: true,
        };

        assert_eq!(descriptor, descriptor);
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x03),
            descriptor_length: 0x01,
        };
        let descriptor = AudioStreamDescriptor {
            header,
            free_format_flag: true,
            id: 0x01,
            layer: 0x02,
            variable_rate_audio_indicator: true,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Audio Stream Descriptor\nFree Format Flag: true\nId: 1\nLayer: 2\nVariable Rate Audio Indicator: true\n"
        );
    }
}
