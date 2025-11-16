use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct SliEntry {
    pub first: u16,
    pub number: u16,
    pub picture: u8,
}

#[cfg(not(target_arch = "wasm32"))]
impl SliEntry {
    pub fn new(entry: &rtcp::payload_feedbacks::slice_loss_indication::SliEntry) -> Self {
        Self {
            first: entry.first,
            number: entry.number,
            picture: entry.picture,
        }
    }
}
