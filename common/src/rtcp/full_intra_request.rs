use bincode::{Decode, Encode};
use crate::rtcp::fir_entry::FirEntry;

#[derive(Decode, Encode, Debug, Clone)]
pub struct FullIntraRequest {
    pub sender_ssrc: u32,
    pub media_ssrc: u32,
    pub fir: Vec<FirEntry>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FullIntraRequest {
    pub fn new(packet: &rtcp::payload_feedbacks::full_intra_request::FullIntraRequest) -> Self {
        let fir = packet.fir.iter().map(FirEntry::new).collect();

        Self {
            sender_ssrc: packet.sender_ssrc,
            media_ssrc: packet.media_ssrc,
            fir,
        }
    }
}