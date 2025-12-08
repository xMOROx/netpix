use super::reception_report::ReceptionReport;
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct SenderReport {
    pub ssrc: u32,
    pub ntp_time: u64,
    pub rtp_time: u32,
    pub packet_count: u32,
    pub octet_count: u32,
    pub reports: Vec<ReceptionReport>,
    // ignoring profile extensions ATM
    // as we won't be able to decode them anyways
}


impl SenderReport {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(packet: &rtcp::sender_report::SenderReport) -> Self {
        let reports = packet.reports.iter().map(ReceptionReport::new).collect();

        Self {
            ssrc: packet.ssrc,
            ntp_time: packet.ntp_time,
            rtp_time: packet.rtp_time,
            packet_count: packet.packet_count,
            octet_count: packet.octet_count,
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
