use super::reception_report::ReceptionReport;
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct ReceiverReport {
    pub ssrc: u32,
    pub reports: Vec<ReceptionReport>,
    // ignoring profile extensions ATM
    // as we won't be able to decode them anyways
}

impl ReceiverReport {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(packet: &rtcp::receiver_report::ReceiverReport) -> Self {
        let reports = packet.reports.iter().map(ReceptionReport::new).collect();

        Self {
            ssrc: packet.ssrc,
            reports,
        }
    }

    pub fn get_ssrcs(&self) -> Vec<u32> {
        let mut ssrcs = Vec::with_capacity(1 + self.reports.len());
        ssrcs.push(self.ssrc);
        ssrcs.extend(self.reports.iter().map(|r| r.ssrc));

        ssrcs
    }
}
