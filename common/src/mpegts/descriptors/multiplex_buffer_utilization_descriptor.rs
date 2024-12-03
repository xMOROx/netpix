use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const LTW_OFFSET_MASK: u8 = 0b0111_1111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct MultiplexBufferUtilizationDescriptor {
    pub header: DescriptorHeader,
    pub bound_valid_flag: bool,
    pub ltw_offset_lower_bound: Option<u16>,
    pub ltw_offset_upper_bound: Option<u16>,
}

impl ParsableDescriptor<MultiplexBufferUtilizationDescriptor>
    for MultiplexBufferUtilizationDescriptor
{
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(
        header: DescriptorHeader,
        data: &[u8],
    ) -> Option<MultiplexBufferUtilizationDescriptor> {
        if data.len() != 4 {
            return None;
        }

        let reader = BitReader::new(data);
        let bound_valid_flag = reader.get_bit(0, 7)?;

        let ltw_offset_lower_bound = if bound_valid_flag {
            reader.get_bits_u16(0, LTW_OFFSET_MASK, 0xFF)
        } else {
            None
        };

        let ltw_offset_upper_bound = if bound_valid_flag {
            reader.get_bits_u16(2, LTW_OFFSET_MASK, 0xFF)
        } else {
            None
        };

        Some(MultiplexBufferUtilizationDescriptor {
            header,
            bound_valid_flag,
            ltw_offset_lower_bound,
            ltw_offset_upper_bound,
        })
    }
}

impl std::fmt::Display for MultiplexBufferUtilizationDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bound Valid Flag: {}\nLTW Offset Lower Bound: {:?}\nLTW Offset Upper Bound: {:?}",
            self.bound_valid_flag, self.ltw_offset_lower_bound, self.ltw_offset_upper_bound
        )
    }
}

impl PartialEq for MultiplexBufferUtilizationDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.bound_valid_flag == other.bound_valid_flag
            && self.ltw_offset_lower_bound == other.ltw_offset_lower_bound
            && self.ltw_offset_upper_bound == other.ltw_offset_upper_bound
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_unmarshall_maximum_buffer_utilization_descriptor() {
        let bytes = vec![0x0c, 0x04, 0x80, 0xb4, 0x81, 0x68];

        let descriptor = MultiplexBufferUtilizationDescriptor::unmarshall(
            DescriptorHeader {
                descriptor_tag: DescriptorTag::from(0x0c),
                descriptor_length: 4,
            },
            &bytes[2..],
        )
        .unwrap();

        assert_eq!(
            descriptor,
            MultiplexBufferUtilizationDescriptor {
                header: DescriptorHeader {
                    descriptor_tag: DescriptorTag::from(0x0c),
                    descriptor_length: 4,
                },
                bound_valid_flag: true,
                ltw_offset_lower_bound: Some(180),
                ltw_offset_upper_bound: Some(360),
            }
        );
    }

    #[test]
    fn test_unmarshall_maximum_buffer_utilization_descriptor_no_bound() {
        let bytes = vec![0x0c, 0x04, 0x00, 0x00, 0x00, 0x00];

        let descriptor = MultiplexBufferUtilizationDescriptor::unmarshall(
            DescriptorHeader {
                descriptor_tag: DescriptorTag::from(0x0c),
                descriptor_length: 4,
            },
            &bytes[2..],
        )
        .unwrap();

        assert_eq!(
            descriptor,
            MultiplexBufferUtilizationDescriptor {
                header: DescriptorHeader {
                    descriptor_tag: DescriptorTag::from(0x0c),
                    descriptor_length: 4,
                },
                bound_valid_flag: false,
                ltw_offset_lower_bound: None,
                ltw_offset_upper_bound: None,
            }
        );
    }

    #[test]
    fn test_unmarshall_maximum_buffer_utilization_descriptor_invalid_length() {
        let bytes = vec![
            0b00000000, // bound_valid_flag = false
            0b00000000, // ltw_offset_lower_bound = 0
            0b00000000, // ltw_offset_upper_bound = 0
        ];

        let descriptor = MultiplexBufferUtilizationDescriptor::unmarshall(
            DescriptorHeader {
                descriptor_tag: DescriptorTag::from(0x0c),
                descriptor_length: 3,
            },
            &bytes,
        );

        assert_eq!(descriptor, None);
    }
}
