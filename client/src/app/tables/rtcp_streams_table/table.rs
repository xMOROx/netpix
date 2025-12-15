use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{
        FilterHelpContent, FilterInput, TABLE_HEADER_TEXT_SIZE,
        common::*,
        tables::rtcp_streams_table::{filters::*, types::*},
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::{f64_to_ntp, ntp_to_f64, ntp_to_time_string},
};
use eframe::emath::Vec2;
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};
use ewebsock::WsSender;
use log::info;
use netpix_common::{RtcpPacket, packet::SessionPacket, rtcp::*};
use rustc_hash::FxHashMap;
use std::any::Any;

pub struct RtcpStreamsTable {
    streams: RefStreams,
    filter_input: FilterInput,
    config: TableConfig,
    ws_sender: WsSender,
    pub alias_helper: StreamAliasHelper,
}

// This macro implicitly uses `parse_filter` from the current scope.
// We ensure the correct one is in scope via the `use` statement above.
impl_table_base!(
    RtcpStreamsTable;
    alias_helper: StreamAliasHelper;

    FilterHelpContent::builder("RTCP Stream Filters") // Help content expression
        .filter("ssrc", "Filter by SSRC (hexadecimal or decimal)")
        .example("ssrc:0x1234abcd")
        .example("ssrc:305441741")
        .build(),
    "rtcp_streams",
    "RTCP Streams"
    ;

    ui: |self, ctx| {
        if self.filter_input.show(ctx) {
            self.check_filter();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });
    }
    ;

    build_header: |self, header| {

        let headers = [
            ("SSRC", "Synchronization Source Identifier (hex)"),
            ("Direction", "Are packets incoming or outgoing"),
            ("Avg Bitrate (kbps)", "Average bitrate calculated from Sender Reports"),
            ("Plots", "Graphs showing bitrate and loss over time"),
        ];

        for (label, desc) in headers {
            header.col(|ui| {
                ui.label(RichText::new(label.to_string()).size(TABLE_HEADER_TEXT_SIZE).strong())
                    .on_hover_text(desc.to_string());
            });
        }
    }
    ; // Separator
    build_table_body: |self, body| {
        let streams = self.streams.borrow();
        let filtered_streams: Vec<_> = streams
            .rtcp_streams
            .iter()
            .filter(|(_, stream)| {
                let ctx = RtcpStreamFilterContext {
                    stream,
                    source_addr: &stream.source_addr.to_string(),
                    destination_addr: &stream.destination_addr.to_string(),
                };
                self.stream_matches_filter(&ctx)
            })
            .collect();

        if filtered_streams.is_empty() {
            body.rows(30.0, 1, |mut row| {
                 row.col(|ui| {
                     ui.centered_and_justified(|ui| {
                        ui.label("No RTCP streams data available or matching filter");
                     });
                 });
             });
            return;
        }

        let row_height = 200.0;

    body.rows(row_height, filtered_streams.len(), |mut row| {
        let id = row.index();
        if let Some((_key, stream_data)) = filtered_streams.get(id) {

            let row_color = self.alias_helper.get_color(stream_data.ssrc);
            let alias = self.alias_helper.get_alias(stream_data.ssrc);

            row.col(|ui| {
                    ui.label(format!("0x{:08X}", stream_data.ssrc));
                    ui.centered_and_justified(|ui|{
                        ui.colored_label(row_color,alias);
                    });
            });
            row.col(|ui| {
                random_label_example(ui,stream_data.ssrc);
            });
            row.col(|ui| { ui.label(format!("{:.1}", stream_data.current_avg_bitrate_bps / 1000.0)); });

            row.col(|ui| {
        ui.vertical_centered_justified(|ui| {
            let max_y = stream_data.bitrate_history.iter().fold(0.0, |acc : f64, p| acc.max(p.y));

            let top_bound = if max_y > 0.0 {
                max_y * 1.3
            } else {
                100_000.0
            };

            // VERY EXPENSIVE!
            // TO DO: Change to PlotPoints::Borrowed after updating rust version
            // https://docs.rs/egui_plot/latest/egui_plot/enum.PlotPoints.html#variant.Borrowed
            let line_avg = Line::new(PlotPoints::Owned(stream_data.bitrate_history.clone()));

            let markers = egui_plot::Points::new(PlotPoints::Owned(stream_data.bitrate_history.clone()))
                .radius(2.5)
                .color(egui::Color32::from_rgb(255, 100, 100));

            Plot::new(format!(
                "{}{}{}",
                stream_data.ssrc, stream_data.source_addr, stream_data.destination_addr
            ))
            .show_background(false)
            .show_axes([true, true])
            .x_axis_formatter(|mark,_range| {
                ntp_to_time_string(f64_to_ntp(mark.value)).to_string()
            })
            .y_axis_formatter(|mark, _range| {
                let real_bps = mark.value;
                format!("{:.0}kbits", real_bps / 1_000.0)
            })
            .label_formatter(|_name, pt| {
                let real_bps = pt.y;
                format!(
                    "time: {}\navg. bitrate = {:.3}kbits",
                    ntp_to_time_string(f64_to_ntp(pt.x)),
                    real_bps / 1_000.0
                )
            })
            .set_margin_fraction(Vec2::new(0.1, 0.1))
            .include_y(0.0)
            .include_y(top_bound)
            .allow_scroll(false)
            .allow_drag(false)
            .allow_zoom(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line_avg);
                plot_ui.points(markers);
            });
            ui.add_space(7.0);
                });
            });
        } else {
             info!("Error getting stream data at index {}", id);
        }
    });

    }
);

declare_table!(RtcpStreamsTable, FilterType, {
    height(60.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(120.0), 120.0, None, false, true), // SSRC
        column(Some(70.0), 70.0, None, false, true),
        column(Some(150.0), 150.0, None, false, true), // Avg Bitrate
        column(None, 600.0, None, false, false),       // Plots
    )
});

impl RtcpStreamsTable {
    fn stream_matches_filter(&self, stream_data: &RtcpStreamFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }
        parse_filter(self.filter_input.get_filter())
            .map(|filter| filter.matches(stream_data))
            .unwrap_or(true)
    }
}


pub fn random_label_example(ui: &mut egui::Ui, ssrc: u32) {
    // LOGIC FIX: Don't use thread_rng() here! It will re-generate on every frame
    // (mouse movement, scroll) causing the text to flicker rapidly.

    // Instead, use the SSRC to make a deterministic "random" choice.
    // We mix the bits slightly so consecutive SSRCs don't look too similar.
    let is_outgoing = (ssrc.wrapping_mul(1664525).wrapping_add(1013904223)) % 2 == 0;

    let label_text = if is_outgoing {
        "Outgoing"
    } else {
        "Incoming"
    };

    ui.columns(1, |columns| {
        columns[0].vertical_centered_justified(|ui| {
            let color = if is_outgoing {
                egui::Color32::from_rgb(20, 150, 20) // Green
            } else {
                egui::Color32::from_rgb(150, 20, 20) // Red
            };

            ui.label(egui::RichText::new(label_text).color(color).strong());
        });
    });
}