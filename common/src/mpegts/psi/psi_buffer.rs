use crate::mpegts;
use crate::mpegts::psi::{ProgramSpecificInformationHeader};

pub trait PsiBuffer<T, U: FragmentaryPsi> {
    fn new(last_section_number: u8) -> Self;
    fn is_complete(&self) -> bool;
    fn last_section_number(&self) -> u8;
    fn add_fragment(&mut self, fragment: U);
    fn get_fragments(&self) -> &Vec<U>;
    fn build(&self) -> Option<T>;
}

pub trait FragmentaryPsi {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self>
    where
        Self: Sized;
    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader>;

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