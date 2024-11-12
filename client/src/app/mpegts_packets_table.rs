use crate::app::is_stream_visible;
use crate::streams::{RefStreams, Streams};
use eframe::epaint::Color32;
use egui::RichText;
use egui_extras::{Column, TableBody, TableBuilder};
use rtpeeker_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use rtpeeker_common::StreamKey;
use std::collections::HashMap;
use web_time::{Duration, Instant};

#[derive(Clone)]
pub struct MpegTsPacketsTable {
    streams: RefStreams,
    streams_visibility: HashMap<StreamKey, bool>,
}

impl MpegTsPacketsTable {
    pub fn new(streams: RefStreams) -> Self {
        Self {
            streams,
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
                let mut selected = is_stream_visible(&mut self.streams_visibility, *key);
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
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(40.0))
            .column(Column::remainder().at_least(80.0))
            .columns(Column::remainder().at_least(100.0), 2)
            .column(Column::remainder().at_most(50.0))
            .columns(Column::remainder().at_least(140.0), 7)
            .column(Column::remainder().at_least(80.0))
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

        let mpegts_packets: Vec<_> = streams
            .mpeg_ts_streams
            .iter()
            .flat_map(|(_, stream)| {
                let transport_stream_id = stream.transport_stream_id;
                stream.mpegts_stream_info.packets.iter().map(move |packet| {
                    let key = (
                        packet.source_addr,
                        packet.destination_addr,
                        packet.protocol,
                        transport_stream_id,
                    );
                    (packet, key)
                })
            })
            .filter(|(_, key)| *is_stream_visible(&mut self.streams_visibility, *key))
            .map(|(packet, _)| packet)
            .collect();

        if mpegts_packets.is_empty() {
            return;
        }

        let mut pmt_pids: Vec<PIDTable> = vec![];
        let mut es_pids: Vec<PIDTable> = vec![];
        let mut pcr_pids: Vec<PIDTable> = vec![];
        let mut transport_stream_id: u32 = 0;

        let mut alias_to_display: HashMap<StreamKey, String> = HashMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            alias_to_display.insert(*key, stream.alias.to_string());
            transport_stream_id = stream.transport_stream_id;
            let pat = stream.mpegts_stream_info.pat.clone();
            if pat.is_some() {
                let pat = pat.unwrap();
                //TODO: add to alias pat.transport_stream_id and program.program_number
                pat.programs.iter().for_each(|program| {
                    if program.program_map_pid.is_none() {
                        return;
                    }
                    pmt_pids.push(program.program_map_pid.unwrap().into());
                });

                let pmt = stream.mpegts_stream_info.pmt.clone();
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
                    pcr_pids.push(single_pmt.unwrap().fields.pcr_pid.into());
                });
            }
        });

        let first_ts = mpegts_packets
            .first()
            .map(|p| p.time)
            .unwrap_or(Duration::ZERO);

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
                let timestamp = mpegts_packet.time.saturating_sub(first_ts);
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
                        let is_pmt = pmt_pids.contains(&PIDTable::PID(pid));
                        let is_es = es_pids.contains(&PIDTable::PID(pid));
                        let is_pcr = pcr_pids.contains(&PIDTable::PID(pid));

                        match (is_pmt, is_es, is_pcr) {
                            (true, _, _) => format!("Program Map Table ({})", pid),
                            (_, true, true) => format!("PCR+ES ({})", pid),
                            (_, true, false) => format!("Elementary Stream ({})", pid),
                            (_, false, true) => format!("PCR Table ({})", pid),
                            _ => format!("PID ({})", pid),
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
                let payload_size: usize = mpegts_packet
                    .content
                    .fragments
                    .iter()
                    .map(|fragment| {
                        if fragment.header.adaptation_field_control
                            == AdaptationFieldControl::AdaptationFieldOnly
                        {
                            return 0;
                        }
                        fragment.clone().payload.unwrap().data.len()
                    })
                    .sum();
                ui.label(payload_size.to_string());
            });
        });
    }
}

fn format_text(value: String) -> RichText {
    if value.contains("Program Association Table") {
        RichText::from(value).color(Color32::GREEN)
    } else if value.contains("Program Map Table") {
        RichText::from(value).color(Color32::LIGHT_BLUE)
    } else if value.contains("PCR Table") && value.contains("Elementary Stream") {
        RichText::from(format!("{} (PCR+ES)", value))
    } else if value.contains("PCR Table") {
        RichText::from(value)
    } else if value.contains("Elementary Stream") {
        RichText::from(value)
    } else {
        RichText::from(value)
    }
}
