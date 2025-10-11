use crate::webrtc::rtclog2::{IncomingRtcpPackets, OutgoingRtcpPackets};

pub enum RtcpPacketType {
    Incoming,
    Outgoing,
}

pub struct LogRtcpPacket {
    pub timestamp_ms: ::core::option::Option<i64>,
    pub raw_packet: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    pub number_of_deltas: ::core::option::Option<u32>,
    pub timestamp_ms_deltas: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    pub raw_packet_blobs: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    pub type_: RtcpPacketType,
}

impl LogRtcpPacket {
    pub fn from_incoming(packet: IncomingRtcpPackets) -> LogRtcpPacket {
        LogRtcpPacket{
            timestamp_ms: packet.timestamp_ms,
            raw_packet: packet.raw_packet,
            number_of_deltas: packet.number_of_deltas,
            timestamp_ms_deltas: packet.timestamp_ms_deltas,
            raw_packet_blobs: packet.raw_packet_blobs,
            type_: RtcpPacketType::Incoming,
        }
    }

    pub fn from_outgoing(packet: OutgoingRtcpPackets) -> LogRtcpPacket {
        LogRtcpPacket{
            timestamp_ms: packet.timestamp_ms,
            raw_packet: packet.raw_packet,
            number_of_deltas: packet.number_of_deltas,
            timestamp_ms_deltas: packet.timestamp_ms_deltas,
            raw_packet_blobs: packet.raw_packet_blobs,
            type_: RtcpPacketType::Outgoing,
        }
    }
}