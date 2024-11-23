use self::SettingsXAxis::*;
use super::is_rtp_stream_visible;
use crate::streams::{RefStreams, Streams};
use eframe::egui;
use eframe::egui::TextBuffer;
use eframe::epaint::Color32;
use egui::plot::{
    Line, LineStyle, MarkerShape, Plot, PlotBounds, PlotPoint, PlotPoints, PlotUi, Points, Text,
};
use egui::Ui;
use egui::{Align2, RichText};
use rtpeeker_common::packet::SessionPacket;
use rtpeeker_common::rtcp::ReceptionReport;
use rtpeeker_common::rtp::payload_type::MediaType;
use rtpeeker_common::RtpStreamKey;
use rtpeeker_common::{Packet, RtcpPacket, RtpPacket};
use std::cell::Ref;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

struct PointData {
    x: f64,
    y_low: f64,
    y_top: f64,
    on_hover: String,
    color: Color32,
    radius: f32,
    is_rtcp: bool,
    marker_shape: MarkerShape,
}

struct StreamSeparatorLine {
    x_start: f64,
    x_end: f64,
    y: f64,
}

struct StreamText {
    x: f64,
    y: f64,
    on_hover: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SettingsXAxis {}

impl SettingsXAxis {
    fn all() -> Vec<Self> {
        vec![]
    }
}

impl Display for SettingsXAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name = "Placeholder";

        write!(f, "{}", name)
    }
}

pub struct MpegTsStreamPlot {
    streams: RefStreams,
    points_data: Vec<PointData>,
    stream_separator_lines: Vec<StreamSeparatorLine>,
    stream_texts: Vec<StreamText>,
    requires_reset: bool,
    streams_visibility: HashMap<RtpStreamKey, bool>,
    last_rtp_packets_len: usize,
    set_plot_bounds: bool,
    slider_max: i64,
    slider_start: i64,
    slider_length: i64,
    first_draw: bool,
}

impl MpegTsStreamPlot {
    pub fn new(streams: RefStreams) -> Self {
        Self {
            streams,
            points_data: Vec::new(),
            stream_separator_lines: Vec::new(),
            stream_texts: Vec::new(),
            requires_reset: false,
            streams_visibility: HashMap::default(),
            last_rtp_packets_len: 0,
            set_plot_bounds: false,
            slider_max: 10000,
            slider_start: 0,
            slider_length: 1,
            first_draw: true,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.collapsing("Help", |ui| {
                    Self::build_help_section(ui);
                });
                ui.collapsing("Settings", |ui| {
                    self.options_ui(ui);
                });
            });
            self.plot_ui(ui);
        });
    }

    fn build_help_section(ui: &mut Ui) {}

    fn options_ui(&mut self, ui: &mut Ui) {}

    fn plot_bounds_ui_options(&mut self, ui: &mut Ui) {}

    fn axis_settings(&mut self, ui: &mut Ui) {}

    fn reset_button(&mut self, ui: &mut Ui) {
        if ui.button("Reset to initial state").clicked() {
            self.requires_reset = true;
        }
    }

    fn plot_ui(&mut self, ui: &mut Ui) {}

    fn draw_points(&mut self, plot_ui: &mut PlotUi) {}

    fn refresh_points(&mut self) {
        self.points_data.clear();
        self.stream_separator_lines.clear();
        self.stream_texts.clear();
        let mut points_x_and_y_top: Vec<(f64, f64)> = Vec::new();
        let mut previous_stream_max_y = 0.0;
        let mut biggest_margin = 0.0;
        let mut previous_stream_height = 0.0;

        let mut stream_separator_length = 0.0;
    }
}

fn get_highest_y(
    streams: &Ref<Streams>,
    points_x_and_y_top: &mut Vec<(f64, f64)>,
    settings_x_axis: SettingsXAxis,
    this_stream_y_baseline: f64,
) -> f64 {
    0.0
}
