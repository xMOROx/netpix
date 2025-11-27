use crate::rtcp::payload_feedbacks::sli_entry::SliEntry;
use bincode::{Decode, Encode};
#[derive(Decode, Encode, Debug, Clone)]
pub struct SliceLossIndication {
    pub sender_ssrc: u32,
    pub media_ssrc: u32,
    pub sli_entries: Vec<SliEntry>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SliceLossIndication {
    pub fn new(
        packet: &rtcp::payload_feedbacks::slice_loss_indication::SliceLossIndication,
    ) -> Self {
        let sli_entries = packet.sli_entries.iter().map(SliEntry::new).collect();

        Self {
            sender_ssrc: packet.sender_ssrc,
            media_ssrc: packet.media_ssrc,
            sli_entries,
        }
    }
}
