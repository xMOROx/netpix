use egui::{Color32, ComboBox, Label, RichText, TextWrapMode, Ui, Widget};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use log::{error, warn};
use netpix_common::{Request, Response, Source};
use netpix_common::packet::PacketDirection;
use crate::streams::RefStreams;

use super::{
    SOURCE_KEY, TAB_KEY,
    common::{PlotRegistry, TableRegistry},
    get_initial_state,
    plots::RtpStreamsPlot,
    tab::Tab,
    tables::{
        IceCandidatesTable, MpegTsInformationTable, MpegTsPacketsTable, MpegTsStreamsTable,
        PacketsTable, RtcpPacketsTable, RtcpStreamsTable, RtpPacketsTable, RtpStreamsTable,
        StunPacketsTable,
    },
    ui_components::types::{AppBottomBar, AppSidePanel, AppTopBar},
};

pub use super::common::types::*;

pub struct App {
    pub(crate) ws_sender: WsSender,
    pub(crate) ws_receiver: WsReceiver,
    pub(crate) is_capturing: bool,
    pub(crate) streams: RefStreams,
    pub(crate) sources: Vec<Source>,
    pub(crate) selected_source: Option<Source>,
    pub(crate) tab: Tab,
    pub(crate) table_registry: TableRegistry,
    pub(crate) plot_registry: PlotRegistry,
    pub(crate) discharged_count: usize,
    pub(crate) overwritten_count: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.is_capturing {
            self.receive_packets()
        }

        AppSidePanel::build(self, ctx);
        AppTopBar::build(self, ctx, frame);
        AppBottomBar::build(self, ctx);

        let table_id = self.tab.get_table_id();
        if let Some(table) = self.table_registry.get_table_mut(table_id) {
            table.ui(ctx);
        }
        if let Some(plot) = self.plot_registry.get_plot_mut(table_id) {
            plot.ui(ctx);
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let host = &cc.integration_info.web_info.location.host;
        let uri = format!("ws://{}/ws", host);

        let ctx = cc.egui_ctx.clone();
        let wakeup = move || ctx.request_repaint(); // wake up UI thread on new message

        let (ws_sender, ws_receiver) =
            ewebsock::connect_with_wakeup(uri, wakeup).expect("Unable to connect to WebSocket");

        let streams = RefStreams::default();
        let mut table_registry = TableRegistry::new();
        let mut plot_registry = PlotRegistry::new();

        table_registry.register::<PacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<RtpPacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<RtcpPacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<RtcpStreamsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<RtpStreamsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsPacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsStreamsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsInformationTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<StunPacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<IceCandidatesTable>(streams.clone(), ws_sender.clone());
        plot_registry.register::<RtpStreamsPlot>(streams.clone(), ws_sender.clone());

        let (tab, selected_source) = get_initial_state(cc);

        Self {
            tab,
            streams,
            ws_sender,
            ws_receiver,
            plot_registry,
            table_registry,
            selected_source,
            is_capturing: true,
            sources: Vec::new(),
            discharged_count: 0,
            overwritten_count: 0,
        }
    }

    fn receive_packets(&mut self) {
        while let Some(msg) = self.ws_receiver.try_recv() {
            let WsEvent::Message(msg) = msg else {
                warn!("Received special message: {:?}", msg);
                continue;
            };

            let WsMessage::Binary(msg) = msg else {
                log::log!(log::Level::Warn, "Received unexpected message: {:?}", msg);
                continue;
            };

            // Handle single message at a time
            let Ok(response) = Response::decode(&msg) else {
                error!("Failed to decode response message");
                continue;
            };

            match response {
                (Response::Packet(packet), _) => {
                    let mut streams = self.streams.borrow_mut();
                    streams.add_packet(packet);
                }
                (Response::Sources(sources), _) => {
                    if let Some(ref source) = self.selected_source {
                        if !sources.contains(source) {
                            self.selected_source = None;
                        } else {
                            self.change_source_request();
                        }
                    }
                    self.sources = sources;
                }
                (Response::Sdp(stream_key, sdp), _) => {
                    let mut streams = self.streams.borrow_mut();
                    if let Some(stream) = streams.rtp_streams.get_mut(&stream_key) {
                        stream.add_sdp(sdp);
                    }
                }
                (Response::PacketsStats(stats), _) => {
                    self.discharged_count = stats.discharged;
                    self.overwritten_count = stats.overwritten;
                }
            }
        }
    }

    pub fn refetch_packets(&mut self) {
        let request = Request::FetchAll;
        let Ok(msg) = request.encode() else {
            error!("Failed to encode a request message");
            return;
        };
        let msg = WsMessage::Binary(msg);

        self.ws_sender.send(msg);
    }

    pub fn change_source_request(&mut self) {
        let selected = self.selected_source.as_ref().unwrap().clone();
        let request = Request::ChangeSource(selected);
        let Ok(msg) = request.encode() else {
            log::error!("Failed to encode a request message");
            return;
        };
        let msg = WsMessage::Binary(msg);
        self.ws_sender.send(msg);
    }
}

pub fn build_alias_row(
    ui: &mut Ui,
    alias: &str,
    row_color: Color32,
    ssrc: u32,
    direction: PacketDirection,
    stream_type: &str
) {
    ui.centered_and_justified(|ui| {
        ui.push_id(ssrc, |ui| {
            ui.menu_button(RichText::new(alias).color(row_color), |ui| {
                ui.set_min_width(200.0);

                ui.vertical(|ui| {
                    ui.label(RichText::new("Stream Information").strong().size(14.0));
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("SSRC:").strong());
                        ui.label(format!("0x{:08X} ({})", ssrc, ssrc));
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Direction:").strong());
                        match direction {
                            PacketDirection::Incoming => {
                                ui.colored_label(Color32::from_rgb(110, 210, 110), "Incoming");
                            }
                            PacketDirection::Outgoing => {
                                ui.colored_label(Color32::from_rgb(210, 110, 110), "Outgoing");
                            }
                            _ => {
                                ui.label(format!("{:?}", direction));
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Source Type:").strong());
                        ui.label(stream_type);
                    });
                });
            });
        });
    });
}