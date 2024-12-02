use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct DataStreamAlignmentDescriptor {
    pub header: DescriptorHeader,
    pub alignment_type: AlignmentType,
}

// TODO: for PES is other table 2.54 when data_alignment_indicator is set
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub enum AlignmentType {
    Reserved,
    Slice,
    SliceOrVideoAccessUnit,
    GOPorSEQ,
    SEQ,
    Custom(u8),
}

impl std::fmt::Display for DataStreamAlignmentDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alignment Type: {}", self.alignment_type)
    }
}
impl std::fmt::Display for AlignmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignmentType::Reserved => write!(f, "Reserved"),
            AlignmentType::Slice => write!(f, "Slice"),
            AlignmentType::SliceOrVideoAccessUnit => write!(f, "Slice or Video Access Unit"),
            AlignmentType::GOPorSEQ => write!(f, "GOP or SEQ"),
            AlignmentType::SEQ => write!(f, "SEQ"),
            AlignmentType::Custom(value) => write!(f, "Custom({})", value),
        }
    }
}

impl PartialEq for DataStreamAlignmentDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.alignment_type == other.alignment_type
    }
}

impl PartialEq for AlignmentType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AlignmentType::Reserved, AlignmentType::Reserved) => true,
            (AlignmentType::Slice, AlignmentType::Slice) => true,
            (AlignmentType::SliceOrVideoAccessUnit, AlignmentType::SliceOrVideoAccessUnit) => true,
            (AlignmentType::GOPorSEQ, AlignmentType::GOPorSEQ) => true,
            (AlignmentType::SEQ, AlignmentType::SEQ) => true,
            (AlignmentType::Custom(a), AlignmentType::Custom(b)) => a == b,
            _ => false,
        }
    }
}

impl From<u8> for AlignmentType {
    fn from(value: u8) -> Self {
        match value {
            0 => AlignmentType::Reserved,
            1 => AlignmentType::Slice,
            2 => AlignmentType::SliceOrVideoAccessUnit,
            3 => AlignmentType::GOPorSEQ,
            4 => AlignmentType::SEQ,
            _ => AlignmentType::Custom(value),
        }
    }
}

impl ParsableDescriptor<DataStreamAlignmentDescriptor> for DataStreamAlignmentDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<DataStreamAlignmentDescriptor> {
        if data.len() != 1 {
            return None;
        }

        let reader = BitReader::new(data);
        let alignment_type = AlignmentType::from(reader.get_bits(0, 0xFF, 0)?);

        Some(DataStreamAlignmentDescriptor {
            header,
            alignment_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_data_stream_alignment_descriptor_unmarshall() {
        let data = vec![0x01];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x06),
            descriptor_length: 0x01,
        };
        let descriptor = DataStreamAlignmentDescriptor {
            header: header.clone(),
            alignment_type: AlignmentType::Slice,
        };

        assert_eq!(
            DataStreamAlignmentDescriptor::unmarshall(header, &data),
            Some(descriptor)
        );
    }

    #[test]
    fn test_data_stream_alignment_descriptor_unmarshall_invalid_length() {
        let data = vec![0x01, 0x02];
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x06),
            descriptor_length: 0x02,
        };

        assert_eq!(
            DataStreamAlignmentDescriptor::unmarshall(header, &data),
            None
        );
    }

    #[test]
    fn test_alignment_type_from() {
        assert_eq!(AlignmentType::from(0), AlignmentType::Reserved);
        assert_eq!(AlignmentType::from(1), AlignmentType::Slice);
        assert_eq!(
            AlignmentType::from(2),
            AlignmentType::SliceOrVideoAccessUnit
        );
        assert_eq!(AlignmentType::from(3), AlignmentType::GOPorSEQ);
        assert_eq!(AlignmentType::from(4), AlignmentType::SEQ);
        assert_eq!(AlignmentType::from(5), AlignmentType::Custom(5));
    }

    #[test]
    fn test_data_stream_alignment_descriptor_eq() {
        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::from(0x06),
            descriptor_length: 0x01,
        };
        let descriptor1 = DataStreamAlignmentDescriptor {
            header: header.clone(),
            alignment_type: AlignmentType::Slice,
        };
        let descriptor2 = DataStreamAlignmentDescriptor {
            header,
            alignment_type: AlignmentType::Slice,
        };

        assert_eq!(descriptor1, descriptor2);
    }
}
