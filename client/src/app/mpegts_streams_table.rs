use crate::streams::mpeg_ts_streams::MpegTsStream;
use crate::streams::RefStreams;
use eframe::emath::Vec2;
use egui::plot::{Line, Plot, PlotPoints};
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::WsSender;

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
            ("Stream alias", "Stream alias"),
            ("Program alias", "Program alias"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Number of packets", "Number of packets in stream"),
            (
                "Duration",
                "Difference between last timestamp and first timestamp.",
            ),
            (
                "Mean bitrate",
                "Sum of packet sizes (IP header included) divided by stream's duration",
            ),
            (
                "Mean MPEGTS bitrate",
                "Sum of packet sizes (MPEGTS only) divided by stream's duration",
            ),
            (
                "Mean packet rate",
                "Number of packets divided by stream's duration in seconds",
            ),
            // ("Lost packets", "Percentage of lost packets"),
            (
                "Bitrate history",
                "Plot representing bitrate for all of the stream's packets",
            ),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::initial(50.0).at_least(50.0))
            .column(Column::initial(50.0).at_least(50.0))
            .columns(Column::initial(140.0).at_least(140.0), 2)
            .columns(Column::initial(80.0).at_least(80.0), 2)
            .column(Column::initial(70.0).at_least(70.0))
            .column(Column::initial(70.0).at_least(70.0))
            .columns(Column::initial(80.0).at_least(80.0), 1)
            .column(Column::remainder().at_least(380.0).resizable(false))
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
        let keys: Vec<_> = streams.mpeg_ts_streams.keys().cloned().collect();

        body.rows(100.0, streams.mpeg_ts_streams.len(), |id, mut row| {
            let key = keys.get(id).unwrap();
            let stream = streams.mpeg_ts_streams.get_mut(key).unwrap();

            row.col(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut stream.alias).frame(false);
                ui.add(text_edit);
            });

            row.col(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut stream.alias).frame(false);
                ui.add(text_edit);
            });

            row.col(|ui| {
                ui.label(stream.mpegts_stream_info.source_addr.to_string());
            });

            row.col(|ui| {
                ui.label(stream.mpegts_stream_info.destination_addr.to_string());
            });

            row.col(|ui| {
                ui.label(stream.mpegts_stream_info.packets.len().to_string());
            });

            row.col(|ui| {
                let duration = stream.get_duration().as_secs_f64();
                ui.label(format!("{:.2} s", duration));
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_bitrate() / 1000.0;
                ui.label(format!("{:.2} kbps", bitrate));
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_mpegts_bitrate() / 1000.0;
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

fn build_bitrate_plot(ui: &mut egui::Ui, stream: &MpegTsStream) {
    ui.vertical_centered_justified(|ui| {
        let points: PlotPoints = stream
            .mpegts_stream_info
            .packets
            .iter()
            .enumerate()
            .filter_map(|(ix, mpegts)| Some([ix as f64, (mpegts.bitrate as f64 / 1000.0)]))
            .collect();

        let line = Line::new(points).name("Bitrate");
        let key = format!(
            "{}{}{}{}",
            0,
            stream.mpegts_stream_info.source_addr,
            stream.mpegts_stream_info.destination_addr,
            stream.mpegts_stream_info.protocol
        );
        Plot::new(key)
            .show_background(false)
            .show_axes([true, true])
            .label_formatter(|_name, value| {
                format!("packet id: {}\nbitrate = {:.3} kbps", value.x, value.y)
            })
            .set_margin_fraction(Vec2::new(0.1, 0.1))
            .allow_scroll(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        ui.add_space(7.0);
    });
}
