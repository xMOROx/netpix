use crate::define_filter_context;
use crate::streams::rtpStream::RtpStream;

define_filter_context!(RtpStreamFilterContext,
    stream: RtpStream,
    alias: str,
    source_addr: str,
    destination_addr: str
);

pub struct SdpWindow {
    pub open: bool,
    pub sdp: String,
}

impl Default for SdpWindow {
    fn default() -> Self {
        Self {
            open: false,
            sdp: String::new(),
        }
    }
}
