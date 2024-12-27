use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};

const LTW_OFFSET_MASK: u8 = 0b0111_1111;

implement_descriptor! {
    pub struct MultiplexBufferUtilizationDescriptor {
        pub bound_valid_flag: bool,
        pub ltw_offset_lower_bound: Option<u16>,
        pub ltw_offset_upper_bound: Option<u16>
    }
    unmarshall_impl: |header, data| {
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
    ;
    custom_display: impl std::fmt::Display for MultiplexBufferUtilizationDescriptor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Multiplex Buffer Utilization Descriptor")?;
            writeln!(f, "Bound Valid Flag: {:?}", self.bound_valid_flag)?;
            if let Some(ltw_offset_lower_bound) = self.ltw_offset_lower_bound {
                writeln!(f, "Ltw Offset Lower Bound: {:?}", ltw_offset_lower_bound)?;
            }
            if let Some(ltw_offset_upper_bound) = self.ltw_offset_upper_bound {
                writeln!(f, "Ltw Offset Upper Bound: {:?}", ltw_offset_upper_bound)?;
            }
            write!(f, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;

    #[test]
    fn test_unmarshall_maximum_buffer_utilization_descriptor() {
        let bytes = [0x0c, 0x04, 0x80, 0xb4, 0x81, 0x68];

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
        let bytes = [0x0c, 0x04, 0x00, 0x00, 0x00, 0x00];

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

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let descriptor = MultiplexBufferUtilizationDescriptor {
            header: DescriptorHeader {
                descriptor_tag: DescriptorTag::from(0x0c),
                descriptor_length: 4,
            },
            bound_valid_flag: true,
            ltw_offset_lower_bound: Some(180),
            ltw_offset_upper_bound: Some(360),
        };

        assert_eq!(
            descriptor.to_string(),
            "Multiplex Buffer Utilization Descriptor\nBound Valid Flag: true\nLtw Offset Lower Bound: 180\nLtw Offset Upper Bound: 360\n"
        );
    }
}
