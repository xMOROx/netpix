use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
#[cfg(not(target_arch = "wasm32"))]
const MULTIPLE_FRAME_RATE_FLAG: u8 = 0b1000_0000;
#[cfg(not(target_arch = "wasm32"))]
const FRAME_RATE_CODE: u8 = 0b0111_1000;
#[cfg(not(target_arch = "wasm32"))]
const MPEG_1_ONLY_FLAG: u8 = 0b0000_0100;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_PARAMETER_FLAG: u8 = 0b0000_0010;
#[cfg(not(target_arch = "wasm32"))]
const STILL_PICTURE_FLAG: u8 = 0b0000_0001;
#[cfg(not(target_arch = "wasm32"))]
const CHROMA_FORMAT: u8 = 0b1100_0000;
#[cfg(not(target_arch = "wasm32"))]
const FRAME_RATE_EXTENSION_FLAG: u8 = 0b0010_0000;
#[cfg(not(target_arch = "wasm32"))]
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

impl ParsableDescriptor<VideoStreamDescriptor> for VideoStreamDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<VideoStreamDescriptor> {
        let descriptor_length:usize = header.descriptor_length as usize;
        if descriptor_length > MAX_DESCRIPTOR_LENGTH {
            return None;
        }

        let multiple_frame_rate_flag = (data[0] & MULTIPLE_FRAME_RATE_FLAG) != 0;
        let frame_rate_code = (data[0] & FRAME_RATE_CODE) >> 3;
        let mpeg_1_only_flag = (data[0] & MPEG_1_ONLY_FLAG) == 0;
        let constrained_parameter_flag = (data[0] & CONSTRAINED_PARAMETER_FLAG) != 0;
        let still_picture_flag = (data[0] & STILL_PICTURE_FLAG) != 0;


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
            let profile_and_level_indication = Some(data[1]);
            let chroma_format = Some((data[2] & CHROMA_FORMAT) >> 6);
            let frame_rate_extension_flag = Some((data[2] & FRAME_RATE_EXTENSION_FLAG) != 0);

            Some(VideoStreamDescriptor {
                header,
                multiple_frame_rate_flag,
                frame_rate_code,
                mpeg_1_only_flag,
                constrained_parameter_flag,
                still_picture_flag,
                profile_and_level_indication,
                chroma_format,
                frame_rate_extension_flag,
            })
        }
    }
}

impl PartialEq for VideoStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        let header = self.header == other.header;
        let multiple_frame_rate_flag = self.multiple_frame_rate_flag == other.multiple_frame_rate_flag;
        let frame_rate_code = self.frame_rate_code == other.frame_rate_code;
        let mpeg_1_only_flag = self.mpeg_1_only_flag == other.mpeg_1_only_flag;
        let constrained_parameter_flag = self.constrained_parameter_flag == other.constrained_parameter_flag;
        let still_picture_flag = self.still_picture_flag == other.still_picture_flag;
        let profile_and_level_indication = self.profile_and_level_indication == other.profile_and_level_indication;
        let chroma_format = self.chroma_format == other.chroma_format;
        let frame_rate_extension_flag = self.frame_rate_extension_flag == other.frame_rate_extension_flag;

        header && multiple_frame_rate_flag && frame_rate_code && mpeg_1_only_flag && constrained_parameter_flag && still_picture_flag && profile_and_level_indication && chroma_format && frame_rate_extension_flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::types::DescriptorType;
    #[test]
    fn test_video_stream_descriptor_unmarshall_with_only_flag_to_false() {
        let data = vec![0x02, 0x03, 0b1000_1101, 0x03, 0b0111_1111];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x02),
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

        assert_eq!(VideoStreamDescriptor::unmarshall(header,&data[2..]),  Some(descriptor));
    }

    #[test]
    fn test_video_stream_descriptor_unmarshall_with_only_flag_to_true() {
        let data = vec![0x02, 0x01, 0b1000_1010];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x02),
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

        assert_eq!(VideoStreamDescriptor::unmarshall(header,&data[2..]),  Some(descriptor));
    }

    #[test]
    fn test_video_stream_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x02),
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

