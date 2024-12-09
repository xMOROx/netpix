use egui;
use netpix_common::mpegts::descriptors::{
    audio_stream::AudioStreamDescriptor, avc_video_descriptor::AvcVideoDescriptor,
    copyright_descriptor::CopyrightDescriptor,
    iso_639_language_descriptor::Iso639LanguageDescriptor,
    maximum_bitrate_descriptor::MaximumBitrateDescriptor,
    multiplex_buffer_utilization_descriptor::MultiplexBufferUtilizationDescriptor,
    system_clock_descriptor::SystemClockDescriptor, video_stream::VideoStreamDescriptor,
    video_window_descriptor::VideoWindowDescriptor, Descriptors,
};

pub fn build_label(ui: &mut egui::Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let label = egui::RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(normal.into());
    });
}

pub fn build_avc_video_descriptor(ui: &mut egui::Ui, desc: &AvcVideoDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "AVC Video:", "");
        ui.indent("avc_indent", |ui| {
            build_label(ui, "Profile:", desc.profile_idc.to_string());
            build_label(ui, "Level:", desc.level_idc.to_string());
            build_label(ui, "Still Present:", desc.avc_still_present.to_string());
        });
    });
}

pub fn build_copyright_descriptor(ui: &mut egui::Ui, desc: &CopyrightDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "Copyright:", "");
        ui.indent("copyright_indent", |ui| {
            build_label(
                ui,
                "Identifier:",
                format!("{:#x}", desc.copyright_identifier),
            );
            if !desc.additional_copyright_info.is_empty() {
                build_label(
                    ui,
                    "Additional Info:",
                    format!("{:?}", desc.additional_copyright_info),
                );
            }
        });
    });
}

pub fn build_iso639_language_descriptor(ui: &mut egui::Ui, desc: &Iso639LanguageDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "Language:", "");
        ui.indent("lang_indent", |ui| {
            for section in &desc.section {
                build_label(
                    ui,
                    &section.language_code,
                    format!("({})", section.audio_type),
                );
            }
        });
    });
}

pub fn build_video_stream_descriptor(ui: &mut egui::Ui, desc: &VideoStreamDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "Video Stream:", "");
        ui.indent("video_indent", |ui| {
            build_label(ui, "Frame Rate:", desc.frame_rate_code.to_string());
            if let Some(profile) = desc.profile_and_level_indication {
                build_label(ui, "Profile Level:", profile.to_string());
            }
            if let Some(chroma) = desc.chroma_format {
                build_label(ui, "Chroma Format:", chroma.to_string());
            }
        });
    });
}

pub fn build_audio_stream_descriptor(ui: &mut egui::Ui, desc: &AudioStreamDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "Audio Stream:", "");
        ui.indent("audio_indent", |ui| {
            build_label(ui, "Layer:", desc.layer.to_string());
            build_label(
                ui,
                "Variable Rate:",
                desc.variable_rate_audio_indicator.to_string(),
            );
        });
    });
}

pub fn build_maximum_bitrate_descriptor(ui: &mut egui::Ui, desc: &MaximumBitrateDescriptor) {
    ui.vertical(|ui| {
        build_label(
            ui,
            "Maximum Bitrate:",
            format!("{} kbps", desc.maximum_bitrate * 50),
        );
    });
}

pub fn build_multiplex_buffer_descriptor(
    ui: &mut egui::Ui,
    desc: &MultiplexBufferUtilizationDescriptor,
) {
    ui.vertical(|ui| {
        build_label(ui, "Buffer Utilization:", "");
        ui.indent("buffer_indent", |ui| {
            if let Some(lower) = desc.ltw_offset_lower_bound {
                build_label(ui, "Lower Bound:", lower.to_string());
            }
            if let Some(upper) = desc.ltw_offset_upper_bound {
                build_label(ui, "Upper Bound:", upper.to_string());
            }
        });
    });
}

pub fn build_system_clock_descriptor(ui: &mut egui::Ui, desc: &SystemClockDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "System Clock:", "");
        ui.indent("clock_indent", |ui| {
            build_label(
                ui,
                "External Clock:",
                desc.external_clock_reference_indicator.to_string(),
            );
            build_label(
                ui,
                "Accuracy:",
                format!(
                    "{}/{}",
                    desc.clock_accuracy_integer, desc.clock_accuracy_exponent
                ),
            );
        });
    });
}

pub fn build_video_window_descriptor(ui: &mut egui::Ui, desc: &VideoWindowDescriptor) {
    ui.vertical(|ui| {
        build_label(ui, "Video Window:", "");
        ui.indent("window_indent", |ui| {
            build_label(
                ui,
                "Offset:",
                format!("({}, {})", desc.horizontal_offset, desc.vertical_offset),
            );
            build_label(ui, "Priority:", desc.window_priority.to_string());
        });
    });
}

pub fn show_descriptor_modal(ctx: &egui::Context, descriptor: &Descriptors, open: &mut bool) {
    egui::Window::new("Descriptor Details")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            match descriptor {
                Descriptors::AvcVideoDescriptor(desc) => build_avc_video_descriptor(ui, desc),
                Descriptors::CopyrightDescriptor(desc) => build_copyright_descriptor(ui, desc),
                Descriptors::Iso639LanguageDescriptor(desc) => {
                    build_iso639_language_descriptor(ui, desc)
                }
                Descriptors::VideoStreamDescriptor(desc) => build_video_stream_descriptor(ui, desc),
                Descriptors::AudioStreamDescriptor(desc) => build_audio_stream_descriptor(ui, desc),
                Descriptors::MaximumBitrateDescriptor(desc) => {
                    build_maximum_bitrate_descriptor(ui, desc)
                }
                Descriptors::MultiplexBufferUtilizationDescriptor(desc) => {
                    build_multiplex_buffer_descriptor(ui, desc)
                }
                Descriptors::SystemClockDescriptor(desc) => build_system_clock_descriptor(ui, desc),
                Descriptors::VideoWindowDescriptor(desc) => build_video_window_descriptor(ui, desc),
                _ => return,
            }

            ui.add_space(8.0);
            if ui.button("Close").clicked() {
                *open = false;
            }
        });
}
