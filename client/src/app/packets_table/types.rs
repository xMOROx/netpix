use netpix_common::packet::Packet;
use web_time::Duration;

pub struct PacketInfo<'a> {
    pub packet: &'a Packet,
    pub timestamp: Duration,
}
