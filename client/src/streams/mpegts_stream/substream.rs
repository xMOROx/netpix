use crate::streams::mpegts_stream::{MpegTsPacketInfo, MpegTsStreamInfo};
use crate::streams::stream_statistics::{
    Bitrate, Bytes, PacketsTime, Statistics, StreamStatistics,
};
use rtpeeker_common::mpegts::header::PIDTable;
use rtpeeker_common::mpegts::psi::pat::ProgramAssociationTable;
use rtpeeker_common::mpegts::psi::pmt::stream_types::StreamType;
use rtpeeker_common::mpegts::psi::pmt::ProgramMapTable;
use rtpeeker_common::packet::TransportProtocol;
use rtpeeker_common::{MpegtsPacket, Packet};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

pub type SubStreamKey = (
    SocketAddr,
    SocketAddr,
    TransportProtocol,
    u16,
    u16,
    StreamType,
);

pub type MpegtsSubStreams = HashMap<SubStreamKey, MpegtsSubStream>;

#[derive(Debug, Clone)]
pub struct MpegtsSubStream {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub protocol: TransportProtocol,
    pub packets: Vec<MpegTsPacketInfo>,
    pub pat: ProgramAssociationTable,
    pub pmt: HashMap<PIDTable, ProgramMapTable>,
    pub transport_stream_id: u16,
    pub program_number: u16,
    pub stream_type: StreamType,
    pub statistics: Statistics,
}

impl MpegtsSubStream {
    pub fn new(
        key: &SubStreamKey,
        pat: ProgramAssociationTable,
    ) -> Self {
        Self {
            pmt: HashMap::new(),
            source_addr: key.0,
            destination_addr: key.1,
            protocol: key.2,
            statistics: Statistics::default(),
            packets: vec![],
            transport_stream_id: key.3,
            program_number: key.4,
            stream_type: key.clone().5,
            pat,
        }
    }

    pub fn add_mpegts_packet(&mut self, packet: MpegTsPacketInfo) {
        self.update_mpegts_parameters(packet);
    }
    pub fn add_pat(&mut self, pat: ProgramAssociationTable) {
        self.pat = pat;
    }
    pub fn add_pmt(&mut self, pid: PIDTable, pmt: ProgramMapTable) {
        self.pmt.insert(pid, pmt);
    }

    fn update_rates(&self, mpegts_info: &mut MpegTsPacketInfo) {
        let cutoff = mpegts_info.time.saturating_sub(Duration::from_secs(1));

        let last_second_packets = self.packets.iter().rev().map_while(|pack| match pack {
            MpegTsPacketInfo { time, .. } if *time > cutoff => Some(pack.bytes),
            _ => None,
        });

        mpegts_info.packet_rate = last_second_packets.clone().count() + 1;

        let bytes = last_second_packets.sum::<usize>() + mpegts_info.bytes;
        mpegts_info.bitrate = bytes * 8;
    }
    fn update_mpegts_parameters(&mut self, mut mpegts_info: MpegTsPacketInfo) {

        mpegts_info.time_delta = mpegts_info
            .time
            .saturating_sub(self.packets.last().unwrap().time);

        self.update_rates(&mut mpegts_info);

        let mpegts_bytes = MpegTsStreamInfo::count_payload_bytes(&mpegts_info.content);

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

        self.statistics
            .set_packet_rate(self.statistics.get_packet_rate() + 1.0);

        self.packets.push(mpegts_info);
    }
    fn create_statistics(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Statistics {
        let packet_bytes = packet.length;
        let mpegts_packet_bytes = MpegTsStreamInfo::count_payload_bytes(mpegts_packet);

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
