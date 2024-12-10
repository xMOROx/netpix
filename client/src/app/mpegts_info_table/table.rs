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
use egui::Widget;
use crate::app::utils::{FilterHelpContent, FilterInput};

pub struct MpegTsInformationTable {
    streams: RefStreams,
    streams_visibility: HashMap<MpegtsStreamKey, bool>,
    open_modal: OpenModal,
    filter_input: FilterInput,
}

impl MpegTsInformationTable {
    pub fn new(streams: RefStreams) -> Self {
        let help = FilterHelpContent::builder("MPEG-TS Packet Filters")
            .filter("alias:<stream_alias>", "Filter by stream alias")
            .filter("pid:<number>", "Filter by PID value")
            .filter(
                "type:<value>",
                "Filter by packet type (PAT, PMT)",
            )
            .build();
        Self {
            streams,
            streams_visibility: HashMap::default(),
            open_modal: OpenModal::default(),
            filter_input: FilterInput::new(help),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.filter_input.show(ctx) {
            self.check_filter();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.build_table(ui);
        });

        if self.open_modal.is_open {
            if let Some(descriptor) = &self.open_modal.descriptor.clone() {
                descriptor_ui::show_descriptor_modal(ctx, descriptor, &mut self.open_modal);
            }
        }
    }

    fn check_filter(&mut self) {
        
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
}
