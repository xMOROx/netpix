use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use super::is_mpegts_stream_visible;
use crate::streams::RefStreams;
use egui_extras::{Column, TableBody, TableBuilder};
use ewebsock::WsSender;
use std::net::SocketAddr;
use std::time::Duration;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::mpegts::psi::pat::pat_buffer::PatBuffer;
use netpix_common::mpegts::psi::pmt::pmt_buffer::PmtBuffer;
use netpix_common::MpegtsStreamKey;

enum MpegTsInfo {
    PatBuffer(* const PatBuffer),
    PmtBuffer(* const PmtBuffer),
}
struct MpegTsInfoRow {
    source_addr: SocketAddr,
    destination_addr: SocketAddr,
    time: Duration,
    info: MpegTsInfo,
    counter: usize,
}

#[derive(Hash, Eq, PartialEq, Ord)]
struct RowKey {
    pid: PIDTable,
    alias: String,
}

// impl Ord for RowKey {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if !self.alias.eq(&other.alias) {
//             return self.alias.cmp(&other.alias);
//         }
//         self.pid.cmp(&other.pid)
//     }
// }

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
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            ("Type", "Type of mpegts packet"),
            ("Duplications", "Number of duplicated packets"),
            ("Packet count", "Number of packets in mpegts packet"),
            ("Addition information", "Additional information"),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(80.0))
            .columns(Column::remainder().at_least(130.0), 2)
            .columns(Column::remainder().at_least(40.0), 3)
            .column(Column::remainder().at_least(800.0))
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

        let mut mpegts_rows: BTreeMap<RowKey, MpegTsInfoRow> = BTreeMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            let aggregator = &stream.aggregator;
            stream.stream_info.packets.iter().for_each(|packet| {
                packet.content.fragments.iter().for_each(|fragment| {
                    let header = &fragment.header;
                    // TODO: handle multiple streams
                    if let PIDTable::ProgramAssociation = header.pid {
                        if aggregator.is_pat_complete() {
                            let info = MpegTsInfo::PatBuffer(&aggregator.pat_buffer);
                            let key = RowKey {pid: header.pid, alias: stream.alias.clone()};
                            if mpegts_rows.contains_key(&key) {
                                let val = mpegts_rows.get_mut(&key).unwrap();
                                val.counter += 1;
                                val.time = packet.time;
                            } else {
                                mpegts_rows.insert(
                                    key,
                                    MpegTsInfoRow {
                                        info,
                                        counter: 0,
                                        source_addr: packet.packet_association_table.source_addr,
                                        destination_addr: packet.packet_association_table.destination_addr,
                                        time: packet.time
                                    });
                            }
                        }
                    } else if let PIDTable::PID(pid) = header.pid {
                        if aggregator.is_pmt_complete(pid) {
                            let info = MpegTsInfo::PmtBuffer(
                                aggregator.pmt_buffers.get(&pid).unwrap()
                            );
                            let key = RowKey {pid: header.pid, alias: stream.alias.clone()};
                            if mpegts_rows.contains_key(&key) {
                                let val = mpegts_rows.get_mut(&key).unwrap();
                                val.counter += 1;
                                val.time = packet.time;
                            } else {
                                mpegts_rows.insert(
                                    key,
                                    MpegTsInfoRow {
                                        info,
                                        counter: 0,
                                        source_addr: packet.packet_association_table.source_addr,
                                        destination_addr: packet.packet_association_table.destination_addr,
                                        time: packet.time
                                    });
                            }
                        }
                    }
                })
            })
        });
        let keys = mpegts_rows.keys().collect::<Vec<_>>();
        body.rows(25.0, mpegts_rows.len(), |row_ix, mut row| {
            let key = keys.get(row_ix).unwrap();
            let mpegts_row = mpegts_rows.get(key).unwrap();
            let mpegts_info = &mpegts_row.info;

            row.col(|ui| {
                ui.label(&key.alias);
            });
            row.col(|ui| {
                let timestamp = mpegts_row.time;
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });
            row.col(|ui| {
                ui.label(mpegts_row.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(mpegts_row.destination_addr.to_string());
            });
            row.col(|ui| {
                ui.label(key.pid.to_string());
            });
            row.col(|ui| {
                ui.label(&mpegts_row.counter.to_string());
            });
            row.col(|ui| {
                ui.label("Placeholder");
            });
            row.col(|ui| {
                ui.label("Lorem ipsum dolor sit amet");
            });
        })
    }
}
