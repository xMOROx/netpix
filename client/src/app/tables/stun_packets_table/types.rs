use netpix_common::{StunPacket, packet::Packet};

pub struct StunFilterContext<'a> {
    pub packet: &'a StunPacket,
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
}
