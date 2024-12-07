use std::cmp::{max, Ordering};
use std::collections::{BTreeMap, HashMap};
use super::is_mpegts_stream_visible;
use crate::streams::RefStreams;
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::WsSender;
use std::net::SocketAddr;
use std::time::Duration;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::mpegts::psi::pat::pat_buffer::PatBuffer;
use netpix_common::mpegts::psi::pat::ProgramAssociationTable;
use netpix_common::mpegts::psi::pmt::pmt_buffer::PmtBuffer;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;
use netpix_common::mpegts::psi::psi_buffer::PsiBuffer;
use netpix_common::MpegtsStreamKey;

const LINE_HEIGHT: f32 = 20.0;

struct MpegTsInfo {
    pat: Option<ProgramAssociationTable>,
    pmt: Option<ProgramMapTable>,
}

#[derive(Hash, Eq, PartialEq, Ord, Clone)]
struct RowKey {
    pid: PIDTable,
    alias: String,
}

impl PartialOrd for RowKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.alias.eq(&other.alias) {
            return self.alias.partial_cmp(&other.alias);
        }
        self.pid.partial_cmp(&other.pid)
    }
}

pub struct MpegTsInformationTable {
    streams: RefStreams,
    ws_sender: WsSender,
    streams_visibility: HashMap<MpegtsStreamKey, bool>,
}

impl MpegTsInformationTable {
    pub fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            ws_sender,
            streams_visibility: HashMap::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);
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

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("Stream alias", "Stream alias"),
            ("Type", "Type of mpegts packet"),
            ("Packet count", "Number of packets in mpegts packet"),
            ("Addition information", "Additional information"),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .columns(Column::remainder().at_least(100.0).at_most(200.0), 3)
            .column(Column::remainder().at_least(1000.0))
            .header(30.0, |mut header| {
                header_labels.iter().for_each(|(label, desc)| {
                    header.col(|ui| {
                        ui.heading(label.to_string())
                            .on_hover_text(desc.to_string());
                    });
                });
            })
            .body(|body| {
                self.build_table_body(body);
            });
    }

    fn build_table_body(&mut self, body: TableBody) {
        let streams = &self.streams.borrow();
        let mut mpegts_rows: BTreeMap<RowKey, MpegTsInfo> = BTreeMap::default();
        let mut row_height: BTreeMap<RowKey, f32> = BTreeMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(_key, stream)| {
            if let Some(pat) = &stream.stream_info.pat {
                let info = MpegTsInfo { pat: Some(pat.clone()), pmt: None };
                let key = RowKey { pid: PIDTable::ProgramAssociation, alias: stream.alias.clone() };
                mpegts_rows.insert(key.clone(), info);
                row_height.insert(key, pat.programs.len() as f32 * LINE_HEIGHT);
            }
            stream.stream_info.pmt.iter().for_each(|(key, pmt)| {
                let info = MpegTsInfo { pat: None, pmt: Some(pmt.clone()) };
                let key = RowKey { pid: key.clone(), alias: stream.alias.clone() };
                mpegts_rows.insert(key.clone(), info);
                let mut counter: usize = 0;
                pmt.elementary_streams_info.iter().for_each(|e|{
                    counter += 3;
                    counter += e.descriptors.len()
                });
                row_height.insert(key, counter as f32 * LINE_HEIGHT);
            });
        });

        let keys = mpegts_rows.keys().collect::<Vec<_>>();
        body.heterogeneous_rows(row_height.values().map(|height| *height).into_iter(), |mut row| {
            let key = keys.get(row.index()).unwrap();
            let info = mpegts_rows.get(key).unwrap();

            row.col(|ui| {
                ui.label(&key.alias);
            });
            row.col(|ui| {
                ui.label(key.pid.to_string());
            });
            row.col(|ui| {
                if let Some(pat) = &info.pat {
                    ui.label(pat.fragment_count.to_string());
                } else if let Some(pmt) = &info.pmt {
                    ui.label(pmt.fragment_count.to_string());
                }
            });
            row.col(|ui| {
                if let Some(pat) = &info.pat {
                    let mut programs = String::new();
                    pat.programs.iter().for_each(|program| {
                        programs += format!("Program number: {} ", program.program_number).as_str();
                        if let Some(network_pid) = program.network_pid {
                            programs += format!("Network PID: {}\n", network_pid).as_str();
                        } else if let Some(program_map_pid) = program.program_map_pid {
                            programs += format!("Program map PID: {}\n", program_map_pid).as_str();
                        }
                    });
                    ui.label(programs);
                } else if let Some(pmt) = &info.pmt {
                    if pmt.elementary_streams_info.len() > 0 {
                        let mut streams_info = String::from("Elementary stream info:\n");
                        pmt.elementary_streams_info.iter().for_each(|stream_info| {
                            streams_info += format!("\tStream type: {}\n", stream_info.stream_type).as_str();
                            streams_info += format!("\tElementary PID: {}\n", PIDTable::from(stream_info.elementary_pid)).as_str();
                            if stream_info.descriptors.len() > 0 {
                                streams_info += "\tDescriptors:\n";
                                stream_info.descriptors.iter().for_each(|desc| {
                                    streams_info += format!("\t\t{}\n", desc).as_str();
                                });
                            }
                        });
                        ui.label(streams_info);
                    }
                }
            });
        })
    }
}
