use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{
        FilterHelpContent, FilterInput, TABLE_HEADER_TEXT_SIZE, common::*,
        tables::rtcp_packets_table::*,
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::ntp_to_string,
};
use egui::ecolor::Hsva;
use egui::{Color32, RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::packet::PacketDirection;
use netpix_common::rtcp::extended_reports::BlockType;
use netpix_common::rtcp::payload_feedbacks::PayloadFeedback;
use netpix_common::rtcp::{ExtendedReport, TransportFeedback};
use netpix_common::{
    packet::SessionPacket,
    rtcp::{
        Goodbye, ReceiverReport, ReceptionReport, RtcpPacket, SenderReport, SourceDescription,
        payload_feedbacks::{
            FullIntraRequest, PictureLossIndication, ReceiverEstimatedMaximumBitrate,
            SliceLossIndication,
        },
    },
};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RtcpPacketsTable {
    streams: RefStreams,
    filter_input: FilterInput,
    config: TableConfig,
    ws_sender: WsSender,
    pub alias_helper: StreamAliasHelper,
}

impl_table_base!(
    RtcpPacketsTable;
    alias_helper: StreamAliasHelper;
    FilterHelpContent::builder("RTCP Packet Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("type", "Filter by RTCP packet type")
        .filter("ssrc", "Filter by SSRC of packet")
        .filter("dir", "Filter by direction")
        .filter("alias", "Filter by alias")
        .example("source:192.168 AND type:sender")
        .example("dest:10.0.0 OR type:receiver")
        .build(),
    "rtcp_packets", "RTCP Packets"
    ;
    ui: |self, ctx| {
        if self.filter_input.show(ctx) {
            self.check_filter();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }
    ;
    build_header: |self, header| {
        let headers = [
            ("No.", "Packet number (including skipped packets) + compound RTCP packet number inside the parentheses"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Type", "Type of the RTCP packet"),
            ("Alias", "Alias for SSRC"),
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

        for packet in streams.packets.values() {
            let rtcp = match &packet.contents {
                SessionPacket::Rtcp(rtcp) => rtcp,
                _ => continue,
            };

            for (idx, rtcp_packet) in rtcp.iter().enumerate() {
                let alias = self.alias_helper.get_alias(rtcp_packet.get_ssrc().unwrap_or(0));
                let ctx = RtcpFilterContext {
                    packet: rtcp_packet,
                    source_addr: &packet.source_addr.to_string(),
                    destination_addr: &packet.destination_addr.to_string(),
                    direction: &packet.metadata.direction.to_string(),
                    ssrc: &rtcp_packet.get_ssrc_merged(),
                    alias: &alias,
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

            let meta = &info.packet.metadata;
            let is_synthetic = meta.is_synthetic_addr;

            let ssrc = info.rtcp_packet.get_ssrc().unwrap_or(0);
            let row_color = self.alias_helper.get_color(ssrc);
            let alias = self.alias_helper.get_alias(ssrc);

            row.col(|ui| {
                ui.label(format!("{} ({})", info.id, info.compound_index));
            });
            row.col(|ui| {
                let timestamp = info.packet.timestamp - first_ts;
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });
           row.col(|ui| {
                if is_synthetic {
                    match meta.direction {
                        PacketDirection::Incoming => ui.strong("Remote (Incoming)"),
                        PacketDirection::Outgoing => ui.label("Local"),
                        _ => ui.label("?"),
                    };
                } else {
                    ui.label(info.packet.source_addr.to_string());
                }
            });
            row.col(|ui| {
                if is_synthetic {
                    match meta.direction {
                        PacketDirection::Incoming => ui.label("Local"),
                        PacketDirection::Outgoing => ui.strong("Remote (Outgoing)"),
                        _ => ui.label("?"),
                    };
                } else {
                    ui.label(info.packet.destination_addr.to_string());
                }
            });
            row.col(|ui| {
                ui.label(info.rtcp_packet.get_type_name().to_string());
            });
            row.col(|ui| {
                ui.centered_and_justified(|ui|{
                    ui.colored_label(row_color,alias);
                });
            });
            row.col(|ui| {
                build_packet(ui, info.rtcp_packet,&mut self.alias_helper);
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
        column(Some(50.0), 50.0, None, false, true),
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
        RtcpPacket::TransportSpecificFeedback(_) => 3.0,
        _ => 1.0,
    };

    length * 20.0
}

#[derive(Default)]
pub struct StreamAliasHelper {
    cache: RefCell<HashMap<u32, String>>,
}

impl StreamAliasHelper {
    pub fn get_alias(&self, ssrc: u32) -> String {
        let mut cache = self.cache.borrow_mut();

        if let Some(alias) = cache.get(&ssrc) {
            return alias.clone();
        }

        let index = cache.len() as u32;
        let alias = self.index_to_letter(index);

        cache.insert(ssrc, alias.clone());
        alias
    }

    fn index_to_letter(&self, mut index: u32) -> String {
        let mut result = Vec::with_capacity(4);
        loop {
            let remainder = index % 26;
            result.push((b'A' + remainder as u8) as char);
            if index < 26 {
                break;
            }
            index = (index / 26) - 1;
        }
        result.into_iter().rev().collect()
    }

    pub fn get_color(&self, ssrc: u32) -> Color32 {
        let mut cache = self.cache.borrow_mut();

        let index = if let Some(_) = cache.get(&ssrc) {
            0
        } else {
            let index = cache.len() as u32;
            let alias = self.index_to_letter(index);
            cache.insert(ssrc, alias);
            0
        };

        let hash = (ssrc as u64).wrapping_mul(11400714819323198485);
        let hue = (hash as f32) / (u64::MAX as f32); // 0.0 - 1.0

        // High saturation and value for visibility against dark backgrounds
        // Maybe different logic for dark mode?
        Color32::from(Hsva::new(hue, 0.7, 0.9, 1.0))
    }

    pub fn print_ssrc(&self, ssrc: u32) -> String {
        format!("{:x} | alias: {}", ssrc, self.get_alias(ssrc))
    }
}
pub fn build_packet(ui: &mut Ui, packet: &RtcpPacket, alias_helper: &mut StreamAliasHelper) {
    match packet {
        RtcpPacket::SenderReport(report) => build_sender_report(ui, report, alias_helper),
        RtcpPacket::ReceiverReport(report) => build_receiver_report(ui, report, alias_helper),
        RtcpPacket::SourceDescription(desc) => build_source_description(ui, desc, alias_helper),
        RtcpPacket::Goodbye(bye) => build_goodbye(ui, bye, alias_helper),
        RtcpPacket::PayloadSpecificFeedback(pf) => match pf {
            PayloadFeedback::PictureLossIndication(pli) => {
                build_picture_loss_indication(ui, pli, alias_helper)
            }
            PayloadFeedback::ReceiverEstimatedMaximumBitrate(remb) => {
                build_receiver_estimated_maximum_bitrate(ui, remb, alias_helper)
            }
            PayloadFeedback::SliceLossIndication(sli) => {
                build_slice_loss_indication(ui, sli, alias_helper)
            }
            PayloadFeedback::FullIntraRequest(fir) => {
                build_full_intra_request(ui, fir, alias_helper)
            }
        },
        RtcpPacket::ExtendedReport(xr) => build_extended_report(ui, xr, alias_helper),
        RtcpPacket::TransportSpecificFeedback(tf) => {
            build_transport_feedback(ui, tf, alias_helper);
        }
        RtcpPacket::Other(packet_type) => {
            ui.label(format!("Packet type: {:?}", packet_type));
        }
        _ => {
            ui.label("Unsupported packet type");
        }
    };
}

fn build_sender_report(ui: &mut Ui, report: &SenderReport, alias_helper: &mut StreamAliasHelper) {
    build_ssrc_row(ui, "Source:", report.ssrc, alias_helper);
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
    build_reception_reports(ui, &report.reports, alias_helper);
}

fn build_receiver_report(
    ui: &mut Ui,
    report: &ReceiverReport,
    alias_helper: &mut StreamAliasHelper,
) {
    build_ssrc_row(ui, "Source:", report.ssrc, alias_helper);
    ui.separator();
    build_reception_reports(ui, &report.reports, alias_helper);
}

fn build_reception_reports(
    ui: &mut Ui,
    reports: &Vec<ReceptionReport>,
    alias_helper: &mut StreamAliasHelper,
) {
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
                build_ssrc_row(ui, "SSRC:", report.ssrc, alias_helper);
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

fn build_source_description(
    ui: &mut Ui,
    desc: &SourceDescription,
    alias_helper: &mut StreamAliasHelper,
) {
    let mut first = true;
    ui.horizontal(|ui| {
        for chunk in &desc.chunks {
            if !first {
                ui.separator();
            } else {
                first = false;
            }
            ui.vertical(|ui| {
                build_ssrc_row(ui, "Source:", chunk.source, alias_helper);
                for item in &chunk.items {
                    build_label(ui, item.sdes_type.to_string(), item.text.clone());
                }
            });
        }
    });
}

fn build_goodbye(ui: &mut Ui, bye: &Goodbye, alias_helper: &mut StreamAliasHelper) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Sources:").strong());
        for (i, ssrc) in bye.sources.iter().enumerate() {
            if i > 0 {
                ui.label(", ");
            }
            ui.label(format!("{:x}", ssrc))
                .on_hover_text(format!("Alias: {}", alias_helper.get_alias(*ssrc)));
        }
    });

    build_label(ui, "Reason:", bye.reason.clone());
}

fn build_picture_loss_indication(
    ui: &mut Ui,
    pli: &PictureLossIndication,
    alias_helper: &mut StreamAliasHelper,
) {
    build_ssrc_row(ui, "Sender SSRC:", pli.sender_ssrc, alias_helper);
    build_ssrc_row(ui, "Media SSRC:", pli.media_ssrc, alias_helper);
}

fn build_receiver_estimated_maximum_bitrate(
    ui: &mut Ui,
    remb: &ReceiverEstimatedMaximumBitrate,
    alias_helper: &mut StreamAliasHelper,
) {
    build_ssrc_row(ui, "Sender SSRC:", remb.sender_ssrc, alias_helper);
    let kbps = remb.bitrate / 1000.0;
    build_label(ui, "Bitrate:", format!("{:.2} kbps", kbps));

    if !remb.ssrcs.is_empty() {
        ui.horizontal(|ui| {
            ui.label(RichText::new("SSRCs:").strong());
            for (i, ssrc) in remb.ssrcs.iter().enumerate() {
                if i > 0 {
                    ui.label(", ");
                }
                ui.label(format!("{:x}", ssrc))
                    .on_hover_text(format!("Alias: {}", alias_helper.get_alias(*ssrc)));
            }
        });
    }
}

fn build_slice_loss_indication(
    ui: &mut Ui,
    sli: &SliceLossIndication,
    alias_helper: &mut StreamAliasHelper,
) {
    build_ssrc_row(ui, "Sender SSRC:", sli.sender_ssrc, alias_helper);
    build_ssrc_row(ui, "Media SSRC:", sli.media_ssrc, alias_helper);

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

fn build_full_intra_request(
    ui: &mut Ui,
    fir: &FullIntraRequest,
    alias_helper: &mut StreamAliasHelper,
) {
    build_ssrc_row(ui, "Sender SSRC:", fir.sender_ssrc, alias_helper);
    build_ssrc_row(ui, "Media SSRC:", fir.media_ssrc, alias_helper);

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
                    build_ssrc_row(ui, "SSRC:", entry.ssrc, alias_helper);
                    build_label(ui, "Sequence number:", entry.sequence_number.to_string());
                });
            }
        });
    }
}

fn build_extended_report(ui: &mut Ui, xr: &ExtendedReport, alias_helper: &mut StreamAliasHelper) {
    build_ssrc_row(ui, "Sender SSRC:", xr.sender_ssrc, alias_helper);

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
                                build_ssrc_row(ui, "SSRC:", dlrr_report.ssrc, alias_helper);
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

fn build_transport_feedback(
    ui: &mut Ui,
    tf: &TransportFeedback,
    alias_helper: &mut StreamAliasHelper,
) {
    ui.vertical(|ui| {
        build_label(ui, "Type:", tf.get_type_name());
        build_ssrc_row(ui, "Sender SSRC:", tf.sender_ssrc, alias_helper);
        build_ssrc_row(ui, "Media SSRC:", tf.media_ssrc, alias_helper);
    });
}

fn build_ssrc_row(ui: &mut Ui, label: &str, ssrc: u32, helper: &StreamAliasHelper) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).strong());
        ui.label(format!("{:x}", ssrc))
            .on_hover_text(format!("Alias: {}", helper.get_alias(ssrc)));
    });
}

fn build_label(ui: &mut Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let source_label = RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(source_label);
        ui.label(normal.into());
    });
}
