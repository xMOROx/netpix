#[cfg(test)]
mod tests;

use crate::mpegts::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use crate::mpegts::psi::pat::ProgramAssociationTable;
use crate::mpegts::psi::psi_buffer::PsiBuffer;
use crate::utils::{DataAccumulator, DataValidator};
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct PatBuffer {
    last_section_number: u8,
    pat_fragments: Vec<FragmentaryProgramAssociationTable>,
}

impl DataAccumulator for PatBuffer {
    fn accumulate_payload(&self) -> Vec<u8> {
        self.pat_fragments
            .iter()
            .fold(Vec::new(), |mut acc, fragment| {
                acc.extend_from_slice(&fragment.payload);
                acc
            })
    }

    fn accumulate_descriptors(&self) -> Vec<u8> {
        Vec::new() // PAT doesn't have descriptors
    }
}

impl DataValidator for PatBuffer {
    fn validate(&self) -> bool {
        if self.pat_fragments.is_empty() {
            return false;
        }
        self.is_complete()
    }
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
        if !self.validate() {
            return None;
        }

        self.pat_fragments.first().and_then(|first| {
            ProgramAssociationTable::build(
                first.transport_stream_id,
                &self.accumulate_payload(),
                self.pat_fragments.len(),
            )
        })
    }

    fn clear(&mut self) {
        self.last_section_number = 0;
        self.pat_fragments.clear();
    }
}

impl PatBuffer {
    pub fn get_transport_stream_id(&self) -> u16 {
        self.pat_fragments
            .first()
            .map(|f| f.transport_stream_id)
            .unwrap_or(0)
    }

    pub fn is_fragment_inside(&self, fragment: &FragmentaryProgramAssociationTable) -> bool {
        self.pat_fragments.first().map_or(false, |first| {
            (self.pat_fragments.len() as u8) >= fragment.header.section_number
                && first.transport_stream_id == fragment.transport_stream_id
        })
    }
}
