use crate::define_filter_context;
use netpix_common::{RtpPacket, RtpStreamKey, packet::SessionPacket};
use web_time::Duration;

define_filter_context!(RtpFilterContext,
    packet: RtpPacket,
    source_addr: str,
    destination_addr: str,
    alias: str
);

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub packet: &'a SessionPacket,
    pub timestamp: Duration,
    pub key: RtpStreamKey,
}
