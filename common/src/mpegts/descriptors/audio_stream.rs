use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const ID: u8 = 0b0100_0000;
const LAYER: u8 = 0b0011_0000;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct AudioStreamDescriptor {
    pub header: DescriptorHeader,
    pub free_format_flag: bool,
    pub id: u8,
    pub layer: u8,
    pub variable_rate_audio_indicator: bool,
}

impl ParsableDescriptor<AudioStreamDescriptor> for AudioStreamDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<AudioStreamDescriptor> {
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

impl std::fmt::Display for AudioStreamDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Free Format Flag: {}\nID: {}\nLayer: {}\nVariable Rate Audio Indicator: {}",
            self.free_format_flag, self.id, self.layer, self.variable_rate_audio_indicator
        )
    }
}

impl PartialEq for AudioStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.free_format_flag == other.free_format_flag
            && self.id == other.id
            && self.layer == other.layer
            && self.variable_rate_audio_indicator == other.variable_rate_audio_indicator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

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
}
