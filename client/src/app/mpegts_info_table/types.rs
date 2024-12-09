use netpix_common::mpegts::descriptors::Descriptors;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::mpegts::psi::pat::ProgramAssociationTable;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;
use std::cmp::Ordering;

pub const LINE_HEIGHT: f32 = 20.0;

pub struct MpegTsInfo {
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: Option<ProgramMapTable>,
}

#[derive(Default)]
pub struct OpenModal {
    pub descriptor: Option<Descriptors>,
}

#[derive(Hash, Eq, PartialEq, Ord, Clone)]
pub struct RowKey {
    pub pid: PIDTable,
    pub alias: String,
}

impl PartialOrd for RowKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.alias.eq(&other.alias) {
            return self.alias.partial_cmp(&other.alias);
        }
        self.pid.partial_cmp(&other.pid)
    }
}
