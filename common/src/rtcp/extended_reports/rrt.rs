use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct ReceiverReferenceTimeReportBlock {
    pub ntp_timestamp: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ReceiverReferenceTimeReportBlock {
    pub fn new(packet: &rtcp::extended_report::ReceiverReferenceTimeReportBlock) -> Self {
        Self {
            ntp_timestamp: packet.ntp_timestamp,
        }
    }
}
