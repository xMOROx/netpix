use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};


#[cfg(not(target_arch = "wasm32"))]
const NO_VIEW_SCALABILITY_FLAG: u8 = 0b1000_0000;
#[cfg(not(target_arch = "wasm32"))]
const NO_TEMPORAL_SCALABILITY_FLAG: u8 = 0b0100_0000;
#[cfg(not(target_arch = "wasm32"))]
const NO_SPATIAL_SCALABILITY_FLAG: u8 = 0b0010_0000;
#[cfg(not(target_arch = "wasm32"))]
const NO_QUALITY_SCALABILITY_FLAG: u8 = 0b0001_0000;
#[cfg(not(target_arch = "wasm32"))]
const HIERARCHY_TYPE: u8 = 0b0000_1111;
#[cfg(not(target_arch = "wasm32"))]
const HIERARCHY_LAYER_INDEX: u8 = 0b0011_1111;
#[cfg(not(target_arch = "wasm32"))]
const TREF_PRESENT_FLAG: u8 = 0b1000_0000;
#[cfg(not(target_arch = "wasm32"))]
const HIERARCHY_EMBEDDED_LAYER_INDEX: u8 = 0b0011_1111;
#[cfg(not(target_arch = "wasm32"))]
const HIERARCHY_CHANNEL: u8 = 0b0011_1111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct HierarchyDescriptor {
    pub header: DescriptorHeader,
    pub no_view_scalability_flag: bool,
    pub no_temporal_scalability_flag: bool,
    pub no_spatial_scalability_flag: bool,
    pub no_quality_scalability_flag: bool,
    pub hierarchy_type: HierarchyType,
    pub hierarchy_layer_index: u8,
    pub tref_present_flag: bool,
    pub hierarchy_embedded_layer_index: u8,
    pub hierarchy_channel: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum HierarchyType {
    Reserved,
    SpatialScalability,
    SNRScalability,
    TemporalScalability,
    DataPartitioning,
    ExtensionBitstream,
    PrivateStream,
    MultiViewProfile,
    CombinedScalabilityOrMvHevcSubpartition,
    MvcVideoSubBitstreamOrMvcdVideoSubBitstream,
    AuxiliaryPictureLayer,
    BaseLayerOrOtherType,
}

impl ParsableDescriptor<HierarchyDescriptor> for HierarchyDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<HierarchyDescriptor> {
        if data.len() != 4 {
            return None;
        }

        let no_view_scalability_flag = (data[0] & NO_VIEW_SCALABILITY_FLAG) != 0;
        let no_temporal_scalability_flag = (data[0] & NO_TEMPORAL_SCALABILITY_FLAG) != 0;
        let no_spatial_scalability_flag = (data[0] & NO_SPATIAL_SCALABILITY_FLAG) != 0;
        let no_quality_scalability_flag = (data[0] & NO_QUALITY_SCALABILITY_FLAG) != 0;
        let hierarchy_type = HierarchyType::from(data[1] & HIERARCHY_TYPE);
        let hierarchy_layer_index = data[2] & HIERARCHY_LAYER_INDEX;
        let tref_present_flag = (data[3] & TREF_PRESENT_FLAG) != 0;
        let hierarchy_embedded_layer_index = data[3] & HIERARCHY_EMBEDDED_LAYER_INDEX;
        let hierarchy_channel = data[3] & HIERARCHY_CHANNEL;

        Some(HierarchyDescriptor {
            header,
            no_view_scalability_flag,
            no_temporal_scalability_flag,
            no_spatial_scalability_flag,
            no_quality_scalability_flag,
            hierarchy_type,
            hierarchy_layer_index,
            tref_present_flag,
            hierarchy_embedded_layer_index,
            hierarchy_channel,
        })
    }
}

impl PartialEq for HierarchyDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.no_view_scalability_flag == other.no_view_scalability_flag
            && self.no_temporal_scalability_flag == other.no_temporal_scalability_flag
            && self.no_spatial_scalability_flag == other.no_spatial_scalability_flag
            && self.no_quality_scalability_flag == other.no_quality_scalability_flag
            && self.hierarchy_type == other.hierarchy_type
            && self.hierarchy_layer_index == other.hierarchy_layer_index
            && self.tref_present_flag == other.tref_present_flag
            && self.hierarchy_embedded_layer_index == other.hierarchy_embedded_layer_index
            && self.hierarchy_channel == other.hierarchy_channel
    }
}

impl PartialEq for HierarchyType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HierarchyType::Reserved, HierarchyType::Reserved) => true,
            (HierarchyType::SpatialScalability, HierarchyType::SpatialScalability) => true,
            (HierarchyType::SNRScalability, HierarchyType::SNRScalability) => true,
            (HierarchyType::TemporalScalability, HierarchyType::TemporalScalability) => true,
            (HierarchyType::DataPartitioning, HierarchyType::DataPartitioning) => true,
            (HierarchyType::ExtensionBitstream, HierarchyType::ExtensionBitstream) => true,
            (HierarchyType::PrivateStream, HierarchyType::PrivateStream) => true,
            (HierarchyType::MultiViewProfile, HierarchyType::MultiViewProfile) => true,
            (HierarchyType::CombinedScalabilityOrMvHevcSubpartition, HierarchyType::CombinedScalabilityOrMvHevcSubpartition) => true,
            (HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream, HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream) => true,
            (HierarchyType::AuxiliaryPictureLayer, HierarchyType::AuxiliaryPictureLayer) => true,
            (HierarchyType::BaseLayerOrOtherType, HierarchyType::BaseLayerOrOtherType) => true,
            _ => false
        }
    }
}

impl From<u8> for HierarchyType {
    fn from(original: u8) -> Self {
        match original {
            1 => HierarchyType::SpatialScalability,
            2 => HierarchyType::SNRScalability,
            3 => HierarchyType::TemporalScalability,
            4 => HierarchyType::DataPartitioning,
            5 => HierarchyType::ExtensionBitstream,
            6 => HierarchyType::PrivateStream,
            7 => HierarchyType::MultiViewProfile,
            8 => HierarchyType::CombinedScalabilityOrMvHevcSubpartition,
            9 => HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream,
            10 => HierarchyType::AuxiliaryPictureLayer,
            15 => HierarchyType::BaseLayerOrOtherType,
            _ => HierarchyType::Reserved
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;
    use crate::mpegts::descriptors::types::DescriptorType;

    #[test]
    fn test_hierarchy_descriptor_unmarshall() {
        let data = vec![0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x04),
            descriptor_length: 0x04,
        };
        let descriptor = HierarchyDescriptor {
            header: header.clone(),
            no_view_scalability_flag: false,
            no_temporal_scalability_flag: false,
            no_spatial_scalability_flag: false,
            no_quality_scalability_flag: false,
            hierarchy_type: HierarchyType::Reserved,
            hierarchy_layer_index: 0,
            tref_present_flag: false,
            hierarchy_embedded_layer_index: 0,
            hierarchy_channel: 0,
        };

        assert_eq!(HierarchyDescriptor::unmarshall(header, &data), Some(descriptor));
    }

    #[test]
    fn test_hierarchy_descriptor_unmarshall_with_flags() {
        let data = vec![0b1111_0000, 0b0000_0001, 0b0011_1111, 0b1011_1111];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x04),
            descriptor_length: 0x04,
        };
        let descriptor = HierarchyDescriptor {
            header: header.clone(),
            no_view_scalability_flag: true,
            no_temporal_scalability_flag: true,
            no_spatial_scalability_flag: true,
            no_quality_scalability_flag: true,
            hierarchy_type: HierarchyType::SpatialScalability,
            hierarchy_layer_index: 0b0011_1111,
            tref_present_flag: true,
            hierarchy_embedded_layer_index: 0b0011_1111,
            hierarchy_channel: 0b0011_1111,
        };

        assert_eq!(HierarchyDescriptor::unmarshall(header, &data), Some(descriptor));
    }

    #[test]
    fn test_hierarchy_descriptor_unmarshall_invalid_length() {
        let data = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorType::from(0x04),
            descriptor_length: 0x03,
        };

        assert_eq!(HierarchyDescriptor::unmarshall(header, &data), None);
    }

    #[test]
    fn test_hierarchy_type_from() {
        assert_eq!(HierarchyType::from(0), HierarchyType::Reserved);
        assert_eq!(HierarchyType::from(1), HierarchyType::SpatialScalability);
        assert_eq!(HierarchyType::from(2), HierarchyType::SNRScalability);
        assert_eq!(HierarchyType::from(3), HierarchyType::TemporalScalability);
        assert_eq!(HierarchyType::from(4), HierarchyType::DataPartitioning);
        assert_eq!(HierarchyType::from(5), HierarchyType::ExtensionBitstream);
        assert_eq!(HierarchyType::from(6), HierarchyType::PrivateStream);
        assert_eq!(HierarchyType::from(7), HierarchyType::MultiViewProfile);
        assert_eq!(HierarchyType::from(8), HierarchyType::CombinedScalabilityOrMvHevcSubpartition);
        assert_eq!(HierarchyType::from(9), HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream);
        assert_eq!(HierarchyType::from(10), HierarchyType::AuxiliaryPictureLayer);
        assert_eq!(HierarchyType::from(15), HierarchyType::BaseLayerOrOtherType);
    }
}