use super::filters::parse_filter;
use crate::{
    app::{
        common::*, tables::stun_packets_table::StunFilterContext, FilterHelpContent, FilterInput,
        TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column,
    filter_system::FilterExpression,
    impl_table_base,
    streams::RefStreams,
};
use eframe::epaint::Color32;
use egui::RichText;
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::packet::SessionPacket;
use std::fmt::Write as _;
use std::{any::Any, collections::HashMap};

declare_table_struct!(StunPacketsTable);

impl_table_base!(
    StunPacketsTable,
    FilterHelpContent::builder("STUN Packet Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("type", "Filter by STUN message type")
        .filter("transaction", "Filter by transaction ID")
        .filter("length", "Filter by message length")
        .example("source:10.0.0 AND type:binding")
        .example("(dest:192.168 OR dest:10.0.0) AND NOT type:allocate")
        .build(),
    "stun_packets", "STUN Packets"
    ;
    build_header: |self, header| {
        let headers = [
            ("No.", "Packet number"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Type", "STUN message type (e.g., binding, allocate)"),
            ("Transaction ID", "STUN transaction identifier"),
            ("Length", "STUN message length"),
            ("Attributes", "STUN message attributes"),
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
        let streams = self.streams.borrow();

        let filtered_packets: Vec<_> = streams
            .packets
            .values()
            .filter(|packet| matches!(packet.contents, SessionPacket::Stun(_)))
            .filter(|packet| {
                if let SessionPacket::Stun(ref stun_packet) = packet.contents {
                    let ctx = StunFilterContext {
                        packet: stun_packet,
                        source_addr: &packet.source_addr.to_string(),
                        destination_addr: &packet.destination_addr.to_string(),
                    };

                    self.packet_matches_filter(&ctx)
                } else {
                    false
                }
            })
            .collect();

        let first_ts = filtered_packets.first().map(|p| p.timestamp).unwrap_or_default();
        let rows_heights: Vec<f32> = filtered_packets
            .iter()
            .map(|packet| match &packet.contents {
            SessionPacket::Stun(sp) => 20.0 + (sp.attributes.len().max(1) as f32 * 14.0),
            _ => 30.0,
            })
            .collect();

        body.heterogeneous_rows(rows_heights.into_iter(), |mut row| {
            let packet = &filtered_packets[row.index()];
            let SessionPacket::Stun(ref stun_packet) = packet.contents else {
            return;
            };

            let src = packet.source_addr.to_string();
            let dst = packet.destination_addr.to_string();

            // ID
            row.col(|ui| {
            ui.label(RichText::new(packet.id.to_string()).monospace());
            });

            // Time (relative to the first visible packet)
            row.col(|ui| {
            let timestamp = packet.timestamp - first_ts;
            let txt = format!("{:.4} s", timestamp.as_secs_f64());
            ui.label(RichText::new(txt).monospace())
                .on_hover_text("Time since first visible STUN packet");
            });

            // Source
            row.col(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                RichText::new(&src)
                    .monospace()
                    .color(Color32::from_rgb(80, 170, 255)),
                );
                if ui.small_button("Copy").on_hover_text("Copy source address").clicked() {
                ui.output_mut(|o| o.copied_text = src.clone());
                }
            });
            });

            // Destination
            row.col(|ui| {
            ui.horizontal(|ui| {
                ui.label(
                RichText::new(&dst)
                    .monospace()
                    .color(Color32::from_rgb(255, 140, 100)),
                );
                if ui.small_button("Copy").on_hover_text("Copy destination address").clicked() {
                ui.output_mut(|o| o.copied_text = dst.clone());
                }
            });
            });

            // Type (color-coded)
            row.col(|ui| {
            let t = stun_packet.message_type.as_string();
            let lower = t.to_lowercase();
            let color = if lower.contains("bind") {
                Color32::from_rgb(120, 200, 120)
            } else if lower.contains("alloc") {
                Color32::from_rgb(200, 160, 120)
            } else if lower.contains("refresh") {
                Color32::from_rgb(120, 180, 200)
            } else if lower.contains("channel") {
                Color32::from_rgb(200, 120, 180)
            } else {
                Color32::LIGHT_GRAY
            };
            ui.label(RichText::new(t).strong().color(color));
            });

            // Transaction ID (grouped hex + copy)
            row.col(|ui| {
            let mut tx = String::with_capacity(stun_packet.transaction_id.len() * 3);
            for (i, b) in stun_packet.transaction_id.iter().enumerate() {
                let _ = write!(&mut tx, "{:02x}", b);
                if i % 2 == 1 && i + 1 != stun_packet.transaction_id.len() {
                tx.push(' ');
                }
            }
            ui.horizontal(|ui| {
                ui.label(RichText::new(&tx).monospace());
                if ui.small_button("Copy").on_hover_text("Copy transaction ID").clicked() {
                ui.output_mut(|o| o.copied_text = tx.clone());
                }
            });
            });

            // Length
            row.col(|ui| {
            ui.label(RichText::new(stun_packet.message_length.to_string()).monospace());
            });

            // Attributes (each on its own line, monospace, bullet-prefixed)
            row.col(|ui| {
            ui.vertical(|ui| {
                if stun_packet.attributes.is_empty() {
                ui.label(
                    RichText::new("(no attributes)")
                    .italics()
                    .color(Color32::GRAY),
                );
                } else {
                for attr in stun_packet.attributes.iter() {
                    let line = attr.as_string_with_txid(&stun_packet.transaction_id);
                    ui.label(RichText::new(format!("• {}", line)).monospace());
                }
                }
            });
        });
    });
});

declare_table!(StunPacketsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(None, 40.0, Some(50.0), false, true),
        column(None, 80.0, Some(80.0), false, true),
        column(None, 130.0, None, false, true),
        column(None, 130.0, None, false, true),
        column(None, 120.0, None, false, true),
        column(None, 200.0, None, false, true),
        column(None, 80.0, None, false, true),
        column(None, 200.0, None, false, true),
    )
});

impl StunPacketsTable {
    fn packet_matches_filter(&self, ctx: &StunFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim();
        parse_filter(filter)
            .map(|filter_type| filter_type.matches(ctx))
            .unwrap_or(true)
    }
}
