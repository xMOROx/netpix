use crate::define_column;
use crate::app::common::{TableBase, TableConfig};
use crate::app::utils::{FilterHelpContent, FilterInput};
use crate::declare_table;
use crate::filter_system::FilterExpression;
use crate::streams::RefStreams;
use egui_extras::{Column, TableRow};
use egui_extras::TableBuilder;

use super::filters::{parse_filter, FilterContext};
use super::types::PacketInfo;
use egui_extras::TableBody;
use ewebsock::{WsMessage, WsSender};
use netpix_common::packet::{Packet, SessionProtocol};
use netpix_common::Request;

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

pub struct PacketsTable {
    streams: RefStreams,
    filter_input: FilterInput,
    ws_sender: Option<WsSender>,
    config: TableConfig,
}

impl TableBase for PacketsTable {
    fn new(streams: RefStreams) -> Self {
        let help = FilterHelpContent::builder("Network Packet Filters")
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
            .build();

        Self {
            streams,
            filter_input: FilterInput::new(help),
            ws_sender: None,
            config: TableConfig::default(),
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        if self.filter_input.show(ctx) {
            self.check_filter();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }

    fn check_filter(&mut self) {
        let filter = self.filter_input.get_filter();
        if filter.is_empty() {
            self.filter_input.set_error(None);
            return;
        }

        let result = parse_filter(&filter.to_lowercase());
        self.filter_input.set_error(result.err());
    }

    fn build_header(&mut self, header: &mut TableRow) {
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
                ui.strong(header_text);
            });
        }
    }

    fn build_table_body(&mut self, body: TableBody) {
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
}

impl PacketsTable {
    pub fn new_with_sender(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            ws_sender: Some(ws_sender),
            ..Self::new(streams)
        }
    }

    fn packet_matches_filter(&self, packet: &Packet) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim();
        let ctx = FilterContext { packet };

        parse_filter(&filter)
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
            if let Some(ws_sender) = &mut self.ws_sender {
                ws_sender.send(WsMessage::Binary(msg));
            } else {
                log::error!("Websocket sender is not set");
            }
        } else {
            log::error!("Failed to encode request message");
        }
    }
}
