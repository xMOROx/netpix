use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct FirEntry {
    pub ssrc: u32,
    pub sequence_number: u8,
}

#[cfg(not(target_arch = "wasm32"))]
impl FirEntry {
    pub fn new(entry: &rtcp::payload_feedbacks::full_intra_request::FirEntry) -> Self {
        Self {
            ssrc: entry.ssrc,
            sequence_number: entry.sequence_number,
        }
    }
}