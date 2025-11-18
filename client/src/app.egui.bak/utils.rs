use std::collections::HashMap;

use crate::filter_system::ParseError;
use egui::{Align2, Color32, RichText, TextEdit, Vec2};
use netpix_common::{RtpStreamKey, Source};

use super::{tab::Tab, SOURCE_KEY, TAB_KEY};

pub fn get_initial_state(cc: &eframe::CreationContext<'_>) -> (Tab, Option<Source>) {
    if let Some(storage) = cc.storage {
        let tab = match storage.get_string(TAB_KEY) {
            Some(tab_str) => Tab::from_string(tab_str).unwrap_or(Tab::Packets),
            _ => Tab::Packets,
        };

        let source = match storage.get_string(SOURCE_KEY) {
            Some(src_str) => Source::from_string(src_str),
            _ => None,
        };

        (tab, source)
    } else {
        (Tab::Packets, None)
    }
}

pub fn side_button(text: &str) -> egui::Button {
    egui::Button::new(text)
        .min_size((30.0, 30.0).into())
        .rounding(egui::Rounding::same(9.0))
}

pub fn is_rtp_stream_visible(
    streams_visibility: &mut HashMap<RtpStreamKey, bool>,
    key: RtpStreamKey,
) -> &mut bool {
    streams_visibility.entry(key).or_insert(true)
}
