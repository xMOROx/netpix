#[cfg(test)]
mod tests;

use crate::mpegts::psi::pmt::fragmentary_pmt::FragmentaryProgramMapTable;
use crate::mpegts::psi::pmt::{PmtFields, ProgramMapTable};
use crate::mpegts::psi::psi_buffer::PsiBuffer;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PmtBuffer {
    last_section_number: u8,
    pmt_fragments: Vec<FragmentaryProgramMapTable>,
}

impl PsiBuffer<ProgramMapTable, FragmentaryProgramMapTable> for PmtBuffer {
    fn new(last_section_number: u8) -> Self {
        PmtBuffer {
            last_section_number,
            pmt_fragments: Vec::new(),
        }
    }

    fn is_complete(&self) -> bool {
        self.pmt_fragments.len() as u8 == self.last_section_number + 1
    }

    fn last_section_number(&self) -> u8 {
        self.last_section_number
    }

    fn set_last_section_number(&mut self, last_section_number: u8) {
        if self.last_section_number == 0 {
            self.last_section_number = last_section_number;
        }
    }

    fn add_fragment(&mut self, fragment: FragmentaryProgramMapTable) {
        self.pmt_fragments.push(fragment);
    }

    fn get_fragments(&self) -> &Vec<FragmentaryProgramMapTable> {
        &self.pmt_fragments
    }

    fn build(&mut self) -> Option<ProgramMapTable> {
        if !self.is_complete() {
            return None;
        }

        let (cumulated_payload, cumulated_descriptors_payload) = self.accumulator();
        let fields = PmtFields {
            program_number: self.pmt_fragments[0].fields.program_number,
            pcr_pid: self.pmt_fragments[0].fields.pcr_pid,
            program_info_length: self.pmt_fragments[0].fields.program_info_length,
        };

        let pmt =
            ProgramMapTable::build(fields, &cumulated_descriptors_payload, &cumulated_payload);
        pmt
    }

    fn clear(&mut self) {
        self.last_section_number = 0;
        self.pmt_fragments.clear();
    }
}

impl PmtBuffer {
    pub fn get_program_number(&self) -> u16 {
        self.pmt_fragments[0].fields.program_number
    }

    pub fn is_fragment_inside(&self, fragment: &FragmentaryProgramMapTable) -> bool {
        if self.pmt_fragments.is_empty() {
            return false;
        }
        (self.pmt_fragments.len() as u8) >= fragment.header.section_number
            && self.pmt_fragments[0].fields.program_number == fragment.fields.program_number
    }

    fn accumulator(&self) -> (Vec<u8>, Vec<u8>) {
        let cumulated_payload = self
            .pmt_fragments
            .iter()
            .fold(Vec::new(), |mut acc, fragment| {
                acc.extend_from_slice(&fragment.payload);
                acc
            });

        let cumulated_descriptors_payload =
            self.pmt_fragments
                .iter()
                .fold(Vec::new(), |mut acc, fragment| {
                    acc.extend_from_slice(&fragment.descriptors_payload);
                    acc
                });

        (cumulated_payload, cumulated_descriptors_payload)
    }
}
