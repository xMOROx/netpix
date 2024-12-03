use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;
use serde::{Deserialize, Serialize};

const LEAK_VALID_FLAG: u8 = 0b0000_0001;

implement_descriptor! {
    pub struct StdDescriptor {
        pub leak_valid_flag: bool
    }
    unmarshall_impl: |header, data| {
        if data.len() != 1 {
            return None;
        }

        let reader = BitReader::new(data);
        let leak_valid_flag = reader.get_bits(0, LEAK_VALID_FLAG, 0)? != 0;

        Some(StdDescriptor {
            header,
            leak_valid_flag,
        })
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
        assert!(descriptor.leak_valid_flag);
    }

    #[test]
    fn test_should_display_audio_stream_descriptor() {
        let header = DescriptorHeader {
            descriptor_tag: 0x0A.into(),
            descriptor_length: 1,
        };
        let descriptor = StdDescriptor {
            header: header.clone(),
            leak_valid_flag: true,
        };

        assert_eq!(
            format!("{}", descriptor),
            "Std Descriptor\nLeak Valid Flag: true\n"
        );
    }
}
