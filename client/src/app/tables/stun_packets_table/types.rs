use crate::define_filter_context;
use netpix_common::{packet::SessionPacket, StunPacket};

define_filter_context!(StunFilterContext,
    packet: StunPacket,
    source_addr: str,
    destination_addr: str
);

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub id: u64,
    pub packet: &'a SessionPacket,
    pub stun_packet: &'a StunPacket,
} 