
use rtpeeker_common::mpegts::aggregator::MpegtsAggregator;
use rtpeeker_common::mpegts::header::PIDTable;
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
use web_time::Duration;

#[derive(Debug)]
pub struct MpegTsPacketInfo {
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub protocol: TransportProtocol,
    pub content: MpegtsPacket,
    pub id: usize,
    pub time: Duration,
    pub time_delta: Duration,
}

#[derive(Debug)]
pub struct MpegTsInfo {
    pub packets: Vec<MpegTsPacketInfo>,
    pub pat: Option<ProgramAssociationTable>,
    pub pmt: HashMap<PIDTable, ProgramMapTable>,
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
        }
    }
}

impl MpegTsInfo {
    pub fn new(packet: &Packet, mpegts_packet: &MpegtsPacket) -> Self {
        let mpegts_packet_info = MpegTsPacketInfo::new(packet, mpegts_packet);

        Self {
            packets: vec![mpegts_packet_info],
            pat: None,
            pmt: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct MpegTsStream {
    pub alias: String,
    pub mpegts_info: MpegTsInfo,
    pub mpegts_aggregator: MpegtsAggregator,
    pub transport_stream_id: u32,
    first_time: Duration,
    last_time: Duration,
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

        let mpegts_packet = MpegTsPacketInfo {
            content: mpegts.clone(),
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
            id: packet.id,
            time: packet.timestamp,
            time_delta: Duration::from_secs(0),
        };

        let mpegts_info = MpegTsInfo {
            pat,
            pmt: HashMap::new(),
            packets: vec![mpegts_packet],
        };

        Self {
            alias: default_alias,
            first_time: packet.timestamp,
            last_time: packet.timestamp,
            transport_stream_id,
            mpegts_info,
            mpegts_aggregator,
        }
    }

    pub fn get_duration(&self) -> Duration {
        self.last_time.checked_sub(self.first_time).unwrap()
    }

    pub fn get_mean_packet_rate(&self) -> f64 {
        let duration = self.get_duration().as_secs_f64();
        self.mpegts_info.packets.len() as f64 / duration
    }

    pub fn add_mpegts_packet(&mut self, packet: &Packet, mpegts: &MpegtsPacket) {
        self.determine_type(packet);

        let mpegts_packet = MpegTsPacketInfo {
            content: mpegts.clone(),
            source_addr: packet.source_addr,
            destination_addr: packet.destination_addr,
            protocol: packet.transport_protocol,
            id: packet.id,
            time: packet.timestamp,
            time_delta: Duration::from_secs(0),
        };

        self.update_mpegts_parameters(mpegts_packet);
    }

    fn update_mpegts_parameters(&mut self, mut mpegts_info: MpegTsPacketInfo) {
        mpegts_info.time_delta = mpegts_info
            .time
            .checked_sub(self.mpegts_info.packets.last().unwrap().time)
            .unwrap_or(Duration::ZERO);

        self.first_time = min(self.first_time, mpegts_info.time);
        self.last_time = max(self.last_time, mpegts_info.time);

        self.mpegts_info.packets.push(mpegts_info);
    }

    fn determine_type(&mut self, packet: &Packet) {
        if let SessionPacket::Mpegts(mpegts) = &packet.contents {
            let pat = self.process_pat(mpegts);

            if let Some(pat) = pat {
                self.mpegts_info.pat = Some(pat.clone());
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
                self.mpegts_info.pmt.insert(pid.into(), program_map_table);
            }
        }
    }
}
