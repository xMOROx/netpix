use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{
        common::*, tables::rtcp_packets_table::*, FilterHelpContent, FilterInput,
        TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::ntp_to_string,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::rtcp::extended_reports::BlockType;
use netpix_common::rtcp::payload_feedbacks::PayloadFeedback;
use netpix_common::rtcp::ExtendedReport;
use netpix_common::{
    packet::SessionPacket,
    rtcp::{
        payload_feedbacks::{
            FullIntraRequest, PictureLossIndication, ReceiverEstimatedMaximumBitrate,
            SliceLossIndication,
        },
        Goodbye, ReceiverReport, ReceptionReport, RtcpPacket, SenderReport, SourceDescription,
    },
};
use std::any::Any;

declare_table_struct!(RtcpPacketsTable);

impl_table_base!(
    RtcpPacketsTable,
    FilterHelpContent::builder("RTCP Packet Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("type", "Filter by RTCP packet type")
        .example("source:192.168 AND type:sender")
        .example("dest:10.0.0 OR type:receiver")
        .build(),
    "rtcp_packets", "RTCP Packets"
    ;
    build_header: |self, header| {
        let headers = [
            ("No.", "Packet number (including skipped packets) + compound RTCP packet number inside the parentheses"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Type", "Type of the RTCP packet"),
            ("Data", "Data specific to RTCP packet's type"),
        ];

        for (label, desc) in headers {
            header.col(|ui| {
                ui.label(RichText::new(label.to_string()).size(TABLE_HEADER_TEXT_SIZE).strong())
                    .on_hover_text(desc.to_string());
            });
        }
    }
    ;
    build_table_body: |self, body| {
        let streams = &self.streams.borrow();
        let mut rtcp_packets = Vec::new();

        // Collect RTCP packets with metadata
        for packet in streams.packets.values() {
            let rtcp = match &packet.contents {
                SessionPacket::Rtcp(ref rtcp) => rtcp,
                _ => continue,
            };

            for (idx, rtcp_packet) in rtcp.iter().enumerate() {
                let ctx = RtcpFilterContext {
                    packet: rtcp_packet,
                    source_addr: &packet.source_addr.to_string(),
                    destination_addr: &packet.destination_addr.to_string(),
                };

                if !self.packet_matches_filter(&ctx) {
                    continue;
                }

                rtcp_packets.push(PacketInfo {
                    id: packet.id as u64,
                    packet,
                    rtcp_packet,
                    compound_index: idx + 1,
                });
            }
        }

        if rtcp_packets.is_empty() {
            return;
        }

        let heights = rtcp_packets.iter().map(|info| get_row_height(info.rtcp_packet));
        let first_ts = streams.packets.first().unwrap().timestamp;

        body.heterogeneous_rows(heights, |mut row| {
            let info = &rtcp_packets[row.index()];

            row.col(|ui| {
                ui.label(format!("{} ({})", info.id, info.compound_index));
            });
            row.col(|ui| {
                let timestamp = info.packet.timestamp - first_ts;
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });
            row.col(|ui| {
                ui.label(info.packet.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(info.packet.destination_addr.to_string());
            });
            row.col(|ui| {
                ui.label(info.rtcp_packet.get_type_name().to_string());
            });
            row.col(|ui| {
                build_packet(ui, info.rtcp_packet);
            });
        });
    }
);

declare_table!(RtcpPacketsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(60.0), 60.0, None, false, true),
        column(Some(70.0), 70.0, None, false, true),
        column(Some(150.0), 150.0, None, false, true),
        column(Some(150.0), 150.0, None, false, true),
        column(Some(250.0), 250.0, None, false, true),
        column(None, 200.0, None, false, true),
    )
});

impl RtcpPacketsTable {
    fn packet_matches_filter(&self, ctx: &RtcpFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        parse_filter(self.filter_input.get_filter())
            .map(|filter| filter.matches(ctx))
            .unwrap_or(true)
    }
}

fn get_row_height(packet: &RtcpPacket) -> f32 {
    let length = match packet {
        RtcpPacket::Goodbye(_) => 2.0,
        RtcpPacket::SourceDescription(sd) => {
            sd.chunks
                .iter()
                .map(|chunk| chunk.items.len() + 1)
                .max()
                // AFFAIR, 0 chunk SDES packet is possible, although useless
                .unwrap_or(1) as f32
        }
        RtcpPacket::ReceiverReport(rr) => match rr.reports.len() {
            0 => 2.7,
            _ => 9.0,
        },
        RtcpPacket::SenderReport(sr) => match sr.reports.len() {
            0 => 4.7,
            _ => 11.0,
        },
        RtcpPacket::ExtendedReport(xr) => match xr.reports.len() {
            0 => 2.0,
            1 => 5.5,
            _ => 8.0,
        },
        RtcpPacket::PayloadSpecificFeedback(pf) => match pf {
            PayloadFeedback::PictureLossIndication(_) => 2.0,
            PayloadFeedback::FullIntraRequest(fir) => {
                if fir.fir.is_empty() {
                    3.0
                } else {
                    5.0
                }
            }
            PayloadFeedback::ReceiverEstimatedMaximumBitrate(remb) => match remb.ssrcs.len() {
                0 => 2.0,
                _ => 3.0,
            },
            PayloadFeedback::SliceLossIndication(sli) => {
                if sli.sli_entries.is_empty() {
                    3.0
                } else {
                    6.0
                }
            }
        },
        _ => 1.0,
    };

    length * 20.0
}

fn build_packet(ui: &mut Ui, packet: &RtcpPacket) {
    match packet {
        RtcpPacket::SenderReport(report) => build_sender_report(ui, report),
        RtcpPacket::ReceiverReport(report) => build_receiver_report(ui, report),
        RtcpPacket::SourceDescription(desc) => build_source_description(ui, desc),
        RtcpPacket::Goodbye(bye) => build_goodbye(ui, bye),
        RtcpPacket::PayloadSpecificFeedback(pf) => match pf {
            PayloadFeedback::PictureLossIndication(pli) => build_picture_loss_indication(ui, pli),
            PayloadFeedback::ReceiverEstimatedMaximumBitrate(remb) => {
                build_receiver_estimated_maximum_bitrate(ui, remb)
            }
            PayloadFeedback::SliceLossIndication(sli) => build_slice_loss_indication(ui, sli),
            PayloadFeedback::FullIntraRequest(fir) => build_full_intra_request(ui, fir),
        },
        RtcpPacket::ExtendedReport(xr) => build_extended_report(ui, xr),
        RtcpPacket::TransportSpecificFeedback(tf) => {
            ui.label(tf.get_type_name());
        }
        RtcpPacket::Other(packet_type) => {
            ui.label(format!("Packet type: {:?}", packet_type));
        }
        _ => {
            ui.label("Unsupported packet type");
        }
    };
}

fn build_sender_report(ui: &mut Ui, report: &SenderReport) {
    build_label(ui, "Source:", format!("{:x}", report.ssrc));
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            let datetime = ntp_to_string(report.ntp_time);
            build_label(ui, "NTP time:", datetime);
            build_label(ui, "RTP time:", report.rtp_time.to_string());
        });
        ui.vertical(|ui| {
            build_label(ui, "Packet count:", report.packet_count.to_string());
            build_label(ui, "Octet count:", report.octet_count.to_string());
        });
    });
    ui.separator();
    build_reception_reports(ui, &report.reports);
}

fn build_receiver_report(ui: &mut Ui, report: &ReceiverReport) {
    build_label(ui, "Source:", format!("{:x}", report.ssrc));
    ui.separator();
    build_reception_reports(ui, &report.reports);
}

fn build_reception_reports(ui: &mut Ui, reports: &Vec<ReceptionReport>) {
    if reports.is_empty() {
        let text = RichText::new("No reception reports").strong();
        ui.label(text);
        return;
    }

    let mut first = true;
    ui.horizontal(|ui| {
        for report in reports {
            if !first {
                ui.separator();
            } else {
                first = false;
            }
            let fraction_lost = (report.fraction_lost as f64 / u8::MAX as f64) * 100.0;
            let delay = report.delay as f64 / u16::MAX as f64 * 1000.0;
            ui.vertical(|ui| {
                build_label(ui, "SSRC:", format!("{:x}", report.ssrc));
                build_label(ui, "Fraction lost:", format!("{}%", fraction_lost));
                build_label(ui, "Cumulative lost:", report.total_lost.to_string());
                build_label(
                    ui,
                    "Extended highest sequence number:",
                    report.last_sequence_number.to_string(),
                );
                build_label(
                    ui,
                    "Interarrival jitter:",
                    format!("{} RTP timestamp units", report.jitter),
                );
                build_label(
                    ui,
                    "Last SR timestamp:",
                    report.last_sender_report.to_string(),
                );
                build_label(ui, "Delay since last SR:", format!("{:.4} ms", delay));
            });
        }
    });
}

fn build_source_description(ui: &mut Ui, desc: &SourceDescription) {
    let mut first = true;
    ui.horizontal(|ui| {
        for chunk in &desc.chunks {
            if !first {
                ui.separator();
            } else {
                first = false;
            }
            ui.vertical(|ui| {
                build_label(ui, "Source:", format!("{:x}", chunk.source));
                for item in &chunk.items {
                    build_label(ui, item.sdes_type.to_string(), item.text.clone());
                }
            });
        }
    });
}

fn build_goodbye(ui: &mut Ui, bye: &Goodbye) {
    let ssrcs = bye
        .sources
        .iter()
        .map(|ssrc| format!("{:x}", ssrc))
        .collect::<Vec<_>>()
        .join(", ");

    build_label(ui, "Sources:", ssrcs);
    build_label(ui, "Reason:", bye.reason.clone());
}

fn build_picture_loss_indication(ui: &mut Ui, pli: &PictureLossIndication) {
    build_label(ui, "Sender SSRC:", format!("{:x}", pli.sender_ssrc));
    build_label(ui, "Media SSRC:", format!("{:x}", pli.media_ssrc));
}

fn build_receiver_estimated_maximum_bitrate(ui: &mut Ui, remb: &ReceiverEstimatedMaximumBitrate) {
    build_label(ui, "Sender SSRC:", format!("{:x}", remb.sender_ssrc));
    let kbps = remb.bitrate / 1000.0;
    build_label(ui, "Bitrate:", format!("{:.2} kbps", kbps));
    let ssrcs = remb
        .ssrcs
        .iter()
        .map(|s| format!("{:x}", s))
        .collect::<Vec<_>>()
        .join(", ");
    if !ssrcs.is_empty() {
        build_label(ui, "SSRCs:", ssrcs);
    }
}

fn build_slice_loss_indication(ui: &mut Ui, sli: &SliceLossIndication) {
    build_label(ui, "Sender SSRC:", format!("{:x}", sli.sender_ssrc));
    build_label(ui, "Media SSRC:", format!("{:x}", sli.media_ssrc));

    if sli.sli_entries.is_empty() {
        ui.label(RichText::new("No SLI entries").strong());
    } else {
        ui.separator();
        let mut first = true;
        ui.horizontal(|ui| {
            for e in &sli.sli_entries {
                if !first {
                    ui.separator();
                } else {
                    first = false;
                }
                ui.vertical(|ui| {
                    build_label(ui, "First macroblock:", e.first.to_string());
                    build_label(ui, "Number of macroblocks:", e.number.to_string());
                    build_label(ui, "Picture ID:", e.picture.to_string());
                });
            }
        });
    }
}

fn build_full_intra_request(ui: &mut Ui, fir: &FullIntraRequest) {
    build_label(ui, "Sender SSRC:", format!("{:x}", fir.sender_ssrc));
    build_label(ui, "Media SSRC:", format!("{:x}", fir.media_ssrc));

    if fir.fir.is_empty() {
        ui.label(RichText::new("No FIR entries").strong());
    } else {
        ui.separator();
        let mut first = true;
        ui.horizontal(|ui| {
            for entry in &fir.fir {
                if !first {
                    ui.separator();
                } else {
                    first = false;
                }
                ui.vertical(|ui| {
                    build_label(ui, "SSRC:", format!("{:x}", entry.ssrc));
                    build_label(ui, "Sequence number:", entry.sequence_number.to_string());
                });
            }
        });
    }
}

fn build_extended_report(ui: &mut Ui, xr: &ExtendedReport) {
    build_label(ui, "Sender SSRC:", format!("{:x}", xr.sender_ssrc));

    if xr.reports.is_empty() {
        ui.separator();
        ui.label(RichText::new("No report blocks").strong());
        return;
    }

    ui.separator();

    let mut first_block = true;
    for report in &xr.reports {
        if !first_block {
            ui.separator();
        } else {
            first_block = false;
        }

        build_label(ui, "Block type:", report.get_type_name().to_string());

        match report {
            BlockType::ReceiverReferenceTime(rrt) => {
                let datetime = ntp_to_string(rrt.ntp_timestamp);
                build_label(ui, "NTP timestamp:", datetime);
            }
            BlockType::DLRR(dlrr) => {
                if dlrr.reports.is_empty() {
                    ui.label(RichText::new("No DLRR reports").strong());
                } else {
                    ui.horizontal(|ui| {
                        let mut first = true;
                        for dlrr_report in &dlrr.reports {
                            if !first {
                                ui.separator();
                            } else {
                                first = false;
                            }
                            ui.vertical(|ui| {
                                build_label(ui, "SSRC:", format!("{:x}", dlrr_report.ssrc));
                                build_label(ui, "Last RR:", dlrr_report.last_rr.to_string());
                                build_label(ui, "DLRR:", dlrr_report.dlrr.to_string());
                            });
                        }
                    });
                }
            }
            _ => {}
        }
    }
}
fn build_label(ui: &mut Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let source_label = RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(source_label);
        ui.label(normal.into());
    });
}
