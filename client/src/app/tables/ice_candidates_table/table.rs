use super::{types::*, visualization::*};
use crate::app::common::TableBase;
use crate::streams::RefStreams;
use egui::*;
use ewebsock::WsSender;
use netpix_common::packet::{Packet, PacketDirection, SessionPacket};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct IcePacketCounts {
    stun: usize,
    media: usize,
}

pub struct IceCandidatesTable {
    streams: RefStreams,
    #[allow(dead_code)]
    ws_sender: WsSender,
    visualization: IceCandidatesVisualization,
    cached_data: Option<IceCandidatesData>,
    last_packet_counts: IcePacketCounts,
}

impl TableBase for IceCandidatesTable {
    fn new(streams: RefStreams, ws_sender: WsSender) -> Self {
        Self {
            streams,
            ws_sender,
            visualization: IceCandidatesVisualization::default(),
            cached_data: None,
            last_packet_counts: IcePacketCounts::default(),
        }
    }

    fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            let packet_counts = self.count_ice_packets();

            if self.cached_data.is_none() || packet_counts != self.last_packet_counts {
                self.cached_data = Some(self.build_ice_data());
                self.last_packet_counts = packet_counts;
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

fn is_media(packet: &Packet) -> bool {
    matches!(
        packet.contents,
        SessionPacket::Rtp(_) | SessionPacket::Rtcp(_)
    )
}

fn is_stun_server(addr: &SocketAddr) -> bool {
    let ip = addr.ip().to_string();
    matches!(addr.port(), 3478 | 19302)
        || ip.starts_with("74.125.")
        || ip.starts_with("2001:4860:4864:")
}

fn pair_key(packet: &Packet, local_ips: &HashSet<IpAddr>) -> CandidatePairKey {
    let (source, destination) = (packet.source_addr, packet.destination_addr);
    let source_is_local = match packet.metadata.direction {
        PacketDirection::Outgoing => true,
        PacketDirection::Incoming => false,
        PacketDirection::Unknown => {
            local_ips.contains(&source.ip()) || !local_ips.contains(&destination.ip())
        }
    };
    let (local, remote) = if source_is_local {
        (source, destination)
    } else {
        (destination, source)
    };

    CandidatePairKey {
        local_candidate: local.to_string(),
        remote_candidate: remote.to_string(),
    }
}

impl IceCandidatesTable {
    fn count_ice_packets(&self) -> IcePacketCounts {
        let streams = self.streams.borrow();
        let mut counts = IcePacketCounts::default();
        for packet in streams.packets.values() {
            match packet.contents {
                SessionPacket::Stun(_) => counts.stun += 1,
                _ if is_media(packet) => counts.media += 1,
                _ => {}
            }
        }
        counts
    }

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

        let local_ips: HashSet<IpAddr> = stun_packets
            .iter()
            .filter_map(|(packet, _)| {
                if is_stun_server(&packet.destination_addr) {
                    Some(packet.source_addr.ip())
                } else if is_stun_server(&packet.source_addr) {
                    Some(packet.destination_addr.ip())
                } else {
                    None
                }
            })
            .collect();

        let mut pair_keys: HashMap<(SocketAddr, SocketAddr), CandidatePairKey> = HashMap::new();

        for (packet, stun) in stun_packets {
            let addrs = (packet.source_addr, packet.destination_addr);
            let key = pair_keys
                .entry(addrs)
                .or_insert_with(|| pair_key(packet, &local_ips))
                .clone();
            pair_keys
                .entry((addrs.1, addrs.0))
                .or_insert_with(|| key.clone());

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
                        media_packets_count: 0,
                    });

            let involves_server =
                is_stun_server(&packet.source_addr) || is_stun_server(&packet.destination_addr);
            self.update_pair_stats(entry, stun, involves_server);

            if packet.timestamp > entry.last_seen {
                entry.last_seen = packet.timestamp;
            }
        }

        for packet in streams.packets.values().filter(|packet| is_media(packet)) {
            let addrs = (packet.source_addr, packet.destination_addr);
            if let Some(stats) = pair_keys
                .get(&addrs)
                .and_then(|key| data.candidate_pairs.get_mut(key))
            {
                stats.record_media_packet(packet.timestamp);
            }
        }

        data
    }

    fn update_pair_stats(
        &self,
        stats: &mut CandidatePairStats,
        stun: &netpix_common::StunPacket,
        involves_server: bool,
    ) {
        fn priority(s: CandidatePairState) -> u8 {
            match s {
                CandidatePairState::Frozen => 0,
                CandidatePairState::Gathering => 1,
                CandidatePairState::Gathered => 2,
                CandidatePairState::InProgress => 3,
                CandidatePairState::Nominated => 4,
                CandidatePairState::SendingMedia => 5,
                CandidatePairState::Failed => 6,
                CandidatePairState::Disconnected => 7,
            }
        }
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
                let new_state = CandidatePairState::Gathering;
                if priority(new_state) > priority(stats.state) {
                    stats.state = new_state;
                }
            }
            ("Success Response", true, true, false, false) => {
                stats.responses_received += 1;
                let new_state = CandidatePairState::Gathered;
                if priority(new_state) > priority(stats.state) {
                    stats.state = new_state;
                }
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
                let new_state = CandidatePairState::Nominated;
                if priority(new_state) > priority(stats.state) {
                    stats.state = new_state;
                }
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
