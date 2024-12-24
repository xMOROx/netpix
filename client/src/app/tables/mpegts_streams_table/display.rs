use crate::streams::mpegts_stream::substream::MpegtsSubStream;
use egui::{self, Vec2};
use egui_plot::{Line, Plot, PlotPoints};

pub fn build_bitrate_plot(ui: &mut egui::Ui, stream: &MpegtsSubStream) {
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
