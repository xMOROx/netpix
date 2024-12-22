use crate::app::common::types::TableConfig;
use crate::streams::mpegts_stream::packet_info::MpegTsPacketInfo;
use netpix_common::MpegtsStreamKey;
use web_time::Duration;

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub packet: &'a MpegTsPacketInfo,
    pub timestamp: Duration,
    pub key: MpegtsStreamKey,
}
