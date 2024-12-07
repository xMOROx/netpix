mod filters;

use crate::filter_system::FilterExpression;
use crate::streams::RefStreams;
use egui::widgets::TextEdit;
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::{WsMessage, WsSender};
use filters::{parse_filter, FilterContext};
use netpix_common::packet::{Packet, SessionProtocol};
use netpix_common::Request;

pub struct PacketsTable {
    streams: RefStreams,
    ws_sender: WsSender,
    filter_buffer: String,
}

impl PacketsTable {
    pub fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            ws_sender,
            filter_buffer: String::new(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("filter_bar").show(ctx, |ui| {
            self.build_filter(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }

    fn build_filter(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let text_edit = TextEdit::singleline(&mut self.filter_buffer)
                .font(egui::style::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .hint_text("Examples: source:192.168, proto:udp AND length:>100");
            let response = ui
                .small_button("ℹ Help")
                .on_hover_text("Show filter syntax help");

            ui.add(text_edit);
        });
    }

    fn check_filter(&self) -> bool {
        if self.filter_buffer.is_empty() {
            return true;
        }

        let filter = self.filter_buffer.trim();
        parse_filter(filter).is_ok()
    }

    fn packet_matches_filter(&self, packet: &Packet) -> bool {
        if self.filter_buffer.is_empty() {
            return true;
        }

        let filter = self.filter_buffer.trim();
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
        let filter_valid = self.check_filter();
        let mut requests = Vec::new();
        let streams = self.streams.borrow();
        let packets = &streams.packets;

        if packets.is_empty() {
            return;
        }

        let first_timestamp = packets.first().unwrap().timestamp;
        let filtered_count = packets
            .values()
            .filter(|packet| filter_valid && self.packet_matches_filter(packet))
            .count();

        body.rows(25.0, filtered_count, |mut row| {
            let row_index = row.index();

            let packet = packets
                .values()
                .filter(|packet| filter_valid && self.packet_matches_filter(packet))
                .nth(row_index)
                .unwrap();

            row.col(|ui| {
                ui.label(packet.id.to_string());
            });
            let timestamp = packet.timestamp - first_timestamp;
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

        // cannot take mutable reference to self
        // unless `packets` is dropped, hence the `request` vector
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
