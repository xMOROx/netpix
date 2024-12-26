use crate::define_filter_context;
use netpix_common::{rtcp::*, Packet, RtcpPacket};

define_filter_context!(RtcpFilterContext,
    packet: RtcpPacket,
    source_addr: str,
    destination_addr: str
);

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub id: u64,
    pub packet: &'a Packet,
    pub rtcp_packet: &'a RtcpPacket,
    pub compound_index: usize,
}
