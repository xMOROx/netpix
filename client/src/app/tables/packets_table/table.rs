use super::{
    filters::{parse_filter, FilterContext},
    types::PacketInfo,
};
use crate::{
    app::{
        common::{TableBase, TableConfig},
        FilterHelpContent, FilterInput, TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column,
    filter_system::FilterExpression,
    impl_table_base,
    streams::RefStreams,
};
use egui::RichText;
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::{WsMessage, WsSender};
use netpix_common::{
    packet::{Packet, SessionProtocol},
    Request,
};
use std::any::Any;

declare_table_struct!(PacketsTable);

impl_table_base!(
    PacketsTable,
    FilterHelpContent::builder("Network Packet Filters")
        .filter("source:<ip>", "Filter by source IP address")
        .filter("dest:<ip>", "Filter by destination IP address")
        .filter(
            "proto:<protocol> or protocol:<protocol>",
            "Filter by protocol (TCP, UDP, RTP, RTCP, MPEG-TS)",
        )
        .filter("type:<protocol>", "Filter by protocol type")
        .filter("length:<op><size>", "Filter by packet size")
        .example("source:192.168 AND proto:udp")
        .example("length:>100 AND type:rtp")
        .example("NOT dest:10.0.0.1")
        .example("(proto:tcp AND length:>500) OR source:192.168")
    .build(),
    "packets", "Network Packets"
    ;
    build_header: |self, header| {
        let headers = [
            "No.",
            "Time",
            "Source",
            "Destination",
            "Protocol",
            "Length",
            "Treated as",
        ];

        for header_text in headers {
            header.col(|ui| {
                ui.label(RichText::new(header_text.to_string()).size(TABLE_HEADER_TEXT_SIZE).strong());
            });
        }
    }
    ;
    build_table_body: |self, body| {
        let filter_valid = self.filter_input.get_error().is_none();
        let mut requests = Vec::new();
        let streams = self.streams.borrow();
        let packets = &streams.packets;

        let mut packets_info: Vec<_> = packets.values().collect();
        packets_info.sort_by_key(|p| p.timestamp);

        let first_ts = packets_info
            .first()
            .map(|p| p.timestamp)
            .unwrap_or_default();

        let filtered_packets: Vec<_> = if filter_valid {
            packets_info
                .into_iter()
                .filter(|packet| self.packet_matches_filter(packet))
                .collect()
        } else {
            Vec::new()
        };

        body.rows(self.config.row_height, filtered_packets.len(), |mut row| {
            let packet = &filtered_packets[row.index()];
            let timestamp = packet.timestamp - first_ts;

            // ID column
            row.col(|ui| {
                ui.label(packet.id.to_string());
            });

            // Time column
            row.col(|ui| {
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });

            // Source column
            row.col(|ui| {
                ui.label(packet.source_addr.to_string());
            });

            // Destination column
            row.col(|ui| {
                ui.label(packet.destination_addr.to_string());
            });

            // Protocol column
            row.col(|ui| {
                ui.label(packet.transport_protocol.to_string());
            });

            // Length column
            row.col(|ui| {
                ui.label(packet.length.to_string());
            });

            // Session protocol column with context menu
            let (_, resp) = row.col(|ui| {
                ui.label(packet.session_protocol.to_string());
            });

            resp.context_menu(|ui| {
                if let Some(req) = self.build_parse_menu(ui, packet) {
                    requests.push(req);
                }
            });
        });

        drop(streams);
        requests
            .iter()
            .for_each(|req| self.send_parse_request(req.clone()));
    }
);

declare_table!(PacketsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(None, 40.0, None, false, true),
        column(None, 130.0, None, false, true),
        column(None, 100.0, None, false, true),
        column(None, 100.0, None, false, true),
        column(None, 80.0, None, false, true),
        column(None, 80.0, None, false, true),
        column(None, 100.0, None, false, true),
    )
});

impl PacketsTable {
    fn packet_matches_filter(&self, packet: &Packet) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim();
        let ctx = FilterContext { packet };

        parse_filter(filter)
            .map(|filter_type| filter_type.matches(&ctx))
            .unwrap_or(true)
    }

    fn build_parse_menu(&self, ui: &mut egui::Ui, packet: &Packet) -> Option<Request> {
        let mut request = None;
        ui.label(format!("Parse {} as:", packet.id));

        SessionProtocol::all().iter().for_each(|protocol| {
            let is_current = packet.session_protocol == *protocol;
            if ui.radio(is_current, protocol.to_string()).clicked() {
                request = Some(Request::Reparse(packet.id, *protocol));
            }
        });

        ui.separator();
        ui.label("This will have effect on every client!");

        request
    }

    fn send_parse_request(&mut self, request: Request) {
        if let Ok(msg) = request.encode() {
            self.ws_sender.send(WsMessage::Binary(msg));
        } else {
            log::error!("Failed to encode request message");
        }
    }
}
