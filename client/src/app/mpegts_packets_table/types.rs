use crate::streams::mpegts_stream::MpegTsPacketInfo;
use netpix_common::MpegtsStreamKey;
use web_time::Duration;

#[derive(Clone)]
pub struct PacketInfo<'a> {
    pub packet: &'a MpegTsPacketInfo,
    pub timestamp: Duration,
    pub key: MpegtsStreamKey,
}

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub row_height: f32,
    pub header_height: f32,
    pub space_after_filter: f32,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            row_height: 25.0,
            header_height: 30.0,
            space_after_filter: 5.0,
        }
    }
}
