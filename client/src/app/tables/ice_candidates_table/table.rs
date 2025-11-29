use super::{types::*, visualization::*};
use crate::app::common::TableBase;
use crate::streams::RefStreams;
use egui::*;
use ewebsock::WsSender;
use std::any::Any;

pub struct IceCandidatesTable {
    streams: RefStreams,
    #[allow(dead_code)]
    ws_sender: WsSender,
    visualization: IceCandidatesVisualization,
    cached_data: Option<IceCandidatesData>,
    last_stun_packet_count: usize,
}

impl TableBase for IceCandidatesTable {
    fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            ws_sender,
            visualization: IceCandidatesVisualization::default(),
            cached_data: None,
            last_stun_packet_count: 0,
        }
    }

    fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let current_stun_count = self
                .streams
                .borrow()
                .packets
                .values()
                .filter(|p| matches!(
                    &p.contents,
                    netpix_common::packet::SessionPacket::Stun(_)
                ))
                .count();

            if self.cached_data.is_none() || current_stun_count != self.last_stun_packet_count {
                self.cached_data = Some(self.build_ice_data());
                self.last_stun_packet_count = current_stun_count;
            }

            if let Some(ref data) = self.cached_data {
                self.visualization.show(ui, data);
            }
        });
    }

    fn check_filter(&mut self) {}
    fn build_header(&mut self, _header: &mut egui_extras::TableRow) {}
    fn build_table_body(&mut self, _body: egui_extras::TableBody) {}
    fn table_id(&self) -> &'static str {
        "ice_candidates"
    }
    fn table_name(&self) -> &'static str {
        "ICE Candidates"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl IceCandidatesTable {
    fn build_ice_data(&self) -> IceCandidatesData {
        let streams = self.streams.borrow();
        let mut data = IceCandidatesData::default();

        let mut stun_packets: Vec<_> = streams
            .packets
            .values()
            .filter_map(|p| {
                if let netpix_common::packet::SessionPacket::Stun(stun) = &p.contents {
                    Some((p, stun))
                } else {
                    None
                }
            })
            .collect();

        stun_packets.sort_by_key(|(p, _)| p.timestamp);

        let is_stun_server = |addr: &str| -> bool {
            // Check for common STUN server ports (3478, 19302)
            addr.ends_with(":3478") || addr.ends_with(":19302") ||
            // Check for Google STUN servers
            addr.contains("74.125.") || addr.contains("2001:4860:4864:")
        };

        for (packet, stun) in stun_packets {
            let local_is_server = is_stun_server(&packet.source_addr.to_string());
            let remote_is_server = is_stun_server(&packet.destination_addr.to_string());

            let key = match (local_is_server, remote_is_server) {
                (true, false) => CandidatePairKey {
                    local_candidate: packet.destination_addr.to_string(),
                    remote_candidate: packet.source_addr.to_string(),
                },
                _ => CandidatePairKey {
                    local_candidate: packet.source_addr.to_string(),
                    remote_candidate: packet.destination_addr.to_string(),
                },
            };

            let entry =
                data.candidate_pairs
                    .entry(key.clone())
                    .or_insert_with(|| CandidatePairStats {
                        key: key.clone(),
                        state: CandidatePairState::Frozen,
                        checks_sent: 0,
                        checks_received: 0,
                        responses_received: 0,
                        responses_sent: 0,
                        failures: 0,
                        avg_rtt_ms: 0.0,
                        first_seen: packet.timestamp,
                        last_seen: packet.timestamp,
                    });

            self.update_pair_stats(entry, stun, local_is_server, remote_is_server);

            if packet.timestamp > entry.last_seen {
                entry.last_seen = packet.timestamp;
            }
        }

        data
    }

    fn update_pair_stats(
        &self,
        stats: &mut CandidatePairStats,
        stun: &netpix_common::StunPacket,
        local_is_server: bool,
        remote_is_server: bool,
    ) {
        let involves_server = local_is_server || remote_is_server;

        let has_username = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "USERNAME");
        let has_priority = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "PRIORITY");
        let has_ice_controlling = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "ICE-CONTROLLING");
        let has_ice_controlled = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "ICE-CONTROLLED");
        let has_use_candidate = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "USE-CANDIDATE");
        let has_xor_mapped = stun
            .attributes
            .iter()
            .any(|attr| attr.get_type_name() == "XOR-MAPPED-ADDRESS");

        let is_gathering = involves_server && !has_username;
        let is_gathered = is_gathering && has_xor_mapped;
        let is_connectivity_check = !involves_server
            && (has_username || has_priority || has_ice_controlling || has_ice_controlled);

        let message_class = stun.message_type.class.as_string();

        match (
            message_class.as_str(),
            is_gathering,
            is_gathered,
            is_connectivity_check,
            has_use_candidate,
        ) {
            ("Request", true, false, false, false) => {
                stats.checks_sent += 1;
                stats.state = CandidatePairState::Gathering;
            }
            ("Success Response", true, true, false, false) => {
                stats.responses_received += 1;
                stats.state = CandidatePairState::Gathered;
            }
            ("Request", false, false, true, false) => {
                stats.checks_received += 1;
                if stats.state != CandidatePairState::Nominated {
                    stats.state = CandidatePairState::InProgress;
                }
            }
            ("Success Response", false, false, true, true) => {
                stats.checks_received += 1;
                if stats.state != CandidatePairState::Nominated {
                    stats.state = CandidatePairState::InProgress;
                }
            }
            ("Request", false, false, _, true) => {
                stats.checks_received += 1;
                stats.state = CandidatePairState::Nominated;
            }
            ("Error Response", _, _, _, _) => {
                stats.failures += 1;
                if stats.state != CandidatePairState::Nominated && !is_gathering {
                    stats.state = CandidatePairState::Failed;
                }
            }
            _ => {}
        }
    }
}
