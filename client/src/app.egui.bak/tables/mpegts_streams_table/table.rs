use super::{
    display::build_bitrate_plot,
    filters::{parse_filter, FilterContext},
};
use crate::{
    app::{
        common::{TableBase, TableConfig},
        FilterHelpContent, FilterInput, TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column,
    filter_system::FilterExpression,
    impl_table_base,
    streams::{
        mpegts_stream::substream::MpegtsSubStream, stream_statistics::StreamStatistics, RefStreams,
    },
};
use egui::{RichText, Window};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::mpegts::psi::pmt::stream_types::{stream_type_into_unique_letter, StreamType};
use std::{any::Any, collections::HashMap};

declare_table_struct!(
    MpegTsStreamsTable,
    open_modal: bool
);

impl_table_base!(
    MpegTsStreamsTable;
    open_modal: bool;
    FilterHelpContent::builder("MPEG-TS Streams Filters")
            .filter("alias:<value>", "Filter by stream alias")
            .filter("program:<number>", "Filter by program number")
            .filter("source:<ip>", "Filter by source IP address")
            .filter("dest:<ip>", "Filter by destination IP address")
            .filter("fragments:<op><number>", "Filter by number of fragments")
            .filter("duration:<op><seconds>", "Filter by stream duration")
            .filter("bitrate:<op><kbps>", "Filter by mean bitrate")
            .filter("fragmentrate:<op><number>", "Filter by fragment rate")
            .example("alias:stream1 AND bitrate:>1000")
            .example("source:192.168 OR dest:10.0")
            .example("fragments:>100 AND duration:<10")
            .example("(program:1 AND bitrate:>500) OR fragmentrate:>30")
            .build(),
    "mpegts_streams", "MPEG-TS Streams"
    ;
    ui: |self, ctx| {
        if self.filter_input.show(ctx) {
            self.check_filter();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);

            // Show modal if open
            if self.open_modal {
                self.show_stream_type_info_modal(ctx);
            }
        });

    }
    ;
    build_header: |self, header| {
        let headers = [
            (
                "Stream alias",
                "Stream alias for the stream is made up of the transport stream id and stream type",
            ),
            ("Program number", "Program number from PMT table"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            (
                "Number of packets",
                "Number of packets in MPEG-TS stream",
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
                "Mean MPEG-TS bitrate",
                "Sum of packet sizes (MPEG-TS only) divided by stream's duration",
            ),
            (
                "Mean packet rate",
                "Number of packets divided by stream's duration in seconds",
            ),
            (
                "Bitrate history",
                "Plot representing bitrate for all of the stream's fragments",
            ),
        ];

        for (label, desc) in headers {
            header.col(|ui| {
                ui.label(RichText::new(label.to_string()).size(TABLE_HEADER_TEXT_SIZE).strong())
                    .on_hover_text(desc.to_string());
            });
        }

    }
    ;
    build_table_body: |self, body| {
        let filter_valid = self.filter_input.get_error().is_none();
        let mut streams = self.streams.borrow_mut();
        let mut keys: Vec<_> = vec![];
        let mut substreams: HashMap<_, _> = HashMap::new();

        for stream in streams.mpeg_ts_streams.values_mut() {
            for (key, substream) in stream.substreams.iter() {
                if filter_valid && self.stream_matches_filter(substream) {
                    keys.push(key);
                    substreams.insert(key, substream.clone());
                }
            }
        }

        body.rows(100.0, keys.len(), |mut row| {
            let id = row.index();
            let key = keys.get(id).unwrap();
            let stream = substreams.get_mut(key).unwrap();

            row.col(|ui| {
                ui.horizontal(|ui| {
                    let text_edit =
                        egui::TextEdit::singleline(&mut stream.aliases.stream_alias).frame(false);
                    ui.add(text_edit);
                });
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
);

declare_table!(MpegTsStreamsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(80.0), 80.0, Some(80.0), false, true),
        column(Some(80.0), 80.0, Some(80.0), false, true),
        column(Some(120.0), 120.0, Some(135.0), false, true),
        column(Some(120.0), 120.0, Some(135.0), false, true),
        column(Some(97.0), 97.0, Some(112.0), false, true),
        column(Some(75.0), 75.0, Some(90.0), false, true),
        column(Some(80.0), 80.0, Some(95.0), false, true),
        column(Some(115.0), 115.0, Some(120.0), false, true),
        column(Some(130.0), 130.0, Some(145.0), false, true),
        column(None, 130.0, None, false, false),
    )
});

impl MpegTsStreamsTable {
    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let streams = &self.streams.borrow();
        let has_stream_types = streams
            .mpeg_ts_streams
            .values()
            .flat_map(|stream| stream.substreams.values())
            .next()
            .is_some();

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let mut button = egui::Button::new("ℹ Stream Types");

                // Change button appearance when modal is open
                if self.open_modal {
                    button = button.fill(ui.visuals().selection.bg_fill);
                }

                if ui.add_enabled(has_stream_types, button).clicked() {
                    self.open_modal = !self.open_modal; // Toggle modal state
                }
            });
        });
    }

    fn stream_matches_filter(&self, stream: &MpegtsSubStream) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim().to_lowercase();
        let ctx = FilterContext { stream };

        parse_filter(&filter)
            .map(|filter_type| filter_type.matches(&ctx))
            .unwrap_or(true) // Show all streams if filter parsing fails
    }

    fn show_stream_type_info_modal(&mut self, ctx: &egui::Context) {
        let streams = self.streams.borrow();
        let mut unique_stream_types: Vec<StreamType> = streams
            .mpeg_ts_streams
            .values()
            .flat_map(|stream| stream.substreams.values())
            .map(|substream| substream.stream_type)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        unique_stream_types.sort();

        Window::new("Stream Type Information")
            .resizable(false)
            .open(&mut self.open_modal)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    for stream_type in unique_stream_types {
                        let letter = stream_type_into_unique_letter(&stream_type);
                        ui.label(format!("{} - {}", letter, stream_type));
                    }
                });
            });
    }
}
