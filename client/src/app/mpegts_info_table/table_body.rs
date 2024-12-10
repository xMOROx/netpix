use super::constants::*;
use super::types::{MpegTsInfo, OpenModal, RowKey, LINE_HEIGHT};
use egui_extras::TableBody;
use netpix_common::mpegts::descriptors::Descriptors;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::mpegts::psi::pat::ProgramAssociationTable;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;
use std::collections::BTreeMap;

fn format_pat_header(program_number: u16) -> egui::RichText {
    egui::RichText::new(format!("Program #{}", program_number)).strong()
}

fn format_pat_info(pat: &ProgramAssociationTable) -> Vec<(u16, String)> {
    pat.programs
        .iter()
        .map(|program| {
            let pid_info = if let Some(network_pid) = program.network_pid {
                format!("Network PID: {}", network_pid)
            } else if let Some(program_map_pid) = program.program_map_pid {
                format!("Program map PID: {}", program_map_pid)
            } else {
                String::new()
            };
            (program.program_number, pid_info)
        })
        .collect()
}

fn format_stream_header(pid: u16, _stream_type: String) -> egui::RichText {
    egui::RichText::new(format!("Stream {}:", pid)).strong()
}

fn get_descriptor_button_info(descriptor: &Descriptors) -> Option<&'static str> {
    match descriptor {
        Descriptors::AvcVideoDescriptor(_) => Some("AVC Video"),
        Descriptors::CopyrightDescriptor(_) => Some("Copyright"),
        Descriptors::Iso639LanguageDescriptor(_) => Some("Language"),
        Descriptors::VideoStreamDescriptor(_) => Some("Video Stream"),
        Descriptors::AudioStreamDescriptor(_) => Some("Audio Stream"),
        Descriptors::MaximumBitrateDescriptor(_) => Some("Max Bitrate"),
        Descriptors::MultiplexBufferUtilizationDescriptor(_) => Some("Buffer Util"),
        Descriptors::SystemClockDescriptor(_) => Some("System Clock"),
        Descriptors::VideoWindowDescriptor(_) => Some("Video Window"),
        Descriptors::CaDescriptor(_) => Some("CA"),
        Descriptors::DataStreamAlignmentDescriptor(_) => Some("Alignment"),
        Descriptors::HierarchyDescriptor(_) => Some("Hierarchy"),
        Descriptors::PrivateDataIndicatorDescriptor(_) => Some("Private Data"),
        Descriptors::RegistrationDescriptor(_) => Some("Registration"),
        Descriptors::StdDescriptor(_) => Some("STD"),
        Descriptors::TargetBackgroundGridDescriptor(_) => Some("Grid"),
        Descriptors::UserPrivate(_) => Some("User Private"),
        Descriptors::Unknown => None,
    }
}

fn calculate_pmt_row_height(pmt: &ProgramMapTable, available_width: f32) -> f32 {
    let groups_per_row = (available_width / (ESTIMATED_GROUP_WIDTH + 4.0))
        .floor()
        .max(1.0);
    let rows = (pmt.elementary_streams_info.len() as f32 / groups_per_row).ceil();
    rows * GROUP_HEIGHT + (rows - 1.0).max(0.0) * ROW_SPACING + VERTICAL_PADDING
}

pub fn build_table_body(
    body: TableBody,
    mpegts_rows: &BTreeMap<RowKey, MpegTsInfo>,
    open_modal: &mut OpenModal,
) {
    let keys = mpegts_rows.keys().collect::<Vec<_>>();
    let row_height: BTreeMap<RowKey, f32> = mpegts_rows
        .iter()
        .map(|(key, info)| {
            let height = match &info.pat {
                Some(pat) => pat.programs.len() as f32 * LINE_HEIGHT,
                None => match &info.pmt {
                    Some(pmt) => calculate_pmt_row_height(pmt, 1100.0),
                    None => 0.0,
                },
            };
            (key.clone(), height)
        })
        .collect();

    body.heterogeneous_rows(
        row_height.values().map(|height| *height).into_iter(),
        |mut row| {
            let key = keys.get(row.index()).unwrap();
            let info = mpegts_rows.get(key).unwrap();

            row.col(|ui| {
                let mut binding = key.alias.clone();
                let text_edit = egui::TextEdit::singleline(&mut binding).frame(false);
                ui.add(text_edit);
            });
            row.col(|ui| {
                let label = match key.pid {
                    PIDTable::ProgramAssociation => key.pid.to_string(),
                    PIDTable::PID(pid) => format!("Program map ({})", pid),
                    _ => String::default(),
                };
                ui.label(label);
            });
            row.col(|ui| {
                if let Some(pat) = &info.pat {
                    ui.label(pat.fragment_count.to_string());
                } else if let Some(pmt) = &info.pmt {
                    ui.label(pmt.fragment_count.to_string());
                }
            });
            row.col(|ui| {
                if let Some(pmt) = &info.pmt {
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        ui.horizontal_wrapped(|ui| {
                            let available_width = ui.available_width();
                            let max_groups_per_row =
                                (available_width / (ESTIMATED_GROUP_WIDTH + 4.0))
                                    .floor()
                                    .max(1.0) as usize;

                            let mut current_group = 0;
                            let mut is_first_row = true;
                            for stream_info in &pmt.elementary_streams_info {
                                if current_group >= max_groups_per_row {
                                    current_group = 0;
                                    ui.end_row();
                                    if !is_first_row {
                                        ui.add_space(16.0); // Add space between rows
                                    }
                                    is_first_row = false;
                                }

                                ui.group(|ui| {
                                    ui.set_width(ESTIMATED_GROUP_WIDTH);
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format_stream_header(
                                                stream_info.elementary_pid,
                                                stream_info.stream_type.to_string(),
                                            ));
                                            ui.label(stream_info.stream_type.to_string());
                                        });

                                        if !stream_info.descriptors.is_empty() {
                                            ui.horizontal_wrapped(|ui| {
                                                ui.label(
                                                    egui::RichText::new("Descriptors:").strong(),
                                                );
                                                ui.spacing_mut().item_spacing.x = 2.0;
                                                for (idx, descriptor) in
                                                    stream_info.descriptors.iter().enumerate()
                                                {
                                                    if let Some(button_text) =
                                                        get_descriptor_button_info(descriptor)
                                                    {
                                                        let mut button =
                                                            egui::Button::new(button_text);
                                                        if open_modal
                                                            .active_descriptor
                                                            .as_ref()
                                                            .map(|(i, d)| (*i, d))
                                                            == Some((idx, descriptor))
                                                        {
                                                            button = button.fill(
                                                                ui.visuals().selection.bg_fill,
                                                            );
                                                        }
                                                        if ui.add(button).clicked() {
                                                            if open_modal
                                                                .active_descriptor
                                                                .as_ref()
                                                                .map(|(i, _)| *i)
                                                                == Some(idx)
                                                            {
                                                                open_modal.descriptor = None;
                                                                open_modal.active_descriptor = None;
                                                                open_modal.is_open = false;
                                                            } else {
                                                                open_modal.descriptor =
                                                                    Some((idx, descriptor.clone()));
                                                                open_modal.active_descriptor =
                                                                    Some((idx, descriptor.clone()));
                                                                open_modal.is_open = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    });
                                });
                                ui.add_space(4.0);
                                current_group += 1;
                            }
                        });
                        ui.add_space(8.0);
                    });
                } else if let Some(pat) = &info.pat {
                    ui.vertical(|ui| {
                        for (program_number, pid_info) in format_pat_info(pat) {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format_pat_header(program_number));
                                    ui.label(pid_info);
                                });
                            });
                        }
                    });
                }
            });
        },
    )
}

fn build_label(ui: &mut egui::Ui, label: String, value: String) {
    ui.label(format!("{} {}", label, value));
}
