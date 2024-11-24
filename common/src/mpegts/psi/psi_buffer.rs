use crate::mpegts;
use crate::mpegts::psi::ProgramSpecificInformationHeader;
use crate::utils::{ByteOperations, DataParser, DataValidator};

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

pub trait FragmentaryPsi: DataParser<Output = Self> + DataValidator {
    fn unmarshall(data: &[u8], is_pointer_field: bool) -> Option<Self>
    where
        Self: Sized;
    fn unmarshall_header(data: &[u8]) -> Option<ProgramSpecificInformationHeader>;

    fn determine_last_byte(data: &[u8]) -> usize {
        ByteOperations::find_padding_end(data, mpegts::PADDING_BYTE, 3)
            .map(|pos| pos)
            .unwrap_or(data.len())
    }
}
