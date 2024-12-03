use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const FRAME_RATE_CODE_MASK: u8 = 0b0111_1000;
const CHROMA_FORMAT_MASK: u8 = 0b1100_0000;
const MAX_DESCRIPTOR_LENGTH: usize = 3;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct VideoStreamDescriptor {
    pub header: DescriptorHeader,
    pub multiple_frame_rate_flag: bool,
    pub frame_rate_code: u8,
    pub mpeg_1_only_flag: bool,
    pub constrained_parameter_flag: bool,
    pub still_picture_flag: bool,
    pub profile_and_level_indication: Option<u8>,
    pub chroma_format: Option<u8>,
    pub frame_rate_extension_flag: Option<bool>,
}

impl std::fmt::Display for VideoStreamDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut profile_and_level_indication = String::from("None");
        let mut chroma_format = String::from("None");
        let mut frame_rate_extension_flag = String::from("None");

        if let Some(value) = self.profile_and_level_indication {
            profile_and_level_indication = value.to_string();
        }

        if let Some(value) = self.chroma_format {
            chroma_format = value.to_string();
        }

        if let Some(value) = self.frame_rate_extension_flag {
            frame_rate_extension_flag = value.to_string();
        }

        write!(f, "Multiple Frame Rate Flag: {}\nFrame Rate Code: {}\nMPEG 1 Only Flag: {}\nConstrained Parameter Flag: {}\nStill Picture Flag: {}\nProfile And Level Indication: {}\nChroma Format: {}\nFrame Rate Extension Flag: {}", self.multiple_frame_rate_flag, self.frame_rate_code, self.mpeg_1_only_flag, self.constrained_parameter_flag, self.still_picture_flag, profile_and_level_indication, chroma_format, frame_rate_extension_flag)
    }
}

impl ParsableDescriptor<VideoStreamDescriptor> for VideoStreamDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<VideoStreamDescriptor> {
        let descriptor_length: usize = header.descriptor_length as usize;
        if descriptor_length > MAX_DESCRIPTOR_LENGTH {
            return None;
        }

        let reader = BitReader::new(data);

        let multiple_frame_rate_flag = reader.get_bit(0, 7)?;
        let frame_rate_code = reader.get_bits(0, FRAME_RATE_CODE_MASK, 3)?;
        let mpeg_1_only_flag = !reader.get_bit(0, 2)?;
        let constrained_parameter_flag = reader.get_bit(0, 1)?;
        let still_picture_flag = reader.get_bit(0, 0)?;

        if mpeg_1_only_flag {
            Some(VideoStreamDescriptor {
                header,
                multiple_frame_rate_flag,
                frame_rate_code,
                mpeg_1_only_flag,
                constrained_parameter_flag,
                still_picture_flag,
                profile_and_level_indication: None,
                chroma_format: None,
                frame_rate_extension_flag: None,
            })
        } else {
            Some(VideoStreamDescriptor {
                header,
                multiple_frame_rate_flag,
                frame_rate_code,
                mpeg_1_only_flag,
                constrained_parameter_flag,
                still_picture_flag,
                profile_and_level_indication: Some(data[1]),
                chroma_format: Some(reader.get_bits(2, CHROMA_FORMAT_MASK, 6)?),
                frame_rate_extension_flag: Some(reader.get_bit(2, 5)?),
            })
        }
    }
}

impl PartialEq for VideoStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        let header = self.header == other.header;
        let multiple_frame_rate_flag =
            self.multiple_frame_rate_flag == other.multiple_frame_rate_flag;
        let frame_rate_code = self.frame_rate_code == other.frame_rate_code;
        let mpeg_1_only_flag = self.mpeg_1_only_flag == other.mpeg_1_only_flag;
        let constrained_parameter_flag =
            self.constrained_parameter_flag == other.constrained_parameter_flag;
        let still_picture_flag = self.still_picture_flag == other.still_picture_flag;
        let profile_and_level_indication =
            self.profile_and_level_indication == other.profile_and_level_indication;
        let chroma_format = self.chroma_format == other.chroma_format;
        let frame_rate_extension_flag =
            self.frame_rate_extension_flag == other.frame_rate_extension_flag;

        header
            && multiple_frame_rate_flag
            && frame_rate_code
            && mpeg_1_only_flag
            && constrained_parameter_flag
            && still_picture_flag
            && profile_and_level_indication
            && chroma_format
            && frame_rate_extension_flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;
    #[test]
    fn test_video_stream_descriptor_unmarshall_with_only_flag_to_false() {
        let data = vec![0x02, 0x03, 0b1000_1101, 0x03, 0b0111_1111];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x02),
            descriptor_length: 0x03,
        };
        let descriptor = VideoStreamDescriptor {
            header: header.clone(),
            multiple_frame_rate_flag: true,
            frame_rate_code: 0x01,
            mpeg_1_only_flag: false,
            constrained_parameter_flag: false,
            still_picture_flag: true,
            profile_and_level_indication: Some(0x03),
            chroma_format: Some(0x01),
            frame_rate_extension_flag: Some(true),
        };

        assert_eq!(
            VideoStreamDescriptor::unmarshall(header, &data[2..]),
            Some(descriptor)
        );
    }

    #[test]
    fn test_video_stream_descriptor_unmarshall_with_only_flag_to_true() {
        let data = vec![0x02, 0x01, 0b1000_1010];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x02),
            descriptor_length: 0x01,
        };
        let descriptor = VideoStreamDescriptor {
            header: header.clone(),
            multiple_frame_rate_flag: true,
            frame_rate_code: 0x01,
            mpeg_1_only_flag: true,
            constrained_parameter_flag: true,
            still_picture_flag: false,
            profile_and_level_indication: None,
            chroma_format: None,
            frame_rate_extension_flag: None,
        };

        assert_eq!(
            VideoStreamDescriptor::unmarshall(header, &data[2..]),
            Some(descriptor)
        );
    }

    #[test]
    fn test_video_stream_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x02),
            descriptor_length: 0x03,
        };
        let descriptor = VideoStreamDescriptor {
            header,
            multiple_frame_rate_flag: true,
            frame_rate_code: 0x01,
            mpeg_1_only_flag: true,
            constrained_parameter_flag: true,
            still_picture_flag: true,
            profile_and_level_indication: Some(0x01),
            chroma_format: Some(0x01),
            frame_rate_extension_flag: Some(true),
        };

        assert_eq!(descriptor, descriptor);
    }
}
