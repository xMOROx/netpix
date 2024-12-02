use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const AVC_COMPATIBLE_FLAGS_MASK: u8 = 0b0000_0011;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct AvcVideoDescriptor {
    pub header: DescriptorHeader,
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

impl ParsableDescriptor<AvcVideoDescriptor> for AvcVideoDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<AvcVideoDescriptor> {
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

impl std::fmt::Display for AvcVideoDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Profile IDC: {}\nConstraint Set 0 Flag: {}\nConstraint Set 1 Flag: {}\nConstraint Set 2 Flag: {}\nConstraint Set 3 Flag: {}\nConstraint Set 4 Flag: {}\nConstraint Set 5 Flag: {}\nAVC Compatible Flags: {}\nLevel IDC: {}\nAVC Still Present: {}\nAVC 24 Hour Picture Flag: {}\nFrame Packing SEI Flag: {}", self.profile_idc, self.constraint_set0_flag, self.constraint_set1_flag, self.constraint_set2_flag, self.constraint_set3_flag, self.constraint_set4_flag, self.constraint_set5_flag, self.avc_compatible_flags, self.level_idc, self.avc_still_present, self.avc_24_hour_picture_flag, self.frame_packing_sei_flag)
    }
}

impl PartialEq for AvcVideoDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.profile_idc == other.profile_idc
            && self.constraint_set0_flag == other.constraint_set0_flag
            && self.constraint_set1_flag == other.constraint_set1_flag
            && self.constraint_set2_flag == other.constraint_set2_flag
            && self.constraint_set3_flag == other.constraint_set3_flag
            && self.constraint_set4_flag == other.constraint_set4_flag
            && self.constraint_set5_flag == other.constraint_set5_flag
            && self.avc_compatible_flags == other.avc_compatible_flags
            && self.level_idc == other.level_idc
            && self.avc_still_present == other.avc_still_present
            && self.avc_24_hour_picture_flag == other.avc_24_hour_picture_flag
    }
}
