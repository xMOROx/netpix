use super::{MpegtsPacket, RtcpPacket, RtpPacket, StunPacket};
use bincode::{Decode, Encode};

use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use std::{fmt, io::Write, time::SystemTime};

#[cfg(not(target_arch = "wasm32"))]
use pnet_packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet as _,
};

#[derive(Encode, Decode, PartialEq, Debug, Copy, Clone)]
pub enum SessionProtocol {
    Unknown,
    Rtp,
    Rtcp,
    Mpegts,
    Stun,
}

impl FromStr for SessionProtocol {
    type Err = ();
    fn from_str(p0: &str) -> Result<Self, Self::Err> {
        match p0.to_lowercase().as_str() {
            "unknown" => Ok(Self::Unknown),
            "rtp" => Ok(Self::Rtp),
            "rtcp" => Ok(Self::Rtcp),
            "mpeg-ts" => Ok(Self::Mpegts),
            "stun" => Ok(Self::Stun),
            _ => Err(()),
        }
    }
}

impl SessionProtocol {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Unknown,
            Self::Rtp,
            Self::Rtcp,
            Self::Mpegts,
            Self::Stun,
        ]
    }
}

impl fmt::Display for SessionProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Self::Unknown => "Unknown",
            Self::Rtp => "RTP",
            Self::Rtcp => "RTCP",
            Self::Mpegts => "MPEG-TS",
            Self::Stun => "STUN",
        };

        write!(f, "{}", res)
    }
}

#[derive(Decode, Encode, Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransportProtocol {
    Tcp,
    Udp,
}

impl fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Self::Udp => "UDP",
            Self::Tcp => "TCP",
        };

        write!(f, "{}", res)
    }
}

#[derive(Decode, Encode, Debug, Clone)]
pub enum SessionPacket {
    Unknown,
    Rtp(RtpPacket),
    Rtcp(Vec<RtcpPacket>),
    Mpegts(MpegtsPacket),
    Stun(StunPacket),
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Packet {
    pub payload: Option<Vec<u8>>,
    pub id: usize,
    pub timestamp: Duration,
    pub length: u32,
    pub source_addr: SocketAddr,
    pub destination_addr: SocketAddr,
    pub transport_protocol: TransportProtocol,
    pub session_protocol: SessionProtocol,
    pub contents: SessionPacket,
    pub creation_time: SystemTime,
}

#[cfg(not(target_arch = "wasm32"))]
impl Packet {
    pub fn build(raw_packet: &pcap::Packet, id: usize) -> Option<Self> {
        let ethernet_packet = EthernetPacket::new(raw_packet)?;
        Self::build_from_ethernet(raw_packet, id, &ethernet_packet)
    }

    fn build_from_ethernet(
        raw_packet: &pcap::Packet,
        id: usize,
        ethernet_packet: &EthernetPacket,
    ) -> Option<Self> {
        match ethernet_packet.get_ethertype() {
            EtherTypes::Ipv4 => Self::build_from_ip4(raw_packet, id, ethernet_packet),
            EtherTypes::Ipv6 => Self::build_from_ip6(raw_packet, id, ethernet_packet),
            _ => None,
        }
    }

    fn build_from_ip4(
        raw_packet: &pcap::Packet,
        id: usize,
        ethernet_packet: &EthernetPacket,
    ) -> Option<Self> {
        let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload())?;
        let source_addr = ipv4_packet.get_source();
        let destination_addr = ipv4_packet.get_destination();
        let ip_payload = ipv4_packet.payload();

        if ip_payload.is_empty() {
            return Self::build_from_transport(
                raw_packet,
                id,
                source_addr.into(),
                destination_addr.into(),
                TransportProtocol::Udp, // default to UDP for empty payload
                ethernet_packet.payload(),
            );
        }

        let transport_protocol = match ipv4_packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => TransportProtocol::Tcp,
            IpNextHeaderProtocols::Udp => TransportProtocol::Udp,
            _ => return None,
        };

        Self::build_from_transport(
            raw_packet,
            id,
            source_addr.into(),
            destination_addr.into(),
            transport_protocol,
            ip_payload,
        )
    }

    fn build_from_ip6(
        raw_packet: &pcap::Packet,
        id: usize,
        ethernet_packet: &EthernetPacket,
    ) -> Option<Self> {
        let ipv6_packet = Ipv6Packet::new(ethernet_packet.payload())?;
        let source_addr = ipv6_packet.get_source();
        let destination_addr = ipv6_packet.get_destination();
        let ip_payload = ipv6_packet.payload();

        if ip_payload.is_empty() {
            return Self::build_from_transport(
                raw_packet,
                id,
                source_addr.into(),
                destination_addr.into(),
                TransportProtocol::Udp, // default to UDP for empty payload
                ethernet_packet.payload(),
            );
        }

        let transport_protocol = match ipv6_packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => TransportProtocol::Tcp,
            IpNextHeaderProtocols::Udp => TransportProtocol::Udp,
            _ => return None,
        };

        Self::build_from_transport(
            raw_packet,
            id,
            source_addr.into(),
            destination_addr.into(),
            transport_protocol,
            ip_payload,
        )
    }

    fn build_from_transport(
        raw_packet: &pcap::Packet,
        id: usize,
        source_addr: std::net::IpAddr,
        destination_addr: std::net::IpAddr,
        transport_protocol: TransportProtocol,
        payload: &[u8],
    ) -> Option<Self> {
        let (source_addr, destination_addr, payload) = match transport_protocol {
            TransportProtocol::Tcp => {
                let tcp_packet = TcpPacket::new(payload)?;
                let source_port = tcp_packet.get_source();
                let destination_port = tcp_packet.get_destination();
                let source_addr = SocketAddr::new(source_addr, source_port);
                let destination_addr = SocketAddr::new(destination_addr, destination_port);
                let tcp_payload = tcp_packet.payload();
                if tcp_payload.is_empty() {
                    (source_addr, destination_addr, payload.to_vec())
                } else {
                    (source_addr, destination_addr, tcp_payload.to_vec())
                }
            }
            TransportProtocol::Udp => {
                let udp_packet = UdpPacket::new(payload)?;
                let source_port = udp_packet.get_source();
                let destination_port = udp_packet.get_destination();
                let source_addr = SocketAddr::new(source_addr, source_port);
                let destination_addr = SocketAddr::new(destination_addr, destination_port);
                let udp_payload = udp_packet.payload();
                if udp_payload.is_empty() {
                    (source_addr, destination_addr, payload.to_vec())
                } else {
                    (source_addr, destination_addr, udp_payload.to_vec())
                }
            }
        };

        Some(Self {
            payload: Some(payload),
            id,
            length: raw_packet.header.len - 14,
            timestamp: get_duration(raw_packet),
            source_addr,
            destination_addr,
            transport_protocol,
            session_protocol: SessionProtocol::Unknown,
            contents: SessionPacket::Unknown,
            creation_time: SystemTime::now(),
        })
    }

    pub fn guess_payload(&mut self) {
        // could use port to determine validity
        // TODO: TURN channels
        //
        // also, some UDP ports are used by other protocols
        // see Wireshark -> View -> Internals -> Dissector Table -> UDP port
        if self.transport_protocol != TransportProtocol::Udp {
            return;
        }

        if let Some(stun) = StunPacket::build(self) {
            self.session_protocol = SessionProtocol::Stun;
            self.contents = SessionPacket::Stun(stun);
            return;
        }

        if let Some(mpegts) = MpegtsPacket::build(self) {
            self.session_protocol = SessionProtocol::Mpegts;
            self.contents = SessionPacket::Mpegts(mpegts);
            return;
        }

        if let Some(rtcp) = RtcpPacket::build(self) {
            if is_rtcp(&rtcp) {
                self.session_protocol = SessionProtocol::Rtcp;
                self.contents = SessionPacket::Rtcp(rtcp);
                return;
            }
        }

        if let Some(rtp) = RtpPacket::build(self) {
            if is_rtp(&rtp) {
                self.session_protocol = SessionProtocol::Rtp;
                self.contents = SessionPacket::Rtp(rtp);
            }
        }
    }

    pub fn parse_as(&mut self, packet_type: SessionProtocol) {
        match packet_type {
            SessionProtocol::Rtp => {
                let Some(rtp) = RtpPacket::build(self) else {
                    return;
                };
                self.session_protocol = packet_type;
                self.contents = SessionPacket::Rtp(rtp);
            }
            SessionProtocol::Rtcp => {
                let Some(rtcp) = RtcpPacket::build(self) else {
                    return;
                };
                self.session_protocol = packet_type;
                self.contents = SessionPacket::Rtcp(rtcp);
            }
            SessionProtocol::Mpegts => {
                let Some(mpegts) = MpegtsPacket::build(self) else {
                    return;
                };
                self.session_protocol = packet_type;
                self.contents = SessionPacket::Mpegts(mpegts);
            }
            SessionProtocol::Stun => {
                let Some(stun) = StunPacket::build(self) else {
                    return;
                };
                self.session_protocol = packet_type;
                self.contents = SessionPacket::Stun(stun);
            }
            SessionProtocol::Unknown => {
                self.session_protocol = packet_type;
                self.contents = SessionPacket::Unknown;
            }
        }
    }

    pub fn save_human(path: &std::path::Path, packets: &[Packet]) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        for pkt in packets {
            writeln!(file, "{}", pkt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Example: timestamp in ms, source->dest, length, id, protocols
        let ts_ms = self
            .timestamp
            .as_secs()
            .saturating_mul(1000)
            .saturating_add(self.timestamp.subsec_millis() as u64);
        let creation_ms = match self.creation_time.duration_since(std::time::UNIX_EPOCH) {
            Ok(dur) => dur
                .as_secs()
                .saturating_mul(1000)
                .saturating_add(dur.subsec_millis() as u64),
            Err(_) => 0,
        };
        // payload length only? printing raw bytes might not be human-friendly:
        let payload_info = match &self.payload {
            Some(bytes) => format!("{} bytes", bytes.len()),
            None => "None".to_string(),
        };
        write!(
            f,
            "id={} ts_ms={} creation_ms={} len={} src={} dst={} proto={:?}/{:?} contents={:?} payload={}",
            self.id,
            ts_ms,
            creation_ms,
            self.length,
            self.source_addr,
            self.destination_addr,
            self.transport_protocol,
            self.session_protocol,
            self.contents,
            payload_info
        )
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn is_rtp(packet: &RtpPacket) -> bool {
    if packet.version != 2 {
        return false;
    }
    if let 72..=79 = packet.payload_type.id {
        return false;
    }
    if packet.ssrc == 0 {
        return false;
    }

    true
}

#[cfg(not(target_arch = "wasm32"))]
fn is_rtcp(packets: &[RtcpPacket]) -> bool {
    let Some(first) = packets.first() else {
        return false;
    };

    if !matches!(
        first,
        RtcpPacket::SenderReport(_)
            | RtcpPacket::ReceiverReport(_)
            | RtcpPacket::Goodbye(_)
            | RtcpPacket::PayloadSpecificFeedback
            | RtcpPacket::ReceiverEstimatedMaximumBitrate(_)
            | RtcpPacket::FullIntraRequest(_)
            | RtcpPacket::PictureLossIndication(_)
            | RtcpPacket::SliceLossIndication(_)
    ) {
        return false;
    }

    true
}

#[cfg(not(target_arch = "wasm32"))]
fn get_duration(raw_packet: &pcap::Packet) -> Duration {
    use std::ops::Add;

    // i64 -> u64, but seconds should never be negative
    let secs = raw_packet.header.ts.tv_sec.try_into().unwrap();
    let micrs = raw_packet.header.ts.tv_usec.try_into().unwrap();

    let sec_duration = Duration::from_secs(secs);
    let micros_duration = Duration::from_micros(micrs);

    sec_duration.add(micros_duration)
}
