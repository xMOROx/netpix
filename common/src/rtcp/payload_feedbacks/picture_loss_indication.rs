use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct PictureLossIndication {
    pub sender_ssrc: u32,
    pub media_ssrc: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl PictureLossIndication {
    pub fn new(
        packet: &rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication,
    ) -> Self {
        Self {
            sender_ssrc: packet.sender_ssrc,
            media_ssrc: packet.media_ssrc,
        }
    }
}
