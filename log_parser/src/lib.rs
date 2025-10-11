pub mod parser;
mod bitstream;
mod types;

// webrtc.rtclog2
pub mod webrtc {
    pub mod rtclog2 {
        include!("gen/webrtc.rtclog2.rs");
    }
}