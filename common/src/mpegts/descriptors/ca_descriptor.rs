use serde::{Deserialize, Serialize};
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

#[cfg(not(target_arch = "wasm32"))]
const CA_PID: u8 = 0b0001_1111;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct CaDescriptor {
    pub header: DescriptorHeader,
    pub ca_system_id: u16,
    pub ca_pid: u16,
    pub private_data: Vec<u8>,
}

impl ParsableDescriptor<CaDescriptor> for CaDescriptor {
    fn descriptor_tag(&self) -> u8 {
        self.header.descriptor_tag.to_u8()
    }

    fn descriptor_length(&self) -> u8 {
        self.header.descriptor_length
    }

    fn unmarshall(header: DescriptorHeader, data: &[u8]) -> Option<CaDescriptor> {
        if data.len() < 4 {
            return None;
        }
        Some(CaDescriptor {
            header,
            ca_system_id: u16::from_be_bytes([data[0], data[1]]),
            ca_pid: ((data[2] & CA_PID) as u16) << 5 | data[3] as u16,
            private_data: data[4..].to_vec(),
        })
    }
}

impl PartialEq for CaDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header &&
            self.ca_system_id == other.ca_system_id &&
            self.ca_pid == other.ca_pid &&
            self.private_data == other.private_data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};

    #[test]
    fn test_unmarshall() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let header = DescriptorHeader {
            descriptor_tag: 0x01.into(),
            descriptor_length: 0x02,
        };
        let ca_descriptor = CaDescriptor {
            header: header.clone(),
            ca_system_id: 0x0102,
            ca_pid: 0x03 << 5 | 0x04,
            private_data: vec![0x05, 0x06],
        };
        assert_eq!(CaDescriptor::unmarshall(header, &data), Some(ca_descriptor));
    }

    #[test]
    fn test_eq() {
        let header = DescriptorHeader {
            descriptor_tag: 0x01.into(),
            descriptor_length: 0x02,
        };
        let ca_descriptor = CaDescriptor {
            header: header.clone(),
            ca_system_id: 0x0102,
            ca_pid: 0x03 << 5 | 0x04,
            private_data: vec![0x05, 0x06],
        };
        assert_eq!(ca_descriptor, ca_descriptor.clone());
    }
}