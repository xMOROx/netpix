use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

const LEAK_VALID_FLAG: u8 = 0b0000_0001;
#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct StdDescriptor {
    pub header: DescriptorHeader,
    pub leak_valid_flag: bool,
}

impl ParsableDescriptor<StdDescriptor> for StdDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<StdDescriptor> {
        if data.len() != 1 {
            return None;
        }


        Some(StdDescriptor {
            header,
            leak_valid_flag: data[0] & LEAK_VALID_FLAG == LEAK_VALID_FLAG,
        })
    }
}

impl std::fmt::Display for StdDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leak Valid Flag: {}", self.leak_valid_flag)
    }
}

impl PartialEq for StdDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.leak_valid_flag == other.leak_valid_flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_unmarshall() {
        let data = vec![0b0000_0001];
        let header = DescriptorHeader {
            descriptor_tag: 0x0A.into(),
            descriptor_length: 1,
        };
        let descriptor = StdDescriptor::unmarshall(header, &data).unwrap();
        assert_eq!(descriptor.leak_valid_flag, true);
    }
}