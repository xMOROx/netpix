#![allow(dead_code)]
use mpegts_stream::MpegTsStream;
use netpix_common::{
    packet::{SessionPacket, TransportProtocol},
    MpegtsStreamKey, Packet, RtcpPacket, RtpStreamKey,
};
use packets::Packets;
use rtpStream::RtpStream;
use std::{cell::RefCell, collections::HashMap, net::SocketAddr, rc::Rc};

pub mod mpegts_stream;
mod packets;
#[allow(non_snake_case)]
pub mod rtpStream;
pub mod stream_statistics;

pub type RefStreams = Rc<RefCell<Streams>>;

#[derive(Debug, Default)]
pub struct Streams {
    pub packets: Packets,
    pub rtp_streams: HashMap<RtpStreamKey, RtpStream>,
    pub mpeg_ts_streams: HashMap<MpegtsStreamKey, MpegTsStream>,
}

impl Streams {
    pub fn clear(&mut self) {
        self.packets.clear();
        self.rtp_streams.clear();
        self.mpeg_ts_streams.clear();
    }

    pub fn add_packet(&mut self, packet: Packet) {
        let is_new = self.packets.is_new(&packet);

        if is_new {
            handle_packet(&mut self.rtp_streams, &mut self.mpeg_ts_streams, &packet);
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

        self.packets.values().for_each(|packet| {
            handle_packet(&mut new_rtp_streams, &mut new_mpegts_streams, packet)
        });

        self.rtp_streams = new_rtp_streams;
        self.mpeg_ts_streams = new_mpegts_streams;
    }
}

// this function need to take streams as an argument as opposed to methods on `Streams`
// to make `Streams::recalculate` work, dunno if there's a better way
fn handle_packet(
    rtp_streams: &mut HashMap<RtpStreamKey, RtpStream>,
    mpegts_streams: &mut HashMap<MpegtsStreamKey, MpegTsStream>,
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
                    RtcpPacket::SenderReport(sr) => vec![sr.ssrc],
                    RtcpPacket::ReceiverReport(rr) => vec![rr.ssrc],
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
        _ => {}
    };
}

fn get_rtcp_stream(
    streams: &mut HashMap<RtpStreamKey, RtpStream>,
    mut source_addr: SocketAddr,
    mut destination_addr: SocketAddr,
    protocol: TransportProtocol,
    ssrc: u32,
) -> Option<&mut RtpStream> {
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
