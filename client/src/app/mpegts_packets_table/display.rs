use super::constants::*;
use egui::Color32;
use netpix_common::mpegts::{header::AdaptationFieldControl, MpegtsFragment};

pub fn format_packet_text(value: String, fragment: Option<&MpegtsFragment>) -> egui::RichText {
    match value {
        s if s.contains(PAT_FORMAT) => egui::RichText::from(s).color(Color32::GREEN),
        s if s.contains(PMT_FORMAT) => egui::RichText::from(s).color(Color32::LIGHT_BLUE),
        s if s.contains(PCR_FORMAT) && s.contains(ES_FORMAT) => format_pcr_es_text(s, fragment),
        s => egui::RichText::from(s),
    }
}

fn format_pcr_es_text(text: String, fragment: Option<&MpegtsFragment>) -> egui::RichText {
    let suffix = fragment.map_or("", get_star_according_payload_size);
    egui::RichText::from(format!("{}{}", text, suffix))
}

fn get_star_according_payload_size(fragment: &MpegtsFragment) -> &'static str {
    match get_fragment_payload_size(fragment) {
        1..=183 => "*",
        _ => "",
    }
}

pub fn get_fragment_payload_size(fragment: &MpegtsFragment) -> usize {
    if fragment.header.adaptation_field_control == AdaptationFieldControl::AdaptationFieldOnly {
        return 0;
    }
    fragment.payload.as_ref().map_or(0, |p| p.data.len())
}

pub fn category_from_stream_type(stream_type: &str) -> &str {
    match stream_type {
        "Video" => "📹",
        "Audio" => "🔊",
        _ => "📦",
    }
}
