use super::is_stream_visible;
use crate::streams::RefStreams;
use eframe::epaint::Color32;
use egui::RichText;
use egui_extras::{Column, TableBody, TableBuilder};
use rtpeeker_common::packet::SessionPacket;
use rtpeeker_common::StreamKey;
use std::collections::HashMap;

pub struct MpegTsPacketsTable {}

impl MpegTsPacketsTable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.options_ui(ui);
            self.build_table(ui);
        });
    }
    // TODO: implement when fields will be available
    // fn options_ui(&mut self, ui: &mut egui::Ui) {
    //     let mut aliases = Vec::new();
    //     let streams = &self.streams.borrow().streams;
    //     let keys: Vec<_> = streams.keys().collect();

    //     keys.iter().for_each(|&key| {
    //         let alias = streams.get(key).unwrap().alias.to_string();
    //         aliases.push((*key, alias));
    //     });
    //     aliases.sort_by(|(_, a), (_, b)| a.cmp(b));

    //     ui.horizontal_wrapped(|ui| {
    //         ui.label("Filter by: ");
    //         aliases.iter().for_each(|(key, alias)| {
    //             let selected = is_stream_visible(&mut self.streams_visibility, *key);
    //             ui.checkbox(selected, alias);
    //         });
    //     });
    //     ui.vertical(|ui| {
    //         ui.add_space(5.0);
    //     });
    // }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("No.", "Packet number (including skipped packets)"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("P1", "Packet No. 1"),
            ("P2", "Packet No. 2"),
            ("P3", "Packet No. 3"),
            ("P4", "Packet No. 4"),
            ("P5", "Packet No. 5"),
            ("P6", "Packet No. 6"),
            ("P7", "Packet No. 7"),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(40.0))
            .column(Column::remainder().at_least(80.0))
            .columns(Column::remainder().at_least(130.0), 2)
            .columns(Column::remainder().at_least(150.0), 7)
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

    fn build_table_body(&mut self, body: TableBody) {}
}
