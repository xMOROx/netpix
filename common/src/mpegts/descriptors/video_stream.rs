use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

const FRAME_RATE_CODE_MASK: u8 = 0b0111_1000;
const CHROMA_FORMAT_MASK: u8 = 0b1100_0000;
const MAX_DESCRIPTOR_LENGTH: usize = 3;

implement_descriptor! {
    pub struct VideoStreamDescriptor {
        pub multiple_frame_rate_flag: bool,
        pub frame_rate_code: u8,
        pub mpeg_1_only_flag: bool,
        pub constrained_parameter_flag: bool,
        pub still_picture_flag: bool,
        pub profile_and_level_indication: Option<u8>,
        pub chroma_format: Option<u8>,
        pub frame_rate_extension_flag: Option<bool>
    }
    unmarshall_impl: |header, data| {
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
    ;
    custom_display: impl std::fmt::Display for VideoStreamDescriptor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Video Stream Descriptor")?;
            writeln!(f, "Multiple Frame Rate Flag: {}", self.multiple_frame_rate_flag)?;
            writeln!(f, "Frame Rate Code: {}", self.frame_rate_code)?;
            writeln!(f, "MPEG 1 Only Flag: {}", self.mpeg_1_only_flag)?;
            writeln!(f, "Constrained Parameter Flag: {}", self.constrained_parameter_flag)?;
            writeln!(f, "Still Picture Flag: {}", self.still_picture_flag)?;
            if let Some(profile_and_level_indication) = self.profile_and_level_indication {
                writeln!(f, "Profile And Level Indication: {}", profile_and_level_indication)?;
            }
            if let Some(chroma_format) = self.chroma_format {
                writeln!(f, "Chroma Format: {}", chroma_format)?;
            }
            if let Some(frame_rate_extension_flag) = self.frame_rate_extension_flag {
                writeln!(f, "Frame Rate Extension Flag: {}", frame_rate_extension_flag)?;
            }
            write!(f, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;
    #[test]
    fn test_video_stream_descriptor_unmarshall_with_only_flag_to_false() {
        let data = [0x02, 0x03, 0b1000_1101, 0x03, 0b0111_1111];
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
        let data = [0x02, 0x01, 0b1000_1010];
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

    #[test]
    fn test_should_display_audio_stream_descriptor() {
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

        assert_eq!(
            format!("{}", descriptor),
            "Video Stream Descriptor\nMultiple Frame Rate Flag: true\nFrame Rate Code: 1\nMPEG 1 Only Flag: true\nConstrained Parameter Flag: true\nStill Picture Flag: true\nProfile And Level Indication: 1\nChroma Format: 1\nFrame Rate Extension Flag: true\n"
        );
    }
}
