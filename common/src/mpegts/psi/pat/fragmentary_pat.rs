use serde::{Deserialize, Serialize};
use crate::mpegts::psi::pat::{CURRENT_NEXT_INDICATOR_MASK, HEADER_AFTER_SECTION_LENGTH_SIZE, HEADER_SIZE, SECTION_LENGTH_UPPER_MASK, SECTION_SYNTAX_INDICATOR_MASK, VERSION_NUMBER_MASK, PADDING_BYTE};
use crate::mpegts::psi::ProgramSpecificInformationHeader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FragmentaryProgramAssociationTable {
    pub header: ProgramSpecificInformationHeader,
    pub transport_stream_id: u16,
    pub payload: Vec<u8>,
}

impl FragmentaryProgramAssociationTable {
    pub fn unmarshall(data: &[u8], pointer_field: bool) -> Option<Self> {
        if data.len() < HEADER_SIZE {
            return None;
        }

        let data = if pointer_field {
            let pointer_field = data[0];
            &data[pointer_field as usize + 1..]
        } else {
            data
        };

        let header = if let Some(header) = Self::unmarshall_header(data) {
            header
        } else {
            return None;
        };

        let transport_stream_id = ((data[3] as u16) << 8) | data[4] as u16;

        let last_byte = Self::determine_last_byte(data);

        let payload = data[HEADER_SIZE + HEADER_AFTER_SECTION_LENGTH_SIZE..last_byte].to_vec();

        Some(FragmentaryProgramAssociationTable {
            header,
            transport_stream_id,
            payload,
        })
    }

    fn determine_last_byte(data: &[u8]) -> usize {
        let mut last_byte = data.len();

        for i in 0..data.len() {
            if data[i] == PADDING_BYTE {
                last_byte = i;
                break;
            }
        }

        last_byte
    }

    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader> {
        let table_id = data[0];
        let section_syntax_indicator = (data[1] & SECTION_SYNTAX_INDICATOR_MASK) != 0;
        let section_length = ((data[1] & SECTION_LENGTH_UPPER_MASK) as u16) << 8 | data[2] as u16;

        if section_length < HEADER_AFTER_SECTION_LENGTH_SIZE as u16 {
            return None;
        }

        let version_number = (data[5] & VERSION_NUMBER_MASK) >> 1;
        let current_next_indicator = (data[5] & CURRENT_NEXT_INDICATOR_MASK) != 0;
        let section_number = data[6];
        let last_section_number = data[7];

        Some(
            ProgramSpecificInformationHeader {
                table_id,
                section_syntax_indicator,
                section_length,
                version_number,
                current_next_indicator,
                section_number,
                last_section_number,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_pat_data() -> Vec<u8> {
        let data: Vec<u8> = vec![
            0x00, 0x00, 0xB0, 0x31, 0x00, 0x14, 0xD7, 0x00, 0x00, 0x00, 0x00, 0xE0,
            0x10, 0x00, 0x01, 0xE0, 0x24, 0x00, 0x02, 0xE0, 0x25, 0x00, 0x03, 0xE0,
            0x30, 0x00, 0x04, 0xE0, 0x31, 0x00, 0x1A, 0xE0, 0x67, 0x00, 0x1C, 0xE0,
            0x6F, 0x43, 0x9D, 0xE3, 0xF1, 0x43, 0xA3, 0xE3, 0xF7, 0x43, 0xAC, 0xE4,
            0x00, 0xC3, 0x69, 0xA6, 0xD8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
        ];

        data
    }

    #[test]
    fn test_fragmentary_pat() {
        let data = create_pat_data();


        // Vector to collect the payloads from each fragment
        let unmarchalled = self::FragmentaryProgramAssociationTable::unmarshall(&data, true).unwrap();
        assert_eq!(unmarchalled.header.table_id, 0);
        assert_eq!(unmarchalled.header.section_syntax_indicator, true);
        assert_eq!(unmarchalled.header.section_length, 49);
        assert_eq!(unmarchalled.header.current_next_indicator, true);
        assert_eq!(unmarchalled.transport_stream_id, 20);
        assert_eq!(unmarchalled.payload.len(), 44);
    }
}
