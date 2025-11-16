use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct DLRRReportBlock {
    pub reports: Vec<DLRRReport>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DLRRReportBlock {
    pub fn new(packet: &rtcp::extended_report::DLRRReportBlock) -> Self {
        let mut reports = vec![];

        for report in &packet.reports {
            reports.push(DLRRReport::new(report));
        }

        Self { reports }
    }
}

#[derive(Decode, Encode, Debug, Clone)]
pub struct DLRRReport {
    pub ssrc: u32,
    pub last_rr: u32,
    pub dlrr: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl DLRRReport {
    fn new(packet: &rtcp::extended_report::DLRRReport) -> Self {
        Self{
            ssrc: packet.ssrc,
            last_rr: packet.last_rr,
            dlrr: packet.dlrr,
        }
    }
}