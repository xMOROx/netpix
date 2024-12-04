use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

const CLOCK_ACCURACY_INTEGER_MASK: u8 = 0b0011_1111;
const CLOCK_ACCURACY_EXPONENT_MASK: u8 = 0b1110_0000;

implement_descriptor! {
    pub struct SystemClockDescriptor {
        pub external_clock_reference_indicator: bool,
        pub clock_accuracy_integer: u8,
        pub clock_accuracy_exponent: u8
    }
    unmarshall_impl: |header, data| {
        if data.len() != 2 {
            return None;
        }

        let reader = BitReader::new(data);

        Some(SystemClockDescriptor {
            header,
            external_clock_reference_indicator: reader.get_bit(0, 7)?,
            clock_accuracy_integer: reader.get_bits(0, CLOCK_ACCURACY_INTEGER_MASK, 0)?,
            clock_accuracy_exponent: reader.get_bits(1, CLOCK_ACCURACY_EXPONENT_MASK, 5)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

    #[test]
    fn test_unmarshall() {
        let data = vec![0b1000_1111, 0b1111_1111];
        let header = DescriptorHeader {
            descriptor_tag: 0x01.into(),
            descriptor_length: 0x02,
        };
        let system_clock_descriptor = SystemClockDescriptor {
            header: header.clone(),
            external_clock_reference_indicator: true,
            clock_accuracy_integer: 0b1111,
            clock_accuracy_exponent: 0b111,
        };
        assert_eq!(
            SystemClockDescriptor::unmarshall(header, &data),
            Some(system_clock_descriptor)
        );
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: 0x01.into(),
            descriptor_length: 0x02,
        };
        let system_clock_descriptor = SystemClockDescriptor {
            header: header.clone(),
            external_clock_reference_indicator: true,
            clock_accuracy_integer: 0b1111,
            clock_accuracy_exponent: 0b111,
        };
        assert_eq!(
            system_clock_descriptor.to_string(),
            "System Clock Descriptor\nExternal Clock Reference Indicator: true\nClock Accuracy Integer: 15\nClock Accuracy Exponent: 7\n"
        );
    }
}
