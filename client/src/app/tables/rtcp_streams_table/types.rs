use crate::define_filter_context;
use crate::streams::rtcp_stream::RtcpStream;

define_filter_context!(RtcpStreamFilterContext,
    stream: RtcpStream,
    source_addr: str,
    destination_addr: str
);
