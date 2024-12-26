use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{common::*, tables::rtcp_packets_table::*, FilterHelpContent, FilterInput},
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::ntp_to_string,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::{packet::SessionPacket, rtcp::*, RtcpPacket};
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
                ui.heading(label).on_hover_text(desc);
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
        column(Some(170.0), 170.0, None, false, true),
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
        _ => {
            ui.label("Unsupported");
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

fn build_label(ui: &mut Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let source_label = RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(source_label);
        ui.label(normal.into());
    });
}
