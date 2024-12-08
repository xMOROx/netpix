use crate::streams::stream_statistics::{Bitrate, Bytes, PacketsTime, Statistics};
use netpix_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use netpix_common::mpegts::psi::pat::ProgramAssociationTable;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;
use netpix_common::{MpegtsPacket, Packet, PacketAssociationTable};
use rustc_hash::FxHashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MpegTsPacketInfo {
    pub packet_association_table: PacketAssociationTable,
    pub content: MpegtsPacket,
    pub id: usize,
    pub time: Duration,
    pub time_delta: Duration,
    pub bytes: usize,
    pub bitrate: usize,     // in the last second, kbps
    pub packet_rate: usize, // packets/s
}

#[derive(Debug, Clone)]
pub struct MpegTsStreamInfo {
    pub packet_association_table: PacketAssociationTable,
    pub packets: Vec<MpegTsPacketInfo>,
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: FxHashMap<PIDTable, ProgramMapTable>,
    pub statistics: Statistics,
}

#[derive(Debug)]
pub struct StatisticsContext<'a> {
    pub packet_info: &'a MpegTsPacketInfo,
    pub mpegts_bytes: usize,
}

impl MpegTsPacketInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Self {
        Self {
            packet_association_table: PacketAssociationTable {
                source_addr: packet.source_addr,
                destination_addr: packet.destination_addr,
                protocol: packet.transport_protocol,
            },
            content: mpegts_packet.clone(),
            id: packet.id,
            time: packet.timestamp,
            time_delta: Duration::from_secs(0),
            bytes: packet.length as usize,
            bitrate: packet.length as usize * 8,
            packet_rate: 1,
        }
    }
}

impl MpegTsStreamInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Self {
        Self {
            packets: vec![MpegTsPacketInfo::new(packet, mpegts_packet)],
            pat: None,
            pmt: FxHashMap::default(),
            statistics: Self::create_statistics(packet, mpegts_packet),
            packet_association_table: PacketAssociationTable {
                source_addr: packet.source_addr,
                destination_addr: packet.destination_addr,
                protocol: packet.transport_protocol,
            },
        }
    }

    pub fn new_with_pat(
        packet: &Packet,
        mpegts_packet: &MpegtsPacket,
        pat: Option<ProgramAssociationTable>,
    ) -> Self {
        Self {
            packets: vec![MpegTsPacketInfo::new(packet, mpegts_packet)],
            pat,
            pmt: FxHashMap::default(),
            statistics: Self::create_statistics(packet, mpegts_packet),
            packet_association_table: PacketAssociationTable {
                source_addr: packet.source_addr,
                destination_addr: packet.destination_addr,
                protocol: packet.transport_protocol,
            },
        }
    }

    fn create_statistics(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Statistics {
        let packet_bytes = packet.length;
        let mpegts_packet_bytes = MpegTsStreamInfo::count_payload_bytes(mpegts_packet);

        Statistics::builder()
            .packets_time(
                PacketsTime::builder()
                    .first_time(packet.timestamp)
                    .last_time(packet.timestamp)
                    .build(),
            )
            .bitrate(
                Bitrate::builder()
                    .frame_bitrate((packet_bytes * 8) as f64)
                    .protocol_bitrate((mpegts_packet_bytes * 8) as f64)
                    .build(),
            )
            .bytes(
                Bytes::builder()
                    .frame_bytes(packet_bytes as f64)
                    .protocol_bytes(mpegts_packet_bytes as f64)
                    .build(),
            )
            .build()
    }

    fn count_payload_bytes(mpegts_packet: &MpegtsPacket) -> usize {
        mpegts_packet
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
            .sum()
    }

    pub fn update_statistics(&mut self, packet_info: &MpegTsPacketInfo) {
        let mpegts_bytes = Self::count_payload_bytes(&packet_info.content);
        let context = StatisticsContext {
            packet_info,
            mpegts_bytes,
        };
        self.update_statistics_internal(context);
    }

    fn update_statistics_internal(&mut self, context: StatisticsContext) {
        self.statistics.add_bytes(
            Bytes::builder()
                .frame_bytes(context.packet_info.bytes as f64)
                .protocol_bytes(context.mpegts_bytes as f64)
                .build(),
        );

        self.statistics.add_bitrate(
            Bitrate::builder()
                .frame_bitrate((context.packet_info.bytes * 8) as f64)
                .protocol_bitrate((context.mpegts_bytes * 8) as f64)
                .build(),
        );

        self.statistics.set_packets_time(
            PacketsTime::builder()
                .first_time(std::cmp::min(
                    self.statistics.get_packets_time().get_first_time(),
                    context.packet_info.time,
                ))
                .last_time(std::cmp::max(
                    self.statistics.get_packets_time().get_last_time(),
                    context.packet_info.time,
                ))
                .build(),
        );

        self.statistics.increment_packet_rate();
    }
}
