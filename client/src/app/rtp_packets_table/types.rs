use crate::app::common::types::TableConfig;
use netpix_common::packet::SessionPacket;
use netpix_common::RtpStreamKey;
use web_time::Duration;

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub packet: &'a SessionPacket,
    pub timestamp: Duration,
    pub key: RtpStreamKey,
}
