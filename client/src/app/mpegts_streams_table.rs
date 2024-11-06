use egui::plot::{Line, Plot, PlotPoints};
use egui::{TextEdit, Vec2};
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::{WsMessage, WsSender};
use rtpeeker_common::{Request, StreamKey};

pub struct MpegTsStreamsTable {}

impl MpegTsStreamsTable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("Stream alias", "Stream alias"),
            ("Program alias", "Program alias"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Number of packets", "Number of packets in stream"),
            (
                "Duration",
                "Difference between last timestamp and first timestamp.",
            ),
            ("Lost packets", "Percentage of lost packets"),
            ("Mean jitter", "Average of jitter for all of the packets"),
            (
                "Jitter history",
                "Plot representing jitter for all of the stream's packets",
            ),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::initial(50.0).at_least(50.0))
            .column(Column::initial(50.0).at_least(50.0))
            .columns(Column::initial(80.0).at_least(80.0), 2)
            .columns(Column::initial(70.0).at_least(70.0), 2)
            .columns(Column::initial(80.0).at_least(80.0), 2)
            .column(Column::remainder().at_least(200.0).resizable(false))
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

fn build_jitter_plot(ui: &mut egui::Ui) {}
