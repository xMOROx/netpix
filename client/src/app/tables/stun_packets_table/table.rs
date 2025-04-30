use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{
        common::*, tables::stun_packets_table::*, FilterHelpContent, FilterInput,
        TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::{packet::SessionPacket, StunPacket};
use std::any::Any;

declare_table_struct!(StunPacketsTable);

impl_table_base!(
    StunPacketsTable,
    FilterHelpContent::builder("STUN Packet Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("type", "Filter by STUN message type")
        .example("source:192.168 AND type:binding")
        .example("dest:10.0.0 OR type:error")
        .build(),
    "stun_packets", "STUN Packets"
    ;
    build_header: |self, header| {
        let headers = [
            ("No.", "Packet number"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Type", "Type of the STUN message"),
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
    build_table_body: |self, body|
        let streams = &self.streams.borrow();
        let mut stun_packets = Vec::new();

        // Collect STUN packets with metadata
        for packet in streams.packets.values() {
            let stun = match &packet.contents {
                SessionPacket::Stun(ref stun) => stun,
                _ => continue,
            };

            let ctx = StunFilterContext {
                packet: stun,
                source_addr: &packet.source_addr.to_string(),
                destination_addr: &packet.destination_addr.to_string(),
            };

            if !self.packet_matches_filter(&ctx) {
                continue;
            }

            stun_packets.push(PacketInfo {
                id: packet.id as u64,
                packet,
                stun_packet: stun,
            });
        }

        if stun_packets.is_empty() {
            return;
        }

        let first_ts = streams.packets.first().unwrap().timestamp;

        body.heterogeneous_rows(
            stun_packets.iter().map(|info| {
                let time = info.packet.timestamp.saturating_sub(first_ts);
                let hours = time.as_secs() / 3600;
                let minutes = (time.as_secs() % 3600) / 60;
                let seconds = time.as_secs() % 60;
                let millis = time.subsec_millis();

                TableRow::new()
                    .col(|ui| {
                        ui.label(format!("{}", info.id));
                    })
                    .col(|ui| {
                        ui.label(format!(
                            "{:02}:{:02}:{:02}.{:03}",
                            hours, minutes, seconds, millis
                        ));
                    })
                    .col(|ui| {
                        ui.label(info.packet.source_addr.to_string());
                    })
                    .col(|ui| {
                        ui.label(info.packet.destination_addr.to_string());
                    })
                    .col(|ui| {
                        ui.label(info.stun_packet.get_message_type_name());
                    })
                    .col(|ui| {
                        let mut attributes = String::new();
                        for attr in &info.stun_packet.attributes {
                            attributes.push_str(&format!(
                                "{} ({} bytes)\n",
                                info.stun_packet.get_attribute_type_name(attr.attribute_type),
                                attr.length
                            ));
                        }
                        ui.label(attributes);
                    })
            }),
        );
    ;
    build_filters: |self, ui| {
        let mut filter_input = self.filter_input.borrow_mut();
        let mut filter_help = self.filter_help.borrow_mut();

        ui.horizontal(|ui| {
            ui.label("Filter:");
            if ui
                .add(FilterInput::new(&mut filter_input, &mut filter_help))
                .changed()
            {
                self.update_filter();
            }
        });
    }
); 