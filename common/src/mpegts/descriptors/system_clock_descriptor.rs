use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const CLOCK_ACCURACY_INTEGER_MASK: u8 = 0b0011_1111;
const CLOCK_ACCURACY_EXPONENT_MASK: u8 = 0b1110_0000;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct SystemClockDescriptor {
    pub header: DescriptorHeader,
    pub external_clock_reference_indicator: bool,
    pub clock_accuracy_integer: u8,
    pub clock_accuracy_exponent: u8,
}

impl std::fmt::Display for SystemClockDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "External Clock Reference Indicator: {}\nClock Accuracy Integer: {}\nClock Accuracy Exponent: {}",
               self.external_clock_reference_indicator, self.clock_accuracy_integer, self.clock_accuracy_exponent)
    }
}

impl ParsableDescriptor<SystemClockDescriptor> for SystemClockDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<SystemClockDescriptor> {
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

impl PartialEq for SystemClockDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.external_clock_reference_indicator == other.external_clock_reference_indicator
            && self.clock_accuracy_integer == other.clock_accuracy_integer
            && self.clock_accuracy_exponent == other.clock_accuracy_exponent
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
}
