use egui::{ComboBox, Label, TextWrapMode, Ui, Widget};
use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use log::{error, warn};
use netpix_common::{Request, Response, Source};

use crate::streams::RefStreams;

use super::{
    common::{PlotRegistry, TableRegistry},
    get_initial_state,
    plots::RtpStreamsPlot,
    side_button,
    tab::Tab,
    tables::{
        MpegTsInformationTable, MpegTsPacketsTable, MpegTsStreamsTable, PacketsTable,
        RtcpPacketsTable, RtpPacketsTable, RtpStreamsTable,
    },
    SOURCE_KEY, TAB_KEY,
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

        self.build_side_panel(ctx);
        self.build_top_bar(ctx, frame);
        self.build_bottom_bar(ctx);

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
        table_registry.register::<RtpStreamsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsPacketsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsStreamsTable>(streams.clone(), ws_sender.clone());
        table_registry.register::<MpegTsInformationTable>(streams.clone(), ws_sender.clone());
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

    fn build_side_panel(&mut self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = (0.0, 8.0).into();
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = 20.0;
        }

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(32.0)
            .show(ctx, |ui| {
                ui.set_style(style);
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);

                    let button = side_button("▶");
                    let resp = ui
                        .add_enabled(!self.is_capturing, button)
                        .on_hover_text("Resume packet capturing");
                    if resp.clicked() {
                        self.is_capturing = true
                    }

                    let button = side_button("⏸");
                    let resp = ui
                        .add_enabled(self.is_capturing, button)
                        .on_hover_text("Stop packet capturing");
                    if resp.clicked() {
                        self.is_capturing = false
                    }

                    let button = side_button("🗑");
                    let resp = ui
                        .add(button)
                        .on_hover_text("Discard previously captured packets");
                    if resp.clicked() {
                        self.streams.borrow_mut().clear();
                    }

                    //TODO: implement more optimal way to do that - with lots of packages it is too much for wasm to handle this
                    let button = side_button("↻");
                    let resp = ui
                        .add(button)
                        .on_hover_text("Refetch all previously captured packets");
                    if resp.clicked() {
                        self.streams.borrow_mut().clear();
                        self.refetch_packets()
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);

                    egui::widgets::global_theme_preference_switch(ui);
                });
            });
    }

    fn build_top_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected = self.tab.display_name();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.build_dropdown_source(ui, frame);
                ui.separator();
                self.build_menu_button(ui, frame);
                Label::new(selected).ui(ui);
            });
        });
    }

    fn build_menu_button(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        ui.menu_button("📑 Open tabs", |ui| {
            ui.heading("Tabs");

            let menu_sections = Tab::sections();

            for (label, sections) in menu_sections {
                ui.menu_button(label, |ui| {
                    for tab in sections {
                        let resp = ui.selectable_value(&mut self.tab, tab, tab.display_name());
                        if resp.clicked() {
                            if let Some(storage) = frame.storage_mut() {
                                storage.set_string(TAB_KEY, tab.to_string());
                            }
                        }
                    }
                });
            }
        });
    }

    fn build_dropdown_source(&mut self, ui: &mut Ui, frame: &mut eframe::Frame) {
        let selected = match self.selected_source {
            Some(ref source) => source.to_string(),
            None => "Select packets source...".to_string(),
        };

        ComboBox::from_id_salt("source_picker")
            .width(300.0)
            .wrap_mode(TextWrapMode::Extend)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                let mut was_changed = false;

                for source in self.sources.iter() {
                    let resp = ui.selectable_value(
                        &mut self.selected_source,
                        Some(source.clone()),
                        source.to_string(),
                    );
                    if resp.clicked() {
                        was_changed = true;
                    }
                }

                if was_changed {
                    self.streams.borrow_mut().clear();
                    self.change_source_request();
                    if let Some(storage) = frame.storage_mut() {
                        let source = self.selected_source.as_ref().unwrap();
                        storage.set_string(SOURCE_KEY, source.to_string());
                    }
                }
            });
    }

    fn build_bottom_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let streams = self.streams.borrow();
                let count = streams.packets.id_count();
                let count_label = format!("Packets: {}", count);

                let captured_count = streams.packets.len();
                let captured_label = format!("Captured: {}", captured_count);

                let discharged_label = format!("Discharged: {}", self.discharged_count);
                let overwritten_label = format!("Overwritten: {}", self.overwritten_count);
                let label = format!(
                    "{} • {} • {} • {}",
                    count_label, captured_label, discharged_label, overwritten_label
                );
                ui.label(label);
            });
        });
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
                Response::Packet(packet) => {
                    let mut streams = self.streams.borrow_mut();
                    streams.add_packet(packet);
                }
                Response::Sources(sources) => {
                    if let Some(ref source) = self.selected_source {
                        if !sources.contains(source) {
                            self.selected_source = None;
                        } else {
                            self.change_source_request();
                        }
                    }
                    self.sources = sources;
                }
                Response::Sdp(stream_key, sdp) => {
                    let mut streams = self.streams.borrow_mut();
                    if let Some(stream) = streams.rtp_streams.get_mut(&stream_key) {
                        stream.add_sdp(sdp);
                    }
                }
                Response::PacketsStats(stats) => {
                    self.discharged_count = stats.discharged;
                    self.overwritten_count = stats.overwritten;
                }
            }
        }
    }

    fn refetch_packets(&mut self) {
        let request = Request::FetchAll;
        let Ok(msg) = request.encode() else {
            error!("Failed to encode a request message");
            return;
        };
        let msg = WsMessage::Binary(msg);

        self.ws_sender.send(msg);
    }

    fn change_source_request(&mut self) {
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
