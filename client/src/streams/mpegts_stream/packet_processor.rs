use crate::streams::mpegts_stream::packet_info::{MpegTsPacketInfo, MpegTsStreamInfo};
use crate::streams::mpegts_stream::substream::{
    MpegtsSubStream, MpegtsSubStreams, SubStreamParameters, SubstreamMpegTsPacketInfo,
};
use netpix_common::mpegts::aggregator::MpegtsAggregator;
use netpix_common::mpegts::header::PIDTable;
use netpix_common::mpegts::psi::pat::fragmentary_pat::FragmentaryProgramAssociationTable;
use netpix_common::mpegts::psi::pat::ProgramAssociationTable;
use netpix_common::mpegts::psi::pmt::fragmentary_pmt::FragmentaryProgramMapTable;
use netpix_common::mpegts::psi::pmt::ProgramMapTable;
use netpix_common::mpegts::psi::psi_buffer::{FragmentaryPsi, PsiBuffer};
use netpix_common::mpegts::MpegtsFragment;
use netpix_common::packet::SessionPacket;
use netpix_common::{MpegtsPacket, Packet, PacketAssociationTable};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MpegtsPacketProcessor {
    aggregator: MpegtsAggregator,
}

impl MpegtsPacketProcessor {
    pub fn new() -> Self {
        Self {
            aggregator: MpegtsAggregator::new(),
        }
    }

    pub fn extract_pat(&mut self, packet: &Packet) -> Option<ProgramAssociationTable> {
        if let SessionPacket::Mpegts(mpegts) = packet.clone().contents {
            mpegts
                .fragments
                .iter()
                .find_map(|fragment| self.process_pat_fragment(fragment))
        } else {
            None
        }
    }

    pub fn determine_type(&mut self, packet: &Packet, stream_info: &mut MpegTsStreamInfo) {
        if let SessionPacket::Mpegts(mpegts) = &packet.contents {
            if let Some(pat) = self.process_pat(mpegts) {
                stream_info.pat = Some(pat.clone());
                self.process_pmt(mpegts, &pat, stream_info);
            }
        }
    }

    pub fn update_packet_info(
        &self,
        packet_info: &mut MpegTsPacketInfo,
        existing_packets: &[MpegTsPacketInfo],
    ) {
        if let Some(last_packet) = existing_packets.last() {
            packet_info.time_delta = packet_info.time.saturating_sub(last_packet.time);
        }
        self.update_rates(packet_info, existing_packets);
    }

    fn process_pat_fragment(
        &mut self,
        fragment: &MpegtsFragment,
    ) -> Option<ProgramAssociationTable> {
        if fragment.header.pid != PIDTable::ProgramAssociation {
            return None;
        }

        let payload = fragment.payload.as_ref()?;
        let pat_fragment = FragmentaryProgramAssociationTable::unmarshall(
            &payload.data,
            fragment.header.payload_unit_start_indicator,
        )?;

        self.aggregator
            .pat_buffer
            .set_last_section_number(pat_fragment.header.last_section_number);
        self.aggregator.add_pat(pat_fragment);
        self.aggregator.get_pat()
    }

    fn process_pat(&mut self, mpegts: &MpegtsPacket) -> Option<ProgramAssociationTable> {
        for fragment in &mpegts.fragments {
            if fragment.header.pid == PIDTable::ProgramAssociation {
                if let Some(payload) = &fragment.payload {
                    if let Some(pat_fragment) = FragmentaryProgramAssociationTable::unmarshall(
                        &payload.data,
                        fragment.header.payload_unit_start_indicator,
                    ) {
                        self.aggregator
                            .pat_buffer
                            .set_last_section_number(pat_fragment.header.last_section_number);
                        self.aggregator.add_pat(pat_fragment);
                        return self.aggregator.get_pat();
                    }
                }
            }
        }
        None
    }

    fn process_pmt(
        &mut self,
        mpegts: &MpegtsPacket,
        pat: &ProgramAssociationTable,
        stream_info: &mut MpegTsStreamInfo,
    ) {
        for fragment in &mpegts.fragments {
            self.process_pmt_fragment(fragment, pat);
        }

        if self.aggregator.is_pat_complete() {
            self.update_pmt_tables(pat, stream_info);
        }
    }

    fn process_pmt_fragment(&mut self, fragment: &MpegtsFragment, pat: &ProgramAssociationTable) {
        for program in &pat.programs {
            if let Some(program_map_pid) = program.program_map_pid {
                if Into::<u16>::into(fragment.header.pid) == program_map_pid {
                    if let Some(payload) = &fragment.payload {
                        if let Some(pmt_fragment) = FragmentaryProgramMapTable::unmarshall(
                            &payload.data,
                            fragment.header.payload_unit_start_indicator,
                        ) {
                            self.aggregator
                                .add_pmt(fragment.header.pid.into(), pmt_fragment);
                        }
                    }
                }
            }
        }
    }

    fn update_pmt_tables(
        &mut self,
        pat: &ProgramAssociationTable,
        stream_info: &mut MpegTsStreamInfo,
    ) {
        for program in &pat.programs {
            if let Some(program_map_pid) = program.program_map_pid {
                if let Some(program_map_table) = self.aggregator.get_pmt(program_map_pid) {
                    stream_info
                        .pmt
                        .insert(program_map_pid.into(), program_map_table.clone());
                }
            }
        }
    }

    fn update_rates(
        &self,
        packet_info: &mut MpegTsPacketInfo,
        existing_packets: &[MpegTsPacketInfo],
    ) {
        let cutoff = packet_info.time.saturating_sub(Duration::from_secs(1));

        let last_second_packets = existing_packets
            .iter()
            .rev()
            .take_while(|pack| pack.time > cutoff);

        packet_info.packet_rate = last_second_packets.clone().count() + 1;
        packet_info.bitrate =
            last_second_packets.map(|pack| pack.bytes).sum::<usize>() * 8 + packet_info.bytes * 8;
    }

    pub fn process_substreams(
        &mut self,
        packet: &Packet,
        alias: &str,
        stream_info: &MpegTsStreamInfo,
        substreams: &mut MpegtsSubStreams,
    ) {
        if let (Some(pat), SessionPacket::Mpegts(_mpegts)) = (&stream_info.pat, &packet.contents) {
            for (program_map_pid, program_map_table) in &stream_info.pmt {
                let context = SubstreamProcessingContext {
                    packet,
                    alias,
                    pat,
                    program_map_table,
                    program_map_pid: u16::from(*program_map_pid),
                    existing_packets: &stream_info.packets,
                };
                self.process_substream_packets(context, substreams);
            }
        }
    }

    fn process_substream_packets(
        &self,
        context: SubstreamProcessingContext,
        substreams: &mut MpegtsSubStreams,
    ) {
        let program_number = context.program_map_table.fields.program_number;
        let packet_association_table = PacketAssociationTable {
            source_addr: context.packet.source_addr,
            destination_addr: context.packet.destination_addr,
            protocol: context.packet.transport_protocol,
        };

        for es_info in &context.program_map_table.elementary_streams_info {
            let key = (
                packet_association_table,
                context.pat.transport_stream_id,
                program_number,
                es_info.stream_type,
            );

            let substream = substreams.entry(key).or_insert_with(|| {
                MpegtsSubStream::new(SubStreamParameters {
                    alias: context.alias.to_string(),
                    key,
                    pat: context.pat.clone(),
                })
            });

            substream.add_pmt(
                context.program_map_pid.into(),
                context.program_map_table.clone(),
            );

            self.process_packet_fragments(
                substream,
                context.packet,
                es_info.elementary_pid,
                context.program_map_pid,
            );

            // Process historical packets
            for mpegts_packet in context.existing_packets {
                self.process_historical_packet(
                    substream,
                    context.packet,
                    mpegts_packet,
                    es_info.elementary_pid,
                    context.program_map_pid,
                );
            }
        }
    }

    fn process_packet_fragments(
        &self,
        substream: &mut MpegtsSubStream,
        packet: &Packet,
        es_pid: u16,
        pmt_pid: u16,
    ) {
        if let SessionPacket::Mpegts(mpegts) = &packet.contents {
            if !substream.is_packet_processed(packet.id) {
                for fragment in &mpegts.fragments {
                    let context = FragmentProcessingContext {
                        substream,
                        packet,
                        fragment,
                        es_pid,
                        pmt_pid,
                    };
                    Self::process_fragment_for_substream(context);
                }
                substream.mark_packet_processed(packet.id);
            }
        }
    }

    fn process_historical_packet(
        &self,
        substream: &mut MpegtsSubStream,
        packet: &Packet,
        mpegts_packet: &MpegTsPacketInfo,
        es_pid: u16,
        pmt_pid: u16,
    ) {
        if !substream.is_packet_processed(mpegts_packet.id) {
            for fragment in &mpegts_packet.content.fragments {
                let context = FragmentProcessingContext {
                    substream,
                    packet,
                    fragment,
                    es_pid,
                    pmt_pid,
                };
                Self::process_fragment_for_substream(context);
            }
            substream.mark_packet_processed(mpegts_packet.id);
        }
    }

    fn process_fragment_for_substream(context: FragmentProcessingContext) {
        if context.fragment.header.pid == PIDTable::from(context.es_pid)
            || context.fragment.header.pid == PIDTable::from(context.pmt_pid)
        {
            context
                .substream
                .add_mpegts_fragment(SubstreamMpegTsPacketInfo::new(
                    context.packet,
                    context.fragment,
                ));
        }
    }
}

#[derive(Debug)]
struct SubstreamProcessingContext<'a> {
    packet: &'a Packet,
    alias: &'a str,
    pat: &'a ProgramAssociationTable,
    program_map_table: &'a ProgramMapTable,
    program_map_pid: u16,
    existing_packets: &'a [MpegTsPacketInfo],
}

#[derive(Debug)]
struct FragmentProcessingContext<'a> {
    substream: &'a mut MpegtsSubStream,
    packet: &'a Packet,
    fragment: &'a MpegtsFragment,
    es_pid: u16,
    pmt_pid: u16,
}
