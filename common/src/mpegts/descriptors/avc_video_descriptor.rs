use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};

const AVC_COMPATIBLE_FLAGS_MASK: u8 = 0b0000_0011;

implement_descriptor! {
    pub struct AvcVideoDescriptor {
        pub profile_idc: u8,
        pub constraint_set0_flag: bool,
        pub constraint_set1_flag: bool,
        pub constraint_set2_flag: bool,
        pub constraint_set3_flag: bool,
        pub constraint_set4_flag: bool,
        pub constraint_set5_flag: bool,
        pub avc_compatible_flags: u8,
        pub level_idc: u8,
        pub avc_still_present: bool,
        pub avc_24_hour_picture_flag: bool,
        pub frame_packing_sei_flag: bool,
    }
    unmarshall_impl: |header, data| {
        if data.len() < 4 {
            return None;
        }

        let reader = BitReader::new(data);

        Some(AvcVideoDescriptor {
            header,
            profile_idc: data[0],
            constraint_set0_flag: reader.get_bit(1, 7)?,
            constraint_set1_flag: reader.get_bit(1, 6)?,
            constraint_set2_flag: reader.get_bit(1, 5)?,
            constraint_set3_flag: reader.get_bit(1, 4)?,
            constraint_set4_flag: reader.get_bit(1, 3)?,
            constraint_set5_flag: reader.get_bit(1, 2)?,
            avc_compatible_flags: reader.get_bits(1, AVC_COMPATIBLE_FLAGS_MASK, 0)?,
            level_idc: data[2],
            avc_still_present: reader.get_bit(3, 7)?,
            avc_24_hour_picture_flag: reader.get_bit(3, 6)?,
            frame_packing_sei_flag: reader.get_bit(3, 5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x03),
            descriptor_length: 0x01,
        };
        let descriptor = AvcVideoDescriptor {
            header: header.clone(),
            profile_idc: 0x01,
            constraint_set0_flag: true,
            constraint_set1_flag: true,
            constraint_set2_flag: false,
            constraint_set3_flag: false,
            constraint_set4_flag: true,
            constraint_set5_flag: false,
            avc_compatible_flags: 0b0000_0010,
            level_idc: 0x02,
            avc_still_present: true,
            avc_24_hour_picture_flag: false,
            frame_packing_sei_flag: true,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Avc Video Descriptor\nProfile Idc: 1\nConstraint Set0 Flag: true\nConstraint Set1 Flag: true\nConstraint Set2 Flag: false\nConstraint Set3 Flag: false\nConstraint Set4 Flag: true\nConstraint Set5 Flag: false\nAvc Compatible Flags: 2\nLevel Idc: 2\nAvc Still Present: true\nAvc 24 Hour Picture Flag: false\nFrame Packing Sei Flag: true\n"
        );
    }
}
