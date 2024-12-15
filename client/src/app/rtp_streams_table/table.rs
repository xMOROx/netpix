use crate::app::common::*;
use crate::app::rtp_streams_table::{filters::*, types::*};
use crate::app::utils::*;
use crate::define_column;
use crate::filter_system::FilterExpression;
use crate::streams::rtpStream::RtpStream;
use crate::streams::RefStreams;
use crate::{declare_table, declare_table_struct, impl_table_base};
use eframe::epaint::Color32;
use egui::{RichText, TextEdit, Vec2};
use egui_extras::TableBuilder;
use egui_extras::{Column, TableBody, TableRow};
use egui_plot::{Line, Plot, PlotPoints};
use ewebsock::{WsMessage, WsSender};
use netpix_common::{Request, RtpStreamKey};

declare_table_struct!(RtpStreamsTable,
    ws_sender: Option<WsSender>,
    chosen_key: Option<RtpStreamKey>,
    sdp_window: SdpWindow
);

declare_table!(RtpStreamsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(50.0), 50.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(140.0), 140.0, None, false, true),
        column(Some(140.0), 140.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(70.0), 70.0, None, false, true),
        column(Some(70.0), 70.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(Some(80.0), 80.0, None, false, true),
        column(None, 380.0, None, false, false),
    )
});

impl_table_base!(
    RtpStreamsTable;
    ws_sender: Option<WsSender>,
    chosen_key: Option<RtpStreamKey>,
    sdp_window: SdpWindow;
    FilterHelpContent::builder("RTP Stream Filters")
        .filter("source", "Filter by source IP address")
        .filter("dest", "Filter by destination IP address")
        .filter("alias", "Filter by stream alias")
        .example("source:10.0.0")
        .example("dest:192.168 AND NOT alias:test")
        .build()
    ;
    ui: |self, ctx| {
                if self.filter_input.show(ctx) {
                    self.check_filter();
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.build_table(ui);
                });

                self.build_sdp_window(ctx);
            }
    ;
    build_header: |self, header| {
        let headers = [
            ("Alias", "Locally assigned SSRC alias to make differentiating streams more convenient"),
            ("SSRC", "RTP SSRC (Synchronization Source Identifier) identifies the source of an RTP stream"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("CNAME", "Source Description CNAME value, if received (latest one if changed mid-stream"),
            ("Payload type", "Payload type of this stream (latest one if changed mid-stream)"),
            ("Packet count", "Number of packets in stream"),
            ("Packet loss", "Percentage of packets lost"),
            ("Duration", "Difference between last timestamp and first timestamp."),
            ("Mean jitter", "Average of jitter for all of the packets"),
            ("Mean bitrate", "Sum of packet sizes (IP header included) divided by stream's duration"),
            ("Mean RTP bitrate", "Sum of packet sizes (RTP only) divided by stream's duration"),
            ("Mean packet rate", "Number of packets divided by stream's duration in seconds"),
            ("Jitter history", "Plot representing jitter for all of the stream's packets")
        ];

        for (label, desc) in headers {
            header.col(|ui| {
                ui.heading(label).on_hover_text(desc);
            });
        }
    }
    ;
    build_table_body: |self, body| {
        let streams = self.streams.borrow();
        let filtered_streams: Vec<_> = streams
            .rtp_streams
            .iter()
            .filter(|(_, stream)| {
                let ctx = RtpStreamFilterContext {
                    stream,
                    alias: &stream.alias,
                    source_addr: &stream.source_addr.to_string(),
                    destination_addr: &stream.destination_addr.to_string(),
                };
                self.stream_matches_filter(&ctx)
            })
            .collect();

        body.rows(self.config.row_height, filtered_streams.len(), |mut row| {
            let id = row.index();
            let (key, stream) = filtered_streams.get(id).unwrap();

            // Alias column - using clone to avoid borrow issues
            let mut alias = stream.alias.clone();
            row.col(|ui| {
                if ui.add(TextEdit::singleline(&mut alias).frame(false)).changed() {
                    if let Ok(mut streams) = self.streams.try_borrow_mut() {
                        if let Some(stream) = streams.rtp_streams.get_mut(key) {
                            stream.alias = alias.clone();
                        }
                    }
                }
            });

            // Rest of the columns
            row.col(|ui| {
                ui.label(format!("{:x}", stream.ssrc));
            });

            // Source/Destination columns
            row.col(|ui| {
                ui.label(stream.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(stream.destination_addr.to_string());
            });

            // CNAME column
            row.col(|ui| {
                ui.label(stream.cname.as_ref().unwrap_or(&"N/A".to_string()));
            });

            // Payload type column
            row.col(|ui| {
                let pt = stream
                    .payload_types
                    .iter()
                    .map(|pt| pt.id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let on_hover = stream
                    .payload_types
                    .iter()
                    .map(|pt| pt.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");

                ui.label(pt).on_hover_text(on_hover);
            });

            // Statistics columns
            row.col(|ui| {
                ui.label(stream.rtp_packets.len().to_string());
            });

            row.col(|ui| {
                let lost = stream.get_expected_count() - stream.rtp_packets.len();
                let lost_fraction = lost as f64 / stream.get_expected_count() as f64;
                ui.label(format!("{:.3}%", lost_fraction * 100.0));
            });

            row.col(|ui| {
                let duration = stream.get_duration().as_secs_f64();
                ui.label(format!("{:.2} s", duration));
            });

            row.col(|ui| {
                let jitter_label = match stream.get_mean_jitter() {
                    Some(jitter) => format!("{:.3} ms", jitter * 1000.0),
                    None => "N/A".to_string(),
                };
                ui.label(jitter_label);
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_bitrate() / 1000.0;
                ui.label(format!("{:.2} kbps", bitrate));
            });

            row.col(|ui| {
                let bitrate = stream.get_mean_rtp_bitrate() / 1000.0;
                ui.label(format!("{:.2} kbps", bitrate));
            });

            row.col(|ui| {
                let packet_rate = stream.get_mean_packet_rate();
                ui.label(format!("{:.1} /s", packet_rate));
            });

            // Jitter plot column
            row.col(|ui| {
                ui.vertical_centered_justified(|ui| {
                    let points: PlotPoints = stream
                        .rtp_packets
                        .iter()
                        .enumerate()
                        .filter_map(|(ix, rtp)| rtp.jitter.map(|jitter| [ix as f64, jitter * 1000.0]))
                        .collect();

                    let line = Line::new(points).name("jitter");
                    let response = Plot::new(format!(
                        "{}{}{}{}",
                        stream.ssrc, stream.source_addr, stream.destination_addr, stream.protocol
                    ))
                    .show_background(false)
                    .show_axes([true, true])
                    .label_formatter(|_name, value| {
                        format!("packet id: {}\njitter = {:.3} ms", value.x, value.y)
                    })
                    .set_margin_fraction(Vec2::new(0.1, 0.1))
                    .allow_scroll(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    })
                    .response;

                    response.context_menu(|ui| {
                        if ui.button("Set SDP").clicked() {
                            self.chosen_key = Some(**key);
                            self.sdp_window.sdp = String::new();
                            self.sdp_window.open = true;
                            ui.close_menu();
                        }
                    });
                    ui.add_space(7.0);
                });
            });
        });
    }
);

impl RtpStreamsTable {
    pub fn new_with_sender(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            filter_input: FilterInput::new(
                FilterHelpContent::builder("RTP Stream Filters")
                    .filter("source", "Filter by source IP address")
                    .filter("dest", "Filter by destination IP address")
                    .filter("alias", "Filter by stream alias")
                    .filter("ssrc", "Filter by SSRC value")
                    .example("source:10.0.0")
                    .example("dest:192.168 AND NOT alias:test")
                    .build(),
            ),
            config: TableConfig::new(100.0, 30.0, 5.0),
            ws_sender: Some(ws_sender),
            chosen_key: None,
            sdp_window: SdpWindow::default(),
        }
    }

    pub fn send_sdp_request(&mut self) {
        let request = Request::ParseSdp(self.chosen_key.unwrap(), self.sdp_window.sdp.clone());

        let Ok(msg) = request.encode() else {
            log::error!("Failed to encode a request message");
            return;
        };
        let msg = WsMessage::Binary(msg);

        self.ws_sender.as_mut().unwrap().send(msg);
    }

    fn stream_matches_filter(&self, ctx: &RtpStreamFilterContext) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim().to_lowercase();
        parse_filter(&filter)
            .map(|filter_type| filter_type.matches(ctx))
            .unwrap_or(true)
    }

    fn build_sdp_window(&mut self, ctx: &egui::Context) {
        let Some((_, _, _, ssrc)) = self.chosen_key else {
            return;
        };

        let mut send_sdp = false;

        egui::Window::new(format!("SDP - {:x}", ssrc))
            .open(&mut self.sdp_window.open)
            .default_width(800.0)
            .default_height(500.0)
            .vscroll(true)
            .show(ctx, |ui| {
                TextEdit::multiline(&mut self.sdp_window.sdp)
                    .hint_text(SDP_PROMPT)
                    .desired_rows(30)
                    .desired_width(f32::INFINITY)
                    .show(ui);
                ui.add_space(10.0);
                if ui.button(format!("Set SDP for {:x}", ssrc)).clicked() {
                    send_sdp = true;
                }
            });

        if send_sdp {
            self.send_sdp_request();
            self.sdp_window.open = false;
        }
    }
}

const SDP_PROMPT: &str = "Paste your SDP media section here, e.g.
m=audio 5004 RTP/AVP 96
c=IN IP4 239.30.22.1
a=rtpmap:96 L24/48000/2
a=recvonly
";
