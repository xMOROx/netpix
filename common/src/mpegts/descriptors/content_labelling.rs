use crate::implement_descriptor;
use crate::mpegts::descriptors::{DescriptorHeader, ParsableDescriptor};
use crate::utils::bits::BitReader;

implement_descriptor! {
    pub struct ContentLabellingDescriptor {
        metadata_application_format: u16,
        metadata_application_format_identifier: Option<u32>,
        content_reference_id_record_flag: bool,
        content_time_base_indicator: u8, //  | 4bits |
        content_reference_id_record_length: Option<u8>, // content_reference_id_record_flag == true
        content_reference_id_bytes: Option<Vec<u8>>, // content_reference_id_record_flag == true
        content_time_base_value: Option<u64>, // | 33bits  |content_time_base_indicator == 1 or content_time_base_indicator == 2
        metadata_time_base_value: Option<u64>, // | 33bits  | content_time_base_indicator == 1 or content_time_base_indicator == 2
        content_id: Option<u8>, // | 7bits  | content_time_base_indicator == 2
        content_time_base_data_length: Option<u8>, // content_time_base_indicator >= 3 and content_time_base_indicator <= 7 - number of reserved bytes
        private_data: Vec<u8>
    }
    unmarshall_impl: |header, data| {

        let mut reader = BitReader::new(data);

        let metadata_application_format = reader.get_bits_u16(0, 0xFF, 0xFF)?;
        let metadata_application_format_identifier = if metadata_application_format == 0xFFFF {
            reader.advance(2);
            reader.get_bits_u32(0)
        } else {
            None
        };

        let content_reference_id_record_flag = reader.get_bit(2, 0)?;
        let content_time_base_indicator = reader.get_bits(2, 0xF0, 4)?;
        let mut content_reference_id_record_length = 0;
        let mut content_reference_id_bytes = vec![];
        if content_reference_id_record_flag {
            reader.advance(1);
            content_reference_id_record_length = reader.get_bits(2, 0xFF, 0).map(|x| x as u8)?;
            content_reference_id_bytes = reader.get_bytes(2, content_reference_id_record_length as usize)?;
            reader.advance(content_reference_id_record_length as usize);
        }
        match content_time_base_indicator {
            value @ (1 | 2)=> {
                let content_time_base_value = reader.get_bits_u40(3, vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF]).map(|x| x as u64);
                let metadata_time_base_value = reader.get_bits_u40(8, vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF]).map(|x| x as u64);
                let content_id = if value == 2 {
                    reader.get_bits(13, 0x7F, 7)
                } else {
                    None
                };
                Some(ContentLabellingDescriptor {
                    header,
                    metadata_application_format,
                    metadata_application_format_identifier,
                    content_reference_id_record_flag,
                    content_time_base_indicator,
                    content_reference_id_record_length: if content_reference_id_record_flag { Some(content_reference_id_record_length) } else { None },
                    content_reference_id_bytes: if content_reference_id_record_flag { Some(content_reference_id_bytes) } else { None },
                    content_time_base_value,
                    metadata_time_base_value,
                    content_id,
                    content_time_base_data_length: None,
                    private_data: reader.remaining_from(10).unwrap(),
                })
            }
            3..=7 => {
                let content_time_base_data_length = reader.get_bits(3, 0xFF, 0)?;
                reader.advance(content_time_base_data_length as usize);
                let private_data = reader.remaining_from(3)?;
                Some(ContentLabellingDescriptor {
                    header,
                    metadata_application_format,
                    metadata_application_format_identifier,
                    content_reference_id_record_flag,
                    content_time_base_indicator,
                    content_reference_id_record_length: if content_reference_id_record_flag { Some(content_reference_id_record_length) } else { None },
                    content_reference_id_bytes: if content_reference_id_record_flag { Some(content_reference_id_bytes) } else { None },
                    content_time_base_value: None,
                    metadata_time_base_value: None,
                    content_id: None,
                    content_time_base_data_length: Some(content_time_base_data_length),
                    private_data,
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mpegts::descriptors::tags::DescriptorTag;
    use crate::mpegts::descriptors::DescriptorHeader;

    #[test]
    fn test_content_labelling_descriptor() {
        // Test case 1: Basic descriptor with content_time_base_indicator = 1
        let data = vec![
            0x00, 0x42, // metadata_application_format
            0x80, // content_reference_id_record_flag=1, content_time_base_indicator=0
            0x03, // content_reference_id_record_length
            0x01, 0x02, 0x03, // content_reference_id_bytes
            0x80, 0x00, 0x00, 0x00, 0x01, // content_time_base_value
            0x80, 0x00, 0x00, 0x00, 0x02, // metadata_time_base_value
            0xFF, 0xFF, // private data
        ];

        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::ContentLabelingDescriptorTag,
            descriptor_length: data.len() as u8,
        };

        let descriptor = ContentLabellingDescriptor::unmarshall(header.clone(), &data).unwrap();

        assert_eq!(descriptor.metadata_application_format, 0x42);
        assert_eq!(descriptor.metadata_application_format_identifier, None);
        assert!(descriptor.content_reference_id_record_flag);
        assert_eq!(descriptor.content_time_base_indicator, 0);
        assert_eq!(descriptor.content_reference_id_record_length, Some(3));
        assert_eq!(
            descriptor.content_reference_id_bytes,
            Some(vec![0x01, 0x02, 0x03])
        );
        assert_eq!(descriptor.content_time_base_value, Some(1));
        assert_eq!(descriptor.metadata_time_base_value, Some(2));
        assert_eq!(descriptor.private_data, vec![0xFF, 0xFF]);

        // Test case 2: Extended format with metadata_application_format_identifier
        let data = vec![
            0xFF, 0xFF, // metadata_application_format = 0xFFFF
            0x00, 0x00, 0x00, 0x42, // metadata_application_format_identifier
            0x20, // content_reference_id_record_flag=0, content_time_base_indicator=2
            0x80, 0x00, 0x00, 0x00, 0x0A, // content_time_base_value
            0x80, 0x00, 0x00, 0x00, 0x0B, // metadata_time_base_value
            0x40, // content_id = 0x40
            0xAA, // private data
        ];

        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::ContentLabelingDescriptorTag,
            descriptor_length: data.len() as u8,
        };

        let descriptor = ContentLabellingDescriptor::unmarshall(header.clone(), &data).unwrap();

        assert_eq!(descriptor.metadata_application_format, 0xFFFF);
        assert_eq!(
            descriptor.metadata_application_format_identifier,
            Some(0x42)
        );
        assert!(!descriptor.content_reference_id_record_flag);
        assert_eq!(descriptor.content_time_base_indicator, 2);
        assert_eq!(descriptor.content_time_base_value, Some(10));
        assert_eq!(descriptor.metadata_time_base_value, Some(11));
        assert_eq!(descriptor.content_id, Some(0x40));
        assert_eq!(descriptor.private_data, vec![0xAA]);

        // Test case 3: Reserved content_time_base_indicator
        let data = vec![
            0x00, 0x42, // metadata_application_format
            0x30, // content_reference_id_record_flag=0, content_time_base_indicator=3
            0x02, // content_time_base_data_length
            0xDE, 0xAD, // reserved bytes
            0xBE, 0xEF, // private data
        ];

        let header = DescriptorHeader {
            descriptor_tag: DescriptorTag::ContentLabelingDescriptorTag,
            descriptor_length: data.len() as u8,
        };

        let descriptor = ContentLabellingDescriptor::unmarshall(header, &data).unwrap();

        assert_eq!(descriptor.metadata_application_format, 0x42);
        assert_eq!(descriptor.content_time_base_indicator, 3);
        assert_eq!(descriptor.content_time_base_data_length, Some(2));
        assert_eq!(descriptor.private_data, vec![0xBE, 0xEF]);
    }
}
