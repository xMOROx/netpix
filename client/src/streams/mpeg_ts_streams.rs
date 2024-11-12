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
    pub packets: Vec<MpegTsPacketInfo>,
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: HashMap<PIDTable, ProgramMapTable>,
    first_time: Duration,
    last_time: Duration,
    stream_bytes: usize,
    bytes: usize,
}

#[derive(Debug, Clone)]
pub struct MpegTsStream {
    pub alias: String,
    pub mpegts_stream_info: MpegTsStreamInfo,
    pub mpegts_aggregator: MpegtsAggregator,
    pub transport_stream_id: u32,
}

impl MpegTsPacketInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Self {
        Self {
            content: mpegts_packet.clone(),
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
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
        let mpegts_packet_info = MpegTsPacketInfo::new(packet, mpegts_packet);

        Self {
            packets: vec![mpegts_packet_info],
            pat: None,
            pmt: HashMap::new(),
            first_time: packet.timestamp,
            last_time: packet.timestamp,
            stream_bytes: packet.length as usize,
            bytes: MpegTsStreamInfo::count_payload_bytes(mpegts_packet),
        }
    }

    pub fn new_with_pat(
        packet: &Packet,
        mpegts_packet: &MpegtsPacket,
        pat: Option<ProgramAssociationTable>,
    ) -> Self {
        let mpegts_packet_info = MpegTsPacketInfo::new(packet, mpegts_packet);

        Self {
            packets: vec![mpegts_packet_info],
            pat,
            pmt: HashMap::new(),
            first_time: packet.timestamp,
            last_time: packet.timestamp,
            stream_bytes: packet.length as usize,
            bytes: MpegTsStreamInfo::count_payload_bytes(mpegts_packet),
        }
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
    pub fn new(
        packet: &Packet,
        mpegts: &MpegtsPacket,
        default_alias: String,
        transport_stream_id: u32,
    ) -> Self {
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

                    if mpegts_aggregator.is_pat_complete() {
                        pat = mpegts_aggregator.get_pat();
                    }
                }
            });
        }

        Self {
            alias: default_alias,
            transport_stream_id,
            mpegts_stream_info: MpegTsStreamInfo::new_with_pat(packet, mpegts, pat),
            mpegts_aggregator,
        }
    }

    pub fn get_duration(&self) -> Duration {
        self.mpegts_stream_info
            .last_time
            .saturating_sub(self.mpegts_stream_info.first_time)
    }

    pub fn get_mean_bitrate(&self) -> f64 {
        let duration = self.get_duration().as_secs_f64();
        self.mpegts_stream_info.stream_bytes as f64 * 8.0 / duration
    }

    pub fn get_mean_rtp_bitrate(&self) -> f64 {
        let duration = self.get_duration().as_secs_f64();
        self.mpegts_stream_info.bytes as f64 * 8.0 / duration
    }

    pub fn get_mean_packet_rate(&self) -> f64 {
        let duration = self.get_duration().as_secs_f64();
        self.mpegts_stream_info.packets.len() as f64 / duration
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

        self.mpegts_stream_info.stream_bytes += mpegts_info.bytes;
        self.mpegts_stream_info.bytes +=
            MpegTsStreamInfo::count_payload_bytes(&mpegts_info.content);

        self.mpegts_stream_info.first_time =
            min(self.mpegts_stream_info.first_time, mpegts_info.time);
        self.mpegts_stream_info.last_time =
            max(self.mpegts_stream_info.last_time, mpegts_info.time);

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

            if self.mpegts_aggregator.is_pat_complete() {
                pat = self.mpegts_aggregator.get_pat();
            }
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

            if !self.mpegts_aggregator.is_pmt_complete(pid) {
                continue;
            };

            if let Some(program_map_table) = self.mpegts_aggregator.get_pmt(pid) {
                self.mpegts_stream_info
                    .pmt
                    .insert(pid.into(), program_map_table);
            }
        }
    }
}
