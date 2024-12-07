mod display;
mod filters;
mod types;

use crate::streams::RefStreams;
use display::{
    format_packet_text, ES_FORMAT, PAT_FORMAT, PCR_ES_FORMAT, PCR_FORMAT, PID_FORMAT, PMT_FORMAT,
};
use egui_extras::{Column, TableBody, TableBuilder};
use types::{PacketInfo, TableConfig};

use crate::app::is_mpegts_stream_visible;
use crate::app::mpegts_packets_table::filters::{parse_filter, FilterContext, PacketFilter};
use egui::{Color32, RichText, TextEdit};
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use netpix_common::mpegts::MpegtsFragment;
use netpix_common::MpegtsStreamKey;
use std::collections::HashMap;
use web_time::Duration;
use crate::filter_system::FilterExpression;

#[derive(Clone)]
pub struct MpegTsPacketsTable {
    streams: RefStreams,
    filter_buffer: String,
    config: TableConfig,
}

impl MpegTsPacketsTable {
    pub fn new(streams: RefStreams) -> Self {
        Self {
            streams,
            filter_buffer: String::new(),
            config: TableConfig::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("filter_bar").show(ctx, |ui| {
            self.build_filter(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.options_ui(ui);
            self.build_table(ui);
        });
    }

    fn build_filter(&mut self, ui: &mut egui::Ui) {
        let text_edit = TextEdit::singleline(&mut self.filter_buffer)
            .font(egui::style::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
            .hint_text(
                "Examples: ALIAS:A, DESC:123 AND TYPE:PAT, SOURCE:192.168 OR DEST:10.0, NOT PID:4096, (TYPE:PMT AND PAYLOAD:>1000)",
            );

        ui.horizontal(|ui| {
            ui.add(text_edit);
        });
    }

    fn packet_matches_filter(
        &self,
        info: &PacketInfo,
        pmt_pids: &[PIDTable],
        es_pids: &[PIDTable],
        pcr_pids: &[PIDTable],
    ) -> bool {
        if self.filter_buffer.is_empty() {
            return true;
        }

        let filter = self.filter_buffer.trim().to_lowercase();

        let streams = self.streams.borrow();
        let stream_alias = streams
            .mpeg_ts_streams
            .get(&info.key)
            .map(|s| s.alias.to_string());

        let ctx = FilterContext {
            packet: info.packet,
            pmt_pids,
            es_pids,
            pcr_pids,
            stream_alias,
        };

        if let Some(filter_type) = parse_filter(&filter) {
            filter_type.matches(&ctx)
        } else {
            true
        }
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        let streams = &self.streams.borrow().mpeg_ts_streams;
        let mut aliases = Vec::with_capacity(streams.len());

        for (&key, stream) in streams.iter() {
            aliases.push((key, stream.alias.to_string()));
        }
        aliases.sort_by(|(_, a), (_, b)| a.cmp(b));

        ui.horizontal_wrapped(|ui| {
            ui.label("Aliases: ");
            for (_, alias) in &aliases {
                ui.label(alias);
            }
        });

        ui.add_space(self.config.space_after_filter);
    }

    fn build_table(&mut self, ui: &mut egui::Ui) {
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .column(Column::initial(40.0).at_least(40.0).at_most(50.0))
            .column(Column::initial(40.0).at_least(40.0).at_most(50.0))
            .column(Column::initial(80.0).at_least(80.0))
            .columns(Column::initial(140.0).at_least(140.0).at_most(155.0), 2)
            .columns(Column::initial(160.0).at_least(160.0), 7)
            .column(Column::remainder().at_least(80.0));

        self.build_table_with_config(table);
    }

    fn build_table_with_config(&mut self, table: TableBuilder) {
        table
            .header(self.config.header_height, |mut header| {
                header.col(|ui| {
                    ui.strong("ID");
                });
                header.col(|ui| {
                    ui.strong("Alias");
                });
                header.col(|ui| {
                    ui.strong("Time");
                });
                header.col(|ui| {
                    ui.strong("Source");
                });
                header.col(|ui| {
                    ui.strong("Destination");
                });
                header.col(|ui| {
                    ui.strong("PID 1");
                });
                header.col(|ui| {
                    ui.strong("PID 2");
                });
                header.col(|ui| {
                    ui.strong("PID 3");
                });
                header.col(|ui| {
                    ui.strong("PID 4");
                });
                header.col(|ui| {
                    ui.strong("PID 5");
                });
                header.col(|ui| {
                    ui.strong("PID 6");
                });
                header.col(|ui| {
                    ui.strong("PID 7");
                });
                header.col(|ui| {
                    ui.strong("Payload Size");
                });
            })
            .body(|body| {
                self.build_table_body(body);
            });
    }

    fn build_table_body(&mut self, body: TableBody) {
        let streams = &self.streams.borrow();

        let mut alias_to_display: HashMap<MpegtsStreamKey, String> = HashMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            alias_to_display.insert(*key, stream.alias.to_string());
        });

        let mut mpegts_packets: Vec<_> = streams
            .mpeg_ts_streams
            .iter()
            .flat_map(|(key, stream)| {
                stream
                    .stream_info
                    .packets
                    .iter()
                    .map(move |packet| (key, packet))
            })
            .collect();

        mpegts_packets
            .sort_by(|(_key1, packet1), (_key2, packet2)| packet1.time.cmp(&packet2.time));

        let mpegts_packets: Vec<_> = mpegts_packets
            .into_iter()
            .map(|(_, packet)| packet)
            .collect();

        if mpegts_packets.is_empty() {
            return;
        }

        let mut pmt_pids: Vec<PIDTable> = vec![];
        let mut es_pids: Vec<PIDTable> = vec![];
        let mut pcr_pids: Vec<PIDTable> = vec![];

        let mut alias_to_display: HashMap<MpegtsStreamKey, String> = HashMap::default();
        streams.mpeg_ts_streams.iter().for_each(|(key, stream)| {
            alias_to_display.insert(*key, stream.alias.to_string());
            if let Some(pat) = &stream.stream_info.pat {
                pat.programs.iter().for_each(|program| {
                    if program.program_map_pid.is_none() {
                        return;
                    }
                    pmt_pids.push(program.program_map_pid.unwrap().into());
                });

                let pmt = stream.stream_info.pmt.clone();
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
                    packet.packet_association_table.source_addr,
                    packet.packet_association_table.destination_addr,
                    packet.packet_association_table.protocol,
                );
                PacketInfo {
                    packet,
                    timestamp,
                    key,
                }
            })
            .collect();

        let filtered_packets: Vec<_> = packets_with_info
            .into_iter()
            .filter(|info| self.packet_matches_filter(info, &pmt_pids, &es_pids, &pcr_pids))
            .collect();

        body.rows(
            self.config.row_height,
            filtered_packets.len(),
            |mut row| {
                let row_ix = row.index();
                let info = &filtered_packets[row_ix];

                row.col(|ui| {
                    ui.label(info.packet.id.to_string());
                });

                row.col(|ui| {
                    let binding = String::new();
                    let alias = alias_to_display.get(&info.key).unwrap_or(&binding);
                    ui.label(alias);
                });

                row.col(|ui| {
                    let timestamp = info.packet.time.saturating_sub(first_ts);
                    ui.label(format!("{:.4}", timestamp.as_secs_f64()));
                });
                row.col(|ui| {
                    ui.label(info.packet.packet_association_table.source_addr.to_string());
                });
                row.col(|ui| {
                    ui.label(
                        info.packet
                            .packet_association_table
                            .destination_addr
                            .to_string(),
                    );
                });

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
                    });

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
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(format_packet_text(
                        labels.next().unwrap_or_default(),
                        fragments_iter.next().copied(),
                    ));
                });

                row.col(|ui| {
                    ui.label(payload_size.to_string());
                });
            },
        );
    }
}
