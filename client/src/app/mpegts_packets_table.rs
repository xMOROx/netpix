use crate::streams::RefStreams;
use eframe::epaint::Color32;
use egui::RichText;
use egui_extras::{TableBody, TableBuilder};
use rtpeeker_common::mpegts::header::PIDTable;
use rtpeeker_common::mpegts::FRAGMENT_SIZE;
use rtpeeker_common::StreamKey;
use std::collections::HashMap;

const ROWS_PER_PAGE: usize = 100;
#[derive(Clone)]
pub struct MpegTsPacketsTable {
    streams: RefStreams,
    streams_visibility: HashMap<StreamKey, bool>,
    cached_packets: Vec<CachedPacket>,
    scroll_offset: usize,
    needs_refresh: bool,
}

#[derive(Clone)]
struct CachedPacket {
    id: u64,
    time: f64,
    source_addr: String,
    dest_addr: String,
    alias: String,
    fragments: Vec<String>,
    payload_len: usize,
}

impl MpegTsPacketsTable {
    pub fn new(streams: RefStreams) -> Self {
        Self {
            streams,
            streams_visibility: HashMap::default(),
            cached_packets: Vec::new(),
            scroll_offset: 0,
            needs_refresh: true,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.needs_refresh {
            self.update_cache();
            self.needs_refresh = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);
        });
    }

    fn update_cache(&mut self) {
        let streams_visibility = &self.streams_visibility;
        let streams = self.streams.borrow();

        self.cached_packets = streams
            .mpeg_ts_streams
            .iter()
            .flat_map(|(_key, stream)| {
                let transport_stream_id = stream.transport_stream_id;

                stream.mpegts_info.packets.iter().filter_map(move |packet| {
                    let stream_key: StreamKey = (
                        packet.source_addr,
                        packet.destination_addr,
                        packet.protocol,
                        transport_stream_id,
                    );

                    if !is_stream_visible(streams_visibility, &stream_key) {
                        return None;
                    }

                    Some(CachedPacket {
                        id: packet.id as u64,
                        time: packet.time.as_secs_f64(),
                        source_addr: packet.source_addr.to_string(),
                        dest_addr: packet.destination_addr.to_string(),
                        alias: stream.alias.clone(),
                        fragments: packet
                            .content
                            .fragments
                            .iter()
                            .map(|f| format_fragment(&f.header.pid))
                            .collect(),
                        payload_len: packet.content.number_of_fragments * FRAGMENT_SIZE,
                    })
                })
            })
            .collect();
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
                let mut selected = is_stream_visible(&mut self.streams_visibility, key);
                ui.checkbox(&mut selected, alias);
            });
        });
        ui.vertical(|ui| {
            ui.add_space(5.0);
        });
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("No.", "Packet number (including skipped packets)"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            (
                "Alias",
                "Locally Assigned alias to make differentiating streams more convenient",
            ),
            ("P1", "Packet No. 1"),
            ("P2", "Packet No. 2"),
            ("P3", "Packet No. 3"),
            ("P4", "Packet No. 4"),
            ("P5", "Packet No. 5"),
            ("P6", "Packet No. 6"),
            ("P7", "Packet No. 7"),
            ("Payload Length", "Mpegts payload length"),
        ];

        let total_rows = self.cached_packets.len();
        let visible_rows = ROWS_PER_PAGE.min(total_rows);
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .body(|body| {
                let start = self.scroll_offset;
                let end = (start + visible_rows).min(total_rows);

                body.rows(25.0, end - start, |row_idx, mut row| {
                    let packet = &self.cached_packets[start + row_idx];

                    row.col(|ui| {
                        ui.label(packet.id.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("{:.4}", packet.time));
                    });
                    row.col(|ui| {
                        ui.label(&packet.source_addr);
                    });
                    row.col(|ui| {
                        ui.label(&packet.dest_addr);
                    });
                    row.col(|ui| {
                        ui.label(&packet.alias);
                    });

                    for fragment in &packet.fragments {
                        row.col(|ui| {
                            ui.label(format_colored_text(fragment));
                        });
                    }

                    row.col(|ui| {
                        ui.label(packet.payload_len.to_string());
                    });
                });
            });

        if total_rows > ROWS_PER_PAGE {
            let max_scroll = total_rows - ROWS_PER_PAGE;
            ui.add(egui::Slider::new(&mut self.scroll_offset, 0..=max_scroll));
        }
    }

    fn build_table_body(&mut self, body: TableBody) {
        let streams = &self.streams.borrow();
        let mpegts_packets: Vec<_> = streams
            .mpeg_ts_streams
            .iter()
            .flat_map(|(_, stream)| {
                let transport_stream_id = stream.transport_stream_id;
                stream.mpegts_info.packets.iter().map(move |packet| {
                    let key:StreamKey = (
                        packet.source_addr,
                        packet.destination_addr,
                        packet.protocol,
                        transport_stream_id,
                    );
                    (packet, key)
                })
            })
            .filter(|(_, key)| is_stream_visible(&mut self.streams_visibility, key))
            .map(|(packet, _)| packet)
            .collect();

        if mpegts_packets.is_empty() {
            return;
        }

        let mut pmt_pids: Vec<PIDTable> = vec![];
        let mut es_pids: Vec<PIDTable> = vec![];
        let mut transport_stream_id: u32 = 0;

        let mut alias_to_display: HashMap<StreamKey, String> = HashMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            alias_to_display.insert(*key, stream.alias.to_string());
            transport_stream_id = stream.transport_stream_id;
            let pat = stream.mpegts_info.pat.clone();
            if pat.is_some() {
                let pat = pat.unwrap();
                //TODO: add to alias pat.transport_stream_id and program.program_number
                pat.programs.iter().for_each(|program| {
                    if program.program_map_pid.is_none() {
                        return;
                    }
                    pmt_pids.push(program.program_map_pid.unwrap().into());
                });

                let pmt = stream.mpegts_info.pmt.clone();
                pmt_pids.iter().for_each(|pmt_pid| {
                    let single_pmt = pmt.get(pmt_pid);
                    if single_pmt.is_none() {
                        return;
                    }
                    single_pmt
                        .unwrap()
                        .elementary_streams_info
                        .iter()
                        .for_each(|es| {
                            es_pids.push(es.elementary_pid.into());
                        });
                });
            }
        });

        let first_ts = mpegts_packets.first().unwrap().time;

        body.rows(25.0, mpegts_packets.len(), |row_ix, mut row| {
            let mpegts_packet = mpegts_packets.get(row_ix).unwrap();

            let key = (
                mpegts_packet.source_addr,
                mpegts_packet.destination_addr,
                mpegts_packet.protocol,
                transport_stream_id,
            );

            row.col(|ui| {
                ui.label(mpegts_packet.id.to_string());
            });
            row.col(|ui| {
                let timestamp = mpegts_packet.time - first_ts;
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });
            row.col(|ui| {
                ui.label(mpegts_packet.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(mpegts_packet.destination_addr.to_string());
            });
            row.col(|ui| {
                ui.label(alias_to_display.get(&key).unwrap().to_string());
            });

            let mut labels = mpegts_packet
                .content
                .fragments
                .iter()
                .map(|fragment| match fragment.header.pid {
                    PIDTable::ProgramAssociation => String::from("Program Association Table"),
                    PIDTable::ConditionalAccess => String::from("ConditionalAccess"),
                    PIDTable::TransportStreamDescription => {
                        String::from("TransportStreamDescription")
                    }
                    PIDTable::AdaptiveStreamingInformation => {
                        String::from("AdaptiveStreamingInformation")
                    }
                    PIDTable::NullPacket => String::from("NullPacket"),
                    PIDTable::PID(pid) => {
                        if pmt_pids.contains(&PIDTable::PID(pid)) {
                            format!("Program Map Table ({})", pid)
                        } else if es_pids.contains(&PIDTable::PID(pid)) {
                            format!("Elementary Stream ({})", pid)
                        } else {
                            format!("PID ({})", pid)
                        }
                    }
                    PIDTable::IPMPControlInformation => String::from("IPMPControlInformation"),
                })
                .into_iter();

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label(format_text(labels.next().unwrap_or_default()));
            });

            row.col(|ui| {
                ui.label((mpegts_packet.content.number_of_fragments * FRAGMENT_SIZE).to_string());
            });
        });
    }
}

fn format_fragment(pid: &PIDTable) -> String {
    match pid {
        PIDTable::ProgramAssociation => "PAT".to_string(),
        PIDTable::PID(pid) => format!("PID {}", pid),
        _ => pid.to_string(),
    }
}

fn format_colored_text(text: &str) -> RichText {
    if text.contains("PAT") {
        RichText::new(text).color(Color32::GREEN)
    } else if text.contains("PMT") {
        RichText::new(text).color(Color32::LIGHT_BLUE)
    } else {
        RichText::new(text)
    }
}

fn format_text(value: String) -> RichText {
    if value.contains("Program Association Table") {
        RichText::new(value).color(Color32::GREEN)
    } else if value.contains("Program Map Table") {
        RichText::new(value).color(Color32::LIGHT_BLUE)
    } else {
        RichText::new(value)
    }
}

pub fn is_stream_visible(streams_visibility: &HashMap<StreamKey, bool>, key: &StreamKey) -> bool {
    *streams_visibility.get(key).unwrap_or(&true)
}
