use super::filters::parse_filter;
use crate::{
    app::{
        common::*, tables::turn_channel_table::TurnChannelFilterContext, FilterHelpContent,
        FilterInput, TABLE_HEADER_TEXT_SIZE,
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

declare_table_struct!(TurnChannelTable);

impl_table_base!(
    TurnChannelTable,
    FilterHelpContent::builder("TURN Channel Data Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("channel", "Filter by TURN channel number")
        .filter("length", "Filter by data length")
        .example("source:10.0.0 AND channel:0x4001")
        .example("(dest:192.168 OR dest:10.0.0) AND NOT channel:0x4002")
        .build(),
    "turn_channel_data", "TURN Channel Data"
    ;
    build_header: |self, header| {
        let headers = [
            ("No.", "Packet number"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Channel", "TURN channel number"),
            ("Length", "Data length"),
            ("Data (hex)", "Hexadecimal representation of the data"),
        ];

        for (label, desc) in headers {
            header.col(|ui| {
                ui.label(RichText::new(label.to_string()).size(TABLE_HEADER_TEXT_SIZE).strong())
                    .on_hover_text(desc.to_string());
            });
        };

    }
    ;
    build_table_body: |self, body| {
        let streams = self.streams.borrow();

        let filtered_packets: Vec<_> = streams
            .packets
            .values()
            .filter(|packet| matches!(packet.contents, SessionPacket::Turn(_)))
            .collect();

        let first_ts = filtered_packets.first().map(|p| p.timestamp).unwrap_or_default();
        body.rows(self.config.row_height, filtered_packets.len(), |mut row| {
            let packet = filtered_packets[row.index()];

            let SessionPacket::Turn(ref turn_data) = packet.contents else {
                return;
            };

            let src = packet.source_addr.to_string();
            let dst = packet.destination_addr.to_string();
            // ID
            row.col(|ui| {
                ui.label(RichText::new(packet.id.to_string()).monospace());
            });

            // Time
            row.col(|ui| {
                let ts = packet.timestamp - first_ts;
                ui.label(RichText::new(format!("{:.6}", ts.as_secs_f64())).monospace());
            });

            // Source
            row.col(|ui| {
                ui.label(
                    RichText::new(&src)
                        .monospace()
                        .color(Color32::from_rgb(80, 170, 255)),
                );
            });

            // Destination
            row.col(|ui| {
                ui.label(
                    RichText::new(&dst)
                        .monospace()
                        .color(Color32::from_rgb(255, 140, 100)),
                );
            });

            // Channel
            row.col(|ui| {
                ui.label(RichText::new(format!("0x{:04X}", turn_data.number)).monospace());
            });

            // Length
            row.col(|ui| {
                ui.label(RichText::new(turn_data.data.len().to_string()).monospace());
            });

            // Data (hex)
            row.col(|ui| {
                let mut hex_data = String::new();
                for (i, byte) in turn_data.data.iter().enumerate() {
                    if i > 0 {
                        if i % 16 == 0 {
                            hex_data.push('\n');
                        } else if i % 8 == 0 {
                            hex_data.push_str("  ");
                        } else {
                            hex_data.push(' ');
                        }
                    }
                    let _ = write!(hex_data, "{:02X}", *byte);
                }
                ui.label(RichText::new(hex_data).monospace());
            });
        });
    }
);

declare_table!(TurnChannelTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(None, 40.0, Some(50.0), false, true),
        column(None, 80.0, Some(80.0), false, true),
        column(None, 130.0, None, false, true),
        column(None, 130.0, None, false, true),
        column(None, 80.0, None, false, true),
        column(None, 80.0, None, false, true),
        column(None, 80.0, None, false, true),
    )
});

impl TurnChannelTable {
    fn packet_matches_filter(&self, ctx: &TurnChannelFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim();
        parse_filter(filter)
            .map(|filter_type| filter_type.matches(ctx))
            .unwrap_or(true)
    }
}
