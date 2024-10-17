use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const EXTERNAL_CLOCK_REFERENCE_INDICATOR: u8 = 0b1000_0000;

#[cfg(not(target_arch = "wasm32"))]
const CLOCK_ACCURACY_INTEGER: u8 = 0b0011_1111;
#[cfg(not(target_arch = "wasm32"))]
const CLOCK_ACCURACY_EXPONENT: u8 = 0b1110_0000;


#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct SystemClockDescriptor {
    pub header: DescriptorHeader,
    pub external_clock_reference_indicator: bool,
    pub clock_accuracy_integer: u8,
    pub clock_accuracy_exponent: u8,
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
        Some(SystemClockDescriptor {
            header,
            external_clock_reference_indicator: data[0] & EXTERNAL_CLOCK_REFERENCE_INDICATOR == EXTERNAL_CLOCK_REFERENCE_INDICATOR,
            clock_accuracy_integer: data[0] & CLOCK_ACCURACY_INTEGER,
            clock_accuracy_exponent: data[1] & CLOCK_ACCURACY_EXPONENT >> 5,
        })
    }
}

impl PartialEq for SystemClockDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.external_clock_reference_indicator == other.external_clock_reference_indicator &&
            self.clock_accuracy_integer == other.clock_accuracy_integer &&
            self.clock_accuracy_exponent == other.clock_accuracy_exponent
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
        assert_eq!(SystemClockDescriptor::unmarshall(header, &data), Some(system_clock_descriptor));
    }
}