use crate::app::common::types::TableConfig;
use crate::streams::mpegts_stream::substream::MpegtsSubStream;

pub struct StreamInfo {
    pub substream: MpegtsSubStream,
    pub id: String,
}
