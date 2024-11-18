use crate::streams::stream_statistics::{Bitrate, Bytes, PacketsTime, Statistics, StreamStatistics};
use rtpeeker_common::mpegts::aggregator::MpegtsAggregator;
use rtpeeker_common::mpegts::header::{AdaptationFieldControl, PIDTable};
use rtpeeker_common::mpegts::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use rtpeeker_common::mpegts::psi::pat::ProgramAssociationTable;
use rtpeeker_common::mpegts::psi::pmt::fragmentary_pmt::FragmentaryProgramMapTable;
use rtpeeker_common::mpegts::psi::pmt::ProgramMapTable;
use rtpeeker_common::mpegts::psi::psi_buffer::{FragmentaryPsi, PsiBuffer};
use rtpeeker_common::mpegts::MpegtsFragment;
use rtpeeker_common::packet::{SessionPacket, TransportProtocol};
use rtpeeker_common::{MpegtsPacket, Packet};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

pub mod substream;

#[derive(Debug, Clone)]
pub struct MpegTsPacketInfo {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub protocol: TransportProtocol,
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
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub protocol: TransportProtocol,
    pub packets: Vec<MpegTsPacketInfo>,
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: HashMap<PIDTable, ProgramMapTable>,
    statistics: Statistics,
}

#[derive(Debug, Clone)]
pub struct MpegTsStream {
    pub alias: String,
    pub mpegts_stream_info: MpegTsStreamInfo,
    pub mpegts_aggregator: MpegtsAggregator,
}

impl MpegTsPacketInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Self {
        Self {
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
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
            pmt: HashMap::new(),
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
            statistics: Self::create_statistics(packet, mpegts_packet),
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
            pmt: HashMap::new(),
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
            statistics: Self::create_statistics(packet, mpegts_packet),
        }
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
}

impl MpegTsStream {
    pub fn new(packet: &Packet, mpegts: &MpegtsPacket, default_alias: String) -> Self {
        let mut mpegts_aggregator = MpegtsAggregator::new();
        let mut pat: Option<ProgramAssociationTable> = None;

        if let SessionPacket::Mpegts(mpegts) = packet.clone().contents {
            mpegts.fragments.iter().for_each(|fragment| {
                if let PIDTable::ProgramAssociation = fragment.header.pid {
                    if fragment.payload.is_none() {
                        return;
                    }
                    let payload = fragment.clone().payload.unwrap().data;
                    let pat_fragment = FragmentaryProgramAssociationTable::unmarshall(
                        &*payload,
                        fragment.header.payload_unit_start_indicator,
                    );
                    if pat_fragment.is_none() {
                        return;
                    }
                    let pat_fragment = pat_fragment.unwrap();

                    mpegts_aggregator
                        .pat_buffer
                        .set_last_section_number(pat_fragment.header.last_section_number);

                    mpegts_aggregator.add_pat(pat_fragment);

                    pat = mpegts_aggregator.get_pat();
                }
            });
        }

        Self {
            alias: default_alias,
            mpegts_stream_info: MpegTsStreamInfo::new_with_pat(packet, mpegts, pat),
            mpegts_aggregator,
        }
    }

    fn update_rates(&self, mpegts_info: &mut MpegTsPacketInfo) {
        let cutoff = mpegts_info.time.saturating_sub(Duration::from_secs(1));

        let last_second_packets = self
            .mpegts_stream_info
            .packets
            .iter()
            .rev()
            .map_while(|pack| match pack {
                MpegTsPacketInfo { time, .. } if *time > cutoff => Some(pack.bytes),
                _ => None,
            });

        mpegts_info.packet_rate = last_second_packets.clone().count() + 1;

        let bytes = last_second_packets.sum::<usize>() + mpegts_info.bytes;
        mpegts_info.bitrate = bytes * 8;
    }

    pub fn add_mpegts_packet(&mut self, packet: &Packet, mpegts: &MpegtsPacket) {
        self.determine_type(packet);
        self.update_mpegts_parameters(MpegTsPacketInfo::new(packet, mpegts));
    }

    fn update_mpegts_parameters(&mut self, mut mpegts_info: MpegTsPacketInfo) {
        mpegts_info.time_delta = mpegts_info
            .time
            .saturating_sub(self.mpegts_stream_info.packets.last().unwrap().time);

        self.update_rates(&mut mpegts_info);

        let mpegts_bytes = MpegTsStreamInfo::count_payload_bytes(&mpegts_info.content);

        self.mpegts_stream_info.statistics.add_bytes(Bytes::new()
            .frame_bytes(mpegts_info.bytes as f64)
            .protocol_bytes(mpegts_bytes as f64)
            .build());

        self.mpegts_stream_info.statistics.add_bitrate(Bitrate::new()
            .frame_bitrate((mpegts_info.bytes * 8) as f64)
            .protocol_bitrate((mpegts_bytes * 8) as f64)
            .build());

        self.mpegts_stream_info.statistics.set_packets_time(PacketsTime::new()
            .first_time(min(self.mpegts_stream_info.statistics.get_packets_time().get_first_time(), mpegts_info.time))
            .last_time(max(self.mpegts_stream_info.statistics.get_packets_time().get_last_time(), mpegts_info.time))
            .build());

        self.mpegts_stream_info.statistics.set_packet_rate(
            self.mpegts_stream_info.statistics.get_packet_rate() + 1.0
        );

        self.mpegts_stream_info.packets.push(mpegts_info);
    }

    fn determine_type(&mut self, packet: &Packet) {
        if let SessionPacket::Mpegts(mpegts) = &packet.contents {
            let pat = self.process_pat(mpegts);

            if let Some(pat) = pat {
                self.mpegts_stream_info.pat = Some(pat.clone());
                self.process_pmt(mpegts, &pat);
            }
        }
    }

    fn process_pat(&mut self, mpegts: &MpegtsPacket) -> Option<ProgramAssociationTable> {
        let mut pat = None;

        for fragment in mpegts.fragments.iter() {
            if fragment.header.pid != PIDTable::ProgramAssociation {
                continue;
            }

            let Some(payload) = &fragment.payload else {
                continue;
            };

            let Some(pat_fragment) = FragmentaryProgramAssociationTable::unmarshall(
                &payload.data,
                fragment.header.payload_unit_start_indicator,
            ) else {
                continue;
            };

            self.mpegts_aggregator
                .pat_buffer
                .set_last_section_number(pat_fragment.header.last_section_number);

            self.mpegts_aggregator.add_pat(pat_fragment);

            pat = self.mpegts_aggregator.get_pat();
        }

        pat
    }

    fn process_pmt(&mut self, mpegts: &MpegtsPacket, pat: &ProgramAssociationTable) {
        for fragment in mpegts.fragments.iter() {
            self.process_pmt_fragment(fragment, pat);
        }

        if self.mpegts_aggregator.is_pat_complete() {
            self.update_pmt_tables(pat);
        }
    }

    fn process_pmt_fragment(&mut self, fragment: &MpegtsFragment, pat: &ProgramAssociationTable) {
        for program in pat.programs.iter() {
            let Some(program_map_pid) = program.program_map_pid else {
                continue;
            };
            if program_map_pid != fragment.header.pid.clone().into() {
                continue;
            };

            let Some(payload) = &fragment.payload else {
                continue;
            };
            let Some(pmt_fragment) = FragmentaryProgramMapTable::unmarshall(
                &payload.data,
                fragment.header.payload_unit_start_indicator,
            ) else {
                continue;
            };

            self.mpegts_aggregator
                .add_pmt(fragment.header.pid.clone().into(), pmt_fragment);
        }
    }

    fn update_pmt_tables(&mut self, pat: &ProgramAssociationTable) {
        for program in pat.programs.iter() {
            let Some(program_map_pid) = program.program_map_pid else {
                continue;
            };
            let pid: u16 = program_map_pid.into();

            if let Some(program_map_table) = self.mpegts_aggregator.get_pmt(pid) {
                self.mpegts_stream_info
                    .pmt
                    .insert(pid.into(), program_map_table);
            }
        }
    }
}

impl StreamStatistics for MpegTsStream {
    fn get_statistics(&self) -> Statistics {
        self.mpegts_stream_info.statistics.clone()
    }

    fn get_duration(&self) -> Duration {
        let packets_time = self.mpegts_stream_info.statistics.get_packets_time();
        packets_time
            .get_last_time()
            .saturating_sub(packets_time.get_first_time())
    }

    fn get_mean_frame_bitrate(&self) -> f64 {
        self.mpegts_stream_info.statistics.get_bitrate().get_frame_bitrate() / self.get_duration().as_secs_f64()
    }
    fn get_mean_protocol_bitrate(&self) -> f64 {
        self.mpegts_stream_info.statistics.get_bitrate().get_protocol_bitrate() / self.get_duration().as_secs_f64()
    }

    fn get_mean_frame_bytes_rate(&self) -> f64 {
        self.mpegts_stream_info.statistics.get_bytes().get_frame_bytes() / self.get_duration().as_secs_f64()
    }

    fn get_mean_protocol_bytes_rate(&self) -> f64 {
        self.mpegts_stream_info.statistics.get_bytes().get_protocol_bytes() / self.get_duration().as_secs_f64()
    }

    fn get_mean_packet_rate(&self) -> f64 {
        self.mpegts_stream_info.statistics.get_packet_rate() / self.get_duration().as_secs_f64()
    }
    fn update_bitrate(&mut self, bitrate: Bitrate) {
        self.mpegts_stream_info.statistics.set_bitrate(bitrate);
    }
    fn update_bytes(&mut self, bytes: Bytes) {
        self.mpegts_stream_info.statistics.set_bytes(bytes);
    }
    fn update_time(&mut self, time: PacketsTime) {
        self.mpegts_stream_info.statistics.set_packets_time(time);
    }

}
