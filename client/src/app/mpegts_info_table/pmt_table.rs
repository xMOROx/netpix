use super::types::OpenModal;
use egui_extras::{Column, TableBody, TableBuilder};
use netpix_common::mpegts::descriptors::Descriptors;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;

pub fn build_pmt_table(ui: &mut egui::Ui, pmt: &ProgramMapTable, open_modal: &mut OpenModal) {
    let header_labels = ["Stream type", "Elementary PID", "Descriptors"];
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .columns(Column::remainder().at_least(100.0), 3)
        .header(20.0, |mut header| {
            header_labels.iter().for_each(|label| {
                header.col(|ui| {
                    ui.heading(label.to_string());
                });
            });
        })
        .body(|body| {
            build_pmt_table_body(body, pmt, open_modal);
        });
}

fn build_pmt_table_body(body: TableBody, pmt: &ProgramMapTable, open_modal: &mut OpenModal) {
    body.rows(20.0, pmt.elementary_streams_info.len(), |mut row| {
        let stream_info = pmt.elementary_streams_info.get(row.index()).unwrap();

        row.col(|ui| {
            ui.label(stream_info.stream_type.to_string());
        });
        row.col(|ui| {
            ui.label(stream_info.elementary_pid.to_string());
        });
        row.col(|ui| {
            if !stream_info.descriptors.is_empty() {
                ui.horizontal(|ui| {
                    for descriptor in &stream_info.descriptors {
                        let (button_text, tooltip) = match descriptor {
                            Descriptors::AvcVideoDescriptor(_) => {
                                ("AVC Video", "Show AVC video descriptor details")
                            }
                            Descriptors::CopyrightDescriptor(_) => {
                                ("Copyright", "Show copyright information")
                            }
                            Descriptors::Iso639LanguageDescriptor(_) => {
                                ("Language", "Show language information")
                            }
                            Descriptors::VideoStreamDescriptor(_) => {
                                ("Video Stream", "Show video stream details")
                            }
                            _ => continue,
                        };

                        let button = egui::Button::new(button_text);
                        if ui.add(button.small()).on_hover_text(tooltip).clicked() {
                            open_modal.descriptor = Some(descriptor.clone());
                        }
                    }
                });
            } else {
                ui.label("No descriptors");
            }
        });
    })
}
