use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET0_FLAG: u8 = 0b1000_0000;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET1_FLAG: u8 = 0b0100_0000;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET2_FLAG: u8 = 0b0010_0000;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET3_FLAG: u8 = 0b0001_0000;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET4_FLAG: u8 = 0b0000_1000;
#[cfg(not(target_arch = "wasm32"))]
const CONSTRAINED_SET5_FLAG: u8 = 0b0000_0100;
#[cfg(not(target_arch = "wasm32"))]
const AVC_COMPATIBLE_FLAGS_MASK: u8 = 0b0000_0011;
#[cfg(not(target_arch = "wasm32"))]
const AVC_STILL_PRESENT_FLAG: u8 = 0b1000_0000;
#[cfg(not(target_arch = "wasm32"))]
const AVC_24_HOUR_PICTURE_FLAG: u8 = 0b0100_0000;
#[cfg(not(target_arch = "wasm32"))]
const FRAME_PACKING_SEI_FLAG: u8 = 0b0010_0000;


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

        let profile_idc = data[0];
        let flags = data[1];
        let level_idc = data[2];
        let avc_compatible_flags = flags & AVC_COMPATIBLE_FLAGS_MASK;
        let constraint_set0_flag = flags & CONSTRAINED_SET0_FLAG == CONSTRAINED_SET0_FLAG;
        let constraint_set1_flag = flags & CONSTRAINED_SET1_FLAG == CONSTRAINED_SET1_FLAG;
        let constraint_set2_flag = flags & CONSTRAINED_SET2_FLAG == CONSTRAINED_SET2_FLAG;
        let constraint_set3_flag = flags & CONSTRAINED_SET3_FLAG == CONSTRAINED_SET3_FLAG;
        let constraint_set4_flag = flags & CONSTRAINED_SET4_FLAG == CONSTRAINED_SET4_FLAG;
        let constraint_set5_flag = flags & CONSTRAINED_SET5_FLAG == CONSTRAINED_SET5_FLAG;
        let avc_still_present = flags & AVC_STILL_PRESENT_FLAG == AVC_STILL_PRESENT_FLAG;
        let avc_24_hour_picture_flag = flags & AVC_24_HOUR_PICTURE_FLAG == AVC_24_HOUR_PICTURE_FLAG;
        let frame_packing_sei_flag = flags & FRAME_PACKING_SEI_FLAG == FRAME_PACKING_SEI_FLAG;

        Some(AvcVideoDescriptor {
            header,
            profile_idc,
            constraint_set0_flag,
            constraint_set1_flag,
            constraint_set2_flag,
            constraint_set3_flag,
            constraint_set4_flag,
            constraint_set5_flag,
            avc_compatible_flags,
            level_idc,
            avc_still_present,
            avc_24_hour_picture_flag,
            frame_packing_sei_flag,
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