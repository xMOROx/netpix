use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const HIERARCHY_TYPE: u8 = 0b0000_1111;

const HIERARCHY_LAYER_INDEX: u8 = 0b0011_1111;

const HIERARCHY_EMBEDDED_LAYER_INDEX: u8 = 0b0011_1111;

const HIERARCHY_CHANNEL: u8 = 0b0011_1111;

implement_descriptor! {
    pub struct HierarchyDescriptor {
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
    unmarshall_impl: |header, data| {
        if data.len() != 4 {
            return None;
        }

        let reader = BitReader::new(data);

        let no_view_scalability_flag = reader.get_bit(0, 7)?;
        let no_temporal_scalability_flag = reader.get_bit(0, 6)?;
        let no_spatial_scalability_flag = reader.get_bit(0, 5)?;
        let no_quality_scalability_flag = reader.get_bit(0, 4)?;
        let hierarchy_type = HierarchyType::from(reader.get_bits(1, HIERARCHY_TYPE, 0)?);
        let hierarchy_layer_index = reader.get_bits(2, HIERARCHY_LAYER_INDEX, 0)?;
        let tref_present_flag = reader.get_bit(3, 7)?;
        let hierarchy_embedded_layer_index =
            reader.get_bits(3, HIERARCHY_EMBEDDED_LAYER_INDEX, 0)?;
        let hierarchy_channel = reader.get_bits(3, HIERARCHY_CHANNEL, 0)?;

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

impl std::fmt::Display for HierarchyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HierarchyType::Reserved => write!(f, "Reserved"),
            HierarchyType::SpatialScalability => write!(f, "Spatial Scalability"),
            HierarchyType::SNRScalability => write!(f, "SNR Scalability"),
            HierarchyType::TemporalScalability => write!(f, "Temporal Scalability"),
            HierarchyType::DataPartitioning => write!(f, "Data Partitioning"),
            HierarchyType::ExtensionBitstream => write!(f, "Extension Bitstream"),
            HierarchyType::PrivateStream => write!(f, "Private Stream"),
            HierarchyType::MultiViewProfile => write!(f, "Multi View Profile"),
            HierarchyType::CombinedScalabilityOrMvHevcSubpartition => {
                write!(f, "Combined Scalability or MV HEVC Subpartition")
            }
            HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream => {
                write!(f, "MVC Video Sub Bitstream or MVCD Video Sub Bitstream")
            }
            HierarchyType::AuxiliaryPictureLayer => write!(f, "Auxiliary Picture Layer"),
            HierarchyType::BaseLayerOrOtherType => write!(f, "Base Layer or Other Type"),
        }
    }
}

impl PartialEq for HierarchyType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (HierarchyType::Reserved, HierarchyType::Reserved)
                | (
                    HierarchyType::SpatialScalability,
                    HierarchyType::SpatialScalability
                )
                | (HierarchyType::SNRScalability, HierarchyType::SNRScalability)
                | (
                    HierarchyType::TemporalScalability,
                    HierarchyType::TemporalScalability
                )
                | (
                    HierarchyType::DataPartitioning,
                    HierarchyType::DataPartitioning
                )
                | (
                    HierarchyType::ExtensionBitstream,
                    HierarchyType::ExtensionBitstream
                )
                | (HierarchyType::PrivateStream, HierarchyType::PrivateStream)
                | (
                    HierarchyType::MultiViewProfile,
                    HierarchyType::MultiViewProfile
                )
                | (
                    HierarchyType::CombinedScalabilityOrMvHevcSubpartition,
                    HierarchyType::CombinedScalabilityOrMvHevcSubpartition
                )
                | (
                    HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream,
                    HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream
                )
                | (
                    HierarchyType::AuxiliaryPictureLayer,
                    HierarchyType::AuxiliaryPictureLayer
                )
                | (
                    HierarchyType::BaseLayerOrOtherType,
                    HierarchyType::BaseLayerOrOtherType
                )
        )
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
            _ => HierarchyType::Reserved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_hierarchy_descriptor_unmarshall() {
        let data = vec![0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x04),
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

        assert_eq!(
            HierarchyDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_hierarchy_descriptor_unmarshall_with_flags() {
        let data = vec![0b1111_0000, 0b0000_0001, 0b0011_1111, 0b1011_1111];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x04),
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

        assert_eq!(
            HierarchyDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_hierarchy_descriptor_unmarshall_invalid_length() {
        let data = vec![0b0000_0000, 0b0000_0000, 0b0000_0000];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x04),
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
        assert_eq!(
            HierarchyType::from(8),
            HierarchyType::CombinedScalabilityOrMvHevcSubpartition
        );
        assert_eq!(
            HierarchyType::from(9),
            HierarchyType::MvcVideoSubBitstreamOrMvcdVideoSubBitstream
        );
        assert_eq!(
            HierarchyType::from(10),
            HierarchyType::AuxiliaryPictureLayer
        );
        assert_eq!(HierarchyType::from(15), HierarchyType::BaseLayerOrOtherType);
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x04),
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

        assert_eq!(
            descriptor.to_string(),
            "Hierarchy Descriptor\nNo View Scalability Flag: false\nNo Temporal Scalability Flag: false\nNo Spatial Scalability Flag: false\nNo Quality Scalability Flag: false\nHierarchy Type: Reserved\nHierarchy Layer Index: 0\nTref Present Flag: false\nHierarchy Embedded Layer Index: 0\nHierarchy Channel: 0\n"
        );
    }
}
