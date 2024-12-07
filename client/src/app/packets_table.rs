mod filters;

use crate::app::utils::{FilterHelpContent, FilterInput};
use crate::filter_system::{validate_filter_syntax, FilterExpression, ParseError};
use crate::streams::RefStreams;
use eframe::epaint::Color32;
use egui::widgets::TextEdit;
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::{WsMessage, WsSender};
use filters::{parse_filter, FilterContext};
use netpix_common::packet::{Packet, SessionProtocol};
use netpix_common::Request;

pub struct PacketsTable {
    streams: RefStreams,
    ws_sender: WsSender,
    filter_input: FilterInput,
}

impl PacketsTable {
    pub fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        let help = FilterHelpContent::builder("Network Packet Filters")
            .filter("source:<ip>", "Filter by source IP address")
            .filter("dest:<ip>", "Filter by destination IP address")
            .filter(
                "proto:<protocol> or protocol:<protocol> ",
                "Filter by protocol (TCP, UDP, RTP, RTCP, MPEG-TS)",
            )
            .filter(
                "type:<protocol>",
                "Filter by protocol type (RTP, RTCP, MPEG-TS)",
            )
            .filter(
                "length:<op><size>",
                "Filter by packet size (operators: <, <=, >, >=)",
            )
            .example("source:192.168 AND proto:udp")
            .example("length:>100 AND type:rtp")
            .example("NOT dest:10.0.0.1")
            .example("(proto:tcp AND length:>500) OR source:192.168")
            .build();

        Self {
            streams,
            ws_sender,
            filter_input: FilterInput::new(help),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
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

    fn packet_matches_filter(&self, packet: &Packet) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim();
        let ctx = FilterContext { packet };

        parse_filter(filter)
            .map(|filter_type| filter_type.matches(&ctx))
            .unwrap_or(true) // Show all packets if filter parsing fails
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("No.", "Packet number (including skipped packets)"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Protocol", "Transport layer protocol"),
            ("Length", "Length of the packet (including IP header)"),
            ("Treated as", "How was the UDP/TCP payload parsed"),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(40.0))
            .column(Column::remainder().at_least(130.0))
            .columns(Column::remainder().at_least(100.0), 2)
            .columns(Column::remainder().at_least(80.0), 2)
            .column(Column::remainder().at_least(100.0))
            .header(30.0, |mut header| {
                header_labels.iter().for_each(|(label, desc)| {
                    header.col(|ui| {
                        ui.heading(label.to_string())
                            .on_hover_text(desc.to_string());
                    });
                });
            })
            .body(|body| {
                self.build_table_body(body);
            });
    }

    fn build_table_body(&mut self, body: TableBody) {
        let filter_valid = self.filter_input.get_error().is_none();
        let mut requests = Vec::new();
        let streams = self.streams.borrow();
        let packets = &streams.packets;

        if packets.is_empty() {
            return;
        }

        let mut all_packets: Vec<_> = packets.values().collect();
        all_packets.sort_by_key(|p| p.timestamp);

        let first_timestamp = all_packets[0].timestamp;

        let filtered_packets: Vec<_> = all_packets
            .iter()
            .filter(|packet| filter_valid && self.packet_matches_filter(packet))
            .collect();

        body.rows(25.0, filtered_packets.len(), |mut row| {
            let packet = filtered_packets[row.index()];
            let timestamp = packet.timestamp - first_timestamp;

            row.col(|ui| {
                ui.label(packet.id.to_string());
            });
            row.col(|ui| {
                ui.label(timestamp.as_secs_f64().to_string());
            });
            row.col(|ui| {
                ui.label(packet.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(packet.destination_addr.to_string());
            });
            row.col(|ui| {
                ui.label(packet.transport_protocol.to_string());
            });
            row.col(|ui| {
                ui.label(packet.length.to_string());
            });
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

    fn build_parse_menu(&self, ui: &mut egui::Ui, packet: &Packet) -> Option<Request> {
        let mut request = None;
        ui.label(format!("Parse {} as:", &packet.id));
        SessionProtocol::all().iter().for_each(|packet_type| {
            let is_type = packet.session_protocol == *packet_type;
            if ui.radio(is_type, packet_type.to_string()).clicked() {
                request = Some(Request::Reparse(packet.id, *packet_type));
            }
        });
        ui.separator();
        ui.label("This will have effect on every client!");

        request
    }

    fn send_parse_request(&mut self, request: Request) {
        let Ok(msg) = request.encode() else {
            log::error!("Failed to encode a request message");
            return;
        };
        let msg = WsMessage::Binary(msg);

        self.ws_sender.send(msg);
    }
}
