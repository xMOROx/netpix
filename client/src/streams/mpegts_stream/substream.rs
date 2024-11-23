use crate::streams::int_to_letter;
use crate::streams::stream_statistics::{
    Bitrate, Bytes, PacketsTime, Statistics, StreamStatistics,
};
use rtpeeker_common::mpegts::header::PIDTable;
use rtpeeker_common::mpegts::psi::pat::ProgramAssociationTable;
use rtpeeker_common::mpegts::psi::pmt::stream_types::{stream_type_into_unique_letter, StreamType};
use rtpeeker_common::mpegts::psi::pmt::ProgramMapTable;
use rtpeeker_common::mpegts::MpegtsFragment;
use rtpeeker_common::{Packet, PacketAssociationTable};
use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub type SubStreamKey = (
    PacketAssociationTable,
    u16, //transport_stream_id
    u16, //program_number
    StreamType,
);

pub type MpegtsSubStreams = HashMap<SubStreamKey, MpegtsSubStream>;

#[derive(Debug, Clone)]
pub struct Aliases {
    pub stream_alias: String,
    pub program_alias: String,
}

#[derive(Debug, Clone)]
pub struct SubstreamMpegTsPacketInfo {
    pub packet_association_table: PacketAssociationTable,
    pub content: MpegtsFragment,
    pub id: usize,
    pub time: Duration,
    pub bytes: usize,
    pub bitrate: usize,     // in the last second, kbps
    pub packet_rate: usize, // packets/s
}

#[derive(Debug, Clone)]
pub struct MpegtsSubStream {
    pub packet_association_table: PacketAssociationTable,
    pub packets: Vec<SubstreamMpegTsPacketInfo>,
    pub pat: ProgramAssociationTable,
    pub pmt: HashMap<PIDTable, ProgramMapTable>,
    pub transport_stream_id: u16,
    pub program_number: u16,
    pub stream_type: StreamType,
    pub statistics: Statistics,
    pub aliases: Aliases,
    processed_packet_ids: HashSet<usize>,
}

impl SubstreamMpegTsPacketInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsFragment) -> Self {
        Self {
            packet_association_table: PacketAssociationTable {
                source_addr: packet.source_addr,
                destination_addr: packet.destination_addr,
                protocol: packet.transport_protocol.clone(),
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

impl MpegtsSubStream {
    pub fn new(key: &SubStreamKey, pat: ProgramAssociationTable) -> Self {
        let aliases = Aliases {
            stream_alias: format!(
                "{}-{}",
                int_to_letter(key.1 as usize),
                stream_type_into_unique_letter(&key.3)
            ),
            program_alias: key.2.to_string(),
        };

        Self {
            pmt: HashMap::new(),
            packet_association_table: key.0.clone(),
            statistics: Statistics::default(),
            packets: Vec::new(),
            transport_stream_id: key.1,
            program_number: key.2,
            stream_type: key.clone().3,
            pat,
            aliases,
            processed_packet_ids: HashSet::new(),
        }
    }

    pub fn is_packet_processed(&self, packet_id: usize) -> bool {
        self.processed_packet_ids.contains(&packet_id)
    }

    pub fn mark_packet_processed(&mut self, packet_id: usize) {
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

        let last_second_packets = self.packets.iter().rev().map_while(|pack| match pack {
            SubstreamMpegTsPacketInfo { time, .. } if *time > cutoff => Some(pack.bytes),
            _ => None,
        });

        mpegts_info.packet_rate = last_second_packets.clone().count() + 1;

        let bytes = last_second_packets.sum::<usize>() + mpegts_info.bytes;
        mpegts_info.bitrate = bytes * 8;
    }

    fn update_mpegts_parameters(&mut self, mut mpegts_info: SubstreamMpegTsPacketInfo) {

        if self.packets.is_empty() {
            self.statistics.set_packets_time(
                PacketsTime::new()
                    .first_time(mpegts_info.time)
                    .last_time(mpegts_info.time)
                    .build(),
            );
        } else {
            self.statistics.set_packets_time(
                PacketsTime::new()
                    .first_time(min(
                        self.statistics.get_packets_time().get_first_time(),
                        mpegts_info.time,
                    ))
                    .last_time(max(
                        self.statistics.get_packets_time().get_last_time(),
                        mpegts_info.time,
                    ))
                    .build(),
            );
        }

        self.update_rates(&mut mpegts_info);

        let mpegts_bytes = mpegts_info.content.size;

        self.statistics.add_bytes(
            Bytes::new()
                .frame_bytes(mpegts_info.bytes as f64)
                .protocol_bytes(mpegts_bytes as f64)
                .build(),
        );

        self.statistics.add_bitrate(
            Bitrate::new()
                .frame_bitrate((mpegts_info.bytes * 8) as f64)
                .protocol_bitrate((mpegts_bytes * 8) as f64)
                .build(),
        );



        self.statistics.increment_packet_rate();

        self.packets.push(mpegts_info);
    }
    fn create_statistics(packet: &Packet, mpegts_fragment: &MpegtsFragment) -> Statistics {
        let packet_bytes = packet.length;
        let mpegts_packet_bytes = mpegts_fragment.size;

        Statistics::new()
            .packets_time(
                PacketsTime::new()
                    .first_time(packet.timestamp)
                    .last_time(packet.timestamp)
                    .build(),
            )
            .bitrate(
                Bitrate::new()
                    .frame_bitrate((packet_bytes * 8) as f64)
                    .protocol_bitrate((mpegts_packet_bytes * 8) as f64)
                    .build(),
            )
            .bytes(
                Bytes::new()
                    .frame_bytes(packet_bytes as f64)
                    .protocol_bytes(mpegts_packet_bytes as f64)
                    .build(),
            )
            .build()
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
