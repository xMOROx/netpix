use super::table_body::build_table_body;
use super::types::{MpegTsInfo, OpenModal, RowKey, LINE_HEIGHT};
use crate::app::is_mpegts_stream_visible;
use crate::app::mpegts_info_table::descriptor_ui;
use crate::streams::RefStreams;
use egui_extras::{Column, TableBuilder};
use ewebsock::WsSender;
use netpix_common::mpegts::descriptors::Descriptors;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::MpegtsStreamKey;
use std::collections::{BTreeMap, HashMap};

pub struct MpegTsInformationTable {
    streams: RefStreams,
    ws_sender: WsSender,
    streams_visibility: HashMap<MpegtsStreamKey, bool>,
    open_modal: OpenModal,
}

impl MpegTsInformationTable {
    pub fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            ws_sender,
            streams_visibility: HashMap::default(),
            open_modal: OpenModal::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);
        });

        if self.open_modal.is_open {
            if let Some(descriptor) = &self.open_modal.descriptor.clone() {
                descriptor_ui::show_descriptor_modal(ctx, descriptor, &mut self.open_modal);
            }
        }
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("Stream alias", "Stream alias"),
            ("Type", "Type of mpegts packet"),
            ("Packet count", "Number of packets in mpegts packet"),
            ("Addition information", "Additional information"),
        ];

        let streams = &self.streams.borrow();
        let mut mpegts_rows: BTreeMap<RowKey, MpegTsInfo> = BTreeMap::default();

        streams.mpeg_ts_streams.iter().for_each(|(_key, stream)| {
            if let Some(pat) = &stream.stream_info.pat {
                let info = MpegTsInfo {
                    pat: Some(pat.clone()),
                    pmt: None,
                };
                let key = RowKey {
                    pid: PIDTable::ProgramAssociation,
                    alias: stream.alias.clone(),
                };
                mpegts_rows.insert(key, info);
            }

            stream.stream_info.pmt.iter().for_each(|(pid, pmt)| {
                let info = MpegTsInfo {
                    pat: None,
                    pmt: Some(pmt.clone()),
                };
                let key = RowKey {
                    pid: PIDTable::PID(u16::from(*pid)),
                    alias: stream.alias.clone(),
                };
                mpegts_rows.insert(key, info);
            });
        });

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(100.0).at_most(100.0))
            .column(Column::remainder().at_least(150.0).at_most(150.0))
            .column(Column::remainder().at_least(100.0).at_most(100.0))
            .column(Column::remainder().at_least(800.0).clip(true))
            .header(30.0, |mut header| {
                header_labels.iter().for_each(|(label, desc)| {
                    header.col(|ui| {
                        ui.heading(label.to_string())
                            .on_hover_text(desc.to_string());
                    });
                });
            })
            .body(|body| {
                build_table_body(body, &mpegts_rows, &mut self.open_modal);
            });
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let mut aliases = Vec::new();
        let streams = &self.streams.borrow().mpeg_ts_streams;
        let keys: Vec<_> = streams.keys().collect();

        keys.iter().for_each(|&key| {
            let alias = streams.get(key).unwrap().alias.to_string();
            aliases.push((*key, alias));
        });
        aliases.sort_by(|(_, a), (_, b)| a.cmp(b));

        ui.horizontal_wrapped(|ui| {
            ui.label("Filter by: ");
            aliases.iter().for_each(|(key, alias)| {
                let selected = is_mpegts_stream_visible(&mut self.streams_visibility, *key);
                ui.checkbox(selected, alias);
            });
        });
        ui.vertical(|ui| {
            ui.add_space(5.0);
        });
    }
}
