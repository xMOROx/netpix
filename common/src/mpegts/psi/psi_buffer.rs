use crate::mpegts;
use crate::mpegts::psi::ProgramSpecificInformationHeader;

pub trait PsiBuffer<T, U: FragmentaryPsi> {
    fn new(last_section_number: u8) -> Self;
    fn is_complete(&self) -> bool;
    fn last_section_number(&self) -> u8;
    fn set_last_section_number(&mut self, last_section_number: u8);
    fn add_fragment(&mut self, fragment: U);
    fn get_fragments(&self) -> &Vec<U>;
    fn build(&mut self) -> Option<T>;
    fn clear(&mut self);
}

pub trait FragmentaryPsi {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self>
    where
        Self: Sized;
    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader>;

    fn determine_last_byte(data: &[u8]) -> usize {
        let mut last_byte = data.len();
        let mut padding_count = 0;

        for (i, _) in data.iter().enumerate() {
            if data[i] == mpegts::PADDING_BYTE {
                padding_count += 1;
                if padding_count == 3 {
                    last_byte = i - 2;
                    break;
                }
            } else {
                padding_count = 0;
            }
        }

        last_byte
    }
}
