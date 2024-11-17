#[cfg(test)]
mod tests;

use crate::mpegts::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use crate::mpegts::psi::pat::ProgramAssociationTable;
use crate::mpegts::psi::psi_buffer::PsiBuffer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatBuffer {
    last_section_number: u8,
    pat_fragments: Vec<FragmentaryProgramAssociationTable>,
}

impl PsiBuffer<ProgramAssociationTable, FragmentaryProgramAssociationTable> for PatBuffer {
    fn new(last_section_number: u8) -> Self {
        PatBuffer {
            last_section_number,
            pat_fragments: Vec::new(),
        }
    }

    fn is_complete(&self) -> bool {
        self.pat_fragments.len() as u8 == self.last_section_number + 1
    }

    fn last_section_number(&self) -> u8 {
        self.last_section_number
    }

    fn set_last_section_number(&mut self, last_section_number: u8) {
        if self.last_section_number == 0 {
            self.last_section_number = last_section_number;
        }
    }

    fn add_fragment(&mut self, fragment: FragmentaryProgramAssociationTable) {
        self.pat_fragments.push(fragment);
    }

    fn get_fragments(&self) -> &Vec<FragmentaryProgramAssociationTable> {
        &self.pat_fragments
    }

    fn build(&mut self) -> Option<ProgramAssociationTable> {
        if !self.is_complete() {
            return None;
        }

        let accumulated_payload =
            self.pat_fragments
                .iter()
                .fold(Vec::new(), |mut acc, fragment| {
                    acc.extend_from_slice(&fragment.payload);
                    acc
                });

        ProgramAssociationTable::build(
            self.pat_fragments[0].transport_stream_id,
            &accumulated_payload,
        )
    }

    fn clear(&mut self) {
        self.last_section_number = 0;
        self.pat_fragments.clear();
    }
}
