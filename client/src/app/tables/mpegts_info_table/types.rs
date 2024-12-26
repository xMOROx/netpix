use netpix_common::mpegts::{
    descriptors::Descriptors,
    header::PIDTable,
    psi::{pat::ProgramAssociationTable, pmt::ProgramMapTable},
};

pub const LINE_HEIGHT: f32 = 32.0;

pub struct MpegTsInfo {
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: Option<ProgramMapTable>,
}

#[derive(Default)]
pub struct OpenModal {
    pub descriptor: Option<(usize, Descriptors)>,
    pub is_open: bool,
    pub active_descriptor: Option<(usize, Descriptors)>,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct RowKey {
    pub pid: PIDTable,
    pub alias: String,
}
