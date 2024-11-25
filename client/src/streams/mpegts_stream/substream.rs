#![allow(dead_code)]
use crate::streams::{int_to_letter, stream_statistics::*};
use rtpeeker_common::{
    mpegts::{
        header::PIDTable,
        psi::{
            pat::ProgramAssociationTable,
            pmt::{
                stream_types::{stream_type_into_unique_letter, StreamType},
                ProgramMapTable,
            },
        },
        MpegtsFragment,
    },
    Packet, PacketAssociationTable,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    cmp::{max, min},
    time::Duration,
};

type PacketId = usize;
type TransportStreamId = u16;
type ProgramNumber = u16;

pub type SubStreamKey = (
    PacketAssociationTable,
    TransportStreamId,
    ProgramNumber,
    StreamType,
);
pub type MpegtsSubStreams = FxHashMap<SubStreamKey, MpegtsSubStream>;

#[derive(Debug, Clone)]
pub struct Aliases {
    pub stream_alias: String,
    pub program_alias: String,
}

impl Aliases {
    fn new(
        transport_stream_id: TransportStreamId,
        program_number: ProgramNumber,
        stream_type: &StreamType,
    ) -> Self {
        Self {
            stream_alias: format!(
                "{}-{}",
                int_to_letter(transport_stream_id as usize),
                stream_type_into_unique_letter(stream_type)
            ),
            program_alias: program_number.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubstreamMpegTsPacketInfo {
    pub packet_association_table: PacketAssociationTable,
    pub content: MpegtsFragment,
    pub id: PacketId,
    pub time: Duration,
    pub bytes: usize,
    pub bitrate: usize,
    pub packet_rate: usize,
}

impl SubstreamMpegTsPacketInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsFragment) -> Self {
        Self {
            packet_association_table: PacketAssociationTable {
                source_addr: packet.source_addr,
                destination_addr: packet.destination_addr,
                protocol: packet.transport_protocol,
            },
            content: mpegts_packet.clone(),
            id: packet.id,
            time: packet.timestamp,
            bytes: packet.length as usize,
            bitrate: packet.length as usize * 8,
            packet_rate: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MpegtsSubStream {
    pub packet_association_table: PacketAssociationTable,
    pub packets: Vec<SubstreamMpegTsPacketInfo>,
    pub pat: ProgramAssociationTable,
    pub pmt: FxHashMap<PIDTable, ProgramMapTable>,
    pub transport_stream_id: TransportStreamId,
    pub program_number: ProgramNumber,
    pub stream_type: StreamType,
    pub statistics: Statistics,
    pub aliases: Aliases,
    processed_packet_ids: FxHashSet<PacketId>,
}

impl MpegtsSubStream {
    pub fn new(key: &SubStreamKey, pat: ProgramAssociationTable) -> Self {
        Self {
            pmt: FxHashMap::default(),
            packet_association_table: key.0,
            statistics: Statistics::default(),
            packets: Vec::new(),
            transport_stream_id: key.1,
            program_number: key.2,
            stream_type: key.3,
            pat,
            aliases: Aliases::new(key.1, key.2, &key.3),
            processed_packet_ids: FxHashSet::default(),
        }
    }

    pub fn is_packet_processed(&self, packet_id: PacketId) -> bool {
        self.processed_packet_ids.contains(&packet_id)
    }

    pub fn mark_packet_processed(&mut self, packet_id: PacketId) {
        self.processed_packet_ids.insert(packet_id);
    }

    pub fn add_mpegts_fragment(&mut self, packet: SubstreamMpegTsPacketInfo) {
        self.update_mpegts_parameters(packet);
    }

    pub fn add_pmt(&mut self, pid: PIDTable, pmt: ProgramMapTable) {
        self.pmt.insert(pid, pmt);
    }

    fn update_rates(&self, mpegts_info: &mut SubstreamMpegTsPacketInfo) {
        let cutoff = mpegts_info.time.saturating_sub(Duration::from_secs(1));
        let last_second_packets = self.get_last_second_packets(cutoff);

        mpegts_info.packet_rate = last_second_packets.clone().count() + 1;
        mpegts_info.bitrate = (last_second_packets.sum::<usize>() + mpegts_info.bytes) * 8;
    }

    fn get_last_second_packets(
        &self,
        cutoff: Duration,
    ) -> impl Iterator<Item = usize> + Clone + '_ {
        self.packets
            .iter()
            .rev()
            .map_while(move |pack| (pack.time > cutoff).then_some(pack.bytes))
    }

    fn update_mpegts_parameters(&mut self, mut mpegts_info: SubstreamMpegTsPacketInfo) {
        self.update_packet_times(&mpegts_info);
        self.update_rates(&mut mpegts_info);
        self.update_statistics(&mpegts_info);
        self.packets.push(mpegts_info);
    }

    fn update_packet_times(&mut self, mpegts_info: &SubstreamMpegTsPacketInfo) {
        let new_time = if self.packets.is_empty() {
            PacketsTime::builder()
                .first_time(mpegts_info.time)
                .last_time(mpegts_info.time)
                .build()
        } else {
            PacketsTime::builder()
                .first_time(min(
                    self.statistics.get_packets_time().get_first_time(),
                    mpegts_info.time,
                ))
                .last_time(max(
                    self.statistics.get_packets_time().get_last_time(),
                    mpegts_info.time,
                ))
                .build()
        };
        self.statistics.set_packets_time(new_time);
    }

    fn update_statistics(&mut self, mpegts_info: &SubstreamMpegTsPacketInfo) {
        let mpegts_bytes = mpegts_info.content.size;

        self.statistics.add_bytes(
            Bytes::builder()
                .frame_bytes(mpegts_info.bytes as f64)
                .protocol_bytes(mpegts_bytes as f64)
                .build(),
        );

        self.statistics.add_bitrate(
            Bitrate::builder()
                .frame_bitrate((mpegts_info.bytes * 8) as f64)
                .protocol_bitrate((mpegts_bytes * 8) as f64)
                .build(),
        );

        self.statistics.increment_packet_rate();
    }
}

impl StreamStatistics for MpegtsSubStream {
    fn get_duration(&self) -> Duration {
        let packets_time = self.statistics.get_packets_time();
        packets_time
            .get_last_time()
            .saturating_sub(packets_time.get_first_time())
    }

    fn get_mean_frame_bitrate(&self) -> f64 {
        self.statistics.get_bitrate().get_frame_bitrate() / self.get_duration().as_secs_f64()
    }
    fn get_mean_protocol_bitrate(&self) -> f64 {
        self.statistics.get_bitrate().get_protocol_bitrate() / self.get_duration().as_secs_f64()
    }

    fn get_mean_frame_bytes_rate(&self) -> f64 {
        self.statistics.get_bytes().get_frame_bytes() / self.get_duration().as_secs_f64()
    }

    fn get_mean_protocol_bytes_rate(&self) -> f64 {
        self.statistics.get_bytes().get_protocol_bytes() / self.get_duration().as_secs_f64()
    }

    fn get_mean_packet_rate(&self) -> f64 {
        self.statistics.get_packet_rate() / self.get_duration().as_secs_f64()
    }
    fn update_bitrate(&mut self, bitrate: Bitrate) {
        self.statistics.set_bitrate(bitrate);
    }
    fn update_bytes(&mut self, bytes: Bytes) {
        self.statistics.set_bytes(bytes);
    }
    fn update_time(&mut self, time: PacketsTime) {
        self.statistics.set_packets_time(time);
    }
}
