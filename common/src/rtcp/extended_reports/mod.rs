pub mod dlrr;
pub mod rrt;

use crate::rtcp::extended_reports::dlrr::DLRRReportBlock;
use crate::rtcp::extended_reports::rrt::ReceiverReferenceTimeReportBlock;
use bincode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone)]
pub struct ExtendedReport {
    pub sender_ssrc: u32,
    pub reports: Vec<BlockType>,
}

impl ExtendedReport {
    pub fn get_type_name(&self) -> &str {
        "Extended Report"
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(pack: &rtcp::extended_report::ExtendedReport) -> Self {
        use rtcp::extended_report::DLRRReportBlock;
        use rtcp::extended_report::DuplicateRLEReportBlock;
        use rtcp::extended_report::LossRLEReportBlock;
        use rtcp::extended_report::PacketReceiptTimesReportBlock;
        use rtcp::extended_report::ReceiverReferenceTimeReportBlock;
        use rtcp::extended_report::StatisticsSummaryReportBlock;
        use rtcp::extended_report::UnknownReportBlock;
        use rtcp::extended_report::VoIPMetricsReportBlock;

        let mut reports = vec![];
        for report in &pack.reports {
            let any = report.as_any();

            if let Some(_block) = any.downcast_ref::<LossRLEReportBlock>() {
                reports.push(BlockType::LossRLE);
            } else if let Some(_block) = any.downcast_ref::<DuplicateRLEReportBlock>() {
                reports.push(BlockType::DuplicateRLE);
            } else if let Some(_block) = any.downcast_ref::<PacketReceiptTimesReportBlock>() {
                reports.push(BlockType::PacketReceiptTimes);
            } else if let Some(block) = any.downcast_ref::<ReceiverReferenceTimeReportBlock>() {
                reports.push(BlockType::ReceiverReferenceTime(
                    rrt::ReceiverReferenceTimeReportBlock::new(block),
                ));
            } else if let Some(block) = any.downcast_ref::<DLRRReportBlock>() {
                reports.push(BlockType::DLRR(dlrr::DLRRReportBlock::new(block)));
            } else if let Some(_block) = any.downcast_ref::<StatisticsSummaryReportBlock>() {
                reports.push(BlockType::StatisticsSummary);
            } else if let Some(_block) = any.downcast_ref::<VoIPMetricsReportBlock>() {
                reports.push(BlockType::VoIPMetrics);
            } else if let Some(_block) = any.downcast_ref::<UnknownReportBlock>() {
                reports.push(BlockType::Unknown);
            }
        }

        Self {
            sender_ssrc: pack.sender_ssrc,
            reports,
        }
    }
}

#[derive(Decode, Encode, Debug, Clone)]
pub enum BlockType {
    Unknown,
    LossRLE,
    DuplicateRLE,
    PacketReceiptTimes,
    ReceiverReferenceTime(ReceiverReferenceTimeReportBlock),
    DLRR(DLRRReportBlock),
    StatisticsSummary,
    VoIPMetrics,
}

impl BlockType {
    pub fn get_type_name(&self) -> &str {
        match self {
            BlockType::Unknown => "Unknown",
            BlockType::LossRLE => "LossRLE",
            BlockType::DuplicateRLE => "DuplicateRLE",
            BlockType::PacketReceiptTimes => "Packet Receipt Times",
            BlockType::ReceiverReferenceTime(_) => "Receiver Reference Time",
            BlockType::DLRR(_) => "DLRR",
            BlockType::StatisticsSummary => "StatisticsSummary",
            BlockType::VoIPMetrics => "VoIPMetrics",
        }
    }
}
