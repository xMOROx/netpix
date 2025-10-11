use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::utils::{f64_to_ntp, ntp_to_f64};
use crate::{
    app::{
        common::*,
        tables::rtcp_streams_table::{filters::*, types::*},
        FilterHelpContent, FilterInput, TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::ntp_to_string,
};
use eframe::emath::Vec2;
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};
use ewebsock::WsSender;
use log::info;
use netpix_common::{packet::SessionPacket, rtcp::*, RtcpPacket};
use rustc_hash::FxHashMap;
use std::any::Any;

declare_table_struct!(RtcpStreamsTable);

// This macro implicitly uses `parse_filter` from the current scope.
// We ensure the correct one is in scope via the `use` statement above.
impl_table_base!(
    RtcpStreamsTable, // Struct name
    FilterHelpContent::builder("RTCP Stream Filters") // Help content expression
        .filter("ssrc", "Filter by SSRC (hexadecimal or decimal)")
        .example("ssrc:0x1234abcd")
        .example("ssrc:305441741")
        .build(),
    "rtcp_streams", // Table ID (string literal)
    "RTCP Streams"  // Display Name (string literal)
    ; // Separator before required blocks
    // The `build_header` block is now in the expected position
    build_header: |self, header| {

        let headers = [
            ("SSRC", "Synchronization Source Identifier (hex)"),
            ("Cumulative Lost", "Total number of packets lost (from Receiver Reports)"),
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

        let row_height = 100.0; // Adjust as needed

    body.rows(row_height, filtered_streams.len(), |mut row| {
        let id = row.index();
        if let Some((_key, stream_data)) = filtered_streams.get(id) {

            row.col(|ui| { ui.label(format!("0x{:08X}", stream_data.ssrc)); });
            row.col(|ui| {
                if let Some(lost) = stream_data.cumulative_lost {
                    ui.label(lost.to_string());
                } else { ui.label("-"); }
            });
            row.col(|ui| { ui.label(format!("{:.1}", stream_data.current_avg_bitrate_bps / 1000.0)); });

            row.col(|ui| {
                ui.vertical_centered_justified(|ui| {
                    let points: Vec<PlotPoint> = stream_data
                    .bitrate_history
                    .iter()
                    .map(|(ntp, bitrate)| {
                        // Transform y into log10 domain:
                        PlotPoint::new(ntp_to_f64(*ntp), (*bitrate as f64).log10())
                    })
                    .collect();
                    let line = Line::new(PlotPoints::Owned(points));
                    Plot::new(format!(
                        "{}{}{}",
                        stream_data.ssrc, stream_data.source_addr, stream_data.destination_addr
                    ))
                    .show_background(false)
                    .show_axes([true, true])
                    .x_axis_formatter(|mark,_range| {
                        format!("{}",ntp_to_string(f64_to_ntp(mark.value)))
                    })
                    .y_axis_formatter(|mark, _range| {
                        let real_bps = 10f64.powf(mark.value);
                        format!("{:.0}kbits", real_bps / 1_000.0)
                    })
                    .label_formatter(|_name, pt| {
                        let real_bps = 10f64.powf(pt.y);
                        format!(
                            "time: {}\navg. bitrate = {:.3}kbits",
                            ntp_to_string(f64_to_ntp(pt.x)),
                            real_bps / 1_000.0
                        )
                    })
                    .set_margin_fraction(Vec2::new(0.1, 0.1))
                    .allow_scroll(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    })
                    .response;
                    ui.add_space(7.0);
                });
            });
        } else {
             info!("Error getting stream data at index {}", id);
        }
    });

    }
);

// Use the specific FilterExpression type from this table's filters module
declare_table!(RtcpStreamsTable, FilterType, {
    height(60.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(120.0), 120.0, None, false, true), // SSRC
        column(Some(150.0), 150.0, None, false, true), // Cumulative Lost
        column(Some(150.0), 150.0, None, false, true), // Avg Bitrate
        column(None, 600.0, None, false, false),       // Plots
    )
});

impl RtcpStreamsTable {
    // This function correctly uses the stream-specific filter logic
    fn stream_matches_filter(&self, stream_data: &RtcpStreamFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }
        // Explicitly call this module's parse_filter
        parse_filter(self.filter_input.get_filter())
            .map(|filter| {
                let matches = filter.matches(stream_data);
                matches
            })
            .unwrap_or(true) // Treat parse errors as matching (show the stream)
    }
}
