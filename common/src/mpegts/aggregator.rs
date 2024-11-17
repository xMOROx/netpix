use super::pes::PacketizedElementaryStream;
use super::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use super::psi::pmt::fragmentary_pmt::FragmentaryProgramMapTable;
use super::psi::psi_buffer::PsiBuffer;
use super::psi::{pat::pat_buffer::PatBuffer, pmt::pmt_buffer::PmtBuffer};
use crate::mpegts::pes::pes_buffer::PesBuffer;
use crate::mpegts::psi::pat::ProgramAssociationTable;
use crate::mpegts::psi::pmt::ProgramMapTable;
use crate::mpegts::MpegtsFragment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MpegtsAggregator {
    pub pat_buffer: PatBuffer,
    pub pmt_buffers: HashMap<u16, PmtBuffer>,
    pub pes_buffers: HashMap<u16, PesBuffer>,
    pat: Option<ProgramAssociationTable>,
    pmt: HashMap<u16, ProgramMapTable>,
    pes: HashMap<u16, PacketizedElementaryStream>,
}

impl MpegtsAggregator {
    pub fn new() -> Self {
        MpegtsAggregator {
            pat_buffer: PatBuffer::new(0),
            pmt_buffers: HashMap::default(),
            pes_buffers: HashMap::default(),
            pat: None,
            pmt: HashMap::default(),
            pes: HashMap::default(),
        }
    }

    pub fn add_pat(&mut self, fragment: FragmentaryProgramAssociationTable) {
        self.pat_buffer.add_fragment(fragment);
    }

    pub fn add_pmt(&mut self, pmt_pid: u16, fragment: FragmentaryProgramMapTable) {
        if let Some(pmt_buffer) = self.pmt_buffers.get_mut(&pmt_pid) {
            if pmt_buffer.is_complete() {
                return;
            }

            if !pmt_buffer.is_fragment_inside(&fragment) {
                pmt_buffer.add_fragment(fragment);
            }
            return;
        }

        let mut pmt_buffer = PmtBuffer::new(fragment.header.last_section_number);
        pmt_buffer.add_fragment(fragment);
        self.pmt_buffers.insert(pmt_pid, pmt_buffer);
    }

    pub fn add_pes(&mut self, pes_pid: u16, fragment: MpegtsFragment) {
        if let Some(pes_buffer) = self.pes_buffers.get_mut(&pes_pid) {
            if pes_buffer.is_complete() {
                return;
            }

            pes_buffer.add_fragment(&fragment);
        }

        let mut pes_buffer = PesBuffer::new();
        pes_buffer.add_fragment(&fragment);
        self.pes_buffers.insert(pes_pid, pes_buffer);
    }

    pub fn get_pat(&mut self) -> Option<ProgramAssociationTable> {
        let pat = self.pat_buffer.build();
        if pat.is_some() {
            self.pat_buffer.clear();
        }

        self.pat = pat.clone();
        pat
    }

    pub fn get_pmt(&mut self, pid: u16) -> Option<ProgramMapTable> {
        if let Some(pmt_buffer) = self.pmt_buffers.get_mut(&pid) {
            let pmt = pmt_buffer.build();
            if pmt.is_some() {
                pmt_buffer.clear();
                self.pmt.insert(pid, pmt.clone().unwrap());
            }

            pmt
        } else {
            None
        }
    }

    pub fn get_pes(&mut self, pid: u16) -> Option<PacketizedElementaryStream> {
        if let Some(pes_buffer) = self.pes_buffers.get_mut(&pid) {
            let pes = pes_buffer.build();
            if pes.is_some() {
                pes_buffer.clear();
                self.pes.insert(pid, pes.clone().unwrap());
            }

            pes
        } else {
            None
        }
    }

    pub fn is_pat_complete(&self) -> bool {
        self.pat.is_some()
    }

    pub fn is_pmt_complete(&self, pid: u16) -> bool {
        self.pmt.get(&pid).is_some()
    }

    pub fn clear(&mut self) {
        self.pat_buffer.clear();
        self.pmt_buffers.clear();
        self.pat = None;
        self.pmt.clear();
    }
}
