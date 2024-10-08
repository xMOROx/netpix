use crate::mpegts::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use crate::mpegts::psi::pat::{ProgramAssociationItem, ProgramAssociationTable};

pub struct PatBuffer {
    last_section_number: u8,
    pat_fragments: Vec<FragmentaryProgramAssociationTable>,
}



impl PatBuffer {
    pub fn new() -> Self {
        PatBuffer {
            last_section_number: 0,
            pat_fragments: Vec::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: FragmentaryProgramAssociationTable) {
        self.pat_fragments.push(fragment);
    }

    pub fn is_complete(&self) -> bool {
        self.pat_fragments.len() as u8 == self.last_section_number + 1
    }

    pub fn build(&self) -> Option<ProgramAssociationTable> {
        if !self.is_complete() {
            return None;
        }

        let cumulated_payload = self.pat_fragments.iter().fold(Vec::new(), |mut acc, fragment| {
            acc.extend_from_slice(&fragment.payload);
            acc
        });

        ProgramAssociationTable::build(self.pat_fragments[0].transport_stream_id, &cumulated_payload)
    }
}