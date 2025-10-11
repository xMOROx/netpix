use bincode::{
    config,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use std::fmt;
use std::str::FromStr;

pub use crate::mpegts::MpegtsPacket;
pub use crate::rtcp::RtcpPacket;
pub use crate::rtp::RtpPacket;
pub use crate::turn::ChannelData;
pub use packet::Packet;
pub use sdp::Sdp;

pub mod mpegts;
pub mod packet;
pub mod rtcp;
pub mod rtp;
pub mod sdp;
mod stream_keys;
pub mod turn;
pub mod utils;

pub use stream_keys::{MpegtsStreamKey, PacketAssociationTable, RtpStreamKey};

pub const PACKET_MAX_AGE_SECS: u64 = 120; // 2 minutes

#[derive(Decode, Encode, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Source {
    File(String),
    Interface(String),
}

impl Source {
    pub fn from_string(src_str: String) -> Option<Self> {
        let words: Vec<_> = src_str.split(' ').collect();

        if words.len() != 2 {
            return None;
        }

        let name = words.last().unwrap().to_string();

        match *words.first().unwrap() {
            "📁" => Some(Source::File(name)),
            "🌐" => Some(Source::Interface(name)),
            _ => None,
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (icon, name) = match self {
            Self::File(file) => ("📁", file),
            Self::Interface(interface) => ("🌐", interface),
        };

        write!(f, "{} {}", icon, name)
    }
}

#[derive(Encode, Decode, Debug, Clone)]
pub enum Request {
    FetchAll,
    Reparse(usize, packet::SessionProtocol),
    ChangeSource(Source),
    ParseSdp(RtpStreamKey, String),
    PacketsStats(PacketsStats),
}

#[derive(Decode, Encode, Debug, Clone)]
pub struct PacketsStats {
    pub discharged: usize,
    pub overwritten: usize,
}

#[derive(Decode, Encode, Debug, Clone)]
pub enum Response {
    Packet(Packet),
    Sources(Vec<Source>),
    Sdp(RtpStreamKey, Sdp),
    PacketsStats(PacketsStats),
}

impl Request {
    pub fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(bytes, config::standard())
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }
}

impl Response {
    pub fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodeError> {
        bincode::decode_from_slice(bytes, config::standard())
    }

    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Copy, Clone)]
pub enum SessionProtocol {
    Unknown,
    Rtp,
    Rtcp,
    Mpegts,
    Turn,
}

impl FromStr for SessionProtocol {
    type Err = ();
    fn from_str(p0: &str) -> Result<Self, Self::Err> {
        match p0.to_lowercase().as_str() {
            "unknown" => Ok(Self::Unknown),
            "rtp" => Ok(Self::Rtp),
            "rtcp" => Ok(Self::Rtcp),
            "mpeg-ts" => Ok(Self::Mpegts),
            "turn" => Ok(Self::Turn),
            _ => Err(()),
        }
    }
}

impl SessionProtocol {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Unknown,
            Self::Rtp,
            Self::Rtcp,
            Self::Mpegts,
            Self::Turn,
        ]
    }
}

impl fmt::Display for SessionProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Self::Unknown => "Unknown",
            Self::Rtp => "RTP",
            Self::Rtcp => "RTCP",
            Self::Mpegts => "MPEG-TS",
            Self::Turn => "TURN",
        };

        write!(f, "{}", res)
    }
}
