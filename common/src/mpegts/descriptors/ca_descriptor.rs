use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use bincode::{Decode, Encode};

const CA_PID_MASK: u8 = 0b0001_1111;

implement_descriptor! {
    pub struct CaDescriptor {
        pub ca_system_id: u16,
        pub ca_pid: u16,
        pub private_data: Vec<u8>,
    }
    unmarshall_impl: |header, data| {
        if data.len() < 4 {
            return None;
        }

        let reader = BitReader::new(data);
        let ca_system_id = reader.get_bits_u16(0, 0xFF, 0xFF)?;
        let ca_pid = reader.get_bits_u16_with_shift(2, CA_PID_MASK, 0xFF, 5)?;

        Some(CaDescriptor {
            header,
            ca_system_id,
            ca_pid,
            private_data: reader.remaining_from(4)?,
        })
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

    #[test]
    fn test_should_display_audio_stream_descriptor() {
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
        assert_eq!(
            format!("{}", ca_descriptor),
            "Ca Descriptor\nCa System Id: 258\nCa Pid: 100\nPrivate Data: [5, 6]\n"
        );
    }
}
