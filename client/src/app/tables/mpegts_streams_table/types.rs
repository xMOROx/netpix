use crate::{app::common::types::TableConfig, streams::mpegts_stream::substream::MpegtsSubStream};

pub struct StreamInfo {
    pub substream: MpegtsSubStream,
    pub id: String,
}
