use crate::app::mpegts_info_table::types::OpenModal;
use egui;
use netpix_common::mpegts::descriptors::audio_stream::AudioStreamDescriptor;
use netpix_common::mpegts::descriptors::avc_video::AvcVideoDescriptor;
use netpix_common::mpegts::descriptors::conditional_access::CaDescriptor;
use netpix_common::mpegts::descriptors::copyright::CopyrightDescriptor;
use netpix_common::mpegts::descriptors::data_stream_alignment::DataStreamAlignmentDescriptor;
use netpix_common::mpegts::descriptors::hierarchy::HierarchyDescriptor;
use netpix_common::mpegts::descriptors::ibp::IbpDescriptor;
use netpix_common::mpegts::descriptors::iso_639_language::Iso639LanguageDescriptor;
use netpix_common::mpegts::descriptors::maximum_bitrate::MaximumBitrateDescriptor;
use netpix_common::mpegts::descriptors::multiplex_buffer_utilization::MultiplexBufferUtilizationDescriptor;
use netpix_common::mpegts::descriptors::private_data_indicator::PrivateDataIndicatorDescriptor;
use netpix_common::mpegts::descriptors::registration::RegistrationDescriptor;
use netpix_common::mpegts::descriptors::smoothing_buffer::SmoothingBufferDescriptor;
use netpix_common::mpegts::descriptors::std_descriptor::StdDescriptor;
use netpix_common::mpegts::descriptors::system_clock::SystemClockDescriptor;
use netpix_common::mpegts::descriptors::target_background_grid::TargetBackgroundGridDescriptor;
use netpix_common::mpegts::descriptors::video_stream::VideoStreamDescriptor;
use netpix_common::mpegts::descriptors::video_window::VideoWindowDescriptor;
use netpix_common::mpegts::descriptors::*;
use std::collections::BTreeMap;

pub trait DescriptorDisplay {
    fn display_name(&self) -> &'static str;
    fn get_display_fields(&self) -> Vec<(&'static str, String)>;
}

fn build_descriptor_ui(ui: &mut egui::Ui, name: &str, fields: Vec<(&str, String)>) {
    ui.vertical(|ui| {
        build_label(ui, name, "");
        ui.indent("descriptor_indent", |ui| {
            for (label, value) in fields {
                build_label(ui, label, value);
            }
        });
    });
}

impl DescriptorDisplay for AvcVideoDescriptor {
    fn display_name(&self) -> &'static str {
        "AVC Video"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Profile", self.profile_idc.to_string()),
            ("Level", self.level_idc.to_string()),
            ("Still Present", self.avc_still_present.to_string()),
        ]
    }
}

impl DescriptorDisplay for AudioStreamDescriptor {
    fn display_name(&self) -> &'static str {
        "Audio Stream"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Layer", self.layer.to_string()),
            (
                "Variable Rate",
                self.variable_rate_audio_indicator.to_string(),
            ),
        ]
    }
}

impl DescriptorDisplay for CaDescriptor {
    fn display_name(&self) -> &'static str {
        "Conditional Access"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("System ID", self.ca_system_id.to_string()),
            ("PID", self.ca_pid.to_string()),
        ]
    }
}

impl DescriptorDisplay for CopyrightDescriptor {
    fn display_name(&self) -> &'static str {
        "Copyright"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        let mut fields = vec![("Identifier", format!("{:#x}", self.copyright_identifier))];
        if !self.additional_copyright_info.is_empty() {
            fields.push((
                "Additional Info",
                format!("{:?}", self.additional_copyright_info),
            ));
        }
        fields
    }
}

impl DescriptorDisplay for DataStreamAlignmentDescriptor {
    fn display_name(&self) -> &'static str {
        "Stream Alignment"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![("Type", self.alignment_type.to_string())]
    }
}

impl DescriptorDisplay for HierarchyDescriptor {
    fn display_name(&self) -> &'static str {
        "Hierarchy"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Type", self.hierarchy_type.to_string()),
            ("Layer Index", self.hierarchy_layer_index.to_string()),
            ("Channel", self.hierarchy_channel.to_string()),
            (
                "View Scalability",
                (!self.no_view_scalability_flag).to_string(),
            ),
            (
                "Temporal Scalability",
                (!self.no_temporal_scalability_flag).to_string(),
            ),
            (
                "Spatial Scalability",
                (!self.no_spatial_scalability_flag).to_string(),
            ),
            (
                "Quality Scalability",
                (!self.no_quality_scalability_flag).to_string(),
            ),
        ]
    }
}

impl DescriptorDisplay for Iso639LanguageDescriptor {
    fn display_name(&self) -> &'static str {
        "Language"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        self.section
            .iter()
            .map(|section| {
                (
                    "Language Code",
                    format!("{} ({})", section.language_code, section.audio_type),
                )
            })
            .collect()
    }
}

impl DescriptorDisplay for MaximumBitrateDescriptor {
    fn display_name(&self) -> &'static str {
        "Maximum Bitrate"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![("Rate", format!("{} kbps", self.maximum_bitrate * 50))]
    }
}

impl DescriptorDisplay for MultiplexBufferUtilizationDescriptor {
    fn display_name(&self) -> &'static str {
        "Buffer Utilization"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        let mut fields = vec![("Bound Valid", self.bound_valid_flag.to_string())];
        if let Some(lower) = self.ltw_offset_lower_bound {
            fields.push(("Lower Bound", lower.to_string()));
        }
        if let Some(upper) = self.ltw_offset_upper_bound {
            fields.push(("Upper Bound", upper.to_string()));
        }
        fields
    }
}

impl DescriptorDisplay for PrivateDataIndicatorDescriptor {
    fn display_name(&self) -> &'static str {
        "Private Data Indicator"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![("Value", self.private_data_indicator.to_string())]
    }
}

impl DescriptorDisplay for RegistrationDescriptor {
    fn display_name(&self) -> &'static str {
        "Registration"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![("Format ID", format!("{:#010X}", self.format_identifier))]
    }
}

impl DescriptorDisplay for StdDescriptor {
    fn display_name(&self) -> &'static str {
        "System Target Decoder"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![("Leak Valid", self.leak_valid_flag.to_string())]
    }
}

impl DescriptorDisplay for SystemClockDescriptor {
    fn display_name(&self) -> &'static str {
        "System Clock"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "External Clock",
                self.external_clock_reference_indicator.to_string(),
            ),
            (
                "Accuracy",
                format!(
                    "{}/{}",
                    self.clock_accuracy_integer, self.clock_accuracy_exponent
                ),
            ),
        ]
    }
}

impl DescriptorDisplay for TargetBackgroundGridDescriptor {
    fn display_name(&self) -> &'static str {
        "Background Grid"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "Grid Size",
                format!("{}x{}", self.horizontal_size, self.vertical_size),
            ),
            ("Aspect Ratio", self.aspect_ratio_information.to_string()),
        ]
    }
}

impl DescriptorDisplay for VideoStreamDescriptor {
    fn display_name(&self) -> &'static str {
        "Video Stream"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        let mut fields = vec![("Frame Rate", self.frame_rate_code.to_string())];
        if let Some(profile) = self.profile_and_level_indication {
            fields.push(("Profile Level", profile.to_string()));
        }
        if let Some(chroma) = self.chroma_format {
            fields.push(("Chroma Format", chroma.to_string()));
        }
        fields
    }
}

impl DescriptorDisplay for VideoWindowDescriptor {
    fn display_name(&self) -> &'static str {
        "Video Window"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "Offset",
                format!("({}, {})", self.horizontal_offset, self.vertical_offset),
            ),
            ("Priority", self.window_priority.to_string()),
        ]
    }
}

impl DescriptorDisplay for SmoothingBufferDescriptor {
    fn display_name(&self) -> &'static str {
        "Smoothing Buffer"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Leak Rate", format!("{} bits/s", self.sb_leak_rate * 400)),
            ("Buffer Size", self.sb_size.to_string()),
        ]
    }
}

impl DescriptorDisplay for IbpDescriptor {
    fn display_name(&self) -> &'static str {
        "IBP"
    }
    fn get_display_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("Closed GOP", self.closed_gop.to_string()),
            ("Identical GOP", self.identical_gop.to_string()),
            ("Max GOP Length", self.max_gop_length.to_string()),
        ]
    }
}

pub fn show_descriptor_modal(
    ctx: &egui::Context,
    descriptor: &(usize, Descriptors),
    modal: &mut OpenModal,
) {
    egui::Window::new("Descriptor Details")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            match &descriptor.1 {
                Descriptors::AvcVideoDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::AudioStreamDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::CaDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::CopyrightDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::DataStreamAlignmentDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::HierarchyDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::Iso639LanguageDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::MaximumBitrateDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::MultiplexBufferUtilizationDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::PrivateDataIndicatorDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::RegistrationDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::StdDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::SystemClockDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::TargetBackgroundGridDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::VideoStreamDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::VideoWindowDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::SmoothingBufferDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::IbpDescriptor(desc) => {
                    build_descriptor_ui(ui, desc.display_name(), desc.get_display_fields())
                }
                Descriptors::UserPrivate(tag) => {
                    build_descriptor_ui(ui, "User Private", vec![("Tag", format!("{:#04X}", tag))])
                }
                Descriptors::Unknown => return,
            }

            ui.add_space(8.0);
            if ui.button("Close").clicked() {
                modal.is_open = false;
                modal.descriptor = None;
                modal.active_descriptor = None;
            }
        });

    if !modal.is_open {
        modal.descriptor = None;
        modal.active_descriptor = None;
    }
}

pub fn build_label(ui: &mut egui::Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let label = egui::RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(normal.into());
    });
}
