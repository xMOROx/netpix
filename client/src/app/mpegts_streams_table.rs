use crate::streams::mpegts_stream::substream::MpegtsSubStream;
use crate::streams::stream_statistics::StreamStatistics;
use crate::streams::RefStreams;
use eframe::emath::Vec2;
use egui_extras::{Column, TableBody, TableBuilder};
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::HashMap;

pub struct MpegTsStreamsTable {
    streams: RefStreams,
}

impl MpegTsStreamsTable {
    pub fn new(streams: RefStreams) -> Self {
        Self { streams }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            (
                "Stream alias",
                "Stream alias for the stream is made up of the transport stream id and stream type",
            ),
            ("Program number", "Program number from PMT table"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            (
                "Number of fragments",
                "Number of fragments in mpegts stream",
            ),
            (
                "Duration",
                "Difference between last timestamp and first timestamp.",
            ),
            (
                "Mean bitrate",
                "Sum of packet sizes (IP header included) divided by stream's duration",
            ),
            (
                "Mean mpegts bitrate",
                "Sum of fragment sizes (mpegts only) divided by stream's duration",
            ),
            (
                "Mean fragment rate",
                "Number of fragments divided by stream's duration in seconds",
            ),
            (
                "Bitrate history",
                "Plot representing bitrate for all of the stream's fragments",
            ),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::initial(80.0).at_most(80.0).at_least(80.0))
            .column(Column::initial(80.0).at_most(80.0).at_least(80.0))
            .columns(Column::initial(140.0).at_least(140.0).at_most(155.0), 2)
            .column(Column::initial(97.0).at_most(112.0).at_least(97.0))
            .column(Column::initial(75.0).at_most(90.0).at_least(75.0))
            .column(Column::initial(80.0).at_most(95.0).at_least(80.0))
            .column(Column::initial(75.0).at_most(90.0).at_least(75.0))
            .column(Column::initial(130.0).at_least(130.0).at_most(145.0))
            .column(Column::remainder().at_least(320.0).resizable(false))
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
        let mut streams = self.streams.borrow_mut();
        let mut keys: Vec<_> = vec![];
        let mut substreams: HashMap<_, _> = HashMap::new();

        for stream in streams.mpeg_ts_streams.values_mut() {
            keys.extend(stream.substreams.keys());
            substreams.extend(stream.substreams.clone());
        }

        body.rows(100.0, keys.len(), |mut row| {
            let id = row.index();
            let key = keys.get(id).unwrap();
            let stream = substreams.get_mut(key).unwrap();

            row.col(|ui| {
                let text_edit =
                    egui::TextEdit::singleline(&mut stream.aliases.stream_alias).frame(false);
                ui.add(text_edit);
            });

            row.col(|ui| {
                let text_edit =
                    egui::TextEdit::singleline(&mut stream.aliases.program_alias).frame(false);
                ui.add(text_edit);
            });

            row.col(|ui| {
                ui.label(stream.packet_association_table.source_addr.to_string());
            });

            row.col(|ui| {
                ui.label(stream.packet_association_table.destination_addr.to_string());
            });

            row.col(|ui| {
                ui.label(stream.packets.len().to_string());
            });

            row.col(|ui| {
                let duration = stream.get_duration().as_secs_f64();
                ui.label(format!("{:.2} s", duration));
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_frame_bitrate() / 1000.0;
                ui.label(format!("{:.2} kbps", bitrate));
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_protocol_bitrate() / 1000.0;
                ui.label(format!("{:.2} kbps", bitrate));
            });

            row.col(|ui| {
                let packet_rate = stream.get_mean_packet_rate();
                ui.label(format!("{:.1} /s", packet_rate));
            });

            row.col(|ui| {
                build_bitrate_plot(ui, stream);
            });
        });
    }
}

fn build_bitrate_plot(ui: &mut egui::Ui, stream: &MpegtsSubStream) {
    ui.vertical_centered_justified(|ui| {
        let points: PlotPoints = stream
            .packets
            .iter()
            .enumerate()
            .map(|(ix, mpegts)| [ix as f64, mpegts.bitrate as f64 / 1000.0])
            .collect();

        let line = Line::new(points).name("Bitrate");
        let key = format!(
            "{}{}{}{}",
            stream.packet_association_table,
            stream.transport_stream_id,
            stream.program_number,
            stream.stream_type
        );
        Plot::new(key)
            .show_background(false)
            .show_axes([true, true])
            .label_formatter(|_name, value| {
                format!("fragment id: {}\nbitrate = {:.3} kbps", value.x, value.y)
            })
            .set_margin_fraction(Vec2::new(0.1, 0.1))
            .allow_scroll(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        ui.add_space(7.0);
    });
}
