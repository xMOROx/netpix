use netpix_common::{packet::Packet, StunPacket};

pub struct StunFilterContext<'a> {
    pub packet: &'a StunPacket,
    pub source_addr: &'a str,
    pub destination_addr: &'a str,
}
