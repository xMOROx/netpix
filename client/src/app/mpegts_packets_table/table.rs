use super::constants::*;
use super::display::format_packet_text;
use super::filters::{parse_filter, FilterContext};
use super::types::PacketInfo;
use crate::app::common::{TableBase, TableConfig};
use crate::app::mpegts_packets_table::display::category_from_stream_type;
use crate::app::utils::{FilterHelpContent, FilterInput};
use crate::define_column;
use crate::filter_system::FilterExpression;
use crate::streams::{RefStreams, Streams};
use crate::{declare_table, declare_table_struct, impl_table_base};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use netpix_common::mpegts::psi::pmt::stream_types::get_stream_type_category;
use std::cell::Ref;
use std::collections::HashMap;
use web_time::Duration;

declare_table_struct!(MpegTsPacketsTable);

declare_table!(MpegTsPacketsTable, FilterType, {
    height(30.0);
    striped(true);
    resizable(true);
    stick_to_bottom(true);
    columns(
        column(Some(60.0), 60.0, Some(60.0), false, true),
        column(Some(40.0), 40.0, Some(50.0), false, true),
        column(Some(80.0), 80.0, Some(100.0), false, true),
        column(Some(80.0), 80.0, Some(80.0), false, true),
        column(Some(140.0), 140.0, None, false, true),
        column(Some(140.0), 140.0, None, false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(Some(140.0), 140.0, Some(140.0), false, true),
        column(None, 80.0, None, false, true),
    )
});

impl_table_base!(
    MpegTsPacketsTable,
    FilterHelpContent::builder("MPEG-TS Packet Filters")
            .filter("source:<ip>", "Filter by source IP address")
            .filter("dest:<ip>", "Filter by destination IP address")
            .filter("alias:<stream_alias>", "Filter by stream alias")
            .filter("pid:<number>", "Filter by PID value")
            .filter(
                "type:<value>",
                "Filter by packet type (PAT, PMT, ES, PCR, PCR+ES)",
            )
            .filter(
                "payload:<op><size>",
                "Filter by payload size (operators: <, <=, >, >=)",
            )
            .filter("program:<number>", "Filter by program number")
            .example("type:PAT AND payload:>1000")
            .example("source:192.168 OR dest:10.0")
            .example("alias:A AND type:PCR")
            .example("(type:Pbuild_table500) OR pid:256")
            .build()
    ;
    build_header: |self, header| {
        let headers = [
            "ID",
            "Alias",
            "Program Number",
            "Time",
            "Source",
            "Destination",
            "PID 1",
            "PID 2",
            "PID 3",
            "PID 4",
            "PID 5",
            "PID 6",
            "PID 7",
            "Payload Size",
        ];

        for header_text in headers {
            header.col(|ui| {
                ui.strong(header_text);
            });
        }
    }
    ;
    build_table_body: |self, body| {

        let filter_valid =
            self.filter_input.get_filter().is_empty() || self.filter_input.get_error().is_none();

        let streams = &self.streams.borrow();

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

        let first_ts = mpegts_packets
            .first()
            .map(|(_, p)| p.time)
            .unwrap_or(Duration::ZERO);

        let packets_info: Vec<PacketInfo> = mpegts_packets
            .iter()
            .map(|(&key, packet)| PacketInfo {
                packet,
                timestamp: packet.time.saturating_sub(first_ts),
                key,
            })
            .collect();

        let filtered_packets: Vec<_> = if filter_valid {
            packets_info
                .into_iter()
                .filter(|info| self.packet_matches_filter(info))
                .collect()
        } else {
            Vec::new()
        };

        body.rows(self.config.row_height, filtered_packets.len(), |mut row| {
            let info = &filtered_packets[row.index()];

            // ID column
            row.col(|ui| {
                ui.label(info.packet.id.to_string());
            });

            // Alias column
            row.col(|ui| {
                let stream = streams.mpeg_ts_streams.get(&info.key);
                if let Some(stream) = stream {
                    ui.label(&stream.alias);
                }
            });

            // Program Number column
            row.col(|ui| {
                let stream = streams.mpeg_ts_streams.get(&info.key);
                if let Some(stream) = stream {
                    if let Some(pat) = &stream.stream_info.pat {
                        let program_numbers: Vec<_> = pat.programs
                            .iter()
                            .map(|prog| prog.program_number.to_string())
                            .collect();
                        ui.label(program_numbers.join(", "));
                    }
                }
            });

            // Time column
            row.col(|ui| {
                ui.label(format!("{:.4}", info.timestamp.as_secs_f64()));
            });

            // Source/Destination columns
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

            // PID columns
            let mut fragments = info.packet.content.fragments.iter();
            let mut labels = fragments
                .clone()
                .map(|fragment| self.format_pid_label(fragment.header.pid, streams));

            // Display up to 7 PID columns
            for _ in 0..7 {
                row.col(|ui| {
                    if let Some(label) = labels.next() {
                        ui.label(format_packet_text(label, fragments.next()));
                    }
                });
            }

            // Payload size column
            row.col(|ui| {
                let payload_size: usize = info
                    .packet
                    .content
                    .fragments
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
                            .map_or(0, |payload| payload.data.len())
                    })
                    .sum();
                ui.label(payload_size.to_string());
            });
        });
    }
);

impl MpegTsPacketsTable {
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

    fn format_pid_label(&self, pid: PIDTable, streams: &Ref<Streams>) -> String {
        match pid {
            PIDTable::ProgramAssociation => PAT_FORMAT.to_string(),
            PIDTable::PID(pid) => {
                let is_pmt = streams.mpeg_ts_streams.values().any(|stream| {
                    stream.stream_info.pat.as_ref().map_or(false, |pat| {
                        pat.programs
                            .iter()
                            .any(|prog| prog.program_map_pid == Some(pid))
                    })
                });

                let is_pcr = streams.mpeg_ts_streams.values().any(|stream| {
                    stream
                        .stream_info
                        .pmt
                        .values()
                        .any(|pmt| pmt.fields.pcr_pid == pid)
                });

                let stream_type = streams.mpeg_ts_streams.values().find_map(|stream| {
                    stream.stream_info.pmt.values().find_map(|pmt| {
                        pmt.elementary_streams_info
                            .iter()
                            .find(|es| es.elementary_pid == pid)
                            .map(|es| es.stream_type.clone())
                    })
                });

                match (is_pmt, stream_type, is_pcr) {
                    (true, _, _) => format!("{} ({})", PMT_FORMAT, pid),
                    (_, Some(st), true) => {
                        let category = category_from_stream_type(get_stream_type_category(&st));
                        format!("{} ({}) {}", category, pid, PCR_ES_FORMAT)
                    }
                    (_, Some(st), false) => {
                        let category = category_from_stream_type(get_stream_type_category(&st));
                        format!("{} ({}) {}", category, pid, ES_FORMAT)
                    }
                    (_, None, true) => format!("{} ({})", PCR_FORMAT, pid),
                    _ => format!("{} ({})", PID_FORMAT, pid),
                }
            }
            _ => String::new(),
        }
    }

    fn packet_matches_filter(&self, info: &PacketInfo) -> bool {
        if self.filter_input.get_filter().is_empty() {
            return true;
        }

        let filter = self.filter_input.get_filter().trim().to_lowercase();
        let streams = self.streams.borrow();
        let stream = streams.mpeg_ts_streams.get(&info.key);

        // Look up program numbers for packet PIDs
        let program_numbers: Vec<u16> = if let Some(stream) = stream {
            if let Some(pat) = &stream.stream_info.pat {
                info.packet
                    .content
                    .fragments
                    .iter()
                    .filter_map(|fragment| {
                        if let PIDTable::PID(pid) = fragment.header.pid {
                            pat.programs
                                .iter()
                                .find(|prog| prog.program_map_pid == Some(pid))
                                .map(|prog| prog.program_number)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let ctx = FilterContext {
            packet: info.packet,
            pmt_pids: &stream
                .map(|s| s.stream_info.pmt.keys().copied().collect::<Vec<_>>())
                .unwrap_or_default(),
            es_pids: &stream
                .map(|s| {
                    s.stream_info
                        .pmt
                        .values()
                        .flat_map(|pmt| {
                            pmt.elementary_streams_info
                                .iter()
                                .map(|es| PIDTable::from(es.elementary_pid))
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            pcr_pids: &stream
                .map(|s| {
                    s.stream_info
                        .pmt
                        .values()
                        .map(|pmt| PIDTable::from(pmt.fields.pcr_pid))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            stream_alias: stream.map(|s| s.alias.clone()),
            program_numbers: &program_numbers,
        };

        parse_filter(&filter)
            .map(|filter_type| filter_type.matches(&ctx))
            .unwrap_or(true)
    }
}
