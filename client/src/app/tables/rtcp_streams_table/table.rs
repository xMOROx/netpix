use super::filters::parse_filter;
use crate::filter_system::FilterExpression;
use crate::{
    app::{
        tables::rtcp_streams_table::{filters::*, types::*},
        common::*, FilterHelpContent, FilterInput,
        TABLE_HEADER_TEXT_SIZE,
    },
    declare_table, declare_table_struct, define_column, impl_table_base,
    streams::RefStreams,
    utils::ntp_to_string,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use ewebsock::WsSender;
use netpix_common::{packet::SessionPacket, rtcp::*, RtcpPacket};
use std::any::Any;
use egui_plot::{Line, Plot, PlotPoints};
use rustc_hash::FxHashMap;
use log::info;


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

        info!("RTCPStreamsTable: Building Header");

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
        info!("RTCPStreamsTable: Building Table Body");

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
        // Use .get() safely, although in this context unwrap is likely fine
        // as we checked filtered_streams.is_empty()
        if let Some((_key, stream_data)) = filtered_streams.get(id) {

            row.col(|ui| { ui.label(format!("0x{:08X}", stream_data.ssrc)); });
            row.col(|ui| {
                if let Some(lost) = stream_data.cumulative_lost {
                    ui.label(lost.to_string());
                } else { ui.label("-"); }
            });
            row.col(|ui| { ui.label(format!("{:.1}", stream_data.current_avg_bitrate_bps / 1000.0)); }); // Using .1 for kbps
            row.col(|ui| {
                // Ensure there's data to plot
                if !stream_data.loss_history.is_empty() {
                    let line = Line::new(PlotPoints::Owned(stream_data.loss_history.clone())); // Clone data for the plot

                    // Create a small plot (sparkline style)
                    Plot::new(format!("loss_plot_{}", stream_data.ssrc)) // Unique ID per row
                        .height(ui.available_height()) // Use available cell height
                        .width(100.0) // Set a fixed width or make it dynamic
                        .show_axes([false, false]) // Hide axes
                        .show_grid(false) // Hide grid
                        .show_background(false) // Hide background
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .allow_boxed_zoom(false)
                        .show(ui, |plot_ui| { // Add the line to the plot
                            plot_ui.line(line);
                        });
                } else {
                    ui.label("-"); // Show placeholder if no history
                }
            });
        } else {
             // Log if somehow we couldn't get the stream data for a valid index
             log::info!("Error getting stream data at index {}", id);
        }
    });

    }
);



// Use the specific FilterExpression type from this table's filters module
declare_table!(RtcpStreamsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(120.0), 120.0, None, false, true), // SSRC
        column(Some(150.0), 150.0, None, false, true), // Cumulative Lost
        column(Some(150.0), 150.0, None, false, true), // Avg Bitrate
        column(None, 200.0, None, false, false),       // Plots
    )
});

impl RtcpStreamsTable {
    // This function correctly uses the stream-specific filter logic
    fn stream_matches_filter(&self, stream_data: &RtcpStreamFilterContext) -> bool {
        info!("I AM ALIVE!");
        if self.filter_input.get_filter().is_empty() {
            return true;
        }
        // Explicitly call this module's parse_filter
        parse_filter(self.filter_input.get_filter())
            .map(|filter| {
                let matches = filter.matches(stream_data);
                log::info!("Filtering stream SSRC 0x{:08X}: Filter result: {}", stream_data.stream.ssrc, matches);
                matches
            })
            .unwrap_or(true) // Treat parse errors as matching (show the stream)
    }
}


