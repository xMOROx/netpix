use crate::app::common::types::TableConfig;
use netpix_common::packet::SessionPacket;
use netpix_common::{RtpPacket, RtpStreamKey};
use web_time::Duration;
use crate::define_filter_context;

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
