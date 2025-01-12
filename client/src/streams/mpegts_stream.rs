#![allow(dead_code)]
use crate::streams::{
    mpegts_stream::substream::MpegtsSubStreams,
    stream_statistics::{Bitrate, Bytes, PacketsTime, StreamStatistics},
};
use netpix_common::mpegts::{payload::RawPayload, pes::PacketizedElementaryStream};
use netpix_common::{
    mpegts::psi::psi_buffer::{FragmentaryPsi, PsiBuffer},
    MpegtsPacket, Packet,
};
use rustc_hash::FxHashMap;
use std::time::Duration;

use packet_info::{MpegTsPacketInfo, MpegTsStreamInfo};
use packet_processor::MpegtsPacketProcessor;

pub mod packet_info;
pub mod packet_processor;
pub mod substream;

const REQUIRED_FIELDS_SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct MpegTsStream {
    pub alias: String,
    pub stream_info: MpegTsStreamInfo,
    pub substreams: MpegtsSubStreams,
    packet_processor: MpegtsPacketProcessor,
}

impl MpegTsStream {
    pub fn new(packet: &Packet, mpegts: &MpegtsPacket, default_alias: String) -> Self {
        let mut packet_processor = MpegtsPacketProcessor::new();
        let pat = packet_processor.extract_pat(packet);

        Self {
            alias: default_alias,
            stream_info: MpegTsStreamInfo::new_with_pat(packet, mpegts, pat),
            substreams: FxHashMap::default(),
            packet_processor,
        }
    }

    pub fn add_mpegts_packet(&mut self, packet: &Packet, mpegts: &MpegtsPacket) {
        let filtered_mpegts = MpegtsPacket {
            number_of_fragments: mpegts.number_of_fragments,
            fragments: mpegts
                .fragments
                .iter()
                .map(|fragment| {
                    let mut frag = fragment.clone();
                    if let Some(_pat) = &self.stream_info.pat {
                        if self.is_pes_pid(u16::from(fragment.header.pid)) {
                            if let Some(payload) = &fragment.payload {
                                if let Some(_pes) = PacketizedElementaryStream::build(&payload.data)
                                {
                                    frag.payload.replace(RawPayload {
                                        data: vec![],
                                        size: payload.size,
                                    });
                                }
                            }
                        }
                    }
                    frag
                })
                .collect(),
        };

        self.packet_processor
            .determine_type(packet, &mut self.stream_info);
        self.update_mpegts_parameters(MpegTsPacketInfo::new(packet, &filtered_mpegts));
        self.packet_processor.process_substreams(
            packet,
            &self.alias,
            &self.stream_info,
            &mut self.substreams,
        );
    }

    fn is_pes_pid(&self, pid: u16) -> bool {
        self.stream_info.pmt.values().any(|pmt| {
            pmt.elementary_streams_info
                .iter()
                .any(|es| es.elementary_pid == pid)
        })
    }

    fn update_mpegts_parameters(&mut self, mut mpegts_info: MpegTsPacketInfo) {
        self.packet_processor
            .update_packet_info(&mut mpegts_info, &self.stream_info.packets);
        self.stream_info.update_statistics(&mpegts_info);
        self.stream_info.packets.push(mpegts_info);
    }
}

impl StreamStatistics for MpegTsStream {
    fn get_duration(&self) -> Duration {
        let packets_time = self.stream_info.statistics.get_packets_time();

        packets_time
            .get_last_time()
            .saturating_sub(packets_time.get_first_time())
    }
    fn get_mean_frame_bitrate(&self) -> f64 {
        self.stream_info
            .statistics
            .get_bitrate()
            .get_frame_bitrate()
            / self.get_duration().as_secs_f64()
    }
    fn get_mean_protocol_bitrate(&self) -> f64 {
        self.stream_info
            .statistics
            .get_bitrate()
            .get_protocol_bitrate()
            / self.get_duration().as_secs_f64()
    }
    fn get_mean_frame_bytes_rate(&self) -> f64 {
        self.stream_info.statistics.get_bytes().get_frame_bytes()
            / self.get_duration().as_secs_f64()
    }
    fn get_mean_protocol_bytes_rate(&self) -> f64 {
        self.stream_info.statistics.get_bytes().get_protocol_bytes()
            / self.get_duration().as_secs_f64()
    }
    fn get_mean_packet_rate(&self) -> f64 {
        self.stream_info.statistics.get_packet_rate() / self.get_duration().as_secs_f64()
    }
    fn update_bitrate(&mut self, bitrate: Bitrate) {
        self.stream_info.statistics.set_bitrate(bitrate);
    }
    fn update_bytes(&mut self, bytes: Bytes) {
        self.stream_info.statistics.set_bytes(bytes);
    }
    fn update_time(&mut self, time: PacketsTime) {
        self.stream_info.statistics.set_packets_time(time);
    }
}
