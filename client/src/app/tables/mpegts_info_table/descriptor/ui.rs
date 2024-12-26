use super::display::DescriptorDisplay;
use crate::app::tables::mpegts_info_table::types::OpenModal;
use egui;
use netpix_common::mpegts::descriptors::*;

pub const DESCRIPTOR_SPACING: f32 = 8.0;
pub const DESCRIPTOR_INDENT: &str = "descriptor_indent";

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

pub fn build_descriptor_ui(ui: &mut egui::Ui, name: &str, fields: Vec<(&str, String)>) {
    ui.vertical(|ui| {
        build_label(ui, name, "");
        ui.indent("descriptor_indent", |ui| {
            for (label, value) in fields {
                build_label(ui, label, value);
            }
        });
    });
}

pub fn build_label(ui: &mut egui::Ui, bold: impl Into<String>, normal: impl Into<String>) {
    let label = egui::RichText::new(bold.into()).strong();
    ui.horizontal(|ui| {
        ui.label(label);
        ui.label(normal.into());
    });
}
