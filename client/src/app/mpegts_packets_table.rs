use crate::app::is_stream_visible;
use crate::streams::mpeg_ts_streams::MpegTsPacketInfo;
use crate::streams::RefStreams;
use egui::{Color32, RichText};
use egui_extras::{Column, TableBody, TableBuilder};
use rtpeeker_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use rtpeeker_common::mpegts::psi::PsiTypes::PMT;
use rtpeeker_common::mpegts::MpegtsFragment;
use rtpeeker_common::StreamKey;
use std::collections::HashMap;
use web_time::Duration;

const ROW_HEIGHT: f32 = 25.0;
const HEADER_HEIGHT: f32 = 30.0;
const SPACE_AFTER_FILTER: f32 = 5.0;

const PAT_FORMAT: &str = "PAT";
const PMT_FORMAT: &str = "PMT";
const PCR_ES_FORMAT: &str = "PCR+ES";
const ES_FORMAT: &str = "ES";
const PCR_FORMAT: &str = "PCR";
const PID_FORMAT: &str = "PID";

#[derive(Clone)]
pub struct MpegTsPacketsTable {
    streams: RefStreams,
    streams_visibility: HashMap<StreamKey, bool>,
}

#[derive(Clone)]
struct PacketInfo<'a> {
    packet: &'a MpegTsPacketInfo,
    timestamp: Duration,
    key: StreamKey,
}

impl MpegTsPacketsTable {
    pub fn new(streams: RefStreams) -> Self {
        let capacity = streams.borrow().mpeg_ts_streams.len();
        Self {
            streams,
            streams_visibility: HashMap::with_capacity(capacity),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);
        });
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let streams = &self.streams.borrow().mpeg_ts_streams;
        let mut aliases = Vec::with_capacity(streams.len());

        for (&key, stream) in streams.iter() {
            aliases.push((key, stream.alias.to_string()));
        }
        aliases.sort_by(|(_, a), (_, b)| a.cmp(b));

        ui.horizontal_wrapped(|ui| {
            ui.label("Filter by: ");
            for (key, alias) in &aliases {
                let mut selected = is_stream_visible(&mut self.streams_visibility, *key);
                ui.checkbox(&mut selected, alias);
            }
        });
        ui.add_space(SPACE_AFTER_FILTER);
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let header_labels = [
            ("No.", "Packet number (including skipped packets)"),
            ("Time", "Packet arrival timestamp"),
            ("Source", "Source IP address and port"),
            ("Destination", "Destination IP address and port"),
            // (
            //     "Alias",
            //     "Locally Assigned alias to make differentiating streams more convenient",
            // ),
            ("P1", "Packet No. 1"),
            ("P2", "Packet No. 2"),
            ("P3", "Packet No. 3"),
            ("P4", "Packet No. 4"),
            ("P5", "Packet No. 5"),
            ("P6", "Packet No. 6"),
            ("P7", "Packet No. 7"),
            (
                "Payload Length",
                "Mpegts payload length without header and adaptation data",
            ),
        ];
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::remainder().at_least(40.0))
            .column(Column::remainder().at_least(80.0))
            .columns(Column::remainder().at_least(100.0), 2)
            .columns(Column::remainder().at_least(160.0), 7)
            .column(Column::remainder().at_least(80.0))
            .header(HEADER_HEIGHT, |mut header| {
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
                stream.mpegts_stream_info.packets.iter().map(move |packet| {
                    let key = (
                        packet.source_addr,
                        packet.destination_addr,
                        packet.protocol,
                        0,
                    );
                    (packet, key)
                })
            })
            .filter_map(|(packet, key)| {
                if *is_stream_visible(&mut self.streams_visibility, key) {
                    Some(packet)
                } else {
                    None
                }
            })
            .collect();

        if mpegts_packets.is_empty() {
            return;
        }

        let mut pmt_pids: Vec<PIDTable> = vec![];
        let mut es_pids: Vec<PIDTable> = vec![];
        let mut pcr_pids: Vec<PIDTable> = vec![];

        let mut alias_to_display: HashMap<StreamKey, String> = HashMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            alias_to_display.insert(*key, stream.alias.to_string());
            if let Some(pat) = &stream.mpegts_stream_info.pat {
                pat.programs.iter().for_each(|program| {
                    if program.program_map_pid.is_none() {
                        return;
                    }
                    pmt_pids.push(program.program_map_pid.unwrap().into());
                });

                let pmt = stream.mpegts_stream_info.pmt.clone();
                pmt_pids.iter().for_each(|pmt_pid| {
                    if let Some(single_pmt) = pmt.get(pmt_pid) {
                        single_pmt.elementary_streams_info.iter().for_each(|es| {
                            es_pids.push(es.elementary_pid.into());
                        });
                        pcr_pids.push(single_pmt.fields.pcr_pid.into());
                    }
                });
            }
        });

        let first_ts = mpegts_packets
            .first()
            .map(|p| p.time)
            .unwrap_or(Duration::ZERO);

        let packets_with_info: Vec<PacketInfo> = mpegts_packets
            .iter()
            .map(|packet| {
                let timestamp = packet.time.saturating_sub(first_ts);
                let key = (
                    packet.source_addr,
                    packet.destination_addr,
                    packet.protocol,
                    0,
                );
                PacketInfo {
                    packet,
                    timestamp,
                    key,
                }
            })
            .collect();
        body.rows(ROW_HEIGHT, mpegts_packets.len(), |row_ix, mut row| {
            let info = &packets_with_info[row_ix];

            row.col(|ui| {
                ui.label(info.packet.id.to_string());
            });
            row.col(|ui| {
                let timestamp = info.packet.time.saturating_sub(first_ts);
                ui.label(format!("{:.4}", timestamp.as_secs_f64()));
            });
            row.col(|ui| {
                ui.label(info.packet.source_addr.to_string());
            });
            row.col(|ui| {
                ui.label(info.packet.destination_addr.to_string());
            });
            // row.col(|ui| {
            //     ui.label(
            //         alias_to_display
            //             .get(&info.key)
            //             .expect("Alias should exist for stream key")
            //             .to_string(),
            //     );
            // });

            let mut labels = info
                .packet
                .content
                .fragments
                .iter()
                .map(|fragment| match fragment.header.pid {
                    PIDTable::ProgramAssociation => String::from(PAT_FORMAT),
                    PIDTable::ConditionalAccess => String::from("CA"),
                    PIDTable::TransportStreamDescription => String::from("TSD"),
                    PIDTable::AdaptiveStreamingInformation => String::from("ASI"),
                    PIDTable::NullPacket => String::from("NullPacket"),
                    PIDTable::PID(pid) => {
                        let is_pmt = pmt_pids.contains(&PIDTable::PID(pid));
                        let is_es = es_pids.contains(&PIDTable::PID(pid));
                        let is_pcr = pcr_pids.contains(&PIDTable::PID(pid));

                        match (is_pmt, is_es, is_pcr) {
                            (true, _, _) => format!("{} ({})", PMT_FORMAT, pid),
                            (_, true, true) => format!("{} ({})", PCR_ES_FORMAT, pid),
                            (_, true, false) => format!("{} ({})", ES_FORMAT, pid),
                            (_, false, true) => format!("{} ({})", PCR_FORMAT, pid),
                            _ => format!("{} ({})", PID_FORMAT, pid),
                        }
                    }
                    PIDTable::IPMPControlInformation => String::from("IPMPControlInformation"),
                })
                .into_iter();

            let fragments: Vec<_> = info.packet.content.fragments.iter().collect();

            let payload_size: usize = fragments
                .iter()
                .map(|fragment| {
                    if fragment.header.adaptation_field_control
                        == AdaptationFieldControl::AdaptationFieldOnly
                    {
                        return 0;
                    }
                    fragment
                        .payload
                        .as_ref()
                        .map_or_else(|| 0, |payload| payload.data.len())
                })
                .sum();
            let mut fragments_iter = fragments.iter();

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(format_text(
                    labels.next().unwrap_or_default(),
                    fragments_iter.next().copied(),
                ));
            });

            row.col(|ui| {
                ui.label(payload_size.to_string());
            });
        });
    }
}

fn format_text(value: String, fragment: Option<&MpegtsFragment>) -> RichText {
    match value {
        s if s.contains(PAT_FORMAT) => RichText::from(s).color(Color32::GREEN),
        s if s.contains(PMT_FORMAT) => RichText::from(s).color(Color32::LIGHT_BLUE),
        s if s.contains(PCR_FORMAT) && s.contains(ES_FORMAT) => {
            if let Some(fragment) = fragment {
                RichText::from(format!(
                    "{}{}",
                    s,
                    get_star_according_payload_size(fragment)
                ))
            } else {
                RichText::from(format!("{}", s))
            }
        }
        s => RichText::from(s),
    }
}

fn get_star_according_payload_size(fragment: &MpegtsFragment) -> &str {
    match get_fragment_payload_size(fragment) {
        1..=183 => "*",
        _ => "",
    }
}

fn get_fragment_payload_size(fragment: &MpegtsFragment) -> usize {
    if fragment.header.adaptation_field_control == AdaptationFieldControl::AdaptationFieldOnly {
        return 0;
    }
    fragment
        .payload
        .as_ref()
        .map_or(0, |payload| payload.data.len())
}
