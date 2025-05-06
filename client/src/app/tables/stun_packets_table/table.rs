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
use std::{any::Any, collections::HashMap};

declare_table_struct!(StunPacketsTable);

impl_table_base!(
    StunPacketsTable,
    FilterHelpContent::builder("STUN Packet Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("type", "Filter by STUN message type")
        .filter("transaction", "Filter by transaction ID")
        .filter("magic", "Filter by magic cookie")
        .filter("length", "Filter by message length")
        .example("source:10.0.0 AND type:binding")
        .example("(dest:192.168 OR dest:10.0.0) AND NOT type:allocate")
        .example("magic:2112A442 AND length:>100")
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
            ("Magic Cookie", "STUN magic cookie value"),
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

        body.rows(self.config.row_height, filtered_packets.len(), |mut row| {
            let packet = &filtered_packets[row.index()];

            let SessionPacket::Stun(ref stun_packet) = packet.contents else {
                return;
            };

            let ctx = StunFilterContext {
                packet: stun_packet,
                source_addr: &packet.source_addr.to_string(),
                destination_addr: &packet.destination_addr.to_string(),
            };

            if !self.packet_matches_filter(&ctx) {
                return;
            }

            // ID column
            row.col(|ui| {
                ui.label(packet.id.to_string());
            });

            // Time column
            row.col(|ui| {
                let timestamp = packet.timestamp - first_ts;
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });

            // Source/Destination columns
            row.col(|ui| {
                ui.label(packet.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(packet.destination_addr.to_string());
            });

            // STUN-specific columns
            row.col(|ui| {
                ui.label(stun_packet.get_message_type_name());
            });

            // Transaction ID column
            row.col(|ui| {
                let tx_id = stun_packet.transaction_id
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                ui.label(tx_id);
            });

            // Magic Cookie column
            row.col(|ui| {
                ui.label(format!("{:08x}", stun_packet.magic_cookie));
            });

            // Length column
            row.col(|ui| {
                ui.label(stun_packet.message_length.to_string());
            });

            // Attributes column
            row.col(|ui| {
                let mut attributes = String::new();
                for attr in &stun_packet.attributes {
                    attributes.push_str(&format!(
                        "{} ({} bytes)\n",
                        stun_packet.get_attribute_type_name(attr.attribute_type),
                        attr.length
                    ));
                }
                ui.label(attributes);
            });

        });
    }
);

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
        column(None, 100.0, None, false, true),
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