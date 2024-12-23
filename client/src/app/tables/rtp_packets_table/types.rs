use crate::{app::common::types::TableConfig, define_filter_context};
use netpix_common::{packet::SessionPacket, RtpPacket, RtpStreamKey};
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
