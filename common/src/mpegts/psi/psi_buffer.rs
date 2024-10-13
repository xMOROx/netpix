use crate::mpegts;
use crate::mpegts::psi::{CURRENT_NEXT_INDICATOR_MASK, MAX_SECTION_LENGTH, ProgramSpecificInformationHeader, SECTION_LENGTH_UPPER_MASK, SECTION_SYNTAX_INDICATOR_MASK, VERSION_NUMBER_MASK};

pub trait PsiBuffer<T, U: FragmentaryPsi> {
    fn new(last_section_number: u8) -> Self;
    fn is_complete(&self) -> bool;
    fn last_section_number(&self) -> u8;
    fn add_fragment(&mut self, fragment: U);
    fn build(&self) -> Option<T>;
}

pub trait FragmentaryPsi {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self> where Self: Sized;
    fn unmarshall_header(data: &[u8], header_after_section_length: usize) -> Option<ProgramSpecificInformationHeader> {
        let table_id = data[0];
        let section_syntax_indicator = (data[1] & SECTION_SYNTAX_INDICATOR_MASK) != 0;
        let section_length = ((data[1] & SECTION_LENGTH_UPPER_MASK) as u16) << 8 | data[2] as u16;

        if section_length < header_after_section_length as u16 {
            return None;
        }

        if section_length > MAX_SECTION_LENGTH {
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

    fn determine_last_byte(data: &[u8]) -> usize {
        let mut last_byte = data.len();

        for i in 0..data.len() {
            if data[i] == mpegts::PADDING_BYTE {
                last_byte = i;
                break;
            }
        }

        last_byte
    }
}