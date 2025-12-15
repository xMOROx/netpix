#![allow(dead_code)]
use crate::streams::rtcp_stream::RtcpStream;
use mpegts_stream::MpegTsStream;
use netpix_common::rtcp::ReceptionReport;
use netpix_common::rtcp::payload_feedbacks::PayloadFeedback;
use netpix_common::{
    MpegtsStreamKey, Packet, RtcpPacket, RtpStreamKey,
    packet::{SessionPacket, TransportProtocol},
};
use packets::Packets;
use rtpStream::RtpStream;
use std::{cell::RefCell, collections::HashMap, net::SocketAddr, rc::Rc};
use std::cell::RefMut;
use eframe::epaint::{Color32, Hsva};
use netpix_common::packet::StreamMetaData;

pub mod mpegts_stream;
pub mod packets;
pub mod rtcp_stream;
#[allow(non_snake_case)]
pub mod rtpStream;
pub mod stream_statistics;

pub type RefStreams = Rc<RefCell<Streams>>;

#[derive(Debug, Default)]
pub struct Streams {
    pub packets: Packets,
    pub rtp_streams: HashMap<RtpStreamKey, RtpStream>,
    pub mpeg_ts_streams: HashMap<MpegtsStreamKey, MpegTsStream>,
    pub rtcp_streams: HashMap<RtpStreamKey, RtcpStream>,
    pub alias_helper: Rc<RefCell<StreamAliasHelper>>
}

impl Streams {
    pub fn clear(&mut self) {
        self.packets.clear();
        self.rtp_streams.clear();
        self.mpeg_ts_streams.clear();
        self.rtcp_streams.clear();
    }

    pub fn add_packet(&mut self, packet: Packet) {
        let is_new = self.packets.is_new(&packet);

        if is_new {
            handle_packet(
                &mut self.rtp_streams,
                &mut self.mpeg_ts_streams,
                &mut self.rtcp_streams,
                self.alias_helper.borrow_mut(),
                &packet,
            );
            self.packets.add_packet(packet);
        } else {
            // if the packet is not new (its id is smaller that the last packet's id)
            // that this must be result of `parse_as` request or refetch (tho packets should be
            // pruned before refetch) in that case, recalculate everything,
            // this can be optimised if it proves to be to slow
            self.packets.add_packet(packet);
            self.recalculate();
        }
    }

    fn recalculate(&mut self) {
        let mut new_rtp_streams = HashMap::new();
        let mut new_mpegts_streams = HashMap::new();
        let mut new_rtcp_streams = HashMap::new();

        self.packets.values().for_each(|packet| {
            handle_packet(
                &mut new_rtp_streams,
                &mut new_mpegts_streams,
                &mut new_rtcp_streams,
                self.alias_helper.borrow_mut(),
                packet,
            )
        });

        self.rtp_streams = new_rtp_streams;
        self.mpeg_ts_streams = new_mpegts_streams;
        self.rtcp_streams = new_rtcp_streams;
    }
}

// this function need to take streams as an argument as opposed to methods on `Streams`
// to make `Streams::recalculate` work, dunno if there's a better way
fn handle_packet(
    rtp_streams: &mut HashMap<RtpStreamKey, RtpStream>,
    mpegts_streams: &mut HashMap<MpegtsStreamKey, MpegTsStream>,
    rtcp_streams: &mut HashMap<RtpStreamKey, RtcpStream>,
    stream_helper: RefMut<StreamAliasHelper>,
    packet: &Packet,
) {
    match packet.contents {
        SessionPacket::Mpegts(ref mpegts) => {
            let stream_key = (
                packet.source_addr,
                packet.destination_addr,
                packet.transport_protocol,
            );

            if let Some(stream) = mpegts_streams.get_mut(&stream_key) {
                stream.add_mpegts_packet(packet, mpegts);
            } else {
                let new_stream =
                    MpegTsStream::new(packet, mpegts, int_to_letter(mpegts_streams.len()));
                mpegts_streams.insert(stream_key, new_stream);
            }
        }
        SessionPacket::Rtp(ref rtp) => {
            let stream_key = (
                packet.source_addr,
                packet.destination_addr,
                packet.transport_protocol,
                rtp.ssrc,
            );

            if let Some(stream) = rtp_streams.get_mut(&stream_key) {
                stream.add_rtp_packet(packet, rtp);
            } else {
                let new_stream = RtpStream::new(packet, rtp, int_to_letter(rtp_streams.len()));
                rtp_streams.insert(stream_key, new_stream);
            }
        }
        SessionPacket::Rtcp(ref packs) => {
            for pack in packs {
                let ssrcs = match pack {
                    RtcpPacket::SenderReport(sr) => {
                        insert_or_update_rtcp_stream(rtcp_streams, sr.ssrc, packet, pack);
                        update_rtcp_streams_with_rr(rtcp_streams, packet, &sr.reports);
                        vec![sr.ssrc]
                    }
                    RtcpPacket::ReceiverReport(rr) => {
                        // Right now no data from RR is used in RTCP streams,
                        // so there is no use to process this
                        // insert_or_update_rtcp_stream(rtcp_streams, rr.ssrc, packet, pack);
                        update_rtcp_streams_with_rr(rtcp_streams, packet, &rr.reports);
                        vec![rr.ssrc]
                    }
                    RtcpPacket::PayloadSpecificFeedback(_pf) => Vec::new(),
                    RtcpPacket::SourceDescription(sd) => {
                        sd.chunks.iter().map(|chunk| chunk.source).collect()
                    }
                    _ => Vec::new(),
                };

                for ssrc in ssrcs {
                    let maybe_stream = get_rtcp_stream(
                        rtp_streams,
                        packet.source_addr,
                        packet.destination_addr,
                        packet.transport_protocol,
                        ssrc,
                    );
                    if let Some(stream) = maybe_stream {
                        stream.add_rtcp_packet(packet.id, packet.timestamp, pack);
                    }
                }
            }
        }
        SessionPacket::Meta(ref meta) => {
            stream_helper.put_meta(meta.clone());
        }
        _ => {}
    };
}

fn insert_or_update_rtcp_stream(
    rtcp_streams: &mut HashMap<RtpStreamKey, RtcpStream>,
    ssrc: u32,
    packet: &Packet,
    pack: &RtcpPacket,
) {
    let key_same_port = (
        packet.source_addr,
        packet.destination_addr,
        packet.transport_protocol,
        ssrc,
    );

    if let Some(stream) = rtcp_streams.get_mut(&key_same_port) {
        stream.update(pack, packet.timestamp);
    } else {
        let mut new_stream = RtcpStream::new(ssrc, packet);
        new_stream.update(pack, packet.timestamp);
        rtcp_streams.insert(key_same_port, new_stream);
    }
}

fn update_rtcp_streams_with_rr(
    streams_arg: &mut HashMap<RtpStreamKey, RtcpStream>,
    pkt: &Packet,
    reception_reports: &[ReceptionReport],
) {
    for report in reception_reports.iter() {
        let key_same_port = (
            pkt.source_addr,
            pkt.destination_addr,
            pkt.transport_protocol,
            report.ssrc,
        );
        if let Some(stream) = streams_arg.get_mut(&key_same_port) {
            stream.update_with_rr(pkt.timestamp, report);
        }
    }
}

fn get_rtcp_stream<T>(
    streams: &mut HashMap<RtpStreamKey, T>,
    mut source_addr: SocketAddr,
    mut destination_addr: SocketAddr,
    protocol: TransportProtocol,
    ssrc: u32,
) -> Option<&mut T> {
    let key_same_port = (source_addr, destination_addr, protocol, ssrc);
    if streams.contains_key(&key_same_port) {
        streams.get_mut(&key_same_port)
    } else {
        source_addr.set_port(source_addr.port() - 1);
        destination_addr.set_port(destination_addr.port() - 1);
        let key_next_port = (source_addr, destination_addr, protocol, ssrc);
        streams.get_mut(&key_next_port)
    }
}

fn int_to_letter(unique_id: usize) -> String {
    if unique_id == 0 {
        return String::from("A");
    }
    let mut result = String::new();
    let mut remaining = unique_id;

    while remaining > 0 {
        let current = (remaining) % 26;
        result.insert(0, (b'A' + current as u8) as char);
        remaining /= 26;
    }

    result
}

#[derive(Default, Clone, Debug)]
pub struct StreamAliasHelper {
    cache: RefCell<std::collections::HashMap<u32, String>>,
    meta: RefCell<HashMap<u32, String>>,
}

impl StreamAliasHelper {
    pub fn get_alias(&self, ssrc: u32) -> String {
        let mut cache = self.cache.borrow_mut();

        if let Some(alias) = cache.get(&ssrc) {
            return alias.clone();
        }

        let index = cache.len() as u32;
        let alias = self.index_to_letter(index);

        cache.insert(ssrc, alias.clone());
        alias
    }

    pub fn put_meta(&self, meta_data: StreamMetaData) {
        let mut meta = self.meta.borrow_mut();

        meta.insert(meta_data.ssrc,meta_data.stream_type.to_string());
    }

    pub fn get_meta(&self, ssrc: u32) -> Option<String> {
        let meta = self.meta.borrow();
        if let Some(meta_data) = meta.get(&ssrc) {
            return Some(meta_data.clone());
        }

        None
    }

    fn index_to_letter(&self, mut index: u32) -> String {
        let mut result = Vec::with_capacity(4);
        loop {
            let remainder = index % 26;
            result.push((b'A' + remainder as u8) as char);
            if index < 26 {
                break;
            }
            index = (index / 26) - 1;
        }
        result.into_iter().rev().collect()
    }

    pub fn get_color(&self, ssrc: u32) -> Color32 {
        let mut cache = self.cache.borrow_mut();

        let index = if let Some(_) = cache.get(&ssrc) {
            0
        } else {
            let index = cache.len() as u32;
            let alias = self.index_to_letter(index);
            cache.insert(ssrc, alias);
            0
        };

        let hash = (ssrc as u64).wrapping_mul(11400714819323198485);
        let hue = (hash as f32) / (u64::MAX as f32); // 0.0 - 1.0

        // High saturation and value for visibility against dark backgrounds
        // Maybe different logic for dark mode?
        Color32::from(Hsva::new(hue, 0.7, 0.9, 1.0))
    }

    pub fn print_ssrc(&self, ssrc: u32) -> String {
        format!("{:x} | alias: {}", ssrc, self.get_alias(ssrc))
    }
}
