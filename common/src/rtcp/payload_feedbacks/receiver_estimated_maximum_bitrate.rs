use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct ReceiverEstimatedMaximumBitrate {
    pub sender_ssrc: u32,
    pub bitrate: f32,
    pub ssrcs: Vec<u32>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ReceiverEstimatedMaximumBitrate {
    pub fn new(packet: &rtcp::payload_feedbacks::receiver_estimated_maximum_bitrate::ReceiverEstimatedMaximumBitrate) -> Self {
        Self {
            sender_ssrc: packet.sender_ssrc,
            bitrate: packet.bitrate,
            ssrcs: packet.ssrcs.clone(),
        }
    }
}